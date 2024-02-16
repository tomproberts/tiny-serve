use std::env;
use std::process::exit;
use tiny_serve::ServeConfig;

fn main() {
    // Parse args
    let config = ServeConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        exit(1);
    });

    // Serve
    if let Err(e) = tiny_serve::run(config) {
        eprintln!("Application error: {e}");
        exit(1);
    }
}
