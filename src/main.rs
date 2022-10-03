// use serde::{Serialize, Deserialize};


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    const BASEURL: &str = "https://aerodatabox.p.rapidapi.com";
    let borrowed_query: &str ="/flights/%7BsearchBy%7D/KL1395/2020-06-10";

    let client = reqwest::Client::new();
    let aeros = client.get(format!("{}{}", BASEURL, borrowed_query))
    .header("X-RapidAPI-Key", "53fd0041f2msh8c3ffa5b5508be0p152202jsn9a0f742df4a8")
    .header("X-RapidAPI-Host", "aerodatabox.p.rapidapi.com")
    .send()
    .await?
    .text()
    .await?;

    println!("{:#?}", aeros);

    Ok(())
}