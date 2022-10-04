use serde::{ Serialize, Deserialize };
use reqwest::Error;

pub const API_KEY: &str = "APIKEY";
pub const API_HOST: &str = "APIHOST";

#[derive(Debug, Serialize, Deserialize)]
pub struct Departures<T> {
    departures: Vec<T>,
}

impl<T> Departures<T> {
    #[tokio::main]
    pub async fn flight_departure_dates() -> Result<(), Error> {
        const BASE_URL: &str =
            "https://aerodatabox.p.rapidapi.com/flights/number/KL1395/dates/2020-06-01/2020-06-15";

        let aerobox_client = reqwest::Client::new();

        let flight_departure_dates: Vec<Departures<String>> = aerobox_client
            .get(BASE_URL)
            .header("X-RapidAPI-Key", API_KEY)
            .header("X-RapidAPI-Host", API_HOST)
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