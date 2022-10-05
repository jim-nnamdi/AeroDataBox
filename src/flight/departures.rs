use serde::{ Serialize, Deserialize };
use reqwest::Error;
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize)]
pub struct Departures<T> {
    departures: Vec<T>,
}

impl<T> Departures<T> {
    #[tokio::main]
    pub async fn flight_departure_dates() -> Result<(), Error> {
        dotenv().ok();
        const BASE_URL: &str =
            "https://aerodatabox.p.rapidapi.com/flights/number/KL1395/dates/2020-06-01/2020-06-15";

        let aerobox_client = reqwest::Client::new();

        let flight_departure_dates: Vec<Departures<String>> = aerobox_client
            .get(BASE_URL)
            .header("X-RapidAPI-Key", std::env::var("APIKEY").expect("api key is required"))
            .header("X-RapidAPI-Host", std::env::var("APIHOST").expect("please cross check the host"))
            .send().await?
            .json().await?;

        println!("{:#?}", flight_departure_dates);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_retrieve_departures() -> Result<(), Error> {
        let result = Departures::<String>::flight_departure_dates();
        match result {
            Err(e) => {
                Err(e)
            }
            _ => Ok(()),
        }
    }
}