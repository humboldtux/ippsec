use skim::prelude::*;

use crate::video::Video;

impl SkimItem for Video {
    fn text(&self) -> Cow<str> {
        let text = match self {
            Video::Academy(a) => format!("{} [{}] -- {}", &a.machine, &a.tag, &a.line),
            Video::Youtube(y) => format!("{} [{}] -- {}", &y.machine, &y.tag, &y.line),
        };
        Cow::Owned(text)
    }
}

pub fn select(videos: Vec<Video>) -> Vec<Video> {
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for video in videos {
        tx.send(Arc::new(video)).unwrap();
    }

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .bind(vec!["alt-z:deselect-all"])
        .build()
        .unwrap();

    drop(tx);

    Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new)
        .iter()
        .map(|selected_item| {
            (**selected_item)
                .as_any()
                .downcast_ref::<Video>()
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<Video>>()
}
