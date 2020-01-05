mod data;

use data::Opt;
use structopt::StructOpt;

use crate::data::Module;
use std::fs;
use std::io::Read;

fn main() {
    let opt: Opt = Opt::from_args();

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
        Err(err) => panic!(
            "Error reading input file (\"{}\"): {}",
            opt.input.display(),
            err
        ),
    };
    if read_size == 0 {
        eprintln!("Input file empty?!");
        return;
    }

    //Convert numbers into Modules for easy usage
    let (modules, errors): (Vec<_>, Vec<_>) = input_text
        .lines()
        .map(|l| l.parse::<Module>())
        .partition(Result::is_ok);

    if errors.len() > 0 {
        eprintln!("Errors parsing text to numbers");
        //TODO: Display errors
        return;
    }

    let modules: Vec<Module> = modules.into_iter().map(Result::unwrap).collect();

    let res: u64 = match opt.total {
        false => modules.iter().map(Module::get_fuel).sum(),
        true => modules.iter().map(Module::get_true_fuel).sum(),
    };

    println!("Result: {}", res);
}
