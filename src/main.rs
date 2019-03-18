mod bitset;
mod bloom;
mod lookup3;

use bloom::Bloom;
use rand::prelude::*;

fn main() {
    let a = 1;
    let c = [1,2,3];
    let d = 'a';
    let e = 'b';
    let f = 12u8;
    let g = 12u8;
    let h = 12u8;
    let i = 12u8;
    let z = vec![12u8, 13u8, 14u8];
    let x = &z[1..];

    println!("{:?}", &a as *const i32);
    println!("{:?}", &c as *const i32);
    println!("{:?}", &d as *const char);
    println!("{:?}", &e as *const char);
    println!("{:?}", &f as *const u8);
    println!("{:?}", &g as *const u8);
    println!("{:?}", &h as *const u8);
    println!("{:?}", &i as *const u8);
    println!("{:?}", &z[0] as *const u8);
    println!("{:?}", &x[0] as *const u8);


    let xx: u32 = &x[0] as *const u8 as u32;
    let zz: u32 = &z[0] as *const u8 as u32;

    println!("{}", xx & 0x3);
    println!("{}", zz & 0x3);
}

fn old_main() {
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