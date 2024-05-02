use crate::repository::Job;
use crate::site::{
    Common, CryptoJobsList, Formatter, NearJobs, Site, SolanaJobs, SubstrateJobs, Web3Careers,
};
use reqwest::Client;
use scraper::{Html, Selector};
use thiserror::Error;

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
    async fn scrape(self) -> Result<Self, Error>
    where
        Self: Sized;

    /// Gets a selector for a specific HTML element.
    fn get_selector(selectors: &str) -> Result<Selector, Error> {
        Selector::parse(selectors).map_err(|e| Error::Selector(e.to_string()))
    }
}

impl Scraper for Web3Careers {
    async fn scrape(mut self) -> Result<Self, Error>
    where
        Self: Sized,
    {
        for i in 1..6 {
            let mut jobs = scrape_for_web3careers(&self, i).await?;
            self.jobs.append(&mut jobs);
        }
        Ok(self)
    }
}

/// Used to scrape web3careers jobsite for a specific page number.
async fn scrape_for_web3careers<T>(t: &T, page_number: u8) -> Result<Vec<Job>, Error>
where
    T: Scraper + Site,
{
    let mut jobs = Vec::new();
    let url = t.get_url();
    let res = Client::new()
        .get(format!("{}?page={}", url, page_number))
        .send()
        .await
        .map_err(|e| Error::Request(url.to_string(), e.to_string()))?;
    if !res.status().is_success() {
        Err(Error::Request(
            url.to_string(),
            format!("Request failed with code {}", res.status().as_u16()),
        ))?;
    }
    let body = res.text().await.map_err(|e| Error::Decode(e.to_string()))?;
    let doc = Html::parse_document(&body);

    // HTML selectors
    let jobs_list_selector =
        T::get_selector("body > main > div > div > div > div > div > table > tbody > tr")?;
    let title_selector = T::get_selector(
        "body > main > div > div > div > div > div > table > tbody > tr > td > div > div > div > a > h2",
    )?;
    let company_selector = T::get_selector(
        "body > main > div > div > div > div > div > table > tbody > tr > td > a > h3",
    )?;
    let location_selector = T::get_selector(
        "body > main > div > div > div > div > div > table > tbody > tr > td:nth-child(4)",
    )?;
    let date_selector = T::get_selector(
        "body > main > div > div > div > div > div > table > tbody > tr > td > time",
    )?;
    let remuneration_selector = T::get_selector(
        "body > main > div > div > div > div > div > table > tbody > tr > td:nth-child(5) > p",
    )?;
    let tag_selector = T::get_selector(
        "body > main > div > div > div > div > div > table > tbody > tr > td > div > span",
    )?;

    for el in doc.select(&jobs_list_selector) {
        let mut job = Job::new();
        job.site = url;

        if let Some(element) = el.select(&title_selector).next() {
            job.title = element.text().collect::<String>().trim().to_owned();
            if let Some(path_raw) = el.value().attr("onclick") {
                job.apply = format!(
                    "{}{}",
                    t.get_url(),
                    Web3Careers::format_apply_link_from(path_raw)
                );
            }
            if let Some(element) = el.select(&company_selector).next() {
                job.company = element.text().collect::<String>().trim().to_owned();
            }
            if let Some(element) = el.select(&location_selector).next() {
                job.location = element.text().collect::<String>().trim().to_owned();
            }
            if let Some(element) = el.select(&date_selector).next() {
                if let Some(date_raw) = element.value().attr("datetime") {
                    job.date_posted = date_raw.split(' ').next().unwrap_or("").to_owned();
                }
            }
            if let Some(element) = el.select(&remuneration_selector).next() {
                let remuneration_raw = element.text().collect::<String>().trim().to_owned();
                if !remuneration_raw.is_empty() {
                    job.remuneration = remuneration_raw
                }
            }
            for tag_el in el.select(&tag_selector) {
                job.tags
                    .push(tag_el.text().collect::<String>().trim().to_owned())
            }

            jobs.push(job);
        }
    }

    Ok(jobs)
}

impl Scraper for CryptoJobsList {
    async fn scrape(mut self) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let url = self.get_url();
        let res = Client::new()
            .get(format!("{url}/engineering?sort=recent"))
            .send()
            .await
            .map_err(|e| Error::Request(url.to_string(), e.to_string()))?;
        if !res.status().is_success() {
            Err(Error::Request(
                url.to_string(),
                format!("Request failed with code {}", res.status().as_u16()),
            ))?;
        }
        let body = res.text().await.map_err(|e| Error::Decode(e.to_string()))?;
        let doc = Html::parse_document(&body);

        // HTML selectors
        let jobs_list_selector =
            Self::get_selector("main > section > section > table > tbody > tr")?;
        let title_selector =
            Self::get_selector("main > section > section > table > tbody > tr > td > div > a")?;
        let company_selector =
            Self::get_selector("main > section > section > table > tbody > tr > td > a")?;
        let location_selector =
            Self::get_selector("main > section > section > table > tbody > tr > td > span")?;
        let date_selector = Self::get_selector(
            "main > section > section > table > tbody > tr > td.job-time-since-creation",
        )?;
        let remuneration_selector = Self::get_selector(
            "main > section > section > table > tbody > tr > td > span.job-salary-text",
        )?;
        let tag_selector =
            Self::get_selector("main > section > section > table > tbody > tr > td > span")?;

        for el in doc.select(&jobs_list_selector) {
            let mut job = Job::new();
            job.site = url;

            if let Some(element) = el.select(&title_selector).next() {
                job.title = element.text().collect::<String>().trim().to_owned();
                if let Some(path) = element.value().attr("href") {
                    job.apply = format!("{}{}", self.get_url(), path);
                }
                if let Some(element) = el.select(&company_selector).next() {
                    job.company = element.text().collect::<String>().trim().to_owned();
                }
                if let Some(element) = el.select(&location_selector).next() {
                    job.location = element.text().collect::<String>().trim().to_owned();
                }
                if let Some(element) = el.select(&date_selector).next() {
                    let date_raw = element.text().collect::<String>().trim().to_owned();
                    job.date_posted = Self::format_date_from(date_raw);
                }
                if let Some(element) = el.select(&remuneration_selector).next() {
                    let remuneration_raw = element.text().collect::<String>().trim().to_owned();
                    job.remuneration = Self::format_remuneration_from(remuneration_raw);
                }
                for tag_el in el.select(&tag_selector) {
                    job.tags
                        .push(tag_el.text().collect::<String>().trim().to_owned())
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

/// Used to scrape for a set identically formatted jobsites (Solana, Substrate, Near).
async fn scrape_for_common<T>(t: &T, query_filter: &str) -> Result<Vec<Job>, Error>
where
    T: Scraper + Site + Common,
{
    let mut jobs = Vec::new();
    let url = t.get_url();
    let res = Client::new()
        .get(format!(
            "{url}?filter={query_filter}"
        ))
        .header(
            "User-Agent",
            "Mozilla/5.0 (iPad; CPU OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148"
        )
        .send()
        .await
        .map_err(|e| Error::Request(url.to_string(), e.to_string()))?;
    if !res.status().is_success() {
        Err(Error::Request(
            url.to_string(),
            format!("Request failed with code {}", res.status().as_u16()),
        ))?;
    }
    let body = res.text().await.map_err(|e| Error::Decode(e.to_string()))?;
    let doc = Html::parse_document(&body);

    // HTML selectors
    let jobs_list_selector = T::get_selector("#content > div > div > div > div > div > div")?;
    let title_selector = T::get_selector(
        "#content > div > div > div > div > div > div > div > div > h4 > a > div > div",
    )?;
    let company_selector = T::get_selector(
        "#content > div > div > div > div > div > div > div > div > div > div > a",
    )?;
    let location_selector = T::get_selector(
        "#content > div > div > div > div > div > div > div > div > div > div > div > meta",
    )?;
    let date_selector = T::get_selector(
        "#content > div > div > div > div > div > div > div > div > div > div > div > div > meta",
    )?;

    for el in doc.select(&jobs_list_selector) {
        let mut job = Job::new();
        job.site = url;

        if let Some(element) = el.select(&title_selector).next() {
            job.title = element.text().collect::<String>().trim().to_owned();
            if let Some(element) = el.select(&company_selector).next() {
                job.company = element.text().collect::<String>().trim().to_owned();
            }
            if let Some(element) = el.select(&location_selector).next() {
                if let Some(v) = element.value().attr("content") {
                    job.location = v.to_owned();
                }
            }
            if let Some(element) = el.select(&date_selector).next() {
                if let Some(v) = element.value().attr("content") {
                    job.date_posted = v.to_owned();
                }
            }

            jobs.push(job);
        }
    }

    Ok(jobs)
}

/// Implements the Scraper trait for common jobsites.
macro_rules! impl_scraper_for_common {
    ($t:ident, $qf:expr) => {
        impl Scraper for $t {
            async fn scrape(mut self) -> Result<Self, Error>
            where
                Self: Sized,
            {
                self.jobs = scrape_for_common(&self, $qf).await?;
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

#[derive(Error, Debug)]
pub enum Error {
    #[error("Selector error: {0}")]
    Selector(String),

    #[error("Error making request to '{0}'. {1}")]
    Request(String, String),

    #[error("Error decoding HTML. {0}")]
    Decode(String),
}
