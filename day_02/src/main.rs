mod data;

use crate::data::Opt;
use common::intcode::{Computer, ComputerStatus};
use structopt::StructOpt;

use std::convert::TryFrom;
use std::fs;
use std::io::{Read, Write};

fn main() {
    let opt = Opt::from_args();

    //Open file
    let mut input_file = match fs::File::open(opt.input.clone()) {
        Ok(file) => file,
        Err(err) => panic!(
            "Error opening input file (\"{}\"): {}",
            opt.input.display(),
            err
        ),
    };

    //Read file to string
    let mut input_text = String::new();
    let read_size = match input_file.read_to_string(&mut input_text) {
        Ok(size) => size,
        Err(err) => {
            eprintln!(
                "Error reading input file (\"{}\"): {}",
                opt.input.display(),
                err
            );
            return;
        }
    };
    if read_size == 0 {
        eprintln!("Input file empty?!");
        return;
    }

    //input_text to numbervector
    let (numbers, errors): (Vec<_>, Vec<_>) = input_text
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>())
        .partition(Result::is_ok);

    if !errors.is_empty() {
        eprintln!("Errors parsing text to numbers");
        //TODO: Display errors
        return;
    }

    let numbers: Vec<u32> = numbers.into_iter().map(Result::unwrap).collect();

    let mut computer = match Computer::try_from(numbers) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let mut memory_prints = Vec::new();

    loop {
        if opt.memory {
            memory_prints.push(computer.clone_memory());
        }

        match computer.step() {
            ComputerStatus::Running => continue,
            ComputerStatus::Complete => break,
        }
    }

    if opt.memory {
        let mut file = fs::File::create("dump.csv").unwrap();
        let output_text = print(memory_prints);
        file.write_all(output_text.as_bytes()).unwrap();
    }

    println!("Result: {}", computer.get_value(opt.position));
}

fn print(memory_print: Vec<Vec<u32>>) -> String {
    let mut res = String::new();

    for sub_v in memory_print {
        for value in sub_v {
            res.push_str(&value.to_string());
            res.push(',');
        }
        res.pop();
        res.push('\n');
    }

    res
}
