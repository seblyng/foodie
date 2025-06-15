pub mod auth;
pub mod friends;
pub mod ingredient;
pub mod oauth;
pub mod recipe;
pub mod users;

pub fn allowed_mails() -> Vec<String> {
    dotenv::var("ALLOWED_MAILS")
        .unwrap_or("sebastian@lyngjohansen.com".to_string())
        .split(',')
        .map(|it| it.to_string())
        .collect::<Vec<_>>()
}
