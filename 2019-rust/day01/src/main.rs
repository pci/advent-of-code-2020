use std::env;
use std::process;
use day01::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Using file {}", config.filename);

    let res = day01::run(config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });
    println!("Result {:?}", res);
}