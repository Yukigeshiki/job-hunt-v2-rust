# job-hunt-v2-rust

A locally run recent job aggregator written in Rust, with a SQLite database, and REPL. Jobs are scraped from job sites and added to the database at start-up and then each time the application is refreshed.

To query jobs you use simplified SQLite syntax. For example, to fetch all senior jobs and order them by date posted you would enter:

```SQL
select jobs where title like "%senior%" order by date_posted;
```

To refresh the database enter:

```
refresh
```

And to exit you can use `CTRL-C` or enter:

```
exit
```

There are currently scrapers for a number of Web3 job sites. I will be adding others sites in the future too. Sites included at the moment:
- https://web3.career/
- https://cryptojobslist.com/
- https://jobs.solana.com/jobs
- https://careers.substrate.io/jobs
- https://careers.near.org/jobs

### How to Run Job Hunt

First make sure you have Rust installed. To do this you can follow the instructions found [here](https://www.rust-lang.org/tools/install).

Once installation is complete, clone this repo and from the root directory run:

```bash
cargo build --release
```

Then run:

```bash
./target/release/jobhunt
```

You should see the below info messages followed by a prompt. Happy Job Hunting!

```
Populating local database. This shouldn't take long...
Population completed successfully! Welcome, please begin your job hunt by entering a query.
```

This project is usable but still under Construction! 🚧
