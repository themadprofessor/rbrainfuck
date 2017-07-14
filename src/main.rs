#![feature(test)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate nom;
extern crate exit_code;
extern crate test;

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader, Write};

#[macro_use]
mod macros;
mod error;
mod ast;
mod machine;
mod input;


fn main() {
    let read = BufReader::new(if let Some(path) = env::args().nth(1) {
            match File::open(path) {
                Ok(read) => input::Input::File(read),
                Err(err) => exit(exit_code::DATA_ERROR, &string_build!("Failed to open file! ", &err.to_string()))
            }
        } else {
            input::Input::Stdin(io::stdin())
        });

    let ast = match ast::from_read(read) {
        Ok(a) => a,
        Err(err) => exit(exit_code::DATA_ERROR, &string_build!("Failed to parse file! ", &err.to_string()))
    };
    let mut machine = machine::Machine::new();
    let exec_res = machine.exec_ast(&ast);
    if exec_res.is_err() {
        let err = exec_res.unwrap_err();
        exit(exit_code::IO_ERROR, &string_build!("Failed to read from stdin! ", &err.to_string()))
    }
}

fn exit<T>(code: i32, msg: &T) -> ! where T: std::fmt::Display {
    if writeln!(std::io::stderr(), "{}", msg).is_err() {
        println!("Failed to print to stderr! {}", msg);
    }
    std::process::exit(code)
}

