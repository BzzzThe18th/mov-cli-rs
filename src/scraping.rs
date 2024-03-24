use reqwest::blocking;
use std::error::Error;
use std::time::Duration;
use crate::structs::{Args, APISourceResults, SearchResults, SeasonData, SeriesInfo, SeriesResult};
use crate::constants::{self, BRAFLIX_API, TMDB_API, TMDB_BRAFLIX_API_KEY};

fn try_get_link_from_api_source(provider: String, args: &Args, client: &blocking::Client, series: &SeriesResult, season: &String, episode: &String) -> Result<String, Box<dyn Error>> {
    print!("trying to fetch from provider {}... ", provider);
    let attempt = || -> Result<String, Box<dyn Error>> {
        let mut year = String::new();
        let release_date = &series.release_date;
        let air_date = &series.first_air_date;
        if !release_date.is_none() {
            let date = release_date.as_ref().unwrap();
            if !date.is_empty() {
                year.push_str("&year=");
                year.push_str(date.split_at(4).0);
            }
        } else if !air_date.is_none() {
            let date = air_date.as_ref().unwrap();
            if !date.is_empty() {
                year.push_str("&year=");
                year.push_str(date.split_at(4).0);
            }
        }
        let url = format!("{0}/{1}/sources-with-title?title={2}{3}&mediaType={4}&episodeId={5}&seasonId={6}&tmdbId={7}", BRAFLIX_API, provider, if series.name.is_none() {series.title.to_owned().unwrap()} else {series.name.to_owned().unwrap()}, year, series.media_type, episode, season, series.id);
        let json: Result<APISourceResults, reqwest::Error> = client.get(&url)
            .send()?
            .json();
        
        /* 
            this'll handle the json decoding safely, unsure why but the safe handling does not cover this? tested with alternate API URL and got 'failed - panicked`
            for every source, does reqwest's serde feature somehow unintentionally pass by this?
            ps: this happened because flixhq is down, braflix seems to have some way to check what's up and not up, need to implement that at some point, would
            also give a chance to make it prettier
            
            bzzzthe18th
        */
        if json.is_err() {
            print!("failed - unknown error");
            return Ok(String::new());
        }
        let safe_json = json.unwrap();
        if !safe_json.sources.is_none() {
            let mut sources = safe_json.sources.unwrap();
            if sources.len() <= 0 {
                print!("failed - sources len 0\n");
                return Ok(String::new());
            }
            sources.sort_by_key(|k| k.quality.to_lowercase().replace("auto", "0").replace("p", "").parse::<i32>().unwrap());
            sources.reverse();

            // TODO: implement subtitle stuff
            // let subs = json.subtitles.unwrap();
            // for sub_track in 0..subs.len() {
            //     println!("Subtitle found with language string {}", subs[sub_track].clone().lang.unwrap_or_else(|| {subs[sub_track].clone().language.expect("Failed to get both languages for sub track")}));
            // }

            let source = if args.quality.is_empty() {&sources[0]} else {&sources[sources.iter().position(|r| r.quality.to_lowercase() == args.quality).unwrap_or_else(||{0})]}; 

            if source.url.ends_with(".m3u8") {
                print!("success - playing ({})\n", sources[0].quality.to_lowercase());
                std::thread::sleep(Duration::from_millis(2500));
                return Ok(source.url.to_string())
            } else {
                print!("failed - encrypted\n");
                return Ok(String::new());
            }
        }
        print!("failed - no sources field\n");
        Ok(String::new())
    };
    let get_safely = || -> Result<String, Box<dyn Error>> {
        let out = attempt().unwrap_or_else(|_| {
            print!("failed - panicked\n");
            String::new()
        });
        Ok(out)
    };
    Ok(get_safely().unwrap())
}

fn get_link_from_api_source(args: &Args, client: &blocking::Client, series: &SeriesResult, season: &String, episode: &String) -> Result<String, Box<dyn Error>> {
    let res = try_get_link_from_api_source("flixhq".to_string(), args, client, series, season, episode).and_then(|result|
        if result.ends_with(".m3u8") { Ok(result) } else { 
            try_get_link_from_api_source("vidsrc".to_string(), args, client, series, season, episode).and_then(|vidsrc_result|{
                if vidsrc_result.ends_with(".m3u8") {Ok(vidsrc_result)} else {
                    try_get_link_from_api_source("vidsrcto".to_string(), args, client, series, season, episode).and_then(|vidsrcto_result|{
                        if vidsrcto_result.ends_with(".m3u8") {Ok(vidsrcto_result)} else {
                            try_get_link_from_api_source("superstream".to_string(), args, client, series, season, episode).and_then(|superstream_result|{
                                if superstream_result.ends_with(".m3u8") { Ok(superstream_result) } else {
                                    try_get_link_from_api_source("febbox".to_string(), args, client, series, season, episode).and_then(|febbox_result|{
                                        if febbox_result.ends_with(".m3u8") { Ok(febbox_result) } else {
                                            try_get_link_from_api_source("overflix".to_string(), args, client, series, season, episode).and_then(|overflix_result|{
                                                if overflix_result.ends_with(".m3u8") {Ok(overflix_result)} else {
                                                    try_get_link_from_api_source("visioncine".to_string(), args, client, series, season, episode).and_then(|_visioncine_result|{
                                                        Ok(String::new())
                                                    })
                                                }
                                            })
                                        }
                                    })
                                }
                            })
                        }
                    })
                }
            })
        }
    );
    Ok(res.unwrap())
}

pub fn get_season_info(client: &blocking::Client, show_id: i32, season: i32) -> Result<SeasonData, Box<dyn Error>> {
    let url = format!("{}/3/tv/{}/season/{}?api_key={}", constants::TMDB_API, show_id, season, constants::TMDB_BRAFLIX_API_KEY);
    let json: SeasonData = client.get(url)
        .send()?
        .json()
        .unwrap();
    Ok(json)
}

pub fn get_series_info(client: &blocking::Client, show_id: i32) -> Result<SeriesInfo, Box<dyn Error>> {
    let url = format!("{}/3/tv/{}?api_key={}", constants::TMDB_API, show_id, constants::TMDB_BRAFLIX_API_KEY);
    let json: SeriesInfo = client.get(url)
        .send()?
        .json()
        .unwrap();
    Ok(json)
}

pub fn get_link(args: &Args, client: &blocking::Client, series: &SeriesResult, season: i32, episode: i32) -> Result<String, Box<dyn Error>> {
    let link = get_link_from_api_source(args, client, series, &season.to_string(), &episode.to_string());
    Ok(link.unwrap())
}

pub fn search(client: &blocking::Client, search_term: &String) -> Result<SearchResults, Box<dyn Error>> {
    let json: SearchResults = client.get([TMDB_API, "/3/search/multi?language=en&page=1&query=", search_term, "&api_key=", TMDB_BRAFLIX_API_KEY].concat())
        .send()?
        .json()
        .unwrap();
    Ok(json)
}