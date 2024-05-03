use crate::repository::Job;
use chrono::{Duration, Local};

/// Job site URLs used for scraping.
pub const WEB3_CAREERS_URL: &str = "https://web3.career";
pub const CRYPTO_JOBS_LIST_URL: &str = "https://cryptojobslist.com";
pub const SOLANA_JOBS_URL: &str = "https://jobs.solana.com/jobs";
pub const SUBSTRATE_JOBS_URL: &str = "https://careers.substrate.io/jobs";
pub const NEAR_JOBS_URL: &str = "https://careers.near.org/jobs";

/// All jobsite structs must implement the Site trait and conform to the structure:
/// ```
/// pub struct Jobsite {
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

/// Website structs can implement the Formatter trait where needed.
pub trait Formatter {
    /// Formats a date from a given elapsed time string, e.g. "1 hour", "3 days", "today", "3d".
    fn format_date_from(time_elapsed: String) -> String;

    /// Formats a remuneration string.
    fn format_remuneration_from(r: String) -> String;

    /// Returns a formatted ("%Y-%m-%d") version of now minus a time duration.
    fn sub_duration_and_format(duration: Duration) -> String {
        Local::now()
            .checked_sub_signed(duration)
            .unwrap_or(Local::now())
            .format("%Y-%m-%d")
            .to_string()
    }

    /// Returns a formatted ("%Y-%m-%d") version of now.
    fn now_and_format() -> String {
        Local::now().format("%Y-%m-%d").to_string()
    }
}

/// Generates a jobsite struct and implements the Site trait.
macro_rules! generate_jobsite_struct_and_impl {
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

generate_jobsite_struct_and_impl!(Web3Careers, WEB3_CAREERS_URL);
generate_jobsite_struct_and_impl!(CryptoJobsList, CRYPTO_JOBS_LIST_URL);
generate_jobsite_struct_and_impl!(SolanaJobs, SOLANA_JOBS_URL);
generate_jobsite_struct_and_impl!(SubstrateJobs, SUBSTRATE_JOBS_URL);
generate_jobsite_struct_and_impl!(NearJobs, NEAR_JOBS_URL);

impl Web3Careers {
    /// Formats an onclick function (as a &str) into a URL path string.
    pub fn format_apply_path_from(a: &str) -> String {
        let v = a.split(' ').collect::<Vec<&str>>();
        match v.len() {
            2 => v[1].replace(['\'', ')'], ""),
            _ => "".into(),
        }
    }
}

impl Formatter for CryptoJobsList {
    fn format_date_from(time_elapsed: String) -> String {
        let v = time_elapsed.chars().collect::<Vec<char>>();
        match v.len() {
            len if len >= 2 => {
                let d: i64 = v[0] as i64 - 0x30;
                match v[1] {
                    'd' => Self::sub_duration_and_format(Duration::days(d)),
                    'w' => Self::sub_duration_and_format(Duration::weeks(d)),
                    'm' => Self::sub_duration_and_format(Duration::days(d * 30)),
                    _ => Self::now_and_format(),
                }
            }
            _ => Self::now_and_format(),
        }
    }

    fn format_remuneration_from(mut r: String) -> String {
        r = r.replace('$', "");
        let rem_v = r.split('-').map(|s| s.trim()).collect::<Vec<&str>>();
        match rem_v.len() {
            2 => format!("${} - ${}", rem_v[0], rem_v[1]),
            _ => "".into(),
        }
    }
}

pub trait Common {}

impl Common for SolanaJobs {}
impl Common for SubstrateJobs {}
impl Common for NearJobs {}
