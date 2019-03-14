pub struct Bitset{
   bits: Vec<u8>,
}

impl Bitset {
    pub fn new(num_bits: usize) -> Result<Bitset, &'static str> {
        if num_bits % 8 != 0 {
            return Err("num_bits must be divisible by 8");
        }

        let len = num_bits / 8;

        Ok(Bitset{
            bits: vec![0; len]
        })
    }

    pub fn get_bit(&self, idx: usize) -> Result<bool, &'static str> {
        if idx >= self.bits.len() * 8 {
            // todo: return error?
            return Err("idx is out of range");
        }

        let array_idx = idx >> 3;
        let bit_idx = idx & 0b111;

        let element = self.bits[array_idx];

        let result = match bit_idx {
            0 => element & 0b00000001,
            1 => element & 0b00000010,
            2 => element & 0b00000100,
            3 => element & 0b00001000,
            4 => element & 0b00010000,
            5 => element & 0b00100000,
            6 => element & 0b01000000,
            7 => element & 0b10000000,
            _ => return Err("this line should never be reached")
        } > 0;

        Ok(result)
    }

    pub fn set_bit(&mut self, idx: usize) {
        if idx >= self.bits.len() * 8 {
            // todo: return error?
            return;
        }

        let array_idx = idx >> 3;
        let bit_idx = idx & 0b111;

        let mut element = self.bits[array_idx];

        element = match bit_idx {
            0 => element | 0b00000001,
            1 => element | 0b00000010,
            2 => element | 0b00000100,
            3 => element | 0b00001000,
            4 => element | 0b00010000,
            5 => element | 0b00100000,
            6 => element | 0b01000000,
            7 => element | 0b10000000,
            _ => element
        };

        self.bits[array_idx] = element;
    }

    pub fn len(&self) -> usize {
        self.bits.len() * 8
    }
}