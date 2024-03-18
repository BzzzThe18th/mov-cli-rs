//external refs
use reqwest::{blocking::{Client, ClientBuilder}, header::{self, HeaderMap}, redirect};
use std::process::Command;
use clap::{self, Parser};
use fzf_wrapped::*;

//internal refs
mod scraping;
use scraping::{get_link, get_series_info, search, SeriesInfo};
mod constants;
use constants::{AGENT, BRAFLIX_REFR};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Media to search for
    #[arg(required=true)]
    search: String,
    /// Immediately use the first result instead of allowing navigation of results
    #[arg(short, long, default_value_t=false, required=false)]
    first: bool,
    /// Instead of opening MPV, print the playlist URL
    #[arg(long, default_value_t=false, required=false)]
    extract: bool,
    /// Download video to home instead of opening MPV
    #[arg(short, long, default_value_t=false, required=false)]
    download: bool,
    /// Season to search for (1 for movies)
    #[arg(short, long, default_value_t=-1, required=false)]
    season: i32,
    /// Episode to search for (1 for movies)
    #[arg(short, long, default_value_t=-1, required=false)]
    episode: i32,
}

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
    let search = search(&client, args.search).unwrap();
    if !args.first {
        display_series(client, args.season, args.episode, search.results)
    } else {
        let result = search.results[0].to_owned();
        if result.media_type == "tv" {
            display_seasons(client, result, args.episode);
        }
    }
}

fn display_series(client: Client, season: i32, episode: i32, search_results: Vec<scraping::SeriesResult>) {
    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .border_label("mov-cli-rs")
        .color(Color::Dark)
        .header("Select a series/movie")
        .header_first(true)
        .build()
        .unwrap();

    let mut series_names = vec![String::new(); (search_results.len() - 1) as usize];
    for i in 0..(search_results.len() - 1) as usize {
        let result = search_results[i].to_owned();
        if search_results[i].title.is_none() {series_names[i] = result.name.unwrap()} else {series_names[i] = result.title.unwrap()}
        
        let mut append_str = String::new();
        if result.first_air_date.is_none() && result.release_date.is_none() {
            append_str = " (N/A)".to_string();
        } else if !result.first_air_date.is_none() {
            let mut date = result.first_air_date.unwrap();
            if !date.is_empty() {
                let air_year = format!(" ({})", date.split_at_mut(4).0);
                append_str = air_year;
            }
        } else if !result.release_date.is_none() {
            let mut date = result.release_date.unwrap();
            if !date.is_empty() {
                let release_year = format!(" ({})", date.split_at_mut(4).0);
                append_str = release_year;
            }
        }
        
        series_names[i].push_str(append_str.as_str());
    }
    let series_name: String = run_with_output(fzf, series_names).unwrap();
    let mut series_index = 0;
    for j in 0..search_results.len() {
        let result = search_results[j].to_owned();
        let mut append_str = String::new();
        if result.first_air_date.is_none() && result.release_date.is_none() {
            append_str = " (N/A)".to_string();
        } else if !result.first_air_date.is_none() {
            let mut date = result.first_air_date.unwrap();
            if !date.is_empty() {
                let air_year = format!(" ({})", date.split_at_mut(4).0);
                append_str = air_year;
            }
        } else if !result.release_date.is_none() {
            let mut date = result.release_date.unwrap();
            if !date.is_empty() {
                let release_year = format!(" ({})", date.split_at_mut(4).0);
                append_str = release_year;
            }
        }

        let mut name = if result.title.is_none() {result.name.unwrap()} else {result.title.unwrap()};
        name.push_str(&append_str);
        if name == series_name {series_index=j}
    }
    let series = search_results[series_index].to_owned();
    
    if season == -1 {
        if series.media_type == "tv" {
            display_seasons(client, series, episode);
        } else {
            play_episode(client, series, 1, 1);
        }
    } else {
        //episode is not specified
        if episode == -1 {
            display_episodes(client, series, season);
        } else {
            play_episode(client, series, season, episode);
        }
    }
}

fn display_seasons(client: Client, series: scraping::SeriesResult, episode: i32) {
    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .border_label("mov-cli-rs")
        .color(Color::Bw)
        .header("Select a season")
        .header_first(true)
        .build()
        .unwrap();

    let series_info: SeriesInfo = get_series_info(&client, series.id).unwrap();
    let seasons = series_info.seasons;

    let mut season_names = vec![String::new(); seasons.clone().len()];
    for i in 0..seasons.clone().len() {
        let season = seasons[i].to_owned();
        season_names[i] = season.name;
        
        let append_str = format!(" ({})", season.air_date.split_at(4).0);
        
        season_names[i].push_str(append_str.as_str());
    }

    let season_name: String = run_with_output(fzf, season_names).unwrap();

    let series_index = seasons.iter().position(|r| [r.name.to_owned(), " (".to_string(), r.air_date.split_at(4).0.to_owned(), ")".to_string()].concat() == season_name).unwrap();
    let season = seasons[series_index].clone();

    //episode is not specified
    if episode == -1 {
        display_episodes(client, series, season.season_number);
    } else {
        play_episode(client, series, season.season_number, episode)
    }
}

fn display_episodes(client: Client, series: scraping::SeriesResult, season: i32) {
    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .border_label("mov-cli-rs")
        .color(Color::Bw)
        .header("Pick an episode")
        .header_first(true)
        .build()
        .unwrap();

    let episodes = scraping::get_season_info(&client, series.id, season).unwrap().episodes;

    let mut episode_nums = vec![String::new(); episodes.len()];
    for i in 0..episodes.len() {
        episode_nums[i] = episodes[i].episode_number.to_string();
    }

    let episode_num = run_with_output(fzf, episode_nums).unwrap();

    play_episode(client, series, season, episode_num.parse::<i32>().unwrap());
}

fn play_episode(client: Client, series: scraping::SeriesResult, season: i32, episode: i32) {
    let media_url = get_link(client, series.to_owned(), season, episode).unwrap();
    Command::new("mpv")
        .arg(&media_url)
        .arg(format!("--force-media-title={0}", if series.media_type == "tv" {series.name.unwrap()} else {series.title.unwrap()}))
        .arg("--no-terminal")
        .spawn()
        .expect("Failed to open MPV");
}