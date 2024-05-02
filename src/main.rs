use job_hunt::scraper::Scraper;
use job_hunt::site::{Site, SolanaJobs};

#[tokio::main]
async fn main() {
    let solana_jobs = SolanaJobs::new();
    solana_jobs.scrape().await.expect("Something went wrong!");
}
