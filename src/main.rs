use std::io;

fn main() -> io::Result<()> {
    let lines = io::stdin().lines();

    for line in lines {
        println!("{}", line?);
    }
    Ok(())
}
