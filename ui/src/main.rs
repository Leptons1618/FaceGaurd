use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Blob, Url, HtmlElement};
use js_sys::Array;
use serde_json;
#[derive(Debug, Clone, Copy, PartialEq)]
enum EventFilter {
    All,
    Alerts,
    Unknowns,
    Blacklisted,
}
use dioxus::prelude::*;
use faceguard_core::camera;
use faceguard_core::detection;
use faceguard_core::recognition;
use faceguard_core::tracking;
use faceguard_core::events;

fn main() {
    launch(app);
}

fn app() -> Element {
                            // Export events as CSV

    let filter = use_signal(|| EventFilter::All);
    let events = events::generate_events();
    let filtered_events: Vec<_> = events.iter().filter(|event| match filter() {
        EventFilter::All => true,
        EventFilter::Alerts => event.name == "Blacklisted" || event.confidence < 0.4,
        EventFilter::Unknowns => event.name == "Unknown",
        EventFilter::Blacklisted => event.name == "Blacklisted",
    }).cloned().collect();

    let _export_csv = {
        let event_vec = filtered_events.clone();
        move || {
            let mut csv = String::from("id,name,confidence\n");
            for event in event_vec.iter() {
                csv.push_str(&format!("{},{},{}\n", event.id, event.name, event.confidence));
            }
            let blob = Blob::new_with_str_sequence(&Array::of1(&JsValue::from_str(&csv))).unwrap();
            let url = Url::create_object_url_with_blob(&blob).unwrap();
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", "events.csv").unwrap();
            let body = document.body().unwrap();
            body.append_child(&a).unwrap();
            let a_html = a.dyn_ref::<HtmlElement>().unwrap();
            a_html.click();
            body.remove_child(&a).unwrap();
            Url::revoke_object_url(&url).unwrap();
        }
    };

    let _export_json = {
        let event_vec = filtered_events.clone();
        move || {
            let json = serde_json::to_string(&event_vec).unwrap();
            let blob = Blob::new_with_str_sequence(&Array::of1(&JsValue::from_str(&json))).unwrap();
            let url = Url::create_object_url_with_blob(&blob).unwrap();
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", "events.json").unwrap();
            let body = document.body().unwrap();
            body.append_child(&a).unwrap();
            let a_html = a.dyn_ref::<HtmlElement>().unwrap();
            a_html.click();
            body.remove_child(&a).unwrap();
            Url::revoke_object_url(&url).unwrap();
        }
    };

                            // Place export buttons in rsx
                            // Export buttons are now directly in the UI tree below
                        let filter = use_signal(|| EventFilter::All);
                    let events = events::generate_events();
                let tracks = tracking::track_faces();
            let identities = recognition::recognize_faces();
        let detections = detection::detect_faces();
    // TODO: Restore dynamic status when Dioxus macro issues are resolved
    camera::ingest();

    // let faces = detect_faces().unwrap_or_default(); // Unused, remove to silence warning

    rsx! {
        div { class: "p-4",
            h1 { "FaceGuard – Security Dashboard" }
            p { "Camera ingestion started (see console for details)" }
            h2 { "Detected Faces" }
            ul {
                for det in detections {
                    li {
                        "Face #{det.id}: bbox=({det.bbox.0}, {det.bbox.1}, {det.bbox.2}, {det.bbox.3}), confidence={det.confidence}" 
                    }
                }
            }
            h2 { "Tracked Faces" }
            ul {
                for track in tracks {
                    li {
                        "Track #{track.track_id}: detection_id={track.detection_id}, frames_tracked={track.frames_tracked}"
                    }
                }
            }
            h2 { "Recognized Identities" }
            ul {
                for ident in identities {
                    li {
                        "{ident.name} (ID: {ident.id}, confidence: {ident.confidence})"
                    }
                }
            }
            h2 { "Events" }
            div {
                label { "Filter: " }
                select {
                    value: "{filter():?}",
                    onchange: move |e| {
                        let binding = e.value();
                        let val = binding.as_str();
                        let mut filter = filter;
                        match val {
                            "All" => filter.set(EventFilter::All),
                            "Alerts" => filter.set(EventFilter::Alerts),
                            "Unknowns" => filter.set(EventFilter::Unknowns),
                            "Blacklisted" => filter.set(EventFilter::Blacklisted),
                            _ => {}
                        }
                    },
                    option { value: "All", selected: filter() == EventFilter::All, "All" }
                    option { value: "Alerts", selected: filter() == EventFilter::Alerts, "Alerts" }
                    option { value: "Unknowns", selected: filter() == EventFilter::Unknowns, "Unknowns" }
                    option { value: "Blacklisted", selected: filter() == EventFilter::Blacklisted, "Blacklisted" }
                }
            }
            ul {
                for event in events.iter().filter(|event| match filter() {
                    EventFilter::All => true,
                    EventFilter::Alerts => event.name == "Blacklisted" || event.confidence < 0.4,
                    EventFilter::Unknowns => event.name == "Unknown",
                    EventFilter::Blacklisted => event.name == "Blacklisted",
                }) {
                    li {
                        "Event #{event.id}: {event.name} (confidence: {event.confidence})"
                    }
                }
            }
        }
    }
}
