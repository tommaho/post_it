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
    
    let text = get_text(&parsed);

    match text {
        Ok(val)=> { // got text, proceed
            
            if let Ok(key) = get_key(){ // got a key, proceed

                println!("Key is: {}", key);

            } else {
                panic!("Key not found. Key expected in secrets/post_it.key ");
            }

            println!("Got a value: {}", val);
       
       
        },
        Err(err)=> println!("There was a problem: {}", err),
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


fn get_text(parsed: &ArgMatches) -> Result<&String, &'static str>{
    //println!("{:#?}", parsed);

    let text_arg = parsed.get_one::<String>("text");
    let input_arg = parsed.get_one::<String>("input");

    // if there's no text arg then use the file input arg
    // if there's no text arg and no file input arg then panic
    // if theres a text arg and a file input arg also panic

    if let Some(text) = text_arg {

        if let Some(_) = input_arg { 
            Err("Cannot take both a text arg and a file arg!")
        } else {
            Ok(text)
        }
  
    } else {

        if let Some(input_file) = input_arg { 

            // let file_text = read_input_file(input_file);
            
            // match file_text {
            //     Ok(contents) => {
            //         contents
            //     },
            //     Err(err) => Err("There was a file error: {}", err), 
            // }

            Ok(input_file) // needs to be the result of file read
        } else {
            Err("Input file error.")
        }
    }
}



  


// TODO Implement
fn read_input_file(input_file: &String) -> Result<String, io::Error> {
    
    let mut text = String::new();

    File::open(input_file)?.read_to_string(&mut text)?;

    Ok(text)
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