fn main() {
    //ENCRYPTION
    let s_box: [u8; 16] = [
        0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB,
        0x76,
    ];

    let input: [u8; 4] = [0x00, 0x03, 0x07, 0x0F];

    //substitution
    let substituted: [u8; 4] = input.map(|b| s_box[b as usize]);

    //permutation
    let permuted: [u8; 4] = [
        substituted[2],
        substituted[0],
        substituted[3],
        substituted[1],
    ];

    //XOR
    let round_key: [u8; 4] = [0xF0, 0x0F, 0xAA, 0x55];

    let encrypted: [u8; 4] = permuted
        .iter()
        .zip(round_key.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();

    println!("Step        | Byte 0 | Byte 1 | Byte 2 | Byte 3");
    println!("------------|--------|--------|--------|--------");
    println!(
        "Input       | {:02X}     | {:02X}     | {:02X}     | {:02X}",
        input[0], input[1], input[2], input[3]
    );
    println!(
        "Substituted | {:02X}     | {:02X}     | {:02X}     | {:02X}",
        substituted[0], substituted[1], substituted[2], substituted[3]
    );
    println!(
        "Permuted    | {:02X}     | {:02X}     | {:02X}     | {:02X}",
        permuted[0], permuted[1], permuted[2], permuted[3]
    );
    println!(
        "Encrypted   | {:02X}     | {:02X}     | {:02X}     | {:02X}",
        encrypted[0], encrypted[1], encrypted[2], encrypted[3]
    );

    //DECRYPTION
    let mut inverse_s_box = [0u8; 256];

    for (i, &val) in s_box.iter().enumerate() {
        inverse_s_box[val as usize] = i as u8;
    }

    let encrypted: [u8; 4] = [0x35, 0x6C, 0xDC, 0x2E];

    let xor_reversed: [u8; 4] = encrypted
        .iter()
        .zip(round_key.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();

    let permuted_reversed: [u8; 4] = [
        xor_reversed[1],
        xor_reversed[3],
        xor_reversed[0],
        xor_reversed[2],
    ];

    let decrypted: [u8; 4] = permuted_reversed.map(|b| inverse_s_box[b as usize]);

    println!(" ");
    println!("Step        | Byte 0 | Byte 1 | Byte 2 | Byte 3");
    println!("------------|--------|--------|--------|--------");
    println!(
        "Encrypted   | {:02X}     | {:02X}     | {:02X}     | {:02X}",
        encrypted[0], encrypted[1], encrypted[2], encrypted[3]
    );
    println!(
        "XOR Rev     | {:02X}     | {:02X}     | {:02X}     | {:02X}",
        xor_reversed[0], xor_reversed[1], xor_reversed[2], xor_reversed[3]
    );
    println!(
        "Perm Rev    | {:02X}     | {:02X}     | {:02X}     | {:02X}",
        permuted_reversed[0], permuted_reversed[1], permuted_reversed[2], permuted_reversed[3]
    );
    println!(
        "Decrypted   | {:02X}     | {:02X}     | {:02X}     | {:02X}",
        decrypted[0], decrypted[1], decrypted[2], decrypted[3]
    );
}
