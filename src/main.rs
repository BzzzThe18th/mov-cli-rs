//external refs
use reqwest::{blocking::ClientBuilder, header::{self, HeaderMap}, redirect};
use clap::{self, Parser};

//internal refs
mod scraping;
use scraping::search;
mod constants;
use constants::{AGENT, BRAFLIX_REFR};
mod player;
mod fzf;
use fzf::*;
mod structs;
use structs::Args;
use player::play_episode;

fn main() {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_str(AGENT).unwrap());
    headers.insert("Referer", header::HeaderValue::from_str(BRAFLIX_REFR).unwrap());
    headers.insert("Origin", header::HeaderValue::from_str(BRAFLIX_REFR).unwrap());
    let client = ClientBuilder::new()
        .redirect(redirect::Policy::none())
        .cookie_store(true)
        .default_headers(headers)
        .build()
        .unwrap();
    
    let args = Args::parse();
    let search = search(&client, &args.search).unwrap();
    if !args.first {
        display_series(&args, &client, &search.results)
    } else {
        let result = search.results[0].to_owned();
        if result.media_type == "tv" {
            if args.season == -1 {
                display_seasons(&args, &client, &result, &search.results);
            } else {
                if args.episode == -1 {
                    display_episodes(&args, &client, &result, args.season, &search.results);
                } else {
                    play_episode(&client, &result, args.season, args.episode);
                }
            }
        } else {
            play_episode(&client, &result, 1, 1);
        }
    }
}

