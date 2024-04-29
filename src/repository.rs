/// The Job struct is the repository primitive.
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