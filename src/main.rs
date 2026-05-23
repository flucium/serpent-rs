// PHI: φ
const PHI: u32 = 0x9E37_79B9;

// Block size
const BLOCK_SIZE: usize = 16;

const SBOX: [[u8; 16]; 8] = [
    [3, 8, 15, 1, 10, 6, 5, 11, 14, 13, 4, 2, 7, 0, 9, 12],
    [15, 12, 2, 7, 9, 0, 5, 10, 1, 11, 14, 8, 6, 13, 3, 4],
    [8, 6, 7, 9, 3, 12, 10, 15, 13, 1, 14, 4, 0, 11, 5, 2],
    [0, 15, 11, 8, 12, 9, 6, 3, 13, 1, 2, 4, 10, 7, 5, 14],
    [1, 15, 8, 3, 12, 0, 11, 6, 2, 5, 4, 10, 9, 14, 7, 13],
    [15, 5, 2, 11, 4, 10, 9, 12, 0, 3, 14, 8, 13, 6, 7, 1],
    [7, 2, 12, 5, 8, 4, 6, 11, 14, 9, 1, 15, 13, 3, 10, 0],
    [1, 13, 15, 0, 14, 8, 2, 11, 7, 4, 12, 10, 9, 3, 5, 6],
];

#[inline]
fn rot_left(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}
#[inline]
fn rot_right(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}

#[inline]
fn xor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    let w: u32 = a[0] ^ b[0];
    let x: u32 = a[1] ^ b[1];
    let y: u32 = a[2] ^ b[2];
    let z: u32 = a[3] ^ b[3];
    [w, x, y, z]
}

fn u8_to_le(bytes: [u8; 16]) -> [u32; 4] {
    [
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
        u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
        u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
    ]
}

fn u32_to_le(arr: [u32; 4]) -> [u8; 16] {
    let w: [u8; _] = arr[0].to_le_bytes();

    let x: [u8; _] = arr[1].to_le_bytes();

    let y: [u8; _] = arr[2].to_le_bytes();

    let z: [u8; _] = arr[3].to_le_bytes();

    [
        w[0], w[1], w[2], w[3], x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], z[0], z[1], z[2],
        z[3],
    ]
}

#[inline]
fn awayfrom_bit(arr: [u32; 4], index: u32) -> u32 {
    let x = (index >> 5) as usize;

    let y = (index & 31) as u32;

    let z = arr[x] >> y;

    z & 1
}

#[inline]
fn on_bit(mut arr: [u32; 4], index: u32, n: u32) -> [u32; 4] {
    let x = (index >> 5) as usize;

    let y = (index & 31) as u32;

    let z = 1u32 << y;

    let amp: u32 = n & 1;

    if amp == 1 {
        arr[x] |= z;
    } else {
        arr[x] &= !z;
    }

    arr
}

fn initial_permutation(arr: [u32; 4]) -> [u32; 4] {
    let mut buffer: [u32; 4] = [0u32; 4];

    for n in 0..128u32 {
        let a: u32 = (n & 3) * 32;

        let b: u32 = n >> 2;

        let c: u32 = a + b;

        let bit = awayfrom_bit(arr, c);

        buffer = on_bit(buffer, n, bit);
    }

    buffer
}

fn final_permutation(arr: [u32; 4]) -> [u32; 4] {
    let mut buffer: [u32; 4] = [0u32; 4];

    for n in 0..128u32 {
        let a: u32 = (n & 31) * 4;

        let b: u32 = n >> 5;

        let c: u32 = a + b;

        let bit = awayfrom_bit(arr, c);

        buffer = on_bit(buffer, n, bit);
    }

    buffer
}

fn sbox(index: usize, arr: [u32; 4]) -> [u32; 4] {
    let mut buffer: [u32; 4] = [0u32; 4];

    for n in 0..32u32 {
        let a: u32 = (arr[0] >> n) & 1;

        let b: u32 = ((arr[1] >> n) & 1) << 1;

        let c: u32 = ((arr[2] >> n) & 1) << 2;

        let d: u32 = ((arr[3] >> n) & 1) << 3;

        let x = a | b | c | d;

        let y = SBOX[index][x as usize] as u32;

        for n2 in 0..4u32 {
            buffer[n2 as usize] |= ((y >> n2) & 1) << n;
        }
    }

    buffer
}

fn inverse_sbox(index: usize, arr: [u32; 4]) -> [u32; 4] {
    let mut tmp: [u8; 16] = [0u8; 16];

    for n in 0..16u8 {
        tmp[SBOX[index][n as usize] as usize] = n;
    }

    let mut buffer: [u32; 4] = [0u32; 4];

    for n in 0..32u32 {
        let a = (arr[0] >> n) & 1;

        let b = ((arr[1] >> n) & 1) << 1;

        let c = ((arr[2] >> n) & 1) << 2;

        let d = ((arr[3] >> n) & 1) << 3;

        let x = a | b | c | d;

        let y = tmp[x as usize] as u32;

        for n2 in 0..4u32 {
            let bit = (y >> n2) & 1;
            buffer[n2 as usize] |= bit << n;
        }
    }

    buffer
}
fn linear_transformation(arr: [u32; 4]) -> [u32; 4] {
    // Serpent LT
    let (mut a, mut b, mut c, mut d): (u32, u32, u32, u32) = (arr[0], arr[1], arr[2], arr[3]);

    a = rot_left(a, 13);
    c = rot_left(c, 3);

    b ^= a ^ c;
    d ^= c ^ (a << 3);

    b = rot_left(b, 1);
    d = rot_left(d, 7);

    a ^= b ^ d;
    c ^= d ^ (b << 7);

    a = rot_left(a, 5);
    c = rot_left(c, 22);

    [a, b, c, d]
}

fn inverse_linear_transformation(arr: [u32; 4]) -> [u32; 4] {
    // Serpent inverse LT
    let (mut a, mut b, mut c, mut d): (u32, u32, u32, u32) = (arr[0], arr[1], arr[2], arr[3]);

    c = rot_right(c, 22);
    a = rot_right(a, 5);

    c ^= d ^ (b << 7);
    a ^= b ^ d;

    d = rot_right(d, 7);
    b = rot_right(b, 1);

    d ^= c ^ (a << 3);
    b ^= a ^ c;

    c = rot_right(c, 3);
    a = rot_right(a, 13);

    [a, b, c, d]
}

fn key_schedule(key: &[u8]) -> [[u32; 4]; 33] {
    // 256bit
    let mut buffer: [u8; 32] = [0u8; 32];

    buffer[..key.len()].copy_from_slice(key);

    if key.len() < 32 {
        buffer[key.len()] = 0x01;

        for i in buffer[key.len() + 1..].iter_mut() {
            *i = 0;
        }
    }

    let mut le_w: [u32; 140] = [0u32; 140];

    for i in 0..8 {
        let n: usize = i * 4;

        let tmp: [u8; 4] = [buffer[n], buffer[n + 1], buffer[n + 2], buffer[n + 3]];

        le_w[i] = u32::from_le_bytes(tmp);
    }

    for i in 8..140usize {
        let a: u32 = le_w[i - 8] ^ le_w[i - 5] ^ le_w[i - 3] ^ le_w[i - 1];

        let b: u32 = PHI ^ ((i - 8) as u32);

        let c: u32 = a ^ b;

        le_w[i] = rot_left(c, 11);
    }

    let mut keys: [[u32; 4]; 33] = [[0u32; 4]; 33];

    for i in 0..33usize {
        let a: u32 = le_w[8 + 4 * i + 0];

        let b: u32 = le_w[8 + 4 * i + 1];

        let c: u32 = le_w[8 + 4 * i + 2];

        let d: u32 = le_w[8 + 4 * i + 3];

        let wrapping_sub: usize = (3usize + 32usize).wrapping_sub(i);

        let index = (wrapping_sub) & 7;

        keys[i] = sbox(index, [a, b, c, d]);
    }

    keys
}

fn encrypt_blocks(blocks: &mut [u8], key: &[u8]) {
    let key: [[u32; 4]; 33] = key_schedule(key);

    for chunk in blocks.chunks_exact_mut(BLOCK_SIZE) {
        let mut buffer: [u8; 16] = [0u8; 16];

        buffer.copy_from_slice(chunk);

        let plain: [u32; 4] = u8_to_le(buffer);

        let mut plain: [u32; 4] = initial_permutation(plain);

        for i in 0..32 {
            plain = xor(plain, key[i]);

            let index: usize = (i & 7) as usize;

            plain = sbox(index, plain);

            if i != 31 {
                plain = linear_transformation(plain);
            } else {
                plain = xor(plain, key[32]);
            }
        }

        let cipher: [u32; 4] = final_permutation(plain);

        chunk.copy_from_slice(&u32_to_le(cipher));
    }
}

fn encrypt(plain: &[u8], key: &[u8]) -> Vec<u8> {
    let mut cipher = plain.to_vec();

    let padding_len = BLOCK_SIZE - (cipher.len() % BLOCK_SIZE);

    cipher.extend(std::iter::repeat_n(padding_len as u8, padding_len));

    encrypt_blocks(&mut cipher, key);

    cipher
}

fn decrypt_blocks(blocks: &mut [u8], key: &[u8]) {
    let key: [[u32; 4]; 33] = key_schedule(key);

    for chunk in blocks.chunks_exact_mut(BLOCK_SIZE) {
        let mut buffer = [0u8; 16];

        buffer.copy_from_slice(chunk);

        let cipher: [u32; 4] = u8_to_le(buffer);

        let mut cipher: [u32; 4] = initial_permutation(cipher);

        cipher = xor(cipher, key[32]);

        cipher = inverse_sbox(7, cipher);

        cipher = xor(cipher, key[31]);

        for i in (0..31).rev() {
            cipher = inverse_linear_transformation(cipher);

            cipher = inverse_sbox((i & 7) as usize, cipher);

            cipher = xor(cipher, key[i]);
        }

        chunk.copy_from_slice(&u32_to_le(final_permutation(cipher)));
    }
}

fn decrypt(cipher: &[u8], key: &[u8]) -> Vec<u8> {
    let mut plain = cipher.to_vec();

    decrypt_blocks(&mut plain, key);

    let padding_len = *plain.last().unwrap() as usize;

    plain[plain.len() - padding_len..]
        .iter()
        .all(|&byte| byte as usize == padding_len);

    plain.truncate(plain.len() - padding_len);

    plain
}

fn main() {
    let plain = b"Hello, World";

    let key = [0u8; 32];

    println!("{:?}", String::from_utf8_lossy(plain));

    let cipher = encrypt(plain, &key);

    println!("{:?}", cipher);

    let decrypted = decrypt(&cipher, &key);

    println!("{:?}", String::from_utf8_lossy(&decrypted));
}
