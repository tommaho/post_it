///
/// 

use std::fs::File;
use std::io::{self, Read};

use clap::{Arg, ArgAction, ArgMatches, Command};

use std::io::Write;

#[allow(unused_imports)]  //not sure what I'll need
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};

fn main() {
    let parsed = parse_cli();
    
    let arg_text = get_arg(&parsed, "text");
    

    //let arg_text = get_arg_text(&parsed);
    let arg_file = get_arg(&parsed, "input");
    let _arg_output = get_arg(&parsed, "output");

    let key = get_key().expect("There was an error with the key.");

    let mut raw_text = String::new();



    if let Some(text) = arg_text {
        raw_text = text;

        if let Some(_) = arg_file {
            panic!("Cannot take both a text arg and a file arg!");
        }      

    } else {
        if let Some(text) = arg_file {

            let contents = std::fs::read_to_string(text);

            match contents {
                Ok(file_text) => raw_text = file_text,
                _ => panic!("There was a problem with the file.")
            }
        }
        
    }

    if raw_text.len() == 0 {

        panic!("Raw text has zero length.");

    } else { 

        println!("Raw text: {}", raw_text);

        // hackery ahead. I'm in over my head here.

        let mut nonce_file = File::open("secrets/nonce.bin").expect("Failed to open nonce.");
        let mut nonce = [0u8; 12];
        
        nonce_file.read_exact(&mut nonce).expect("Failed to read nonce.");

        let cipher = Aes256Gcm::new(&key);
        let ciphertext = cipher.encrypt((&nonce).into(), raw_text.as_ref()).expect("There was an encryption error.");
    
        let mut encrypted_file = File::create("secrets/encrypted.bin").expect("Failed to create encrypted file");
        encrypted_file.write_all(&ciphertext).expect("Failed to write encrypted data to file");
    



        let plaintext = cipher.decrypt((&nonce).into(), ciphertext.as_ref()).expect("Failed to decrypt text.");
    

        //println!("Ciphertext: {:?}", ciphertext);
        println!("Decrypted plaintext: {:?}", String::from_utf8_lossy(&plaintext));
    }


}

fn parse_cli() -> ArgMatches{
    let parsed = 
    Command::new("post_it")
    .version("0.1.0")
    .author("Tom <tcm18@pct.edu>")
    .about("Symmetric key encryption on the command line.")
    .arg(
        Arg::new("text")
            .value_name("TEXT")
            .short('t')
            .long("text")
            .help("Input text enclosed in double quotes, like \"encrypt me\".")
            .default_value(None)
            )
    .arg(Arg::new("input")
            .value_name("INPUT")
            .short('i')
            .long("input")
            .help("Input file to read.")
            .default_value(None)
        )
    .arg(Arg::new("output")
        .value_name("OUTPUT")
        .short('o')
        .long("output")
        .help("File to output.")
        .default_value(None)
    )
    .arg(
        Arg::new("debug")
            .short('d')
            .long("debug")
            .action(ArgAction::SetTrue),
    )
    .get_matches();

    let is_debug = parsed.get_flag("debug");

    if is_debug {
        println!("Debug: {}", is_debug);
        println!("{:#?}", parsed);

        if let Some(text) = parsed.get_one::<String>("text") {
            println!("Value for text: {text}");
        }
    
        if let Some(input_file) = parsed.get_one::<String>("input") {
            println!("Value for input file: {input_file}");
        }
    
        if let Some(output_file) = parsed.get_one::<String>("output") {
            println!("Value for output file: {output_file}");
        }

    }

    parsed

}

fn get_arg(parsed: &ArgMatches, arg: &str) -> Option<String>{

    parsed.get_one::<String>(arg).map(|s| s.to_owned())

}

// fn get_arg_text(parsed: &ArgMatches) -> Option<String>{

//     parsed.get_one::<String>("text").map(|s| s.to_owned())

// }

// fn get_arg_file(parsed: &ArgMatches) -> Option<String>{

//     parsed.get_one::<String>("input").map(|s| s.to_owned())
    
// }

// fn get_arg_out(parsed: &ArgMatches) -> Option<String>{

//     parsed.get_one::<String>("output").map(|s| s.to_owned())
    
// }


fn get_key() -> Result<Key<Aes256Gcm>, io::Error> {

    let mut key_bytes = [0u8; 32];

    let mut file = File::open("secrets/post_it.key")?;
    
    file.read_exact(&mut key_bytes)?;

    Ok(*Key::<Aes256Gcm>::from_slice(&key_bytes))
}

fn _encrypt() {
    println!("TODO");
}

fn _decrypt() {
    println!("TODO");
}

fn _write_to_file() {
    println!("TODO");
}

fn _write_to_stdio() {
    println!("TODO");
}