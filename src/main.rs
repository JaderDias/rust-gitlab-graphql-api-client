extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ApiResponse {
    data: Option<serde_json::Value>,
    errors: Option<Vec<Error>>,
}

#[derive(Deserialize)]
struct Error {
    message: String,
}

fn main() -> Result<(), reqwest::Error> {
    // Your GitLab GraphQL API endpoint URL
    let api_url = "https://gitlab.com/api/graphql";

    // Your GitLab personal access token for authentication
    let access_token = "YOUR_ACCESS_TOKEN";

    // Define your GraphQL query
    let query = r#"{
        users(usernames: ["user1", "user3"]) {
          pageInfo {
            endCursor
            startCursor
            hasNextPage
          }
          nodes {
            id
            username
            publicEmail
            location
            webUrl
            userPermissions {
              createSnippet
            }
          }
        }
      }      
    "#;

    // Create a Reqwest client
    let client = reqwest::blocking::Client::new();

    // Build the request
    let resp = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&json!({ "query": query }))
        .send()?;

    // Check the response status code
    if resp.status().is_success() {
        // Deserialize the JSON response
        let api_response: ApiResponse = resp.json()?;
        if let Some(data) = api_response.data {
            println!("Data: {:?}", data);
        } else if let Some(errors) = api_response.errors {
            for error in errors {
                println!("GraphQL Error: {}", error.message);
            }
        }
    } else {
        println!("Request failed with status code: {:?}", resp.status());
    }

    Ok(())
}
