///
/// cli input might be:
/// post_it "raw text"
/// post_it "raw text" out.file
/// post_it in.file
/// post_it in.file out.file
/// 

use std::fs::File;
use std::io::{self, Read};

use clap::{Arg, ArgAction, ArgMatches, Command};

fn main() {
    let parsed = parse_cli();
    
    // I want to pass a reference to a struct to a function that will return 
    // either the string value entered at the command line or the contents of
    // the file referenced by the file arg
    // if no text is available at either, blow up
    // 

    let arg_text = get_arg_text(&parsed);
    let arg_file = get_arg_file(&parsed);

    let key = get_key().expect("Key file not found.");

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

    println!("Raw text: {}", raw_text);

    println!("Key: {}", key);

    // let path = String::from("test_input.txt");
    // let test = read_file(&path);

    // match test {
    //     Ok(val) => println!("Test value: {}", val),
    //     Err(_) => eprint!("Error."),
    // }


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

fn get_arg_text(parsed: &ArgMatches) -> Option<String>{

    parsed.get_one::<String>("text").map(|s| s.to_owned())

}

fn get_arg_file(parsed: &ArgMatches) -> Option<String>{

    parsed.get_one::<String>("input").map(|s| s.to_owned())
    
}



pub fn get_key() -> Result<String, io::Error> {

    let mut key = String::new();

    File::open("secrets/post_it.key")?.read_to_string(&mut key)?;

    Ok(key)
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