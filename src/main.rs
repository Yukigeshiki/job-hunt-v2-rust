use job_hunt::scraper::Scraper;
use job_hunt::site::{CryptoJobsList, Site, SolanaJobs};

#[tokio::main]
async fn main() {
    let solana_jobs = CryptoJobsList::new();
    solana_jobs.scrape().await.expect("Something went wrong!");
}
