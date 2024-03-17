use reqwest::blocking;
use std::error::Error;
use crate::constants::{TMDB_API, TMDB_BRAFLIX_API_KEY, BRAFLIX_API};

#[derive(serde::Deserialize)]
struct EpisodeInfo {
    // episode_number: i32,
    // id: i32,
    // name: String,
    // runtime: i32,
    // season_number: i32,
    // show_id: i32,
}

#[derive(serde::Deserialize, Clone)]
pub struct SeriesInfo {
    // pub adult: bool,
    pub id: i32,
    pub name: Option<String>,
    pub title: Option<String>,
    // pub original_language: Option<String>,
    pub media_type: String,
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SearchResults {
    // pub page: i32,
    pub results: Vec<SeriesInfo>,
    // pub total_pages: i32,
    // pub total_results: i32,
}

#[derive(serde::Deserialize, Clone)]
struct SubtitleTrack {
    // url: Option<String>,
    // file: Option<String>,
    lang: Option<String>,
    language: Option<String>,
}

#[derive(serde::Deserialize, Clone)]
struct APISource {
    url: String,
    quality: String,
}

#[derive(serde::Deserialize, Clone)]
struct APISourceResults {
    sources: Option<Vec<APISource>>,
    subtitles: Option<Vec<SubtitleTrack>>,
}

#[derive(serde::Deserialize, Clone)]
struct VidSourceData {
    // file: String,
    // sub: Vec<SubtitleTrack>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum IntOrVidSourceData {
    A(VidSourceData),
    B(i32),
}

#[derive(serde::Deserialize)]
struct VidSource {
//     name: String,
//     data: IntOrVidSourceData,
}

fn try_get_link_from_api_source(provider: String, client: blocking::Client, mut series: SeriesInfo, season: String, episode: String) -> Result<String, Box<dyn Error>> {
    if !series.first_air_date.is_none() { series.first_air_date = Some(series.first_air_date.unwrap().split_at_mut(4).0.to_string()); }
    if !series.release_date.is_none() { series.release_date = Some(series.release_date.unwrap().split_at_mut(4).0.to_string()); }
    let url = format!("{0}/{1}/sources-with-title?title={2}&year={3}&mediaType={4}&episodeId={5}&seasonId={6}&tmdbId={7}", BRAFLIX_API, provider, if series.name.is_none() {series.title.unwrap()} else {series.name.unwrap()}, if series.release_date.is_none() {series.first_air_date.unwrap()} else {series.release_date.unwrap()}, series.media_type, episode, season, series.id);
    println!("{}", url);
    let json: APISourceResults = client.get(&url)
        .send()?
        .json()
        .unwrap();
    if !json.sources.is_none() {
        let mut sources = json.sources.unwrap();
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

fn get_link_from_api_source(client: blocking::Client, series: SeriesInfo, season: String, episode: String) -> Result<String, Box<dyn Error>> {
    let res = try_get_link_from_api_source("flixhq".to_string(), client.clone(), series.clone(), season.clone(), episode.clone()).and_then(|result|
        if result.ends_with(".m3u8") { Ok(result) } else { 
            println!("Failed to fetch with Braflix API using provider FlixHQ: {}", if result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
            try_get_link_from_api_source("vidsrc".to_string(), client.clone(), series.clone(), season.clone(), episode.clone()).and_then(|vidsrc_result|{
                if vidsrc_result.ends_with(".m3u8") {Ok(vidsrc_result)} else {
                    println!("Failed to fetch with Braflix API using provider VidSrc: {}", if vidsrc_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                    try_get_link_from_api_source("vidsrcto".to_string(), client.clone(), series.clone(), season.clone(), episode.clone()).and_then(|vidsrcto_result|{
                        if vidsrcto_result.ends_with(".m3u8") {Ok(vidsrcto_result)} else {
                            println!("Failed to fetch with Braflix API using provider VidSrcTo: {}", if vidsrcto_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                            try_get_link_from_api_source("superstream".to_string(), client.clone(), series.clone(), season.clone(), episode.clone()).and_then(|superstream_result|{
                                if superstream_result.ends_with(".m3u8") { Ok(superstream_result) } else {
                                    println!("Failed to fetch with Braflix API using provider SuperStream: {}", if superstream_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                                    try_get_link_from_api_source("febbox".to_string(), client.clone(), series.clone(), season.clone(), episode.clone()).and_then(|febbox_result|{
                                        if febbox_result.ends_with(".m3u8") { Ok(febbox_result) } else {
                                            println!("Failed to fetch with Braflix API using provider Febbox: {}", if febbox_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                                            try_get_link_from_api_source("overflix".to_string(), client.clone(), series.clone(), season.clone(), episode.clone()).and_then(|overflix_result|{
                                                if overflix_result.ends_with(".m3u8") {Ok(overflix_result)} else {
                                                    println!("Failed to fetch with Braflix API using provider OverFlix: {}", if overflix_result.is_empty() {"Errored while attempting to get link from provider"} else {"Resulting highest quality URL was still encrypted"});
                                                    try_get_link_from_api_source("visioncine".to_string(), client.clone(), series.clone(), season.clone(), episode.clone()).and_then(|visioncine_result|{
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

//this function may seem useless but occassionally they add different methods
pub fn get_link(client: blocking::Client, series: SeriesInfo, /*show_id: String,*/ season: String, episode: String) -> Result<String, Box<dyn Error>> {
    let link = get_link_from_api_source(client.clone(), series.clone(), season.clone(), episode.clone());
    Ok(link.unwrap())
}

pub fn search(client: blocking::Client, search_term: String) -> Result<SearchResults, Box<dyn Error>> {
    println!("{}", [TMDB_API, "/3/search/multi?language=en&page=1&query=", &search_term, "&api_key=", TMDB_BRAFLIX_API_KEY].concat());
    let json: SearchResults = client.get([TMDB_API, "/3/search/multi?language=en&page=1&query=", &search_term, "&api_key=", TMDB_BRAFLIX_API_KEY].concat())
        .send()?
        .json()
        .unwrap();
    Ok(json)
}