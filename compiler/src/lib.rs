use std::error::Error;
use std::fs;
use std::path::Path;
use svelte_rs_parser::parse;

pub fn compile_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(path)?;
  compile(&contents)?;
  Ok(())
}

pub fn compile(source: &str) -> Result<(), Box<dyn Error>> {
  parse(source)?;
  Ok(())
}
