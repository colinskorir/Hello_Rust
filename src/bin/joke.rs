use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Joke {
    setup: String,
    punchline: String,
}

#[tokio::main]
async fn main() {
    if let Err(err) = fetch_and_tell_joke().await {
        eprintln!("⚠️  Could not fetch a joke: {err}");
    }
}

async fn fetch_and_tell_joke() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching a programming joke...");

    let client = Client::new();
    let url = "https://official-joke-api.appspot.com/jokes/programming/random";

    let jokes: Vec<Joke> = client.get(url).send().await?.json().await?;
    if let Some(joke) = jokes.first() {
        println!("\n{}\n{}", joke.setup.trim(), joke.punchline.trim());
    } else {
        println!("The joke API returned an empty list. Try again!");
    }

    Ok(())
}

