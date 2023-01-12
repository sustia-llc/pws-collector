use dotenv::dotenv;
use reqwest::Client;
use tokio::{self, time};
use tracing::{debug, error, info};

use crate::{models::PwsdataResponse, settings::get_settings};
mod database;
mod logger;
mod models;
mod services;
mod settings;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    logger::setup();
    database::setup()
        .await
        .expect("Failed to setup database connection");
    services::setup().await.expect("Failed to setup services");
    let settings = get_settings();
    info!("Running in {} mode", &settings.environment);
    let connection = database::get_connection();
    debug!("Database connected: {}", &connection.name());

    let api_key = format!("{}", std::env::var("API_KEY")?);
    let stations = format!("{}", std::env::var("STATIONS")?);
    let stations: Vec<String> = stations.split(",").map(|s| s.to_string()).collect();
    let urls: Vec<String> = stations.iter().map(|station| format!("https://api.weather.com/v2/pws/observations/current?stationId={}&format=json&units=m&apiKey={}", station, api_key)).collect();

    let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";

    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .connect_timeout(std::time::Duration::from_secs(60))
        .timeout(std::time::Duration::from_secs(600))
        .build()
        .unwrap();

    let futures = urls.iter().map(|url| fetch(&client, url));
    let _ = futures::future::join_all(futures).await;

    Ok(())
}

async fn fetch(client: &Client, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = time::interval(std::time::Duration::from_secs(60 * 15));

    loop {
        interval.tick().await;

        let res = client.get(url).send().await.unwrap();
        match res.status() {
            reqwest::StatusCode::OK => {
                let json = &res.json::<PwsdataResponse>().await;
                match json {
                    Ok(json) => {
                        services::insert_pwsdata(json.to_owned()).await.unwrap();
                    }
                    Err(e) => {
                        error!("Error parsing res.json: {}", e);
                    }
                }
            }
            status => {
                // parse station from url
                let station = url.split("stationId=").collect::<Vec<&str>>()[1]
                    .split("&")
                    .collect::<Vec<&str>>()[0];
                error!(
                    "status: {}, path: {}, station: {}",
                    status,
                    res.url().path(),
                    station
                )
            }
        }
    }
}
