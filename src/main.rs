mod xfcrypto;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;


fn main() {
    let e_cont = fs::read_to_string("5mb.rtf").expect("File not found!");
    
    let key = xfcrypto::key_gen();

    let start_time = Instant::now();
    let cph = xfcrypto::uax3_e(&e_cont, &key);
    let plain = xfcrypto::uax3_d(&cph, &key);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("{} Chars Enc/Dec in: {:?}", plain.len(), elapsed_time);

    File::create("d-5mb.rtf").expect("No perm").write_all(plain.as_bytes()).expect("No perm");

    //println!("Encrypted content: {}", cph);
    //println!("Plain: {}", plain);
}
