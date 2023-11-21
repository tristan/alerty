use std::io::{self, Write, BufRead};

pub fn read_line_from_stdin(text: &str) -> Result<Option<String>, io::Error> {
    print!("{}: ", text);
    io::stdout().lock().flush()?;
    io::stdin().lock().lines().next().transpose()
}
