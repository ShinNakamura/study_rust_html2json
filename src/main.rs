fn main() {
    if let Err(err) = html2json::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
