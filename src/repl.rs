use chrono::Local;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::repository::SoftwareJobs;
use crate::ErrorKind;

/// This trait must be implemented by the specific job repo struct to be used in Job Hunt (e.g. SoftwareJobs).
#[allow(async_fn_in_trait)]
pub trait Repl {
    /// Initializes a repository for the job repo type that is implementing this trait; then
    /// initializes the REPL and parses queries.
    async fn init_repl() -> Result<(), ErrorKind>;
}

macro_rules! printreplln {
    ($msg:expr) => {{
        println!("{}", $msg.bold().green())
    }};
}

impl Repl for SoftwareJobs {
    async fn init_repl() -> Result<(), ErrorKind> {
        let mut rl = DefaultEditor::new().map_err(|e| ErrorKind::Repl(e.to_string()))?;
        rl.load_history(".jobhunthistory")
            .map_err(|e| ErrorKind::Repl(e.to_string()))?;

        printreplln!("Populating local database. This shouldn't take long...");
        Self::init_repo().await?;
        printreplln!(
            "Population completed successfully! Welcome, please begin your job hunt by entering a query."
        );

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(mut l) => {
                    rl.add_history_entry(&l)
                        .map_err(|e| ErrorKind::Repl(e.to_string()))?;
                    l = l.trim().to_lowercase();

                    match () {
                        () if l.starts_with("select jobs") => {
                            // fetch and display jobs

                            printreplln!(format!("{} jobs returned.", 5));
                        }
                        () if l == "refresh" => {
                            printreplln!("Refreshing local database...");
                            Self::init_repo().await?;
                            printreplln!(format!(
                                "Refresh completed successfully at {}",
                                Local::now().format("%d-%m-%Y %H:%M:%S")
                            ))
                        }
                        () if l == "exit" => break,
                        () => {
                            printreplln!(format!(
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
                    printreplln!(format!("An error has occurred: {err}"));
                    break;
                }
            }
        }

        printreplln!("Thank you for using Job Hunt. Goodbye!");
        rl.save_history(".jobhunthistory")
            .map_err(|e| ErrorKind::Repl(e.to_string()))?;

        Ok(())
    }
}
