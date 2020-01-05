use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day_01", about = "Day 1 of Advent of Code.")]
pub struct Opt {
    ///Path to inputfile
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    ///If set calculates total fuel needed (fuel for fuel)
    #[structopt(short, long)]
    pub total: bool,
}

pub struct Module {
    mass: u64,
}

impl Module {
    pub fn new(mass: u64) -> Self {
        Module { mass }
    }

    pub fn get_fuel(&self) -> u64 {
        get_fuel_internal(self.mass)
    }

    pub fn get_true_fuel(&self) -> u64 {
        let mut needed_fuel = Vec::new();

        //Calc fuel for mass of module in first iter
        //Every next iter calc fuel for fuel until fuel of fuel iter <= 0
        let mut mass = self.mass;
        loop {
            let fuel = get_fuel_internal(mass);
            if fuel <= 0 {
                break;
            }

            needed_fuel.push(fuel);
            mass = fuel;
        }

        needed_fuel.iter().sum()
    }
}

impl FromStr for Module {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(val) => Ok(Module::new(val)),
            Err(err) => Err(err),
        }
    }
}

fn get_fuel_internal(mass: u64) -> u64 {
    let div = (mass as f64) / 3.;
    let rounded = div.floor() as u64;

    //Fix for Part 2
    //Could underflow with negative...
    if rounded <= 2 {
        return 0;
    }

    let result = rounded - 2;
    result
}
