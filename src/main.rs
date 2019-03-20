mod bitset;
mod bloom;
mod lookup3;

use bloom::Bloom;
use rand::prelude::*;

fn main() {
    driver5();
}

fn driver5() {
  let mut c : u32;
  c=0; 
  c = lookup3::hashlittle(b"",c);

  println!("hash1 is {:x}", c);   /* deadbeef */
  
  c=0xdeadbeef; 
  c = lookup3::hashlittle(b"", c);
  println!("hash is {:x}", c);   /* bd5b7dde */

  c = 0;
  c = lookup3::hashlittle(b"Four score and seven years ago",c);
  println!("hash is {:x}", c);   /* 17770551 */


c = 1;
  c = lookup3::hashlittle(b"Four score and seven years ago",c);
  println!("hash is {:x}", c);   /* e3607cae */
  
//   b=0xdeadbeef, c=0xdeadbeef, hashlittle2("", 0, &c, &b);
//   printf("hash is %.8lx %.8lx\n", c, b);   /* 9c093ccd bd5b7dde */
//   b=0, c=0, hashlittle2("Four score and seven years ago", 30, &c, &b);
//   printf("hash is %.8lx %.8lx\n", c, b);   /* 17770551 ce7226e6 */
//   b=1, c=0, hashlittle2("Four score and seven years ago", 30, &c, &b);
//   printf("hash is %.8lx %.8lx\n", c, b);   /* e3607cae bd371de4 */
//   b=0, c=1, hashlittle2("Four score and seven years ago", 30, &c, &b);
//   printf("hash is %.8lx %.8lx\n", c, b);   /* cd628161 6cbea4b3 */
//   c = hashlittle("Four score and seven years ago", 30, 0);
//   printf("hash is %.8lx\n", c);   /* 17770551 */
//   c = hashlittle("Four score and seven years ago", 30, 1);
//   printf("hash is %.8lx\n", c);   /* cd628161 */
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