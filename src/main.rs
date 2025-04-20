use web_sys::wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();

    videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |e: Event| {
                    // When events are created the target is undefined, it's only
                    // when dispatched does the target get added.
                    let target: Option<EventTarget> = e.target();
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                    if let Some(input) = input {
                        // input_value_handle.set(input.value());
                        let mut v = video.clone();
                        v.title = input.value();
                        on_click.emit(v.clone())
                    }
                })
            };

            // <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
            html! {
                // <button onclick={on_video_select}>
                // { "Click me!" }
                // </button>

                <input onchange={on_video_select}
                    id="cautious-input"
                    type="text"
                    value={video.title.clone()}
                />
            }
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);

    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            {
                let t_videos = vec![
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
                videos.set(t_videos.clone());
            }
            || ()
        });
    }

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>

               <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
