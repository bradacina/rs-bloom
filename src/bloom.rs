use crate::bitset::Bitset;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

pub struct Bloom
{
    bitset: Bitset
}

impl Bloom {
    pub fn new(num_bits: usize, num_hash_func: u8) -> Result<Bloom, &'static str> {
        let bitset = Bitset::new(num_bits)?;

        Ok(Bloom { bitset})
    }

    fn init_hash_funcs(&self) -> Vec<Box<Hasher>> {
        vec![Box::new(DefaultHasher::new())]
    }

    pub fn hash(&mut self, value: &[u8]) {
        for mut hasher in self.init_hash_funcs() {
            hasher.write(value);
            let hashed = hasher.finish();
            dbg!(hashed);
            dbg!(hashed as u8 as usize);
            self.bitset.set_bit(hashed as u8 as usize);
        }
    }

    pub fn contains(&self, value: &[u8]) -> Option<()> {
        for mut hasher in self.init_hash_funcs() {
            hasher.write(value);
            let hashed = hasher.finish();
            dbg!(hashed);
            dbg!(hashed as u8 as usize);
            if !self.bitset.get_bit(hashed as u8 as usize).unwrap() {
                return None;
            }
        }

        // it's a MAYBE (it could be a false positive)
        Some(())
    }
}