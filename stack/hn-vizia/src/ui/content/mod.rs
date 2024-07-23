use crate::repo::HnItem;

use super::theme::{BACKGROUND_WHITE, GREY};
use anyhow::bail;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{str::FromStr, time::Duration};
use url::Url;
use vizia::{
    context::Context,
    layout::Units::Pixels,
    modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
    views::{Button, HStack, Label, ScrollView, VStack},
};

pub(crate) fn render_content_region(cx: &mut Context) {
    VStack::new(cx, |cx| {
        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
            let res = get_items();
            // dbg!(&res);
            if let Some(items) = res.ok() {
                for (idx, item) in items.iter().enumerate() {
                    let seq = idx + 1;
                    let title = item.title();
                    let domain = item
                        .url()
                        .map(|url| url.domain())
                        .flatten()
                        .unwrap_or("???");
                    let score = item.score();
                    let by = item.by();
                    let time_ago = humanize_datetime(item.time());
                    //  chrono::DateTime::from_item.time();
                    Label::new(cx, format!("{seq}. {title} ({domain})"));
                    Label::new(cx, format!("    {score} points by {by} {time_ago}")).color(GREY);
                }
            } else {
                Label::new(cx, "No items found");
            }
            Label::new(cx, "").height(Pixels(10.));
            Button::new(cx, |cx| Label::new(cx, "      More"))
                .height(Pixels(18.0))
                .border_color(BACKGROUND_WHITE)
                .background_color(BACKGROUND_WHITE)
                .color(GREY);
            Label::new(cx, "").height(Pixels(10.));
        });
    })
    .background_color(BACKGROUND_WHITE);
}

fn get_items() -> anyhow::Result<Vec<HnItem>> {
    const MAX_ITEMS: usize = 15;
    use crate::repo::Repository;
    println!("Fetching top items...");
    let top_items = Repository::top_items()?;
    println!("Got {} items for indexing!", top_items.len());
    // dbg!(&top_items);
    let mut retrieved_items = vec![];
    println!("We'll fetch the first {MAX_ITEMS} items");
    for idx in 0..MAX_ITEMS {
        let maybe_cur = top_items.get(idx);
        // dbg!(&maybe_cur);
        if let Some(item_id) = maybe_cur {
            // dbg!(&item_id);
            println!("{i}. Fetching item id: {item_id}", i = idx + 1);
            match Repository::item(item_id) {
                Ok(item) => retrieved_items.push(item),
                Err(err) => {
                    dbg!(err);
                    println!(r#"    \--> Retrying.."#);
                    if let Ok(item) = Repository::item(item_id) {
                        // Retry
                        retrieved_items.push(item)
                    }
                }
            }
        }
    }

    Ok(retrieved_items)
}

fn humanize_datetime(epoch_secs: u32) -> String {
    let maybe_datetime = DateTime::<Utc>::from_timestamp(epoch_secs.into(), 0);
    let maybe_datetime_offset = maybe_datetime.map(|datetime| Utc::now() - datetime);
    // dbg!(&maybe_datetime_offset);
    maybe_datetime_offset
        .map(|timedelta| {
            let secs = timedelta.num_seconds();
            let dur = Duration::from_secs(secs.try_into().unwrap_or_default());
            let mut f = timeago::Formatter::with_language(timeago::English);
            f.num_items(1);
            f.convert(dur)
        })
        .unwrap_or("???".into())
}
