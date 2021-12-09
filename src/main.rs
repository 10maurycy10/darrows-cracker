use data_encoding::HEXLOWER;
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::{env,io};
use std::io::BufRead;

const HEXTABLE: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

mod test;

const CHARSET_LOWER:  &[u8] = b"qwertyuiopasdfghjklzxcvbnm";
const CHARSET_UPPER:  &[u8] = b"QWERTYUIOPASDFGHJKLZXCVBNM";
const CHARSET_HEXUPPER:  &[u8] = b"0123456789ABCDEF";
const CHARSET_HEXLOWER:  &[u8] = b"0123456789abcdef";
const CHARSET_LETTER: &[u8] = b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
const CHARSET_SYM:    &[u8] = b"`~!@#$%^&*()-_=+{[}]|\\;':\",./<>?";
const CHARSET_ZERO:    &[u8] = b"`/!69";
const CHARSET_DIGIT:  &[u8] = b"1234567890";
const CHARSET_ALL:    &[u8] = b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM`~!@#$%^&*()-_=+{[}]|\\;':\",./<>?123467890";
const CHARSET_QMARK:  &[u8] = b"?";

pub fn mask_to_charsets(mask: &str) -> Vec<Vec<u8>> {
    let mut mask = mask.chars();
    let mut charsets = Vec::new();
    
    loop {
        match mask.next() {
            None => break,
            Some(c) => match (c) {
                '?' => match mask.next() {
                    Some(charset) => match charset {
                        'l' => charsets.push(CHARSET_LOWER.to_vec()),
                        'L' => charsets.push(CHARSET_LETTER.to_vec()),
                        'u' => charsets.push(CHARSET_UPPER.to_vec()),
                        'h' => charsets.push(CHARSET_HEXLOWER.to_vec()),
                        'H' => charsets.push(CHARSET_HEXUPPER.to_vec()),
                        's' => charsets.push(CHARSET_SYM.to_vec()),
                        'd' => charsets.push(CHARSET_DIGIT.to_vec()),
                        'a' => charsets.push(CHARSET_ALL.to_vec()),
                        'z' => charsets.push(CHARSET_ZERO.to_vec()),
                        '?' => charsets.push(CHARSET_QMARK.to_vec()),
                        _ => panic!("'{}' is not a valid charset",charset),
                    }
                    None => panic!("syntax error in mask"),
                },
                a => charsets.push(vec![a as u8])
            }
        }
    };
    
    return charsets
}

// convert and copy bytes to hex
// this is faster that encoding for some reson!
#[inline]
fn hex_cpy(input: &[u8; 32], output: &mut [u8; 64]) {
    unsafe {
        for i in 0..32 {
            let bin = input.get_unchecked(i);
            let out1 = output.get_unchecked_mut(i * 2);
            *out1 = HEXTABLE[(bin / 16) as usize];
            drop(out1);
            let out2 = output.get_unchecked_mut(i * 2 + 1);
            *out2 = HEXTABLE[(bin % 16) as usize];
        }
    }
}

#[inline]
// concatinate and hash w/ SHA256
pub fn hashab(a: &[u8], b: &[u8], outbuf: &mut [u8; 32]) {
    let mut context = Sha256::new();
    context.update(a);
    context.update(b);
    let s = context.finalize();
    unsafe {
        for i in 0..32 {
            *outbuf.get_unchecked_mut(i) = *s.get_unchecked(i);
        }
    }
}

#[inline]
// concatinate and hash w/ SHA256
pub fn hashabc(a: &[u8], b: &[u8], c: &[u8], outbuf: &mut [u8; 32]) {
    let mut context = Sha256::new();
    context.update(a);
    context.update(b);
    context.update(c);
    let s = context.finalize();
    unsafe {
        for i in 0..32 {
            *outbuf.get_unchecked_mut(i) = *s.get_unchecked(i);
        }
    }
}

#[inline]
// hash a password with a salt -> outbuf, with the darrows algorithm
//
// Theh hash is SHA256(HEX(SHA256($pass:HEX(SHA256($pass:$salt))))
//
pub fn genhash(pass: &[u8], pass2: &[u8], salt: &[u8], outbuf: &mut [u8; 32]) {
    let mut result_hash_0 = [0; 32];
    hashabc(pass, pass2, salt, &mut result_hash_0);
    let mut result_hash_0_hex = [0; 32 * 2];
    // we have to convert the hash to hex becuse of @ZeroTix's bad code
    hex_cpy(&result_hash_0, &mut result_hash_0_hex);

    let mut result_hash_1 = [0; 32];
    hashabc(pass, pass2, &result_hash_0_hex, &mut result_hash_1);
    let mut result_hash_1_hex = [0; 32 * 2];
    hex_cpy(&result_hash_1, &mut result_hash_1_hex);

    let _result_hash_2 = [0; 32];
    hashab(&[], &result_hash_1_hex, outbuf);
}

fn main() {
    let mut args = env::args().skip(1);
    
    let hash = env::args().skip(1).next().expect("you must specify a hash!");
    
    let mask = env::args().skip(1).next().unwrap_or("".to_string());
    
    let salt = env::args().skip(1).next()
        .unwrap_or("fc8877c24d85d246e3234f2dcca3a33a842c32f81b6a8f7f60696da988a1fea4".to_string());

    let amp_charsets = mask_to_charsets(&mask);
    
    let amp = amp_charsets.as_slice();
        
    // real hash for dev!!!
    let hash_tgt: &[u8] = hash.as_bytes();

    // the salt is concationated as hex
    let hash_salt: &[u8] = salt.as_bytes();

    // decode hash to binary
    let mut tgt_raw_hash = [0; 32];
    let amp_len = amp.len();
    HEXLOWER.decode_mut(hash_tgt, &mut tgt_raw_hash).expect("invalid hash");
    // shadow hash_tgt
    let hash_tgt = tgt_raw_hash;

    if amp_len > 64 {
        panic!("the amplifer is to long! {} is larger that the max of 64!",amp_len)
    }
    
    if amp_len == 0 {
        println!("ATTENTION: you have not specifyed an amplifyer, this will slow down cracking on multicore systems.")
    }
    
    println!("begining cracking");

    let mut counter = 0_usize;

    loop {
        // get 1000 password candidates from stdin
        let candidates: Vec<String> = io::stdin()
            .lock()
            .lines()
            .take(1000)
            .map(|x| x.unwrap())
            .collect();

        if candidates.len() == 0 {
            println!("exahsted!");
            return;
        }

        counter = counter + candidates.len();

        candidates.iter().par_bridge().for_each(|candidate| {
            let mut idx = [0; 64];
            unsafe { loop {
                // calculate next candiate
                let mut i = 0;
                'carry: loop {
                    // edge case for empty amplifyer
                    if (amp_len==0) {
                        break 'carry;
                    }
                    *idx.get_unchecked_mut(i)+=1;
                    if idx.get_unchecked(i)<&amp.get_unchecked(i).len() {
                        break 'carry;
                    }
                    *idx.get_unchecked_mut(i) = 0;
                    i+=1;
                    if i == amp_len {return;}
                }
                
                let mut additional = [0_u8; 64];
                for i in 0..amp_len {
                    *additional.get_unchecked_mut(i) = *amp.get_unchecked(i).get_unchecked(*idx.get_unchecked(i));
                }
                let mut result_hash = [0; 32];
                genhash(candidate.as_bytes(), &additional.get_unchecked(0..amp_len), hash_salt, &mut result_hash);
                if result_hash == hash_tgt {
                    println!("KEY {}: {}{}", counter, candidate, std::str::from_utf8(&additional[0..amp_len]).unwrap())
                }
                // edge case for empty amplifyer
                if (amp_len==0) {return}
            } }
         })
    }
}
