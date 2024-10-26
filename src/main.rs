use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    // '-n' is include blank lines / '-b' is exclude blank lines
    // if no argument is supplied or the only argument is '-'
    if args.len() <= 1 || (args[1].eq("-") && args.len() <= 2) {
        // print std in
        match print_stdin() {
            Ok(_) => { /* do nothing */ }
            Err(_) => { println!("Could not read std in") }
        };
    } else {
        // arguments should be files
        let mut offset: usize = 0;
        for arg in args[1..].iter() {
            match print_file(arg, offset) {
                Ok(new_offset) => { offset = new_offset }
                Err(_) => { println!("Could not read file at: {arg}") }
            }
        }
    }
}

fn print_file(arg: &String, mut offset:usize) -> io::Result<usize> {
    if let Ok(lines) = read_lines(arg) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            offset += 1;
            writeln!(&mut io::stdout(), "{}   {}", offset, line)?;
        }
    }
    Ok(offset) // If we get here it worked?
}

fn print_stdin() -> io::Result<()> {
    let stdin = io::stdin();
    let mut offset = 1;
    for line in io::BufReader::new(stdin).lines() {
        writeln!(&mut io::stdout(), "{} {}", offset, line.unwrap())?;
        offset += 1;
    }
    Ok(())
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}