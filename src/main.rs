mod bitset;
mod bloom;

use bloom::Bloom;
use rand::prelude::*;

fn main() {
    let mut seen = vec![];
    let mut b = Bloom::new(256,10).unwrap();
    let mut rng = rand::thread_rng();
    for _ in 0..300 {
        let word = make_word(&mut rng);
        b.hash(word.as_bytes());
        seen.push(word);
    }

    for w in seen {
        if b.contains(w.as_bytes()).is_none() {
            println!("houston we have a problem")
        }
    }
}

fn make_word(rng: &mut ThreadRng) -> String {
    let mut s = String::new();
    for _ in 0..5 {
        s.push(rng.gen_range(33 as u8, 128 as u8) as char);
    }

    println!("Generated {}", s);
    s
}