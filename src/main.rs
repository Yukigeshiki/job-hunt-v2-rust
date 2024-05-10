use jobhunt::repl::Repl;
use jobhunt::repository::SoftwareJobs;

#[tokio::main]
async fn main() {
    SoftwareJobs::init_repl()
        .await
        .expect("Something went wrong!");
}
