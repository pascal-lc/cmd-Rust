fn main() {
    if let Err(e) = wcrust::get_args().and_then(wcrust::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
