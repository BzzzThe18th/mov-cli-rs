use reqwest::blocking;
use std::error::Error;
use crate::structs::{APISourceResults, SearchResults, SeasonData, SeriesInfo, SeriesResult};
use crate::constants::{self, BRAFLIX_API, TMDB_API, TMDB_BRAFLIX_API_KEY};

fn try_get_link_from_api_source(provider: String, client: &blocking::Client, series: SeriesResult, season: String, episode: String) -> Result<String, Box<dyn Error>> {
    let url = format!("{0}/{1}/sources-with-title?title={2}&mediaType={3}&episodeId={4}&seasonId={5}&tmdbId={6}", BRAFLIX_API, provider, if series.name.is_none() {series.title.unwrap()} else {series.name.unwrap()}, series.media_type, episode, season, series.id);
    let json: APISourceResults = client.get(&url)
        .send()?
        .json()
        .unwrap();
    if !json.sources.is_none() {
        let mut sources = json.sources.unwrap();
        if sources.len() <= 0 {return Ok("".to_string());}
        sources.sort_by_key(|k| k.quality.to_lowercase().replace("auto", "0").replace("p", "").parse::<i32>().unwrap());
        sources.reverse();

        //implement subtitle stuff
        let subs = json.subtitles.unwrap();
        for sub_track in 0..subs.len() {
            println!("Subtitle found with language string {}", subs[sub_track].clone().lang.unwrap_or_else(|| {subs[sub_track].clone().language.expect("Failed to get both languages for sub track")}));
        }

        if sources[0].url.ends_with(".m3u8") { return Ok(sources[0].url.to_string()) } else {
            //aaaaaaaaaaaaaaaaaaaa ok so seems like braflix encrypts the already thrice encrypted vidsrcto URLs so I'm just gonna rip directly...
            //TODO: rip directly from encrypted source
            println!("Resolving encrypted source URLs is not yet implemented, sorry!");
        }
    }
    Ok("".to_string())
}

fn get_link_from_api_source(client: &blocking::Client, series: &SeriesResult, season: String, episode: String) -> Result<String, Box<dyn Error>> {
    let res = try_get_link_from_api_source("flixhq".to_string(), client, series.clone(), season.clone(), episode.clone()).and_then(|result|
        if result.ends_with(".m3u8") { Ok(result) } else { 
            println!("Failed to fetch with Braflix API using provider FlixHQ: {}", if result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
            try_get_link_from_api_source("vidsrc".to_string(), client, series.clone(), season.clone(), episode.clone()).and_then(|vidsrc_result|{
                if vidsrc_result.ends_with(".m3u8") {Ok(vidsrc_result)} else {
                    println!("Failed to fetch with Braflix API using provider VidSrc: {}", if vidsrc_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                    try_get_link_from_api_source("vidsrcto".to_string(), client, series.clone(), season.clone(), episode.clone()).and_then(|vidsrcto_result|{
                        if vidsrcto_result.ends_with(".m3u8") {Ok(vidsrcto_result)} else {
                            println!("Failed to fetch with Braflix API using provider VidSrcTo: {}", if vidsrcto_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                            try_get_link_from_api_source("superstream".to_string(), client, series.clone(), season.clone(), episode.clone()).and_then(|superstream_result|{
                                if superstream_result.ends_with(".m3u8") { Ok(superstream_result) } else {
                                    println!("Failed to fetch with Braflix API using provider SuperStream: {}", if superstream_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                                    try_get_link_from_api_source("febbox".to_string(), client, series.clone(), season.clone(), episode.clone()).and_then(|febbox_result|{
                                        if febbox_result.ends_with(".m3u8") { Ok(febbox_result) } else {
                                            println!("Failed to fetch with Braflix API using provider Febbox: {}", if febbox_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                                            try_get_link_from_api_source("overflix".to_string(), client, series.clone(), season.clone(), episode.clone()).and_then(|overflix_result|{
                                                if overflix_result.ends_with(".m3u8") {Ok(overflix_result)} else {
                                                    println!("Failed to fetch with Braflix API using provider OverFlix: {}", if overflix_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                                                    try_get_link_from_api_source("visioncine".to_string(), client, series.clone(), season.clone(), episode.clone()).and_then(|visioncine_result|{
                                                        println!("Failed to fetch with Braflix API using provider VisionCine: {}", if visioncine_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                                                        Ok("null".to_string())
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

pub fn get_link(client: &blocking::Client, series: &SeriesResult, season: i32, episode: i32) -> Result<String, Box<dyn Error>> {
    let link = get_link_from_api_source(client, series, season.to_string(), episode.to_string());
    Ok(link.unwrap())
}

pub fn search(client: &blocking::Client, search_term: &String) -> Result<SearchResults, Box<dyn Error>> {
    let json: SearchResults = client.get([TMDB_API, "/3/search/multi?language=en&page=1&query=", search_term, "&api_key=", TMDB_BRAFLIX_API_KEY].concat())
        .send()?
        .json()
        .unwrap();
    Ok(json)
}