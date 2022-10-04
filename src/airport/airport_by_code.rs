use reqwest::{ self, Error };

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    code: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Continent {
    code: String,
    name: String,

    #[serde(rename = "timeZone")]
    timezone: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Urls {
    #[serde(rename = "webSite")]
    website: String,

    wikipedia: String,
    twitter: String,

    #[serde(rename = "googleMaps")]
    googlemaps: String,

    #[serde(rename = "flightRadar")]
    flightradar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirportData {
    icao: String,
    iata: String,

    #[serde(rename = "localCode")]
    localcode: String,

    #[serde(rename = "shortName")]
    shortname: String,

    #[serde(rename = "fullName")]
    fullname: String,

    #[serde(rename = "muncipalityName")]
    muncipalityname: String,
    location: Location,
    country: Country,
    continent: Continent,
    urls: Urls,
}

pub fn get_airport_by_code() -> Result<(), Error> {}