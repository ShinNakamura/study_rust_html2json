use std::io::{self, BufRead, Write, BufWriter};
use html_parser;

type MyResult = Result<(), Box<dyn std::error::Error>>;

pub fn run() -> MyResult {
    let input = io::stdin();
    let mut input = io::BufReader::new(input.lock());
    let mut html = String::new();
    loop {
        let mut line = String::new();
        let bytes = input.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        html.push_str(&line);
    }

    let dom = html_parser::Dom::parse(&html)?;
    let json = dom.to_json_pretty()?;
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());
    write!(out, "{}", json)?;
    out.flush()?;
    Ok(())
}
