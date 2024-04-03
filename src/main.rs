fn main() {
    println!("Hello, world!");
    run();
}

fn run() {
    parse_cli();
    read_secret_key();
    encrypt();
    process_encrypted_file();
}

fn parse_cli() {
    //read cli input (may be plaintext to encrypt or ciphertext to decrypt)
    println!("Parsing CLI input");
}

fn read_secret_key() {
    //read_key from secrets/keyfile.key
    println!("Reading secret key.");
}

fn encrypt() {
    //encrypt
    println!("Encrypting");
}

fn process_encrypted_file(){
    //write to encrypted output
    println!("Processing encrypted file.");
}


    




