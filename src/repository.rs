use std::fmt::{Debug, Formatter};

use colored::Colorize;
use rusqlite::Connection;

use crate::scraper::Scraper;
use crate::site::{CryptoJobsList, NearJobs, Site, SolanaJobs, SubstrateJobs, Web3Careers};
use crate::ErrorKind;

const NOT_AVAILABLE: &str = "Not available";

/// The Job struct is the repository primitive.
#[derive(Default)]
pub struct Job {
    pub title: String,
    pub company: String,
    pub date_posted: String,
    pub location: String,
    pub remuneration: String,
    pub tags: Vec<String>,
    pub apply: String,
    pub site: &'static str,
}

impl Job {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn title_contains_any(&self, v: Vec<&str>) -> bool {
        for pat in v {
            if self.title.to_lowercase().contains(pat) {
                return true;
            }
        }
        false
    }
}

/// Pretty print Job for debug.
impl Debug for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let remuneration = if self.remuneration.is_empty() {
            NOT_AVAILABLE
        } else {
            &self.remuneration
        };
        let location = if self.location.is_empty() {
            NOT_AVAILABLE
        } else {
            &self.location
        };
        let tags = if !self.tags.is_empty() {
            format!("[ {} ]", self.tags.join(", "))
        } else {
            NOT_AVAILABLE.to_string()
        };
        let apply = if self.apply.is_empty() {
            NOT_AVAILABLE.green()
        } else {
            self.apply.bright_blue()
        };
        write!(
            f,
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n\n{}",
            "Position:".bold().bright_green(),
            self.title.green(),
            "Company:".bold().bright_green(),
            self.company.green(),
            "Date Posted:".bold().bright_green(),
            self.date_posted.green(),
            "Location:".bold().bright_green(),
            location.green(),
            "Remuneration:".bold().bright_green(),
            remuneration.green(),
            "Tags:".bold().bright_green(),
            tags.green(),
            "Apply:".bold().bright_green(),
            apply,
            "Site:".bold().bright_green(),
            self.site.bright_blue(),
            "+-----------------------------------------------------------------------------------\
            ---------------------------------+\n"
                .green()
        )
    }
}

/// All jobs structs must implement the JobsDbBuilder trait. This will provide the basic ETL operations.
pub trait JobsDbBuilder {
    /// The Error type for the builder.
    type Error;

    /// Initialises the jobs struct with default fields.
    fn new() -> Self;

    /// Takes a vector of Job vectors (one per jobsite scraped) and imports all Jobs into the
    /// jobs struct.
    fn import(self, job_vecs: Vec<Vec<Job>>) -> Self
    where
        Self: Sized;

    /// An optional filter to include only jobs of interest.
    fn filter<F>(self, condition: F) -> Self
    where
        F: Fn(&Job) -> bool;

    /// Adds jobs to the SQLite database. This is the completing method.
    fn add_to_db(self) -> Result<(), Self::Error>;
}

/// Type alias for a job vector.
type Jobs = Vec<Job>;

/// Represents a jobs struct for software jobs. A jobs struct for any job type can be
/// created to implement the JobsDbBuilder trait.
pub struct SoftwareJobs(Jobs);

impl SoftwareJobs {
    pub async fn init() -> Result<(), ErrorKind> {
        let web3_careers = Web3Careers::new().scrape().await?.jobs;
        let crypto_jobs_list = CryptoJobsList::new().scrape().await?.jobs;
        let solana_jobs = SolanaJobs::new().scrape().await?.jobs;
        let substrate_jobs = SubstrateJobs::new().scrape().await?.jobs;
        let near_jobs = NearJobs::new().scrape().await?.jobs;

        SoftwareJobs::new()
            .import(vec![
                web3_careers,
                crypto_jobs_list,
                solana_jobs,
                substrate_jobs,
                near_jobs,
            ])
            .filter(|job| {
                job.title_contains_any(vec!["developer", "engineer", "engineering", "technical"])
            }) // optional filter - in this case filter on engineering jobs
            .add_to_db()?;

        Ok(())
    }
}

impl JobsDbBuilder for SoftwareJobs {
    type Error = ErrorKind;

    fn new() -> Self {
        Self(Default::default())
    }

    fn import(mut self, job_vecs: Vec<Vec<Job>>) -> Self
    where
        Self: Sized,
    {
        for vec in job_vecs {
            self.0.extend(vec)
        }
        self
    }

    fn filter<F>(mut self, condition: F) -> Self
    where
        F: Fn(&Job) -> bool,
    {
        self.0.retain(|job| condition(job));
        self
    }

    fn add_to_db(self) -> Result<(), Self::Error> {
        let conn =
            Connection::open("jobs.db").map_err(|e| ErrorKind::SqliteConnection(e.to_string()))?;
        conn.execute("drop table if exists job", ())
            .map_err(|e| ErrorKind::SqliteQuery(e.to_string()))?;
        conn.execute(
            "create table job (
                id integer primary key,
                title text not null,
                company text not null,
                date_posted date not null,
                location text,
                remuneration text,
                tags json,
                apply text not null,
                site text not null
            )",
            (),
        )
        .map_err(|e| ErrorKind::SqliteQuery(e.to_string()))?;

        for job in &self.0 {
            let tags = serde_json::to_string(&job.tags)
                .map_err(|e| ErrorKind::Serialisation(e.to_string()))?;
            conn.execute(
                "insert into job (
                 title,
                 company,
                 date_posted,
                 location,
                 remuneration,
                 tags,
                 apply,
                 site
            ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                [
                    &job.title,
                    &job.company,
                    &job.date_posted,
                    &job.location,
                    &job.remuneration,
                    &tags,
                    &job.apply,
                    job.site,
                ],
            )
            .map_err(|e| ErrorKind::SqliteQuery(e.to_string()))?;
        }

        Ok(())
    }
}
