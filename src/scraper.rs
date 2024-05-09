use reqwest::header::USER_AGENT;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use crate::repository::Job;
use crate::site::{
    Common, CryptoJobsList, DateFormatter, NearJobs, Site, SolanaJobs, SubstrateJobs, Web3Careers,
};
use crate::ErrorKind;

/// All jobsite structs must implement the Scraper trait.
#[allow(async_fn_in_trait)]
pub trait Scraper {
    /// Scrapes the job website and adds Job instances to the site's jobs array - Job instances have
    /// the structure:
    /// ```
    /// struct Job {
    ///     pub title: String,
    ///     pub company: String,
    ///     pub date_posted: String,
    ///     pub location: String,
    ///     pub remuneration: String,
    ///     pub tags: Vec<String>,
    ///     pub apply: String,
    ///     pub site: &'static str,
    /// }
    /// ```
    /// as defined in repository module.
    async fn scrape(self) -> Result<Self, ErrorKind>
    where
        Self: Sized;

    /// Gets an HTML doc for a jobsite.
    async fn get_html_doc(client: &Client, url_full: &str) -> Result<Html, ErrorKind> {
        let res = client
            .get(url_full)
            .header(
                USER_AGENT,
                "Mozilla/5.0 (iPad; CPU OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148",
            )
            .send()
            .await
            .map_err(|e| ErrorKind::Request(url_full.to_string(), e.to_string()))?;
        if !res.status().is_success() {
            Err(ErrorKind::Request(
                url_full.to_string(),
                format!("Request failed with code {}", res.status().as_u16()),
            ))?;
        }
        let body = res
            .text()
            .await
            .map_err(|e| ErrorKind::Decode(e.to_string()))?;
        let doc = Html::parse_document(&body);
        Ok(doc)
    }

    /// Gets a selector for a specific HTML element.
    fn get_selector(selectors: &str) -> Result<Selector, ErrorKind> {
        Selector::parse(selectors).map_err(|e| ErrorKind::Selector(e.to_string()))
    }
}

trait GetText {
    fn get_text(&self) -> String;
}

impl GetText for ElementRef<'_> {
    fn get_text(&self) -> String {
        self.text().collect::<String>().trim().to_string()
    }
}

impl Scraper for Web3Careers {
    async fn scrape(mut self) -> Result<Self, ErrorKind>
    where
        Self: Sized,
    {
        let client = Client::new();
        let url = self.get_url();
        for i in 1..6 {
            let mut jobs = Self::_scrape(url, &client, i).await?;
            self.jobs.append(&mut jobs);
        }
        Ok(self)
    }
}

impl Web3Careers {
    /// Used to scrape web3careers jobsite for a specific page number.
    async fn _scrape(
        url: &'static str,
        client: &Client,
        page_number: u8,
    ) -> Result<Vec<Job>, ErrorKind>
    where
        Self: Scraper + Site,
    {
        let mut jobs = Vec::new();
        let url_full = format!("{}?page={}", url, page_number);
        let doc = Self::get_html_doc(client, &url_full).await?;

        // HTML selectors
        let jobs_list_selector =
            Self::get_selector("body>main>div>div>div>div>div>table>tbody>tr")?;
        let title_selector =
            Self::get_selector("body>main>div>div>div>div>div>table>tbody>tr>td>div>div>div>a>h2")?;
        let company_selector =
            Self::get_selector("body>main>div>div>div>div>div>table>tbody>tr>td>a>h3")?;
        let location_selector =
            Self::get_selector("body>main>div>div>div>div>div>table>tbody>tr>td:nth-child(4)")?;
        let date_selector =
            Self::get_selector("body>main>div>div>div>div>div>table>tbody>tr>td>time")?;
        let remuneration_selector =
            Self::get_selector("body>main>div>div>div>div>div>table>tbody>tr>td:nth-child(5)>p")?;
        let tag_selector =
            Self::get_selector("body>main>div>div>div>div>div>table>tbody>tr>td>div>span")?;

        for el in doc.select(&jobs_list_selector) {
            let mut job = Job::new();
            job.site = url;

            if let Some(element) = el.select(&title_selector).next() {
                job.title = element.get_text();
                if let Some(path_raw) = el.value().attr("onclick") {
                    job.apply = Web3Careers::format_apply_url_from(url, path_raw);
                }
                if let Some(element) = el.select(&company_selector).next() {
                    job.company = element.get_text();
                }
                if let Some(element) = el.select(&location_selector).next() {
                    job.location = element.get_text();
                }
                if let Some(element) = el.select(&date_selector).next() {
                    if let Some(date_raw) = element.value().attr("datetime") {
                        job.date_posted = Self::format_date_from(date_raw);
                    }
                }
                if let Some(element) = el.select(&remuneration_selector).next() {
                    let remuneration = element.get_text();
                    if !remuneration.is_empty() {
                        job.remuneration = remuneration;
                    }
                }
                for tag_el in el.select(&tag_selector) {
                    job.tags.push(tag_el.get_text());
                }

                jobs.push(job);
            }
        }

        Ok(jobs)
    }
}

impl Scraper for CryptoJobsList {
    async fn scrape(mut self) -> Result<Self, ErrorKind>
    where
        Self: Sized,
    {
        let url = self.get_url();
        let url_full = format!("{url}/engineering?sort=recent");
        let doc = Self::get_html_doc(&Client::new(), &url_full).await?;

        // HTML selectors
        let jobs_list_selector = Self::get_selector("main>section>section>table>tbody>tr")?;
        let title_selector = Self::get_selector("main>section>section>table>tbody>tr>td>div>a")?;
        let company_selector = Self::get_selector("main>section>section>table>tbody>tr>td>a")?;
        let location_selector = Self::get_selector("main>section>section>table>tbody>tr>td>span")?;
        let date_selector =
            Self::get_selector("main>section>section>table>tbody>tr>td.job-time-since-creation")?;
        let remuneration_selector =
            Self::get_selector("main>section>section>table>tbody>tr>td>span.job-salary-text")?;
        let tag_selector = Self::get_selector("main>section>section>table>tbody>tr>td>span")?;

        for el in doc.select(&jobs_list_selector) {
            let mut job = Job::new();
            job.site = url;

            if let Some(element) = el.select(&title_selector).next() {
                job.title = element.get_text();
                if let Some(path) = element.value().attr("href") {
                    job.apply = format!("{}{}", url, path);
                }
                if let Some(element) = el.select(&company_selector).next() {
                    job.company = element.get_text();
                }
                if let Some(element) = el.select(&location_selector).next() {
                    job.location = element.get_text();
                }
                if let Some(element) = el.select(&date_selector).next() {
                    let date_raw = element.get_text();
                    job.date_posted = CryptoJobsList::format_date_from(&date_raw);
                }
                if let Some(element) = el.select(&remuneration_selector).next() {
                    let remuneration_raw = element.get_text();
                    job.remuneration = CryptoJobsList::format_remuneration_from(&remuneration_raw);
                }
                for tag_el in el.select(&tag_selector) {
                    job.tags
                        .push(tag_el.text().collect::<String>().trim().to_owned());
                }
                if !job.tags.is_empty() {
                    job.tags.remove(0);
                }

                self.jobs.push(job);
            }
        }

        Ok(self)
    }
}

/// Implements the Scraper trait for common jobsites.
macro_rules! impl_scraper_for_common {
    ($t:ident, $qp:expr) => {
        impl Scraper for $t {
            async fn scrape(mut self) -> Result<Self, ErrorKind>
            where
                Self: Sized,
            {
                let url = self.get_url();
                let url_full = format!("{url}?filter={}", $qp);
                let doc = Self::get_html_doc(&Client::new(), &url_full).await?;

                // HTML selectors
                let jobs_list_selector = Self::get_selector("#content>div>div>div>div>div>div")?;
                let title_selector =
                    Self::get_selector("#content>div>div>div>div>div>div>div>div>h4>a>div>div")?;
                let company_selector =
                    Self::get_selector("#content>div>div>div>div>div>div>div>div>div>div>a")?;
                let location_selector = Self::get_selector(
                    "#content>div>div>div>div>div>div>div>div>div>div>div>meta",
                )?;
                let date_selector = Self::get_selector(
                    "#content>div>div>div>div>div>div>div>div>div>div>div>div>meta",
                )?;
                let apply_selector = Self::get_selector(
                    "#content>div>div>div>div>div>div>div>div.sc-beqWaB.sc-gueYoa.hcVvkM.MYFxR>a",
                )?;

                for el in doc.select(&jobs_list_selector) {
                    let mut job = Job::new();
                    job.site = url;

                    if let Some(element) = el.select(&title_selector).next() {
                        job.title = element.get_text();
                        if let Some(element) = el.select(&company_selector).next() {
                            job.company = element.get_text();
                        }
                        if let Some(element) = el.select(&location_selector).next() {
                            if let Some(c) = element.value().attr("content") {
                                job.location = c.to_string();
                            }
                        }
                        if let Some(element) = el.select(&date_selector).next() {
                            if let Some(c) = element.value().attr("content") {
                                job.date_posted = c.to_string();
                            }
                        }
                        if let Some(element) = el.select(&apply_selector).next() {
                            if let Some(path_raw) = element.value().attr("href") {
                                job.apply = Self::format_apply_url_from(url, path_raw);
                            }
                        }

                        self.jobs.push(job);
                    }
                }

                Ok(self)
            }
        }
    };
}

impl_scraper_for_common!(
    SolanaJobs,
    "eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19"
);
impl_scraper_for_common!(
    SubstrateJobs,
    "eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19"
);
impl_scraper_for_common!(
    NearJobs,
    "eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19"
);

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::repository::Job;
    use crate::site::{
        CryptoJobsList, NearJobs, Site, SolanaJobs, SubstrateJobs, Web3Careers,
        CRYPTO_JOBS_LIST_URL, NEAR_JOBS_URL, SOLANA_JOBS_URL, SUBSTRATE_JOBS_URL, WEB3_CAREERS_URL,
    };

    use super::Scraper;

    const DATE_REGEX: &str = r"(\d{4})-(\d{2})-(\d{2})( (\d{2}):(\d{2}):(\d{2}))?";
    const REM_REGEX: &str = r"(\$|€)(\d)+k - (\$|€)(\d)+k";

    #[tokio::test]
    async fn test_scrape_web3careers() {
        let jobs = Web3Careers::new().scrape().await.unwrap().jobs;
        assert_eq!(jobs[0].site, WEB3_CAREERS_URL);
        job_assertions(jobs)
    }

    #[tokio::test]
    async fn test_scrape_crypto_jobs_list() {
        let jobs = CryptoJobsList::new().scrape().await.unwrap().jobs;
        assert_eq!(jobs[0].site, CRYPTO_JOBS_LIST_URL);
        job_assertions(jobs)
    }

    #[tokio::test]
    async fn test_scrape_solana_jobs() {
        let jobs = SolanaJobs::new().scrape().await.unwrap().jobs;
        assert_eq!(jobs[0].site, SOLANA_JOBS_URL);
        job_assertions(jobs)
    }

    #[tokio::test]
    async fn test_scrape_substrate_jobs() {
        let jobs = SubstrateJobs::new().scrape().await.unwrap().jobs;
        assert_eq!(jobs[0].site, SUBSTRATE_JOBS_URL);
        job_assertions(jobs)
    }

    #[tokio::test]
    async fn test_scrape_near_jobs() {
        let jobs = NearJobs::new().scrape().await.unwrap().jobs;
        assert_eq!(jobs[0].site, NEAR_JOBS_URL);
        job_assertions(jobs)
    }

    fn job_assertions(jobs: Vec<Job>) {
        assert!(jobs.len() > 0);
        for job in &jobs {
            println!("{}", job.remuneration);
            assert!(!job.title.is_empty());
            assert!(!job.company.is_empty());
            assert!(Regex::new(DATE_REGEX).unwrap().is_match(&job.date_posted));
            assert!(
                Regex::new(REM_REGEX).unwrap().is_match(&job.remuneration)
                    || job.remuneration.is_empty()
            );
            assert!(
                job.apply.starts_with("https")
                    || job.apply.starts_with("mailto")
                    || job.apply.is_empty()
            )
        }
    }
}
