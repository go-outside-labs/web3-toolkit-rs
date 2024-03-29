use std::io;
use std::env;
use std::io::Read;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};


fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn main() -> Result<(), Error> {
    const FILEPATH: &str = "result.txt";

    // write to file
    let mut output = File::create(&FILEPATH)?;
    write!(output, "Cyph3r\n👾")?;

    // read from file
    let input = File::open(&FILEPATH)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    // using read_to_string
    let file = env::args().nth(1).expect("please give a filename");
    let text = read_to_string(&file).expect("bad name");

    println!("file had {} bytes", text.len());

    Ok(())
}
