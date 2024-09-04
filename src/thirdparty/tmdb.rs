use serde::{Serialize, Deserialize};
use reqwest::Error;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub id: u32,
    pub release_date: Option<String>,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResultWrapper {
    pub page: u32,
    pub results: Vec<SearchResult>,
    pub total_pages: u32,
    pub total_results: u32,
}



pub async fn search(query: &String, page: u32) -> Result<SearchResultWrapper, Error> {
    let client = reqwest::Client::new();
    let request_url = format!("https://api.themoviedb.org/3/search/movie?query={search_query}&include_adult=false&language=en-US&page={page_no}", search_query=query, page_no=page);
    let token =  env::var("TMDB_TOKEN").unwrap();
    let response = client.get(&request_url)
        .bearer_auth(token)
        .send()
        .await?;
    let search_result: SearchResultWrapper = response.json().await?;
    Ok(search_result)
}
