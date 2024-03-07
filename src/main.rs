#![allow(dead_code)]
use clap::Parser;
use gql_client::GraphQLErrorMessage;
use reqwest::{blocking::{self, get}, dns::Resolving, header::{self, HeaderMap}, redirect};
use serde_json::{Value};
use std::{default, error::{self, Error}, fmt::format, io::{BufRead, BufReader, Read}, process::Command};

#[derive(Parser)]
struct Cli {
    /// search term to look for
    #[arg(short, long, default_value_t=("%20".to_string()))]
    search_term: String,
}
const AGENT: &str ="Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0";
const ALLANIME_REFR: &str ="https://allanime.to";
const ALLANIME_BASE: &str ="allanime.day";
const ALLANIME_API: &str ="https://api.allanime.day";
const MODE: &str ="dub";
// const DOWNLOAD_DIR: &str =".";
const QUALITY: &str ="best";

fn main() {
    let args = Cli::parse();

    let client = blocking::Client::builder()
        .redirect(redirect::Policy::none())
        .build()
        .unwrap();
    
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_str(AGENT).unwrap());
    headers.insert("Referer", header::HeaderValue::from_str(ALLANIME_REFR).unwrap());
    
    if args.search_term != "%20" {
        search(client, headers.clone(), args.search_term.clone()).expect("Failed search");
    } else {
        println!("{}", "Search For an Anime: ");
        let stdin = std::io::stdin();
        let line1 = stdin.lock().lines().next().unwrap().unwrap();
        let search_results = search(client, headers.clone(), line1).expect("Failed search");
        let json: Value = serde_json::from_str(&search_results).unwrap();
        let episodes = &json["data"]["shows"]["edges"];
        for i in 0..episodes.as_array().unwrap().len() {
            let episode = &episodes[i];
            println!("{}", format!("{} {} ({} episodes)", i + 1, episode["name"], episode["availableEpisodes"]["dub"]))
        }
    }
}

fn search(client: blocking::Client, headers: header::HeaderMap, search_term: String) -> Result<String, Box<dyn Error>> {
    let search_gql = r#"
    query (
        $search: SearchInput
        $limit: Int
        $page: Int
        $translationType: VaildTranslationTypeEnumType
        $countryOrigin: VaildCountryOriginEnumType
        ) {
            shows(
                search: $search
                limit: $limit
                page: $page
                translationType: $translationType
                countryOrigin: $countryOrigin
                ) {
            edges {
                _id
                name
                availableEpisodes
                __typename
            }
        }
    }      
    "#;
    let res = client.get([ALLANIME_API, "/api?variables=%7B%22search%22%3A%7B%22allowAdult%22%3Afalse%2C%22allowUnknown%22%3Afalse%2C%22query%22%3A%22", &search_term, "%22%7D%2C%22limit%22%3A40%2C%22page%22%3A1%2C%22translationType%22%3A%22", MODE, "%22%2C%22countryOrigin%22%3A%22ALL%22%7D&query=", search_gql].concat())
        .headers(headers)
        .send()?
        .text()?;

    Ok(res)
}

fn provider_init(provider_name: &str, other_thing: &str, episode_url_res: String) -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    Command::new(format!(r#"printf "%s" "{0}" | sed -n "{1}" | head -1 | cut -d':' -f2 | sed 's/../&\n/g' | sed 's/^01$/9/g;s/^08$/0/g;s/^05$/=/g;s/^0a$/2/g;s/^0b$/3/g;s/^0c$/4/g;s/^07$/?/g;s/^00$/8/g;s/^5c$/d/g;s/^0f$/7/g;s/^5e$/f/g;s/^17$/\//g;s/^54$/l/g;s/^09$/1/g;s/^48$/p/g;s/^4f$/w/g;s/^0e$/6/g;s/^5b$/c/g;s/^5d$/e/g;s/^0d$/5/g;s/^53$/k/g;s/^1e$/\&/g;s/^5a$/b/g;s/^59$/a/g;s/^4a$/r/g;s/^4c$/t/g;s/^4e$/v/g;s/^57$/o/g;s/^51$/i/g;' | tr -d '\n' | sed "s/\/clock/\/clock\.json/""#, episode_url_res, other_thing))
        .output()
        .unwrap()
        .stdout
        .as_slice()
        .read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(buffer)
}

fn get_links(client: blocking::Client, headers: HeaderMap, provider_id: String) -> Result<String, Box<dyn Error>> {
    let episode_link = client.get(["https://", ALLANIME_BASE, &provider_id].concat())
        .headers(headers)
        .send()?
        .text()?
        .as_str();
    let _ = match episode_link {
        "repackager.wixmp.com" => {
            let extract_link = 
        },
    };
}

fn generate_link(client: blocking::Client, headers: HeaderMap, provider: &str, episode_url_res: String) {
    let provider_id = match provider {
        "1" => provider_init("wixmp", "/Default :/p", episode_url_res),
        "2" => provider_init("dropbox", "/Sak :/p", episode_url_res),
        "3" => provider_init("wetransfer", "/Kir :/p", episode_url_res),
        "4" => provider_init("sharepoint", "/S-mp4 :/p", episode_url_res),
        default => provider_init("gogoanime", "/Luf-mp4 :/p", episode_url_res)
    };
    if (!provider_id.unwrap().is_empty()) {
        get_links(client, headers, provider_id.unwrap());
    }
}

fn get_episode_url(client: blocking::Client, headers: HeaderMap, show_id: String, translation_type: String, episode_string: String) -> Result<String, Box<dyn Error>> {
    let episode_embed_gql = r#"
    query (
        $showId: String!
        $translationType: VaildTranslationTypeEnumType!
        $episodeString: String!
    ) {
        episode(
            showId: $showId
            translationType: $translationType
            episodeString: $episodeString
        ) {
            episodeString
            sourceUrls
        }
    }      
    "#;

    let res = client.get(["http://", ALLANIME_API, "/api?variables=%7B%22showId%22%3A%22", &show_id, "%22%2C%22translationType%22%3A%22", &translation_type, "%22%2C%22episodeString%22%3A%22", &episode_string, "%22%7D&query=", &episode_embed_gql].concat())
        .headers(headers)
        .send()?
        .text()?;
    let mut buffer: String = String::new();
    let cache_dir = tempfile::tempdir().unwrap();
    let providers = vec!["1","2","3","4","5"];
    for provider in providers {
        generate_link(client, headers, &format!("{0}/{1}", cache_dir.path().to_str().unwrap(), provider), res.clone())
    }
    
    Ok(res)
}