use fzf_wrapped::*;
use crate::player::play_episode;
use crate::structs::{Args, SeriesInfo, SeriesResult};
use crate::scraping::{search, get_series_info, get_season_info};
use reqwest::blocking::Client;

static SERIES_OPTIONS: [&str; 2] = ["search again", "quit"];
static OTHER_OPTIONS: [&str; 3] = ["back", "search again", "quit"];

fn prompt_search(args: &Args, client: &Client) {
    print!("Search for: ");
    let input: &String = &text_io::read!("{}\n");
    let search = search(&client, input).unwrap();
    if !args.first {
        display_series(args, client, &search.results)
    } else {
        let result = search.results[0].to_owned();
        if result.media_type == "tv" {
            if args.season == -1 {
                display_seasons(args, client, &result, &search.results);
            } else {
                if args.episode == -1 {
                    display_episodes(args, client, &result, args.season, &search.results);
                } else {
                    play_episode(client, &result, args.season, args.episode);
                }
            }
        } else {
            play_episode(client, &result, 1, 1);
        }
    }
}

pub fn display_series(args: &Args, client: &Client, search_results: &Vec<SeriesResult>) {
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

    let mut _series_options = vec![String::new(); SERIES_OPTIONS.len()];
    for j in 0.._series_options.len() {
        _series_options[j] = SERIES_OPTIONS[j].to_string();
    }

    series_names.append(&mut _series_options);
    let series_name: String = run_with_output(fzf, series_names).unwrap();

    if SERIES_OPTIONS.contains(&series_name.as_str()) {
        match series_name.as_str() {
            "search again" => {
                prompt_search(args, client);
            }
            "quit" => {
                return;
            }
            _ => {
                panic!();
            }
        }
    }

    let mut series_index = 0;
    for k in 0..search_results.len() {
        let result = search_results[k].to_owned();
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
        if name == series_name {series_index=k}
    }
    let series = search_results[series_index].to_owned();
    
    if args.season == -1 {
        if series.media_type == "tv" {
            display_seasons(args, client, &series, &search_results);
        } else {
            play_episode(client, &series, 1, 1);
        }
    } else {
        //episode is not specified
        if args.episode == -1 {
            display_episodes(args, client, &series, args.season, &search_results);
        } else {
            play_episode(client, &series, args.season, args.episode);
        }
    }
}

pub fn display_seasons(args: &Args, client: &Client, series: &SeriesResult, search_results: &Vec<SeriesResult>) {
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
    
    let mut _season_options = vec![String::new(); OTHER_OPTIONS.len()];
    for j in 0.._season_options.len() {
        _season_options[j] = OTHER_OPTIONS[j].to_string();
    }
    season_names.append(&mut _season_options);
    let season_name: String = run_with_output(fzf, season_names).unwrap();

    if OTHER_OPTIONS.contains(&season_name.as_str()) {
        match season_name.as_str() {
            "back" => {
                display_series(args, client, &search_results);
            }
            "search again" => {
                prompt_search(args, client);
            }
            "quit" => {
                return;
            }
            _ => {
                panic!();
            }
        }
    }

    let series_index = seasons.iter().position(|r| [r.name.to_owned(), " (".to_string(), r.air_date.split_at(4).0.to_owned(), ")".to_string()].concat() == season_name).unwrap();
    let season = seasons[series_index].clone();

    //episode is not specified
    if args.episode == -1 {
        display_episodes(args, client, series, season.season_number, &search_results);
    } else {
        play_episode(client, series, season.season_number, args.episode);
    }
}

pub fn display_episodes(args: &Args, client: &Client, series: &SeriesResult, season: i32, search_results: &Vec<SeriesResult>) {
    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .border_label("mov-cli-rs")
        .color(Color::Bw)
        .header("Pick an episode")
        .header_first(true)
        .build()
        .unwrap();

    let episodes = get_season_info(&client, series.id, season).unwrap().episodes;

    let mut episode_nums = vec![String::new(); episodes.len()];
    for i in 0..episodes.len() {
        episode_nums[i] = episodes[i].episode_number.to_string();
    }

    let mut _episode_options = vec![String::new(); OTHER_OPTIONS.len()];
    for j in 0.._episode_options.len() {
        _episode_options[j] = OTHER_OPTIONS[j].to_string();
    }
    episode_nums.append(&mut _episode_options);

    let episode_num = run_with_output(fzf, episode_nums).unwrap();

    if OTHER_OPTIONS.contains(&episode_num.as_str()) {
        match episode_num.as_str() {
            "back" => {display_seasons(args, client, &series, search_results)}
            "search again" => {prompt_search(args, client)}
            "quit" => {return;}
            _ => {panic!();}
        }
    }

    play_episode(client, series, season, episode_num.parse::<i32>().unwrap());
}