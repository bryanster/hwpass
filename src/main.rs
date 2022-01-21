use std::path::Path;
use std::fs;
use std::env;
use std::io::Write;
use std::io::Read;
use std::fs::File;
use enc_file::{encrypt_chacha, decrypt_chacha};
use std::str;

fn get_id() -> String { 
    let mut id = Path::new("/var/lib/dbus/machine-id").exists();
    if id == false {
        id = Path::new("/etc/machine-id").exists();
        if id == false {
            panic!("No HWID was found on system");
        }
        else {
            let hwid = fs::read_to_string("/etc/machine-id").expect("Unable to read file");
            return hwid
        }
    }
    else {
        let mut hwid = fs::read_to_string("/var/lib/dbus/machine-id").expect("Unable to read file");
        if hwid.ends_with('\n') {
            hwid.pop();
        }
        return hwid
    }}

    fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
    
        buffer
    }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "-h" || args[1] == "--help" {
            println!("This is a Password vaulting system that uses a hardware \nlock to make sure that the user is not able to access the \nfile content when removed from the system.\n\n ");
        }
        if args[1] == "-v" || args[1] == "--version" {
            println!("Version: 0.1.0");
        }
        if args[1] == "-e" || args[1] == "--encrypt" {
            let text = b"This a test";
            let keyraw: String = get_id();
            let key: &str=  &keyraw.as_str();
            let text_vec = text.to_vec();
            let ciphertext = encrypt_chacha(text_vec, key).unwrap();
            let mut f = File::create("output.enc").expect("Unable to create file");                                                                                                                                                                                                                                                                     
            f.write_all(&ciphertext).expect("Unable to write data");                                                                                                                                                                                                                                                                                                       
   

        }
        if args[1] == "-d" || args[1] == "--decrypt" {
            let ciphertext = get_file_as_byte_vec("output.enc");
            let keyraw: String = get_id();
            let key: &str=  &keyraw.as_str();
            let plaintextvec = decrypt_chacha(ciphertext, key).unwrap();
            let plaintext = str::from_utf8(&plaintextvec).unwrap();
            println!("{}", plaintext);
    }
}
}