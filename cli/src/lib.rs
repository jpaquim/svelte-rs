use std::error::Error;
use std::path::PathBuf;
use svelte_rs::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  compile_file(&config.path)?;
  Ok(())
}

pub struct Config {
  path: std::path::PathBuf,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    println!("args: \n{:?}", args);
    if args.len() < 2 {
      return Err("not enough arguments");
    }
    let path = PathBuf::from(&args[1]);
    Ok(Config { path })
  }
}
