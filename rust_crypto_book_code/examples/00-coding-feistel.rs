fn main() {
    let left = 0b1010_1010; //178
    let right = 0b0101_0101; //85
    let key = 0b1111_0000; //240

    let (l2, r2) = feistel_encrypt(left, right, key);
    println!("Encrypted: {:08b} {:08b}", l2, r2);

    let (l1, r1) = feistel_decrypt(l2, r2, key);
    println!("Decrypted: {:08b} {:08b}", l1, r1);
}

fn feistel_encrypt(l1: u8, r1: u8, key: u8) -> (u8, u8) {
    //round function
    let f = r1 ^ key;
    //new left = old right
    //new right = old left XOR round function
    (r1, l1 ^ f)
}

fn feistel_decrypt(l2: u8, r2: u8, key: u8) -> (u8, u8) {
    //l1 = r2 XORf(l2, key)
    //r1 - l2
    let f = l2 ^ key;
    let l1 = r2 ^ f;
    (l1, l2)
}
