use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}