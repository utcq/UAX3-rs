use rand::Rng;
use base64;

fn div100(mut num: u32) -> u32 {
    let log_num = (num as f64).log10();
    let rounded_log_num = log_num.floor() as u32;
    let power = 2 - rounded_log_num;
    num = (num / 10u32.pow(power)) * 100;
    num
}

fn key_ret(key: &str) -> u32 {
    let k_bytes_array: &[u8] = key.as_bytes();
    let mut key_val: u32 = 0;
    for byte in k_bytes_array {
        let mag: u32 = (*byte as u32).pow(2);
        let mag = mag/mag.leading_zeros();
        let mag: u32 = div100(mag);

        key_val += mag;
    }
    return key_val;
}

pub fn key_gen() -> String {
    let mut rng = rand::thread_rng();

    let key: u16 = rng.gen();

    return key.to_string()
}

#[allow(deprecated)]   // New b64 implementation sucks
pub fn uax3_e(plain: &str, key: &str) -> String {
    let bytes_array: &[u8] = plain.as_bytes();
    let mut trans_array: Vec<u16> = Vec::with_capacity(bytes_array.len());

    let key_val = key_ret(key);

    for byte in bytes_array {
        let magic: u16 = (*byte as u16) + (key_val as u16);
        trans_array.push(magic);
    }


    let c_string: String = String::from_utf16_lossy(&trans_array).to_string();

    return base64::encode(c_string);
}

#[allow(deprecated)]    // New b64 implementation sucks
pub fn uax3_d(cipher: &str, key: &str) -> String {
    let cipher = base64::decode(cipher).expect("Wrong b64 encoding");
    let cipher = String::from_utf8_lossy(&cipher);
    let trans_array: Vec<u16> = cipher.encode_utf16().collect();

    let key_val = key_ret(key);

    let mut bytes_array: Vec<u8> = Vec::with_capacity(trans_array.len());

    for magic in trans_array {
        let byte: u8 = (magic - key_val as u16) as u8;
        bytes_array.push(byte);
    }

    let plain_string: String = String::from_utf8_lossy(&bytes_array).to_string();

    return plain_string;
}