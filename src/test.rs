use super::*;

#[test]
fn test_ab() {
    // the sha256 hash of "passwd" in hex
    let testab_hash = b"0d6be69b264717f2dd33652e212b173104b4a647b7c11ae72e9885f11cd312fb";
    let testab_a = b"pass";
    let testab_b = b"wd";

    // buffer for binay form of hash
    let mut testab_raw_hash = [0; 32];
    HEXLOWER
        .decode_mut(testab_hash, &mut testab_raw_hash)
        .unwrap();

    let mut result_hash = [0; 32];
    hashab(testab_a, testab_b, &mut result_hash);

    // the computed hash must match sample hash
    assert_eq!(result_hash, testab_raw_hash);
}

#[test]
fn test_abc() {
    // the sha256 hash of "passwd" in hex
    let testab_hash = b"0d6be69b264717f2dd33652e212b173104b4a647b7c11ae72e9885f11cd312fb";
    
    // buffer for binay form of hash
    let mut testab_raw_hash = [0; 32];
    HEXLOWER
        .decode_mut(testab_hash, &mut testab_raw_hash)
        .unwrap();

    let mut result_hash = [0; 32];
    hashabc(b"pas", b"s", b"wd", &mut result_hash);

    // the computed hash must match sample hash
    assert_eq!(result_hash, testab_raw_hash);
}

#[test]
fn test_genhash() {
    // the hash generated by darrows when "passwd" is sent in chat
    let test_hash = b"5447b9c4482c90e26c4f132bcbaa180459a345049a89de82ab880eb582c7ff2b";
    let test_pass = b"pa";
    // the current salt for darrows
    let test_salt = b"fc8877c24d85d246e3234f2dcca3a33a842c32f81b6a8f7f60696da988a1fea4";
    let mut test_raw_hash = [0; 32];
    HEXLOWER.decode_mut(test_hash, &mut test_raw_hash).unwrap();

    let mut result_hash = [0; 32];

    genhash(test_pass, b"sswd", test_salt, &mut result_hash);

    assert_eq!(result_hash, test_raw_hash);
}

#[test]
fn test_mask() {
    let test_mask = "?l?L?a?z?H?h?d?sa".to_string();

    #[rustfmt::skip]
    assert_eq!(mask_to_charsets(&test_mask), vec![vec![113, 119, 101, 114, 116, 121, 117, 105, 111, 112, 97, 115, 100, 102, 103, 104, 106, 107, 108, 122, 120, 99, 118, 98, 110, 109], vec![113, 119, 101, 114, 116, 121, 117, 105, 111, 112, 97, 115, 100, 102, 103, 104, 106, 107, 108, 122, 120, 99, 118, 98, 110, 109, 81, 87, 69, 82, 84, 89, 85, 73, 79, 80, 65, 83, 68, 70, 71, 72, 74, 75, 76, 90, 88, 67, 86, 66, 78, 77], vec![113, 119, 101, 114, 116, 121, 117, 105, 111, 112, 97, 115, 100, 102, 103, 104, 106, 107, 108, 122, 120, 99, 118, 98, 110, 109, 81, 87, 69, 82, 84, 89, 85, 73, 79, 80, 65, 83, 68, 70, 71, 72, 74, 75, 76, 90, 88, 67, 86, 66, 78, 77, 96, 126, 33, 64, 35, 36, 37, 94, 38, 42, 40, 41, 45, 95, 61, 43, 123, 91, 125, 93, 124, 92, 59, 39, 58, 34, 44, 46, 47, 60, 62, 63, 49, 50, 51, 52, 54, 55, 56, 57, 48], vec![96, 47, 33, 54, 57], vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70], vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102], vec![49, 50, 51, 52, 53, 54, 55, 56, 57, 48], vec![96, 126, 33, 64, 35, 36, 37, 94, 38, 42, 40, 41, 45, 95, 61, 43, 123, 91, 125, 93, 124, 92, 59, 39, 58, 34, 44, 46, 47, 60, 62, 63], vec![97]]);
}