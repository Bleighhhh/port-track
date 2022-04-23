use std::env;
use std::fs;
use std::process;
use std::error:Error;

fn main() {
    let args: Vec<String> = env:args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(op: |err: &str| {
        println!("Problem parsin argumens: {}", err);
        process::exit(code: 1);
    });

    println!("Hai comprato {}", config.quantity);
    println!("{}", config.symbol);
    println!("a {} dollari", config.entryprice);

    if let Err(e: Box<dyn Error>) = run(config) {
        println!("Applicaiton error: {}", e);
        process::exit(code:1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents: String  = fs::read_to_string(path:config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    symbol: String,
    quantity: u32,
    entrypice: u32,
}  

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
       return Err("not enough arguements");
    }    

    let symbol: String = args[1];
    let quantity: u32 = args[2];
    let entryprice: u32 = args[3];

    Ok(Config {symbol, quantity, entryprice})

    }
