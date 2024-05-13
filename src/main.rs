use colored::Colorize;

use jobhunt::red_println;
use jobhunt::repl::Repl;
use jobhunt::repository::SoftwareJobs;

#[tokio::main]
async fn main() {
    if let Err(err) = SoftwareJobs::init_repl().await {
        red_println!(err.to_string());
        panic!()
    }
}
