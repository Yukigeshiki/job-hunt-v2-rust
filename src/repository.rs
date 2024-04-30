use colored::Colorize;
use std::fmt::{Debug, Formatter};

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
            NOT_AVAILABLE.into()
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
