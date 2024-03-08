#![allow(dead_code)]
use reqwest::{blocking, header, redirect};
use std::{error::Error, vec};

const AGENT: &str ="Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:123.0) Gecko/20100101 Firefox/123.0";
const BRAFLIX_REFR: &str ="https://www.braflix.video";
const BRAFLIX_VID: &str ="https://vidsrc.braflix.video";
const BRAFLIX_API: &str ="https://api.braflix.video";
const TMDB_API: &str = "https://api.themoviedb.org";
const TMDB_BRAFLIX_API_KEY: &str = "d39245e111947eb92b947e3a8aacc89f";

// TODO: trim down on structs
#[derive(serde::Deserialize)]
struct EpisodeInfo {
    episode_number: i32,
    id: i32,
    name: String,
    runtime: i32,
    season_number: i32,
    show_id: i32,
}

#[derive(serde::Deserialize, Clone)]
struct SeriesInfo {
    adult: bool,
    id: i32,
    name: Option<String>,
    title: Option<String>,
    original_language: Option<String>,
    media_type: String,
    release_date: Option<String>,
    first_air_date: Option<String>,
}

#[derive(serde::Deserialize)]
struct SearchResults {
    page: i32,
    results: Vec<SeriesInfo>,
    total_pages: i32,
    total_results: i32,
}

#[derive(serde::Deserialize, Clone)]
struct SubtitleTrack {
    url: Option<String>,
    file: Option<String>,
    lang: Option<String>,
    language: Option<String>,
}

#[derive(serde::Deserialize)]
struct APISource {
    url: String,
    quality: String,
}

#[derive(serde::Deserialize)]
struct APISourceResults {
    sources: Option<Vec<APISource>>,
    subtitles: Option<Vec<SubtitleTrack>>,
}

#[derive(serde::Deserialize, Clone)]
struct VidSourceData {
    file: String,
    sub: Vec<SubtitleTrack>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum IntOrVidSourceData {
    A(VidSourceData),
    B(i32),
}

#[derive(serde::Deserialize)]
struct VidSource {
    name: String,
    data: IntOrVidSourceData,
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

    // TODO: swtich searching over to cli using clap and skim tldr; do ui
    let search = search(client.clone(), "despicable me 3".to_string()).unwrap();
    let episode_link = get_link_from_api_source(client.clone(), search.results[0].clone(), "1".to_string(), "1".to_string()).and_then(|result|{
        if result.is_empty() {
            get_link_from_vid_source(client, search.results[0].id.to_string(), "1".to_string(), "1".to_string()).and_then(|vid_result|{
                if vid_result.is_empty() {
                    Err("Not able to find any link".into())
                } else {
                    Ok(vid_result)
                }
            })
        } else {
            Ok(result)
        }
    });
    println!("Fetched episode with link {}", episode_link.unwrap());
}

// TODO: pretty all of this up
fn try_flixhq(client: blocking::Client, mut series: SeriesInfo, season: String, episode: String) -> Result<String, Box<dyn Error>> {
    if !series.first_air_date.is_none() { series.first_air_date = Some(series.first_air_date.unwrap().split_at_mut(4).0.to_string()); }
    if !series.release_date.is_none() { series.release_date = Some(series.release_date.unwrap().split_at_mut(4).0.to_string()); }
    let url = format!("{0}/flixhq/sources-with-title?title={1}&year={2}&mediaType={3}&episodeId={4}&seasonId={5}&tmdbId={6}", BRAFLIX_API, if series.name.is_none() {series.title.unwrap()} else {series.name.unwrap()}, if series.release_date.is_none() {series.first_air_date.unwrap()} else {series.release_date.unwrap()}, series.media_type, episode, season, series.id);

    let json: APISourceResults = client.get(&url)
        .send()?
        .json()
        .unwrap();
    if !json.sources.is_none() {
        let mut sources = json.sources.unwrap();
        sources.sort_by_key(|k| k.quality.replace("Auto", "0").replace("p", "").parse::<i32>().unwrap());
        sources.reverse();
        return Ok(sources[0].url.to_string())
    }
    Ok("".to_string())
}

fn try_vidsrc(client: blocking::Client, mut series: SeriesInfo, season: String, episode: String) -> Result<String, Box<dyn Error>> {
    if !series.first_air_date.is_none() { series.first_air_date = Some(series.first_air_date.unwrap().split_at_mut(4).0.to_string()); }
    if !series.release_date.is_none() { series.release_date = Some(series.release_date.unwrap().split_at_mut(4).0.to_string()); }
    let url = format!("{0}/vidsrc/sources-with-title?title={1}&year={2}&mediaType={3}&episodeId={4}&seasonId={5}&tmdbId={6}", BRAFLIX_API, if series.name.is_none() {series.title.unwrap()} else {series.name.unwrap()}, if series.release_date.is_none() {series.first_air_date.unwrap()} else {series.release_date.unwrap()}, series.media_type, episode, season, series.id);
    println!("{}", url);
    let json: APISourceResults = client.get(&url)
        .send()?
        .json()
        .unwrap();
    if !json.sources.is_none() {
        let mut sources = json.sources.unwrap();
        sources.sort_by_key(|k| k.quality.replace("Auto", "0").replace("p", "").parse::<i32>().unwrap());
        sources.reverse();
        return Ok(sources[0].url.to_string())
    }
    println!("Failed to fetch with Braflix API - VidSrc");
    Ok("".to_string())
}

fn get_link_from_api_source(client: blocking::Client, series: SeriesInfo, season: String, episode: String) -> Result<String, Box<dyn Error>> {
    let res = try_flixhq(client.clone(), series.clone(), season.clone(), episode.clone()).and_then(|result|
        if result.ends_with(".m3u8") { Ok(result) } else { 
            println!("Failed to fetch with Braflix API - FlixHQ");
            try_vidsrc(client, series, season, episode)
        }
    );
    Ok(res.unwrap())
}

fn get_link_from_vid_source(client: blocking::Client, show_id: String, season: String, episode: String) -> Result<String, Box<dyn Error>> {
    let url = format!("{0}/vidsrc/{1}?s={2}&e={3}", BRAFLIX_VID, show_id, season, episode);
    let json: Vec<VidSource> = client.get(url)
        .send()?
        .json()
        .unwrap();
    let binding = VidSourceData {
            file: "".to_string(),
            sub: vec![],
        };
    let data = match &json[0].data {
        IntOrVidSourceData::A(source) => source,
        IntOrVidSourceData::B(_) => &binding,
    };
    Ok(data.file.to_string())
}

fn search(client: blocking::Client, search_term: String) -> Result<SearchResults, Box<dyn Error>> {
    println!("{}", [TMDB_API, "/3/search/multi?language=en&page=1&query=", &search_term, "&api_key=", TMDB_BRAFLIX_API_KEY].concat());
    let json: SearchResults = client.get([TMDB_API, "/3/search/multi?language=en&page=1&query=", &search_term, "&api_key=", TMDB_BRAFLIX_API_KEY].concat())
        .send()?
        .json()
        .unwrap();
    Ok(json)
}