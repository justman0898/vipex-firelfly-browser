use simple_logger::SimpleLogger;
use crate::exceptions::custom_error::AppError;

mod url;
mod exceptions;
mod models;
mod dns;

fn main()-> Result<(), AppError> {

    dotenvy::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL")?;

    eprintln!("Using redis url: {}", redis_url);


    Ok(())

}
