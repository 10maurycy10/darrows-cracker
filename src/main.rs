use rayon::prelude::*;
use sha2::{Sha256, Sha512, Digest};
use data_encoding::HEXLOWER;
use std::convert::TryInto;
use std::hint::unreachable_unchecked;
use std::io;
use std::fs::File;
use std::io::BufRead;

const HEXTABLE: [u8; 16] = [b'0',b'1',b'2',b'3',b'4',b'5',b'6',b'7', b'8', b'9',b'a',b'b',b'c',b'd',b'e',b'f'];

// this is faster that encoding for some reson!
#[inline]
fn hex_cpy(input: &[u8; 32], output: &mut [u8; 64]) {
    unsafe {
        for i in 0..32 {
            let bin = input.get_unchecked(i);
            let out1 = output.get_unchecked_mut(i*2);
            *out1 = HEXTABLE[(bin/16) as usize];
            drop(out1);
            let out2 = output.get_unchecked_mut(i*2 + 1);
            *out2 = HEXTABLE[(bin%16) as usize];
        }
    }
}

#[inline]
fn hashab(a: &[u8],b: &[u8], outbuf: &mut [u8; 32]) {
    let mut context = Sha256::new();
    context.update(a);
    context.update(b);
    let s = context.finalize();
    unsafe {
        for i in (0..32) {
            *outbuf.get_unchecked_mut(i) = *s.get_unchecked(i);
        }
    }
     
}

#[inline]
fn genhash(pass: &[u8], salt: &[u8], outbuf: &mut [u8; 32]) {
    let mut result_hash_0 = [0; 32];
    hashab(pass,salt,&mut result_hash_0);
    let mut result_hash_0_hex = [0; 32*2];    
    hex_cpy(&result_hash_0, &mut result_hash_0_hex);
    
    let mut result_hash_1 = [0; 32];
    hashab(pass,&result_hash_0_hex,&mut result_hash_1);
    let mut result_hash_1_hex = [0; 32*2];
    hex_cpy(&result_hash_1, &mut result_hash_1_hex);
    
    let mut result_hash_2 = [0; 32];
    hashab(&[],&result_hash_1_hex,outbuf);
}

#[test]
fn test_ab() {
    let testab_hash = b"0d6be69b264717f2dd33652e212b173104b4a647b7c11ae72e9885f11cd312fb";
    let testab_a = b"pass";
    let testab_b = b"wd";
    let mut testab_raw_hash = [0; 32];
    HEXLOWER.decode_mut(testab_hash,&mut testab_raw_hash).unwrap_or_else(|_| unsafe {unreachable_unchecked()});
    
    let mut result_hash = [0; 32];
    
    hashab(testab_a,testab_b,&mut result_hash);
    
    assert_eq!(result_hash,testab_raw_hash);
}

#[test]
fn test_genhash() {
    let test_hash = b"5447b9c4482c90e26c4f132bcbaa180459a345049a89de82ab880eb582c7ff2b";
    let test_pass = b"passwd";
    let test_salt = b"fc8877c24d85d246e3234f2dcca3a33a842c32f81b6a8f7f60696da988a1fea4";
    let mut test_raw_hash = [0; 32];
    HEXLOWER.decode_mut(test_hash,&mut test_raw_hash).unwrap_or_else(|_| unsafe {unreachable_unchecked()});
    
    let mut result_hash = [0; 32];
    
    genhash(test_pass,test_salt,&mut result_hash);
    
    assert_eq!(result_hash,test_raw_hash);
}

fn main() {
    // real hash for dev!!!
    let hash_tgt = b"aaf450a516f3ba08fceaa80687bb5c0147a515a634464517026cfcae3adea8a9";
    
    // TEST HASHES
    //let hash_tgt = b"5447b9c4482c90e26c4f132bcbaa180459a345049a89de82ab880eb582c7ff2b";
    
    //let hash_tgt = b"0e67594567b12b5dff7924a39e44e2defebc9d53600d9ad34e2dcadc814a47d7";
    
    let hash_salt = b"fc8877c24d85d246e3234f2dcca3a33a842c32f81b6a8f7f60696da988a1fea4";
    
    let mut tgt_raw_hash = [0; 32];
    HEXLOWER.decode_mut(hash_tgt,&mut tgt_raw_hash).unwrap_or_else(|_| unsafe {unreachable_unchecked()});
    let hash_tgt = tgt_raw_hash;
    
    let candidates: Vec<String>  = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    
    println!("begining cracking");
    
    let cracked: Vec<&String> = 
        candidates
        //.map(|l| l.unwrap_or_else(|_| unsafe {unreachable_unchecked()}))
        .iter()
        .par_bridge()
        .filter(|candidate| {
            let mut result_hash = [0; 32];
            genhash(candidate.as_bytes(),hash_salt,&mut result_hash);
            return result_hash == hash_tgt
        })
        .collect();
    
    println!("{:?}",cracked)
}
