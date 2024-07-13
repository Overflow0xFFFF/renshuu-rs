#[tokio::main]
async fn main() {
    let args = renshuu::cli::get_args().unwrap();
    if let Err(e) = renshuu::cli::run(args).await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
