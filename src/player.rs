use std::process::Command;
use reqwest::blocking::Client;
use crate::structs::SeriesResult;
use crate::scraping::get_link;

pub fn play_episode(client: &Client, series: &SeriesResult, season: i32, episode: i32) {
    let media_url = get_link(client, series, season, episode).unwrap();
    Command::new("mpv")
        .arg(&media_url)
        .arg(format!("--force-media-title={0}", if series.media_type == "tv" {series.name.as_ref().unwrap()} else {series.title.as_ref().unwrap()}))
        .arg("--no-terminal")
        .spawn()
        .expect("Failed to open MPV");
}