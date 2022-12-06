use ascii_read::AsciiBufRead;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let handle = io::stdin().lock();
    let mut lines = vec![];

    for line in handle.ascii_lines() {
        lines.push(line?);
    }

    println!("* Input provided:");
    for line in lines {
        println!("{line}");
    }
    Ok(())
}
