// external refs
use std::process::Command;
use reqwest::blocking::Client;
use termsize;
use fzf_wrapped::*;

// internal refs
use crate::structs::{Args, SeriesResult};
use crate::scraping::get_link;
use crate::fzf::prompt_search;

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
        .arg(format!("--force-media-title={0}", if series.title.is_none() {series.name.to_owned().unwrap()} else {series.title.to_owned().unwrap()}))
        .arg("--no-terminal")
        .spawn()
        .expect("Failed to open MPV");
    }

    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .border_label("mov-cli-rs")
        .color(Color::Dark)
        .header("Options")
        .header_first(true)
        .build()
        .unwrap();

    let options = vec!["next", "replay", "previous", "search again", "quit"];
    let selected_option = run_with_output(fzf, options).unwrap();

    match selected_option.as_str() {
        "next" => return play_episode(args, client, series, season, episode + 1),
        "replay" => return play_episode(args, client, series, season, episode),
        "previous" => return play_episode(args, client, series, season, episode - 1),
        "search again" => prompt_search(args, client),
        "quit" => return,
        "" => return,
        _ => panic!(),
    }
}