#[macro_use]
extern crate log;
extern crate env_logger;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::env;
use data::Mem;

mod data;

fn main() {
    env_logger::init().expect("Failed to initialise logger!");

    let mut mem = Mem::new();
    let mut loop_buf = Vec::new();
    let mut in_loop = false;
    let mut skip = false;
    debug!("Initialised memory");

    let reader = if let Some(path) = env::args().nth(1) {
        BufReader::new(File::open(path).expect("Failed to read file!"))
    } else {
        BufReader::new(io::stdin().lock())
    };
    debug!("Opened file successfully");

    for l in reader.lines() {
        for char in l.unwrap().chars() {
            if skip == true && char != ']' {
                continue;
            } else {
                skip = false;
            }
            match char {
                ']' => {
                    in_loop = false;
                    loop {
                        if mem.get_curr() == 0 {
                            break;
                        }
                        for c in &loop_buf {
                            exec(&mut mem, &c);
                        }
                    }
                    loop_buf.clear();
                },
                '[' => {
                    if mem.get_curr() == 0 {
                        skip = true;
                    } else {
                        in_loop = true;
                    }
                },
                '>' | '<' | '+' | '-' | '.' | ',' => {
                    if in_loop {
                        loop_buf.push(char);
                    } else {
                        exec(&mut mem, &char);
                    }
                },
                _ => {}
            }
        }
    }
    println!("");
}

fn exec(mem: &mut Mem, char: &char) {
    if *char == '>' {
        mem.mv_right();
    } else if *char == '<' {
        mem.mv_left();
    } else if *char == '+' {
        mem.inc_curr();
    } else if *char == '-' {
        mem.dec_curr();
    } else if *char == '.' {
        print!("{}", mem.get_curr() as char)
    } else if *char == ',' {
        let mut buff = [0; 1];
        std::io::stdin().read_exact(&mut buff).unwrap();
        mem.put_curr(buff[0]);
    }
}
