use regex::Regex;
use std::error::Error;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?s)<script(?P<type>.*)>(?P<script>.*)</script>").unwrap();
}

fn has_script(text: &str) -> bool {
    RE.is_match(text)
}

fn get_script(text: &str) -> Result<&str, Box<dyn Error>> {
    let captures = RE.captures(text).ok_or("No script found on input file")?;
    let script_type = captures
        .name("type")
        .ok_or("No type on <script> tag")?
        .as_str();
    let script_contents = captures
        .name("script")
        .ok_or("Empty <script> tag")?
        .as_str();
    println!("script_type {}", script_type);
    println!("script_contents {}", script_contents);
    Ok(script_contents)
}

pub fn parse(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", source);
    if !has_script(source) {
        return Err("No script section in source file")?;
    }
    let script_type = get_script(source)?;
    println!("{}", script_type);
    Ok(())
}
