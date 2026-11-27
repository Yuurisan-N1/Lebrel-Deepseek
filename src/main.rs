mod cli;
mod job;
mod net;
mod step;
mod support;
mod wire;

use support::banner;

#[tokio::main]
async fn main() {
    banner::print_banner();
    support::log::arm();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(|value| value.as_str()).unwrap_or("run");
    let rest: Vec<String> = args.iter().skip(1).cloned().collect();
    let code = cli::dispatch(command, &rest).await;
    std::process::exit(code);
}
