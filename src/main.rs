use job_hunt::scraper::Scraper;
use job_hunt::site::{CryptoJobsList, Site};

#[tokio::main]
async fn main() {
    let crypto_jobs_list = CryptoJobsList::new();
    crypto_jobs_list
        .scrape()
        .await
        .expect("Something went wrong!");
}
