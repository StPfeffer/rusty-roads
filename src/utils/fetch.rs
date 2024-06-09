use std::collections::HashMap;

use serde::Deserialize;

use crate::db::country::CountryExt;
use crate::error::HttpError;
use crate::models::country::Country;
use crate::AppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiCountry {
    name: String,
    alpha2_code: String,
    alpha3_code: String,
    numeric_code: String,
}

pub async fn fetch_countries(app_state: &AppState) -> Result<(), HttpError> {
    let url = "https://countryapi.io/api/all?apikey=".to_owned()
        + &*std::env::var("COUNTRYAPI_KEY").expect("COUNTRYAPI_KEY must be set");

    let response: HashMap<String, ApiCountry> = reqwest::get(&url)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?
        .json()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let countries: Vec<Country> = response
        .into_values()
        .map(|api_country| Country {
            id: uuid::Uuid::new_v4(),
            name: api_country.name,
            alpha_2: api_country.alpha2_code,
            alpha_3: api_country.alpha3_code,
            numeric_3: api_country.numeric_code,
        })
        .collect();

    for api_country in countries {
        let country = app_state
            .db_client
            .get_country(
                None,
                None,
                Some(&api_country.alpha_2),
                Some(&api_country.alpha_3),
                Some(&api_country.numeric_3),
            )
            .await
            .map_err(|e| HttpError::server_error(e.to_string()))?;

        if let Some(found_country) = country {
            app_state
                .db_client
                .update_country(
                    found_country.id,
                    &api_country.name,
                    &api_country.alpha_2,
                    &api_country.alpha_3,
                    &api_country.numeric_3,
                )
                .await
                .map_err(|e| HttpError::server_error(e.to_string()))?;
        } else {
            app_state
                .db_client
                .save_country(
                    api_country.name,
                    api_country.alpha_2,
                    api_country.alpha_3,
                    api_country.numeric_3,
                )
                .await
                .map_err(|e| HttpError::server_error(e.to_string()))?;
        }
    }

    Ok(())
}
