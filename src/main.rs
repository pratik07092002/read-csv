use csv;
use clap::Parser;
use std::{error::Error, path::PathBuf};
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args{
    csvPath: PathBuf
}
fn read_from_csv(path: &PathBuf) -> Result<(), Box<dyn Error>> {
let mut reader = csv::Reader::from_path(path)?;

for result in reader.records() {
  let record = result?;
        println!("{:?}", record);
}

Ok(())
}

fn main() {
    let args = Args::parse();
   let path = args.csvPath;
    if let Err(e) = read_from_csv(&path) {
        eprintln!("{}",e) 
    }
    println!("Hello, world!");
}
