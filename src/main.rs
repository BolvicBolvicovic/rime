use std::io::{self, Stdin, Stdout};
use termion::raw::{self, IntoRawMode, RawTerminal};


fn init_terminal() -> Result<(Stdin, RawTerminal<Stdout>), Box<dyn std::error::Error>> {
    Ok((io::stdin(), io::stdout().into_raw_mode()?))
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut buffer = String::new();
    let (stdin, stdout) = init_terminal()?;
    while stdin.read_line(&mut buffer)? != 0 {
        print!("echo: {}", buffer);
        if &buffer == "q\n" {
            stdout.suspend_raw_mode()?;
            break;
        }
        buffer.clear();
    }
    Ok(())
}
