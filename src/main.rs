use job_hunt::repository::SoftwareJobs;

#[tokio::main]
async fn main() {
    SoftwareJobs::init().await.unwrap();
}
