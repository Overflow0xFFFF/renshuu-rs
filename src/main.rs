fn main() {
    if let Err(e) = renshuu::cli::get_args().and_then(renshuu::cli::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
