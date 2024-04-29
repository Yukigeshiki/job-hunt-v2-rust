/// Job site URLs used for scraping.
pub const WEB3_CAREERS_URL: &str = "https://web3.career";
pub const CRYPTO_JOBS_LIST_URL: &str = "https://cryptojobslist.com";
pub const SOLANA_JOBS_URL: &str =
    "https://jobs.solana.com/jobs?filter=eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19";
pub const SUBSTRATE_JOBS_URL: &str =
    "https://careers.substrate.io/jobs?filter=eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19";
pub const NEAR_JOBS_URL: &str =
    "https://careers.near.org/jobs?filter=eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19";

/// All website structs must implement the Site trait and conform to the structure:
/// ```
/// pub struct Website {
///    url: &'static str,
///    pub jobs: Vec<job_hunt::repository::Job>,
/// }
/// ```
pub trait Site {
    /// Creates a new instance - default values must be provided in the implementation.
    fn new() -> Self;

    /// Getter for non-public url value.
    fn get_url(&self) -> &'static str;
}

/// Generates a website struct and implements the Site trait.
macro_rules! generate_website_struct_and_impl {
    ($t:ident, $url:ident) => {
        #[derive(Default)]
        pub struct $t {
            url: &'static str,
            pub jobs: Vec<Job>,
        }

        impl Site for $t {
            fn new() -> Self {
                Self {
                    url: $url,
                    ..Default::default()
                }
            }

            fn get_url(&self) -> &'static str {
                self.url
            }
        }
    };
}

generate_website_struct_and_impl!(Web3Careers, WEB3_CAREERS_URL);
generate_website_struct_and_impl!(UseWeb3, CRYPTO_JOBS_LIST_URL);
generate_website_struct_and_impl!(SolanaJobs, SOLANA_JOBS_URL);
generate_website_struct_and_impl!(SubstrateJobs, SUBSTRATE_JOBS_URL);
generate_website_struct_and_impl!(NearJobs, NEAR_JOBS_URL);