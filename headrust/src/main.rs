fn main() {
    if let Err(e) = headrust::get_args().and_then(headrust::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
