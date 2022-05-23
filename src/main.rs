use std::env;
use std::process;
use grepchan::*;

fn main() {
    // collecting arguments from the user
    let args_vec: Vec<String> = env::args().collect();
    let args = Args::new(&args_vec).unwrap_or_else(|err| {
        eprintln!("problem: {}", err);
        process::exit(1);
    });
    if let Err(e) = grep(args){
        eprintln!("There was an eror: {:?}", e);
        process::exit(1);
    }
}