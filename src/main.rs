extern crate clap;
use clap::{Arg, App};
use std::process::Command;

fn main() {
       let matches = App::new("mypipe")
                .version("1.0")
                .author("Hugo TERRAL. <hterral@myges.com>")
                .about("mypipe cmd")
                .arg(Arg::with_name("input")
                    .short("i")
                    .long("--in")
                    .help("Input")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::with_name("output")
                    .short("o")
                    .long("--out")
                    .help("Output")
                    .required(true)
                    .takes_value(true)
                )                      
                .get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let input_cmd = Command::new(input.to_string())
                        .output()
                        .expect("Wrong !");


    let output_cmd = Command::new(output.to_string())
                        .arg(String::from_utf8_lossy(&input_cmd.stdout).to_string())
                        .output()
                        .expect("Wrong !");

                        
    println!("{}", String::from_utf8_lossy(&output_cmd.stdout).to_string());
} 