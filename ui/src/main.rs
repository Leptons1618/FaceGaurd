use dioxus::prelude::*;
use faceguard_core::{camera, detection, events, recognition, tracking};
use js_sys::Array;
use serde_json;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Blob, HtmlElement, HtmlVideoElement, MediaStream, MediaStreamConstraints, Url};

const GLOBAL_STYLE: &str = r#"
    :root {
        color: #e7ecf3;
        background: radial-gradient(circle at 20% 20%, #1f2a3a 0, #0d1320 35%),
                     radial-gradient(circle at 80% 0%, #143149 0, #0d1320 30%),
                     #0d1320;
        font-family: "Space Grotesk", "Manrope", "Inter", system-ui, -apple-system, sans-serif;
    }
    * { box-sizing: border-box; }
    body { margin: 0; background: transparent; }
    .page { max-width: 1100px; margin: 0 auto; padding: 32px 20px 48px; }
    .hero { display: flex; align-items: center; justify-content: space-between; gap: 12px; margin-bottom: 24px; }
    .hero h1 { margin: 0; font-size: 28px; letter-spacing: -0.5px; }
    .hero p { margin: 4px 0 0; color: #9fb3c8; }
    .card { background: rgba(255,255,255,0.04); border: 1px solid rgba(255,255,255,0.08); border-radius: 16px; padding: 18px 18px 16px; backdrop-filter: blur(8px); box-shadow: 0 20px 60px rgba(0,0,0,0.35); }
    .grid { display: grid; grid-template-columns: 1.15fr 0.85fr; gap: 18px; }
    .video-card video { width: 100%; border-radius: 12px; border: 1px solid rgba(255,255,255,0.1); background: #05070d; object-fit: cover; min-height: 320px; }
    .section-title { display: flex; align-items: center; justify-content: space-between; margin: 0 0 10px; }
    .pill { display: inline-flex; align-items: center; gap: 8px; padding: 6px 10px; border-radius: 999px; background: rgba(255,255,255,0.08); color: #b9c8d8; font-size: 13px; }
    .list { list-style: none; padding: 0; margin: 0; display: grid; gap: 8px; }
    .list li { padding: 10px 12px; border: 1px solid rgba(255,255,255,0.08); border-radius: 12px; background: rgba(255,255,255,0.02); color: #dbe5f2; }
    .muted { color: #94a9c3; }
    .events { margin-top: 22px; }
    .controls { display: flex; flex-wrap: wrap; gap: 10px; align-items: center; margin-bottom: 12px; }
    select { background: rgba(255,255,255,0.08); color: #e7ecf3; border: 1px solid rgba(255,255,255,0.14); border-radius: 10px; padding: 8px 12px; }
    button { background: linear-gradient(135deg, #3fb5ff, #7b74ff); border: none; color: white; padding: 10px 14px; border-radius: 12px; cursor: pointer; font-weight: 600; letter-spacing: 0.1px; }
    button.secondary { background: rgba(255,255,255,0.08); border: 1px solid rgba(255,255,255,0.14); }
    .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(140px,1fr)); gap: 10px; margin-top: 6px; }
    .stat { padding: 12px; border-radius: 12px; border: 1px solid rgba(255,255,255,0.08); background: rgba(255,255,255,0.03); }
    .stat span { display: block; color: #9fb3c8; font-size: 13px; }
    .stat strong { font-size: 22px; }
    @media (max-width: 900px) { .grid { grid-template-columns: 1fr; } }
"#;

#[derive(Debug, Clone, Copy, PartialEq)]
enum EventFilter {
    All,
    Alerts,
    Unknowns,
    Blacklisted,
}

fn main() {
    launch(app);
}

fn app() -> Element {
    let filter = use_signal(|| EventFilter::All);
    let events_data = events::generate_events();
    let detections = detection::detect_faces();
    let tracks = tracking::track_faces();
    let identities = recognition::recognize_faces();

    // Kick off webcam capture on mount.
    use_effect(move || {
        spawn_local(async move {
            if let Err(err) = start_camera("camera-feed").await {
                web_sys::console::error_1(&err);
            }
        });
    });

    camera::ingest();

    let export_csv = {
        let filter = filter.clone();
        let events_data = events_data.clone();
        move |_| export_events_csv(filter(), &events_data)
    };

    let export_json = {
        let filter = filter.clone();
        let events_data = events_data.clone();
        move |_| export_events_json(filter(), &events_data)
    };

    rsx! {
        style { "{GLOBAL_STYLE}" }
        div { class: "page",
            header { class: "hero",
                div {
                    h1 { "FaceGuard — Live Security" }
                    p { "Built-in webcam feed with detections, tracks, identities, and events." }
                }
                span { class: "pill", "Beta" }
            }

            section { class: "grid",
                div { class: "card video-card",
                    div { class: "section-title",
                        h2 { "Live Camera" }
                        span { class: "pill", "Camera: webcam" }
                    }
                    video {
                        id: "camera-feed",
                        autoplay: true,
                        playsinline: true,
                        muted: true,
                        controls: false,
                    }
                }

                div { class: "card",
                    div { class: "section-title",
                        h3 { "System Snapshot" }
                        span { class: "muted", "Simulated detections + live camera preview" }
                    }
                    div { class: "stats",
                        div { class: "stat", span { "Detections" } strong { "{detections.len()}" } }
                        div { class: "stat", span { "Tracks" } strong { "{tracks.len()}" } }
                        div { class: "stat", span { "Identities" } strong { "{identities.len()}" } }
                        div { class: "stat", span { "Events" } strong { "{events_data.len()}" } }
                    }
                    h4 { style: "margin: 14px 0 6px;", "Recognized Identities" }
                    ul { class: "list",
                        for ident in identities.iter() {
                            li { "{ident.name} · ID {ident.id} · conf {ident.confidence:.2}" }
                        }
                        if identities.is_empty() {
                            li { class: "muted", "No identities yet" }
                        }
                    }
                }
            }

            section { class: "card events",
                div { class: "section-title",
                    h3 { "Events" }
                    span { class: "muted", "Filter, preview, and export" }
                }
                div { class: "controls",
                    label { "Filter:" }
                    select {
                        value: format!("{:?}", filter()),
                        onchange: move |e| {
                            let mut f = filter;
                            match e.value().as_str() {
                                "All" => f.set(EventFilter::All),
                                "Alerts" => f.set(EventFilter::Alerts),
                                "Unknowns" => f.set(EventFilter::Unknowns),
                                "Blacklisted" => f.set(EventFilter::Blacklisted),
                                _ => {}
                            }
                        },
                        option { value: "All", selected: filter() == EventFilter::All, "All" }
                        option { value: "Alerts", selected: filter() == EventFilter::Alerts, "Alerts" }
                        option { value: "Unknowns", selected: filter() == EventFilter::Unknowns, "Unknowns" }
                        option { value: "Blacklisted", selected: filter() == EventFilter::Blacklisted, "Blacklisted" }
                    }
                    button { onclick: export_csv, "Export CSV" }
                    button { class: "secondary", onclick: export_json, "Export JSON" }
                }
                ul { class: "list",
                    for event in filter_events(filter(), &events_data) {
                        li { "#{event.id} · {event.name} · conf {event.confidence:.2}" }
                    }
                    if filter_events(filter(), &events_data).is_empty() {
                        li { class: "muted", "No events for this filter" }
                    }
                }
            }
        }
    }
}

fn filter_events(filter: EventFilter, events: &[events::FaceEvent]) -> Vec<events::FaceEvent> {
    events
        .iter()
        .cloned()
        .filter(|event| match filter {
            EventFilter::All => true,
            EventFilter::Alerts => event.name == "Blacklisted" || event.confidence < 0.4,
            EventFilter::Unknowns => event.name == "Unknown",
            EventFilter::Blacklisted => event.name == "Blacklisted",
        })
        .collect()
}

fn export_events_csv(filter: EventFilter, events: &[events::FaceEvent]) {
    let event_vec = filter_events(filter, events);
    let mut csv = String::from("id,name,confidence\n");
    for event in event_vec.iter() {
        csv.push_str(&format!("{},{},{}\n", event.id, event.name, event.confidence));
    }
    download_blob(&csv, "events.csv", "text/csv");
}

fn export_events_json(filter: EventFilter, events: &[events::FaceEvent]) {
    if let Ok(json) = serde_json::to_string(&filter_events(filter, events)) {
        download_blob(&json, "events.json", "application/json");
    }
}

fn download_blob(content: &str, filename: &str, _mime: &str) {
    if let Ok(blob) = Blob::new_with_str_sequence(&Array::of1(&JsValue::from_str(content))) {
        if let Ok(url) = Url::create_object_url_with_blob(&blob) {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Ok(a) = document.create_element("a") {
                        let _ = a.set_attribute("href", &url);
                        let _ = a.set_attribute("download", filename);
                        if let Some(body) = document.body() {
                            let _ = body.append_child(&a);
                            if let Some(a_html) = a.dyn_ref::<HtmlElement>() {
                                a_html.click();
                            }
                            let _ = body.remove_child(&a);
                        }
                    }
                }
            }
            let _ = Url::revoke_object_url(&url);
        }
    }
}

async fn start_camera(video_id: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window"))?;
    let navigator = window.navigator();
    let media_devices = navigator
        .media_devices()
        .map_err(|err| JsValue::from(err))?;

    let constraints = MediaStreamConstraints::new();
    constraints.set_video(&JsValue::TRUE);

    let stream_js = JsFuture::from(
        media_devices
            .get_user_media_with_constraints(&constraints)
            .map_err(|err| JsValue::from(err))?,
    )
    .await?;

    let media_stream: MediaStream = stream_js.dyn_into()?;
    let document = window.document().ok_or_else(|| JsValue::from_str("No document"))?;
    let video: HtmlVideoElement = document
        .get_element_by_id(video_id)
        .ok_or_else(|| JsValue::from_str("No video element"))?
        .dyn_into()?;

    video.set_src_object(Some(&media_stream));
    video.set_muted(true);
    let _ = video.play();
    Ok(())
}
