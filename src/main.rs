use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

impl From<String> for CustomError {
    fn from(err: String) -> Self {
        CustomError(err)
    }
}

async fn request(method: &str, url: &str, body: Option<String>, headers: Option<HeaderMap>) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let request_builder = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "PATCH" => client.patch(url),
        "DELETE" => client.delete(url),
        _ => return Err(CustomError(format!("Invalid HTTP method: {}", method)).into()),
    };
    let mut request = match body {
        Some(body) => request_builder.body(body),
        None => request_builder,
    };
    if let Some(headers) = headers {
        request = request.headers(headers);
    }
    let response = request.send().await?.text().await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Define the API endpoint URL and request headers
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    // Make a GET request
    let response = request("GET", url, None, Some(headers)).await;
    println!("GET response: {:?}", response);
    
    Ok(())
}