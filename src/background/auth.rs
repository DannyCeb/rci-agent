use env_file_reader::read_file; // To read environment variables from a file
use serde::Deserialize; // To deserialize the JSON response
use std::{collections::HashMap, sync::Arc, time::Duration}; // Standard libraries for collections, concurrency, and time
use tokio::{sync::Mutex, time::sleep}; // Tokio libraries for concurrency and async sleeping

use crate::utils::{error::RciError, stared_data::SharedData}; // Import the error type defined in the project

// Asynchronous function to authenticate and get the access token
pub async fn auth(shared_data: Arc<Mutex<SharedData>>) -> Result<(), RciError> {
    loop {
        // Read environment variables from the ".env" file
        let env_variables = read_file(".env").unwrap();
        let tenant_id = env_variables.get("TENANT_ID").unwrap(); // Get tenant ID
        let client_id = env_variables.get("APP_ID").unwrap(); // Get client ID
        let client_secret = env_variables.get("CLIENT_SECRET").unwrap(); // Get client secret
        let scope = env_variables.get("SCOPE").unwrap(); // Get the application scope

        // Construct the URL for the token request
        let url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
            tenant_id
        );

        // Prepare the parameters for the request
        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials"); // Grant type
        params.insert("client_id", client_id); // Client ID
        params.insert("client_secret", client_secret); // Client secret
        params.insert("scope", scope); // Scope of the request

        {
            let mut guard = shared_data.lock().await;
            // Create a new HTTP client
            let client = guard.get_http_client_by_mut_ref();
            // Make the POST request to get the token
            let res = client.post(&url).form(&params).send().await?;

            // Deserialize the response to get the access token
            let token: TokenResponse = res.json().await?;
            let token = token.access_token;
            // Update the token value in the Mutex

            guard.set_token(token);
        }
        // Wait 800 seconds before requesting the token again
        sleep(Duration::from_secs(800)).await;
    }
}

// Struct to deserialize the token response
#[derive(Debug, Deserialize)]
struct TokenResponse {
    //_token_type: String, // Type of token
    //_expires_in: u64, // Expiration time
    //_ext_expires_in, u64; // Extended expiration time
    access_token: String, // Access token
}
