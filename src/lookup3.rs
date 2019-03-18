/// hash_size returns the number of elements that n bits addressing can cover
/// 
/// # arguments
/// * `n` the size of the hash in number of bits
#[inline]
pub fn hash_size(n: u8) -> u32 {
    1 << n
}


/// hash_mask generates a binary mask for a hash of size n
/// 
/// # arguments
/// * `n` the size of the hash in number of bits
#[inline]
pub fn hash_mask(n: u8) -> u32 {
    hash_size(n) - 1
}

#[inline]
pub fn rot(x: u32, k: u8) -> u32 {
    x << k | x >> (32 - k)
}

#[inline]
pub fn mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a -= *c;
    *a ^= rot(*c, 4);
    *c += *b;
    *b -= *a;
    *b ^= rot(*a, 6);
    *a += *c;
    *c -= *b;
    *c ^= rot(*b, 8);
    *b += *a;
    *a -= *c;
    *a ^= rot(*c, 16);
    *c += *b;
    *b -= *a;
    *b ^= rot(*a, 19);
    *a += *c;
    *c -= *b;
    *c ^= rot(*b, 4);
    *b += *a;
}

#[inline]
pub fn final_(a: &mut u32, b: &mut u32, c: &mut u32) {
    *c ^= *b;
    *c -= rot(*b, 14);
    *a ^= *c;
    *a -= rot(*c, 11);
    *b ^= *a;
    *b -= rot(*a, 25);
    *c ^= *b;
    *c -= rot(*b, 16);
    *a ^= *c;
    *a -= rot(*c, 4);
    *b ^= *a;
    *b -= rot(*a, 14);
    *c ^= *b;
    *c -= rot(*b, 24);
}

/// Hashes an array of u32's
///
/// # Arguments
/// * `k` - the key, an array of u32 values
/// * `init_val` - the previous hash or an arbitrary value
#[inline]
pub fn hashword(k: &[u32], init_val: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut length = k.len() as u32;
    let mut k = k;

    /* Set up the internal state */
    a = 0xdeadbeef + (length << 2) + init_val;
    b = a;
    c = a;

    /*------------------------------------------------- handle most of the key */
    while length > 3 {
        a += k[0];
        b += k[1];
        c += k[2];
        mix(&mut a, &mut b, &mut c);
        length -= 3;
        k = &k[3..];
    }

    /*------------------------------------------- handle the last 3 u32's */
    match length {
        3 => {
            c += k[2];
            b += k[1];
            a += k[0];
        }
        2 => {
            b += k[1];
            a += k[0];
        }
        1 => a += k[0],
        _ => (),
    }

    final_(&mut a, &mut b, &mut c);
    c
}

/// Hashword2 hashes a slice of u32 values taking as input 2 seeds
/// and outputs 2 independent hashes
///
/// # Arguments
/// * `k` - the key, a slice of u32 values
/// * `pc` - primary seed
/// * `pb` - secondary seed
#[inline]
pub fn hashword2(k: &[u32], pc: u32, pb: u32) -> (u32, u32) {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut length = k.len() as u32;
    let mut k = k;

    /* Set up the internal state */
    a = 0xdeadbeef + (length << 2) + pc;
    b = a;
    c = a;
    c += pb;

    /*------------------------------------------------- handle most of the key */
    while length > 3 {
        a += k[0];
        b += k[1];
        c += k[2];
        mix(&mut a, &mut b, &mut c);
        length -= 3;
        k = &k[3..];
    }

    /*------------------------------------------- handle the last 3 uint32_t's */
    match length {
        3 => {
            c += k[2];
            b += k[1];
            a += k[0];
        }
        2 => {
            b += k[1];
            a += k[0];
        }
        1 => a += k[0],
        _ => (),
    }

    final_(&mut a, &mut b, &mut c);

    /*------------------------------------------------------ report the result */
    (c, b)
}

#[inline]
pub fn hashlittle(key: &[u8], init_val: u32) -> u32 {
  let mut a: u32 = 0;
  let mut b: u32 = 0;
  let mut c: u32 = 0;
  let mut length = key.len() as u32;
  let is_little_endian = cfg!(target_endian="little");

  /* Set up the internal state */
  a = 0xdeadbeef + length + init_val;

  let alignment = &key[0] as *const u8 as u32;
  
  if is_little_endian && ((alignment & 0x3) == 0) {
    let mut k: &[u32] = unsafe {std::mem::transmute(key)}; // read 32 bit chunks

    /*------ all but last block: aligned reads and affect 32 bits of (a,b,c) */
    while length > 12
    {
      a += k[0];
      b += k[1];
      c += k[2];
      mix(&mut a,&mut b,&mut c);
      length -= 12;
      k = &k[3..];
    }

    /*----------------------------- handle the last (probably partial) block */
    /* 
     * "k[2]&0xffffff" actually reads beyond the end of the string, but
     * then masks off the part it's not allowed to read.  Because the
     * string is aligned, the masked-off tail is in the same word as the
     * rest of the string.  Every machine with memory protection I've seen
     * does it on word boundaries, so is OK with this.  But VALGRIND will
     * still catch it and complain.  The masking trick does make the hash
     * noticably faster for short strings (like English words).
     */

    match length {
    12 => {c+=k[2]; b+=k[1]; a+=k[0];}
    11 => {c+=k[2]&0xffffff; b+=k[1]; a+=k[0];}
    10 => {c+=k[2]&0xffff; b+=k[1]; a+=k[0];}
    9 => {c+=k[2]&0xff; b+=k[1]; a+=k[0];}
    8 => {b+=k[1]; a+=k[0];}
    7 => {b+=k[1]&0xffffff; a+=k[0];}
    6 => {b+=k[1]&0xffff; a+=k[0];}
    5 => {b+=k[1]&0xff; a+=k[0];}
    4 => {a+=k[0];}
    3 => a+=k[0]&0xffffff,
    2 => a+=k[0]&0xffff,
    1 => a+=k[0]&0xff,
    _ => return c
    }

  } else if is_little_endian && ((alignment & 0x1) == 0) {
    let mut k: &[u16] = unsafe {std::mem::transmute(key)};

    /*--------------- all but last block: aligned reads and different mixing */
    while length > 12
    {
      a += (k[0] as u32) + ((k[1] as u32)<<16);
      b += (k[2] as u32) + ((k[3] as u32)<<16);
      c += (k[4] as u32) + ((k[5] as u32)<<16);
      mix(&mut a,&mut b,&mut c);
      length -= 12;
      k = &k[6..];
    }

    /*----------------------------- handle the last (probably partial) block */
    let k8: &[u8] = unsafe {std::mem::transmute(k)};
    match length {
    12 => { c+=(k[4] as u32)+((k[5] as u32)<<16);
             b+=(k[2] as u32)+((k[3] as u32)<<16);
             a+=(k[0] as u32)+((k[1] as u32)<<16);
    }
    11 => {c+=(k8[10] as u32)<<16;
    b+=(k[2] as u32)+((k[3] as u32)<<16);
             a+=(k[0] as u32)+((k[1] as u32)<<16);}
    10 => {c+=k[4] as u32;
             b+=(k[2] as u32)+((k[3] as u32)<<16);
             a+=(k[0] as u32)+((k[1] as u32)<<16);
    }
     9 => {c+=k8[8] as u32;
     b+=(k[2] as u32)+((k[3] as u32)<<16);
             a+=(k[0] as u32)+((k[1] as u32)<<16);}
    8 => {b+=(k[2] as u32)+((k[3] as u32)<<16);
             a+=(k[0] as u32)+((k[1] as u32)<<16);}
    7 => {b+=(k8[6] as u32)<<16;
    b+=k[2] as u32;
             a+=(k[0] as u32)+((k[1] as u32)<<16);}
     6 => {b+=k[2] as u32;
             a+=(k[0] as u32)+((k[1] as u32)<<16);
    }
    5 => {b+=k8[4] as u32;a+=(k[0] as u32)+((k[1] as u32)<<16);}
    4 => {a+=(k[0] as u32)+((k[1] as u32)<<16);}
    3 => {a+=(k8[2] as u32)<<16;a+=k[0] as u32;}
    2 => a+=k[0] as u32,
    1 => a+=k8[0] as u32,
    _ => return c
    }

  } else {                        /* need to read the key one byte at a time */
    let mut k: &[u8] = unsafe {std::mem::transmute(key)};

    /*--------------- all but the last block: affect some 32 bits of (a,b,c) */
    while length > 12
    {
      a += k[0] as u32;
      a += (k[1] as u32)<<8;
      a += (k[2] as u32)<<16;
      a += (k[3] as u32)<<24;
      b += k[4] as u32;
      b += (k[5] as u32)<<8;
      b += (k[6] as u32)<<16;
      b += (k[7] as u32)<<24;
      c += k[8] as u32;
      c += (k[9] as u32)<<8;
      c += (k[10] as u32)<<16;
      c += (k[11] as u32)<<24;
      mix(&mut a,&mut b,&mut c);
      length -= 12;
      k = &k[12..];
    }

    /*-------------------------------- last block: affect all 32 bits of (c) */
    match length {
    12 => {
      c+=(k[11] as u32)<<24;
      c+=(k[10] as u32)<<16;
      c+=(k[9] as u32)<<8;
      c+=k[8] as u32;
      b+=(k[7] as u32)<<24;
      b+=(k[6] as u32)<<16;
      b+=(k[5] as u32)<<8;
      b+=k[4] as u32;
      a+=(k[3] as u32)<<24;
      a+=(k[2] as u32)<<16;
      a+=(k[1] as u32)<<8;
      a+=k[0] as u32;
    }
    
    11 =>{ c+=(k[10] as u32)<<16;
    c+=(k[9] as u32)<<8;
      c+=k[8] as u32;
      b+=(k[7] as u32)<<24;
      b+=(k[6] as u32)<<16;
      b+=(k[5] as u32)<<8;
      b+=k[4] as u32;
      a+=(k[3] as u32)<<24;
      a+=(k[2] as u32)<<16;
      a+=(k[1] as u32)<<8;
      a+=k[0] as u32;}
    
    10 => {c+=(k[9] as u32)<<8;
    c+=k[8] as u32;
      b+=(k[7] as u32)<<24;
      b+=(k[6] as u32)<<16;
      b+=(k[5] as u32)<<8;
      b+=k[4] as u32;
      a+=(k[3] as u32)<<24;
      a+=(k[2] as u32)<<16;
      a+=(k[1] as u32)<<8;
      a+=k[0] as u32;}
    
    9 => {c+=k[8] as u32;
    b+=(k[7] as u32)<<24;
      b+=(k[6] as u32)<<16;
      b+=(k[5] as u32)<<8;
      b+=k[4] as u32;
      a+=(k[3] as u32)<<24;
      a+=(k[2] as u32)<<16;
      a+=(k[1] as u32)<<8;
      a+=k[0] as u32;}
    
    8 => {b+=(k[7] as u32)<<24;
    b+=(k[6] as u32)<<16;
      b+=(k[5] as u32)<<8;
      b+=k[4] as u32;
      a+=(k[3] as u32)<<24;
      a+=(k[2] as u32)<<16;
      a+=(k[1] as u32)<<8;
      a+=k[0] as u32;}
    
    7 => {b+=(k[6] as u32)<<16;
    b+=(k[5] as u32)<<8;
      b+=k[4] as u32;
      a+=(k[3] as u32)<<24;
      a+=(k[2] as u32)<<16;
      a+=(k[1] as u32)<<8;
      a+=k[0] as u32;}
    
    6 => {b+=(k[5] as u32)<<8;
    b+=k[4] as u32;
      a+=(k[3] as u32)<<24;
      a+=(k[2] as u32)<<16;
      a+=(k[1] as u32)<<8;
      a+=k[0] as u32;}
    
    5 => {b+=k[4] as u32;
    a+=(k[3] as u32)<<24;
      a+=(k[2] as u32)<<16;
      a+=(k[1] as u32)<<8;
      a+=k[0] as u32;}
    
    4 => {a+=(k[3] as u32)<<24;
    a+=(k[2] as u32)<<16;
      a+=(k[1] as u32)<<8;
      a+=k[0] as u32;}
    
    3 => {a+=(k[2] as u32)<<16;
    a+=(k[1] as u32)<<8;
      a+=k[0] as u32;}
    
    2 => {a+=(k[1] as u32)<<8;
      a+=k[0] as u32;}

    1 => a+=k[0] as u32,
    _ => return c
    }
  }

  final_(&mut a,&mut b,&mut c);
  c
}