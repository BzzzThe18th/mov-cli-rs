use std::process::Command;
use reqwest::blocking::Client;
use crate::structs::{Args, SeriesResult};
use crate::scraping::get_link;
use termsize;

pub fn play_episode(args: &Args,client: &Client, series: &SeriesResult, season: i32, episode: i32) {
    let media_url = get_link(client, series, season, episode).unwrap();
    if args.extract {
        let cols = termsize::get().unwrap().cols;

        println!("{}",vec!["=";cols.into()].concat());
        println!("{}",media_url);
        println!("{}",vec!["=";cols.into()].concat());
    } else {
    Command::new("mpv")
        .arg(&media_url)
        .arg(format!("--force-media-title={0}", if series.media_type == "tv" {series.name.as_ref().unwrap()} else {series.title.as_ref().unwrap()}))
        .arg("--no-terminal")
        .spawn()
        .expect("Failed to open MPV");
    }
}