use reqwest::{ self, Error };
use serde::{ Serialize, Deserialize };
use dotenv::dotenv;

pub const BASE_URL: &str = "https://aerodatabox.p.rapidapi.com/airports/%7BcodeType%7D/DME";

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Continent {
    pub code: String,
    pub name: String,

    #[serde(rename = "timeZone")]
    pub timezone: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Urls {
    #[serde(rename = "webSite")]
    pub website: String,

    pub wikipedia: String,
    pub twitter: String,

    #[serde(rename = "googleMaps")]
    pub googlemaps: String,

    #[serde(rename = "flightRadar")]
    pub flightradar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirportData {
    pub icao: String,
    pub iata: String,

    #[serde(rename = "localCode")]
    pub localcode: String,

    #[serde(rename = "shortName")]
    pub shortname: String,

    #[serde(rename = "fullName")]
    pub fullname: String,

    #[serde(rename = "muncipalityName")]
    pub muncipalityname: String,
    pub location: Location,
    pub country: Country,
    pub continent: Continent,
    pub urls: Urls,
}

impl AirportData {
    #[tokio::main]
    pub async fn get_airport_by_code() -> Result<(), Error> {
        dotenv().ok();
        let client = reqwest::Client::new();
        let airport_data: AirportData = client
            .get(BASE_URL)
            .header("X-RapidAPI-Key", std::env::var("APIKEY").expect("api key is required"))
            .header("X-RapidAPI-Host", std::env::var("APIHOST").expect("please cross check the host"))
            .send().await?
            .json().await?;

        println!("{:#?}", airport_data);
        Ok(())
    }
}