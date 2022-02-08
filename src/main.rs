use clap::Parser;
use std::error::Error;
use std::fs;
use std::io::{Cursor, Read};
use std::str::Utf8Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    read_gb(&args.filename)?;
    Ok(())
}

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    filename: String,
}

fn read_gb(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut f = fs::File::open(filename)?;
    let mut contents = Vec::new();
    f.read_to_end(&mut contents)?;
    let mut buff = Cursor::new(contents);

    buff.set_position(0x103); // move to cartridge header
    buff.set_position(0x134);
    let title_data: &mut [u8] = &mut [0; 16];
    buff.read(title_data)?;
    let title = read_string(title_data)?;
    println!("{}", title);
    Ok(())
}

fn read_string(data: &[u8]) -> Result<&str, Utf8Error> {
    match data.iter().position(|&x| x == 0) {
        Some(i) => { std::str::from_utf8(&data[..i]) }
        None => { std::str::from_utf8(&data) }
    }
}