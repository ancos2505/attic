use serde::Deserialize;
use std::fmt::Display;
use url::Url;

use crate::AppResult;

/// **Wrapper from Hacker News API**
///
/// Documentation: [https://github.com/HackerNews/API]
pub struct Repository;

impl Repository {
    pub fn top_items() -> AppResult<Vec<HnItemId>> {
        let resp = reqwest::blocking::get("https://hacker-news.firebaseio.com/v0/topstories.json")?
            .json()?;
        // dbg!(&resp);
        Ok(resp)
    }
    pub fn item(item_id: &HnItemId) -> AppResult<HnItem> {
        let url = format!(
            "https://hacker-news.firebaseio.com/v0/item/{}.json",
            item_id
        );
        let resp = reqwest::blocking::get(url)?.json()?;
        // dbg!(&resp);
        Ok(resp)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ItemType {
    Story,
    Job,
    Comment,
    Poll,
    PollOpt,
}

#[derive(Debug, Deserialize)]
pub(crate) struct HnItemId(u32);

impl Display for HnItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct HnItem {
    by: String,
    descendants: Option<HnItemId>,
    id: HnItemId,
    kids: Option<Vec<HnItemId>>,
    score: u32,
    time: u32,
    title: String,
    r#type: ItemType,
    url: Option<Url>,
}

impl HnItem {
    pub(crate) fn by(&self) -> &str {
        &self.by
    }

    pub(crate) fn descendants(&self) -> Option<&HnItemId> {
        self.descendants.as_ref()
    }

    pub(crate) fn id(&self) -> &HnItemId {
        &self.id
    }

    pub(crate) fn kids(&self) -> Option<&Vec<HnItemId>> {
        self.kids.as_ref()
    }

    pub(crate) fn score(&self) -> u32 {
        self.score
    }

    pub(crate) fn time(&self) -> u32 {
        self.time
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn r#type(&self) -> &ItemType {
        &self.r#type
    }

    pub(crate) fn url(&self) -> Option<&Url> {
        self.url.as_ref()
    }
}
