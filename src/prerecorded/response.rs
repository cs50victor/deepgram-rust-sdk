use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PrerecordedResponse {
    pub metadata: ListenMetadata,
    pub results: ListenResults,
}

#[derive(Debug, Deserialize)]
pub struct ListenMetadata {
    pub request_id: Uuid,
    pub transaction_key: String,
    pub sha256: String,
    pub created: String,
    pub duration: f64,
    pub channels: usize,
}

#[derive(Debug, Deserialize)]
pub struct ListenResults {
    pub channels: Vec<ChannelResult>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelResult {
    pub search: Option<Vec<SearchResults>>,
    pub alternatives: Vec<ResultAlternative>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResults {
    pub query: String,
    pub hits: Vec<Hit>,
}

#[derive(Debug, Deserialize)]
pub struct Hit {
    pub confidence: f64,
    pub start: f64,
    pub end: f64,
    pub snippet: String,
}

#[derive(Debug, Deserialize)]
pub struct ResultAlternative {
    pub transcript: String,
    pub confidence: f64,
    pub words: Vec<Word>,
}

#[derive(Debug, Deserialize)]
pub struct Word {
    pub word: String,
    pub start: f64,
    pub end: f64,
    pub confidence: f64,
}
