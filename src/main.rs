//external refs
use reqwest::{blocking, header, redirect};
use std::process::Command;
use clap::{self, Parser};

//internal refs
mod scraping;
use scraping::{get_link, search};
mod constants;
use constants::{AGENT, BRAFLIX_REFR};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Use last found english subtitles
    #[arg(long, default_value_t=true)]
    ensub: bool,
    /// Immediately use the first result instead of allowing navigation of results
    #[arg(short, long, default_value_t=false)]
    first: bool,
    /// Media to search for
    #[arg(long)]
    search: String,
    /// Season to search for (1 for movies)
    #[arg(short, long, default_value_t=0)]
    season: i32,
    /// Episode to search for (1 for movies)
    #[arg(short, long, default_value_t=0)]
    episode: i32,
}

fn main() {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_str(AGENT).unwrap());
    headers.insert("Referer", header::HeaderValue::from_str(BRAFLIX_REFR).unwrap());
    headers.insert("Origin", header::HeaderValue::from_str(BRAFLIX_REFR).unwrap());
    let client = blocking::Client::builder()
        .redirect(redirect::Policy::none())
        .cookie_store(true)
        .default_headers(headers)
        .build()
        .unwrap();

    let args = Args::parse();
    if args.search.is_empty() {
        //display search menu
    } else {
        let search = search(client.clone(), args.search).unwrap();
        if args.first {
            if args.season == 0 {
                //display season select menu
                //when season is selected, if episode is 0, display episode select menu, otherwise, play episode
            } else {
                if args.episode == 0 {
                    //display episode select menu
                } else {
                    let media_url = get_link(client, search.results[0].to_owned(), /*search.results[0].id.to_string(),*/ args.season.to_string(), args.episode.to_string()).unwrap();
                    Command::new("mpv")
                        .arg(&media_url)
                        .arg(format!("--force-media-title={0}", search.results[0].name.clone().unwrap()))
                        .arg("--no-terminal")
                        .spawn()
                        .expect("Failed to open MPV");
                    println!("attempted to open mpv with url {}", media_url);
                }
            }
        }
    }
}