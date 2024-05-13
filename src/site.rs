use chrono::{Duration, Local};

use crate::repository::Job;

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
///    pub jobs: Vec<jobhunt::repository::Job>,
/// }
/// ```
pub trait Site {
    /// Creates a new instance - default values must be provided in the implementation.
    fn new() -> Self;

    /// Getter for non-public url value.
    fn get_url(&self) -> &'static str;
}

/// Website structs can implement the Formatter trait where needed.
pub trait DateFormatter {
    /// Formats a date from a given elapsed time string, e.g. "1 hour", "3 days", "today", "3d".
    fn format_date_from(time_elapsed: &str) -> String;

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
    pub fn format_apply_url_from(url: &str, a: &str) -> String {
        let v = a.split(' ').collect::<Vec<&str>>();
        match v.len() {
            2 => format!("{}{}", url, v[1].replace(['\'', ')'], "")),
            _ => "".to_string(),
        }
    }

    /// Formats a date.
    pub fn format_date_from(date_raw: &str) -> String {
        date_raw.split(' ').collect::<Vec<_>>()[0].to_string()
    }
}

impl CryptoJobsList {
    pub fn format_remuneration_from(r: &str) -> String {
        if r.starts_with("EUR") {
            let r = r.replace("EUR", "");
            let rem_v = r.split('-').map(|s| s.trim()).collect::<Vec<&str>>();
            match rem_v.len() {
                2 => format!("€{} - €{}", rem_v[0], rem_v[1]),
                _ => "".to_string(),
            }
        } else {
            let r = r.replace('$', "");
            let rem_v = r.split('-').map(|s| s.trim()).collect::<Vec<&str>>();
            match rem_v.len() {
                2 => format!("${} - ${}", rem_v[0], rem_v[1]),
                _ => "".to_string(),
            }
        }
    }
}

impl DateFormatter for CryptoJobsList {
    fn format_date_from(time_elapsed: &str) -> String {
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
}

pub trait Common {
    /// Formats a raw path to a full url for a common jobsite.
    fn format_apply_url_from(url: &str, path_raw: &str) -> String {
        if path_raw.starts_with("https") {
            path_raw.to_string()
        } else {
            format!("{}{}", url, path_raw).replacen("jobs/", "", 1)
        }
    }
}

impl Common for SolanaJobs {}

impl Common for SubstrateJobs {}

impl Common for NearJobs {}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::site::{Common, CryptoJobsList, DateFormatter, SolanaJobs, Web3Careers};

    #[test]
    fn test_web3careers_format_apply_url() {
        assert_eq!(
            Web3Careers::format_apply_url_from(
                "https://web3.career",
                "tableTurboRowClick(event, '/full-stack-ai-blockchain-systems-engineer-nodeai/66176')",
            ),
            "https://web3.career/full-stack-ai-blockchain-systems-engineer-nodeai/66176"
        );
    }

    #[test]
    fn test_web3careers_format_date() {
        assert_eq!(
            Web3Careers::format_date_from("2024-05-06 12:05:50+07:00"),
            "2024-05-06"
        );
    }

    #[test]
    fn test_crypto_jobs_list_format_remuneration() {
        assert_eq!(
            CryptoJobsList::format_remuneration_from("$ 90k-140k"),
            "$90k - $140k"
        );
        assert_eq!(
            CryptoJobsList::format_remuneration_from("EUR 90k-140k"),
            "€90k - €140k"
        );
    }

    #[test]
    fn test_crypto_jobs_list_format_date() {
        assert_eq!(
            CryptoJobsList::format_date_from("today"),
            CryptoJobsList::now_and_format()
        );
        assert_eq!(
            CryptoJobsList::format_date_from("1d"),
            CryptoJobsList::sub_duration_and_format(Duration::days(1))
        );
        assert_eq!(
            CryptoJobsList::format_date_from("2w"),
            CryptoJobsList::sub_duration_and_format(Duration::weeks(2))
        );
    }

    #[test]
    fn test_common_format_apply_url() {
        assert_eq!(
            SolanaJobs::format_apply_url_from(
                "https://jobs.solana.com/jobs",
                "/companies/solana-foundation-2/jobs/36564322-lead-software-engineer-payments-commerce#content",
            ),
            "https://jobs.solana.com/companies/solana-foundation-2/jobs/36564322-lead-software-engineer-payments-commerce#content"
        );
    }
}
