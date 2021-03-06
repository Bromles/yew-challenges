use yew::prelude::*;

use crate::components::videos_details::*;
use crate::components::videos_list::*;
use crate::models::video::Video;

mod components;
mod models;

#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };

    let details = selected_video.as_ref().map(|video| html![
        <VideoDetails video={video.clone()} />
    ]);

    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];

    html![
        <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
           <VideosList videos={videos} on_click={on_video_select.clone()} />
        </div>
        { for details }
    </>
    ]
}

fn main() {
    yew::start_app::<App>();
}