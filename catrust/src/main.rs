fn main() {
    if let Err(e) = catrust::get_args().and_then(catrust::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
