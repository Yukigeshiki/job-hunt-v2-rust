use job_hunt::scraper::Scraper;
use job_hunt::site::{Site, Web3Careers};

#[tokio::main]
async fn main() {
    let web3_careers = Web3Careers::new();
    web3_careers.scrape().await.expect("Something went wrong!");
}
