use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Media to search for
    #[arg(required=true)]
    pub search: String,
    /// Immediately use the first result instead of allowing navigation of results
    #[arg(short, long, default_value_t=false, required=false)]
    pub first: bool,
    /// Instead of opening MPV, print the playlist URL
    #[arg(long, default_value_t=false, required=false)]
    pub extract: bool,
    /// Select a specific quality (like 1080p, auto)
    #[arg(short, long, default_value_t=String::new(), required=false)]
    pub quality: String,
    /// Download video to home instead of opening MPV
    #[arg(short, long, default_value_t=false, required=false)]
    pub download: bool,
    /// Season to search for (1 for movies)
    #[arg(short, long, default_value_t=-1, required=false)]
    pub season: i32,
    /// Episode to search for (1 for movies)
    #[arg(short, long, default_value_t=-1, required=false)]
    pub episode: i32,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct SeriesResult {
    pub adult: bool,
    pub id: i32,
    pub name: Option<String>,
    pub title: Option<String>,
    pub original_language: Option<String>,
    pub media_type: String,
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SearchResults {
    pub page: i32,
    pub results: Vec<SeriesResult>,
    pub total_pages: i32,
    pub total_results: i32,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct SubtitleTrack {
    pub url: Option<String>,
    pub file: Option<String>,
    pub lang: Option<String>,
    pub language: Option<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct APISource {
    pub url: String,
    pub quality: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct APISourceResults {
    pub sources: Option<Vec<APISource>>,
    pub subtitles: Option<Vec<SubtitleTrack>>,
}

#[derive(serde::Deserialize)]
pub struct EpisodeData {
    pub episode_number: i32,
    pub episode_name: Option<String>,
    pub overview: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SeasonData {
    pub episodes: Vec<EpisodeData>,
}

#[derive(serde::Deserialize, Clone)]
pub struct SeasonInfo {
    pub episode_count: i32,
    pub id: i32,
    pub name: String,
    pub season_number: i32,
    pub air_date: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SeriesInfo {
    pub number_of_seasons: i32,
    pub seasons: Vec<SeasonInfo>,
}