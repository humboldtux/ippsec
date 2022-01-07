mod menu;
mod video;

use video::Video;

fn main() {
    let videos = match Video::load() {
        Ok(videos) => videos,
        Err(err) => {
            eprintln!("Could not load json dataset: {}", err);
            std::process::exit(1);
        }
    };

    let selected_items = menu::select(videos);

    for video in selected_items {
        let url = video.url();

        if webbrowser::open(&url).is_err() {
            eprintln!("Error opening video: url {}", &url);
        }
    }
}
