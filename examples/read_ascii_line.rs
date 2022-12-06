use ascii::AsciiString;
use ascii_read::AsciiBufRead;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut handle = io::stdin().lock();
    let mut lines = vec![];
    let mut line = AsciiString::new();
    loop {
        let n = handle.read_ascii_line(&mut line)?;
        if n == 0 {
            break;
        }
        let _ = line.pop();
        lines.push(line.clone());
        line.clear();
    }

    println!("* Input provided:");
    for line in lines {
        println!("{line}");
    }
    Ok(())
}
