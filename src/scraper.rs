use crate::repository::Job;
use crate::site::{CryptoJobsList, Formatter, Site};
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
            job.site = self.get_url();

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

#[derive(Error, Debug)]
pub enum Error {
    #[error("Selector error: {0}")]
    Selector(String),

    #[error("Error making request to '{0}'. {1}")]
    Request(String, String),

    #[error("Error decoding HTML. {0}")]
    Decode(String),
}
