use reqwest::{blocking::Client, Error};
use serde::{Deserialize, Serialize};

pub use crate::files;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRepositoriesResponse {
    pub id: i32,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub ssh_url: String,
    pub default_branch: String
}

pub fn get_repositories() -> Result<Vec<GetRepositoriesResponse>, Error> {
    let config = files::read_config();
    
    let client = Client::new();

    let url = format!("https://api.github.com/users/{}/repos", config.user);
    
    let token = format!("Bearer {}", config.github_token);    
    
    let req = client.get(url)
    .header("Authorization", token.clone())
    .header("User-Agent", "lulz")
    .build()?;

    let response = client.execute(req)?;

    
    if response.status().is_success() {
        let body = response.json::<serde_json::Value>()?;
        let api_response: Vec<GetRepositoriesResponse> = serde_json::from_value(body).unwrap();
        
        return Ok(api_response);
    }

    panic!()
}