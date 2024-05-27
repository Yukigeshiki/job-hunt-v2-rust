use chrono::Local;
use colored::Colorize;
use rusqlite::Connection;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::repository::{Job, SoftwareJobs};
use crate::{green_println, red_println, ErrorKind};

/// This trait must be implemented by the specific job repo struct to be used in Job Hunt (e.g. SoftwareJobs).
#[allow(async_fn_in_trait)]
pub trait Repl {
    /// Initializes a repository for the job repo type that is implementing this trait; then
    /// initializes the REPL and parses queries.
    async fn init_repl() -> Result<(), ErrorKind>;

    fn select_and_display_jobs(conn: Connection, l: String) -> Result<(), ErrorKind> {
        let query = l.replace("select jobs", "select * from jobs");
        let mut stmt = conn
            .prepare(&query)
            .map_err(|e| ErrorKind::SqliteQuery(e.to_string()))?;

        let jobs = stmt
            .query_map((), |row| {
                let tags: String = row.get(6).unwrap();
                let tags: Vec<String> = serde_json::from_str(&tags).unwrap();
                Ok(Job {
                    title: row.get(1)?,
                    company: row.get(2)?,
                    date_posted: row.get(3)?,
                    location: row.get(4)?,
                    remuneration: row.get(5)?,
                    tags,
                    apply: row.get(7)?,
                    site: row.get(8)?,
                    rem_upper: row.get(9)?,
                    rem_lower: row.get(10)?,
                })
            })
            .map_err(|e| ErrorKind::SqliteQuery(e.to_string()))?;

        let mut cnt = 0;
        for job in jobs {
            let job = job.map_err(|e| ErrorKind::SqliteQuery(e.to_string()))?;
            println!("{:?}", job);
            cnt += 1
        }
        green_println!(format!("{cnt} jobs returned."));

        Ok(())
    }
}

impl Repl for SoftwareJobs {
    async fn init_repl() -> Result<(), ErrorKind> {
        let mut rl = DefaultEditor::new().map_err(|e| ErrorKind::Repl(e.to_string()))?;
        green_println!("Populating local database. This shouldn't take long...");
        Self::init_repo().await?;
        green_println!(
            "Population completed successfully! Welcome, please begin your job hunt by entering a query."
        );
        rl.load_history(".jobhunthistory").ok();

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(mut l) => {
                    rl.add_history_entry(&l)
                        .map_err(|e| ErrorKind::Repl(e.to_string()))?;
                    l = l.trim().to_lowercase();

                    match () {
                        () if l.starts_with("select jobs") => {
                            let conn = Connection::open("jobs.db")
                                .map_err(|e| ErrorKind::SqliteConnection(e.to_string()))?;
                            if let Err(err) = Self::select_and_display_jobs(conn, l) {
                                red_println!(err.to_string())
                            }
                        }
                        () if l == "refresh" => {
                            green_println!("Refreshing local database...");
                            Self::init_repo().await?;
                            green_println!(format!(
                                "Refresh completed successfully at {}",
                                Local::now().format("%d-%m-%Y %H:%M:%S")
                            ))
                        }
                        () if l == "exit" => break,
                        () => {
                            red_println!(format!(
                                "Does not compute! 🤖 \"{l}\" is not a valid query/command.",
                            ))
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    // CTRL-C
                    break;
                }
                Err(ReadlineError::Eof) => {
                    // CTRL-D
                    break;
                }
                Err(err) => {
                    red_println!(format!("An error has occurred: {err}"));
                    break;
                }
            }
        }

        rl.save_history(".jobhunthistory")
            .map_err(|e| ErrorKind::Repl(e.to_string()))?;
        green_println!("Thank you for using Job Hunt. Goodbye!");

        Ok(())
    }
}
