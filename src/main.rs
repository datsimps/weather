use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Weather {
    pub name: String,
    pub wind_speed: String,
    pub temperature: String,
    pub rain_chance: String,
    pub forecast: String,
    pub tomorrow_forecast: String,
}

impl Weather {
    pub fn build(input: serde_json::Value) -> Weather {
        let name = input["properties"]["periods"][0]["name"].to_string();
        let wind_speed = input["properties"]["periods"][0]["windSpeed"].to_string();
        let temperature = input["properties"]["periods"][0]["temperature"].to_string();
        // if rain chance returns "null" then it means "0" percent chance
        let rain_chance = input["properties"]["periods"][0]["probabilityOfPrecipitation"]["value"].to_string();
        let forecast = input["properties"]["periods"][0]["detailedForecast"].to_string();
        let tomorrow_forecast = input["properties"]["periods"][1]["detailedForecast"].to_string();

        Weather {
            name,
            wind_speed,
            temperature,
            rain_chance,
            forecast,
            tomorrow_forecast,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_user_agent: &str = concat!(
        "Daniel Simpson",
        "/",
        "daniel.t.simpson16@gmail.com",
    );

    let url = "https://api.weather.gov/gridpoints/IND/61,66/forecast";

    let client = reqwest::Client::builder()
        .user_agent(app_user_agent)
        .build()?;

    let date = match client.get(url).send().await {
        Ok(resp) => {
            let json: serde_json::Value = resp.json().await?;
            let date = Weather::build(json);
//            println!("name: {}", date.name);
//            println!("wind speed: {}",date.wind_speed);
//            println!("temp: {}",date.temperature);
//            println!("rain chance: {}", date.rain_chance);
//            println!("forecast: {}",date.forecast);
//            println!("tomorrow: {}", date.tomorrow_forecast);
            date
        }
        Err(err) => {
            println!("Reqwest error: {}", err);
            Weather {
                name: "null".to_string(),
                wind_speed: "null".to_string(),
                temperature: "null".to_string(),
                rain_chance: "null".to_string(),
                forecast: "null".to_string(),
                tomorrow_forecast: "null".to_string(),

            }
        }
    };

            println!("name: {}", date.name);
    Ok(())
}
