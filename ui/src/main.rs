use dioxus::prelude::*;
use faceguard_core::{camera, detection, events, recognition, tracking};
use gloo_storage::{LocalStorage, Storage};
use gloo_timers::callback::Interval;
use js_sys::Array;
use serde_json;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, HtmlVideoElement, ImageData,
    MediaStream, MediaStreamConstraints,
};

// Global state keys for persistence
const IDENTITY_DB_KEY: &str = "faceguard_identities";
const EVENT_LOG_KEY: &str = "faceguard_events";

macro_rules! log {
    ($($arg:tt)*) => {
        web_sys::console::log_1(&format!($($arg)*).into());
    };
}

const GLOBAL_STYLE: &str = r#"
    :root {
        color: #e7ecf3;
        background: radial-gradient(circle at 20% 20%, #1f2a3a 0, #0d1320 35%),
                     radial-gradient(circle at 80% 0%, #143149 0, #0d1320 30%),
                     #0d1320;
        font-family: "Space Grotesk", "Manrope", "Inter", system-ui, -apple-system, sans-serif;
    }
    html, body { margin: 0; padding: 0; height: 100%; overflow: hidden; }
    * { box-sizing: border-box; }
    body { background: transparent; }
    
    .container { 
        display: flex; 
        flex-direction: column; 
        height: 100vh; 
        width: 100vw; 
        overflow: hidden;
    }
    
    nav { 
        display: flex; 
        gap: 4px; 
        padding: 12px 20px; 
        border-bottom: 1px solid rgba(255,255,255,0.1); 
        background: rgba(13, 19, 32, 0.9);
        flex-shrink: 0;
        z-index: 100;
    }
    
    nav a { 
        padding: 10px 18px; 
        border-radius: 10px; 
        text-decoration: none; 
        color: #9fb3c8; 
        font-weight: 500; 
        transition: all 0.2s;
        cursor: pointer;
    }
    
    nav a:hover { 
        background: rgba(255,255,255,0.08); 
        color: #e7ecf3; 
    }
    
    nav a.active { 
        background: rgba(63,181,255,0.15); 
        color: #3fb5ff; 
        border: 1px solid rgba(63,181,255,0.3);
    }
    
    .content { 
        flex: 1; 
        overflow: hidden; 
        display: flex;
    }
    
    .page { 
        flex: 1; 
        overflow: auto; 
        padding: 20px;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }
    
    .dashboard-grid {
        display: grid;
        grid-template-columns: 2fr 1fr;
        gap: 12px;
        height: 100%;
        min-height: 0;
    }
    
    .video-panel {
        display: flex;
        flex-direction: column;
        gap: 8px;
        min-height: 0;
    }
    
    .card { 
        background: rgba(255,255,255,0.04); 
        border: 1px solid rgba(255,255,255,0.08); 
        border-radius: 16px; 
        padding: 16px; 
        backdrop-filter: blur(8px); 
        box-shadow: 0 20px 60px rgba(0,0,0,0.35);
        display: flex;
        flex-direction: column;
        min-height: 0;
    }
    
    .video-card { 
        flex: 1;
        position: relative;
    }
    
    .video-card video { 
        width: 100%; 
        height: 100%;
        border-radius: 12px; 
        border: 1px solid rgba(255,255,255,0.1); 
        background: #05070d; 
        object-fit: cover;
    }
    
    .video-container { 
        position: relative; 
        flex: 1;
        overflow: hidden;
    }
    
    .video-overlay { 
        position: absolute; 
        top: 0; 
        left: 0; 
        pointer-events: none; 
    }
    
    .section-title { 
        display: flex; 
        align-items: center; 
        justify-content: space-between; 
        margin: 0; 
        font-size: 14px;
        font-weight: 600;
        color: #e7ecf3;
    }
    
    .pill { 
        display: inline-flex; 
        align-items: center; 
        gap: 8px; 
        padding: 6px 10px; 
        border-radius: 999px; 
        background: rgba(255,255,255,0.08); 
        color: #b9c8d8; 
        font-size: 12px;
    }
    
    .pill.live { background: rgba(76, 175, 80, 0.2); color: #4caf50; }
    .pill.warning { background: rgba(255, 152, 0, 0.2); color: #ffa500; }
    .pill.error { background: rgba(244, 67, 54, 0.2); color: #f44336; }
    
    .list { 
        list-style: none; 
        padding: 0; 
        margin: 0; 
        display: grid; 
        gap: 6px;
        max-height: 200px;
        overflow-y: auto;
    }
    
    .list li { 
        padding: 8px 10px; 
        border: 1px solid rgba(255,255,255,0.08); 
        border-radius: 10px; 
        background: rgba(255,255,255,0.02); 
        color: #dbe5f2;
        font-size: 13px;
    }
    
    .muted { color: #94a9c3; }
    
    .stats { 
        display: grid; 
        grid-template-columns: repeat(2, 1fr);
        gap: 8px;
    }
    
    .stat { 
        padding: 10px; 
        border-radius: 10px; 
        border: 1px solid rgba(255,255,255,0.08); 
        background: rgba(255,255,255,0.03);
        text-align: center;
    }
    
    .stat span { 
        display: block; 
        color: #9fb3c8; 
        font-size: 12px;
    }
    
    .stat strong { 
        font-size: 18px;
        display: block;
    }
    
    .controls { 
        display: flex; 
        flex-wrap: wrap; 
        gap: 8px; 
        align-items: center;
    }
    
    button { 
        background: linear-gradient(135deg, #3fb5ff, #7b74ff); 
        border: none; 
        color: white; 
        padding: 8px 12px; 
        border-radius: 10px; 
        cursor: pointer; 
        font-weight: 600; 
        font-size: 13px;
        transition: all 0.2s;
    }
    
    button:hover { 
        opacity: 0.9; 
        transform: translateY(-1px);
    }
    
    button.secondary { 
        background: rgba(255,255,255,0.08); 
        border: 1px solid rgba(255,255,255,0.14); 
        color: #e7ecf3;
    }
    
    input[type="text"],
    input[type="range"],
    select {
        background: rgba(255,255,255,0.08);
        color: #e7ecf3;
        border: 1px solid rgba(255,255,255,0.14);
        border-radius: 8px;
        padding: 8px 10px;
        font-size: 13px;
    }
    
    input[type="text"]::placeholder {
        color: #6b8fb3;
    }
    
    .registration-form {
        max-width: 500px;
    }
    
    .form-group {
        margin-bottom: 16px;
    }
    
    .form-group label {
        display: block;
        margin-bottom: 6px;
        font-weight: 500;
        font-size: 14px;
    }
    
    .form-group input,
    .form-group textarea {
        width: 100%;
    }
    
    textarea {
        background: rgba(255,255,255,0.08);
        color: #e7ecf3;
        border: 1px solid rgba(255,255,255,0.14);
        border-radius: 8px;
        padding: 8px 10px;
        font-family: inherit;
        min-height: 80px;
        resize: vertical;
    }
    
    .capture-preview {
        width: 100%;
        max-width: 300px;
        border-radius: 12px;
        border: 1px solid rgba(255,255,255,0.1);
        margin: 12px 0;
    }
    
    @media (max-width: 900px) { 
        .dashboard-grid { 
            grid-template-columns: 1fr; 
        } 
    }
"#;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Page {
    Dashboard,
    Events,
    Register,
    Settings,
}

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut current_page = use_signal(|| Page::Dashboard);

    rsx! {
        document::Script {
            r#"
// MediaPipe Face Detection Integration
let faceDetector = null;
let isMediaPipeReady = false;

async function initMediaPipe() {{
    try {{
        const vision = await import('https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.2');
        
        const {{ FaceDetector, FilesetResolver }} = vision;
        
        const filesetResolver = await FilesetResolver.forVisionTasks(
            "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.2/wasm"
        );
        
        faceDetector = await FaceDetector.createFromOptions(filesetResolver, {{
            baseOptions: {{
                modelAssetPath: 'https://storage.googleapis.com/mediapipe-models/face_detector/blaze_face_short_range/float16/1/blaze_face_short_range.tflite',
                delegate: "GPU"
            }},
            runningMode: "VIDEO",
            minDetectionConfidence: 0.5
        }});
        
        isMediaPipeReady = true;
        console.log("✓ MediaPipe Face Detector initialized");
    }} catch (error) {{
        console.warn("Failed to initialize MediaPipe:", error);
        isMediaPipeReady = false;
    }}
}}

window.detectFacesMediaPipe = function(videoId) {{
    if (!isMediaPipeReady || !faceDetector) {{
        return null;
    }}
    
    try {{
        const video = document.getElementById(videoId);
        if (!video || video.readyState < 2) {{
            return null;
        }}
        
        const startTimeMs = performance.now();
        const detections = faceDetector.detectForVideo(video, startTimeMs);
        
        if (!detections || !detections.detections || detections.detections.length === 0) {{
            return [];
        }}
        
        const results = detections.detections.map(detection => {{
            const bbox = detection.boundingBox;
            return {{
                x: bbox.originX,
                y: bbox.originY,
                width: bbox.width,
                height: bbox.height,
                score: detection.categories[0]?.score || 0.5
            }};
        }});
        
        return results;
    }} catch (error) {{
        console.error("MediaPipe detection error:", error);
        return null;
    }}
}};

if (document.readyState === 'loading') {{
    document.addEventListener('DOMContentLoaded', initMediaPipe);
}} else {{
    initMediaPipe();
}}
            "#
        }
        style { "{GLOBAL_STYLE}" }
        div { class: "container",
            nav {
                a {
                    class: if current_page() == Page::Dashboard { "active" } else { "" },
                    onclick: move |_| current_page.set(Page::Dashboard),
                    "Dashboard"
                }
                a {
                    class: if current_page() == Page::Register { "active" } else { "" },
                    onclick: move |_| current_page.set(Page::Register),
                    "Register"
                }
                a {
                    class: if current_page() == Page::Events { "active" } else { "" },
                    onclick: move |_| current_page.set(Page::Events),
                    "Events"
                }
                a {
                    class: if current_page() == Page::Settings { "active" } else { "" },
                    onclick: move |_| current_page.set(Page::Settings),
                    "Settings"
                }
            }
            
            div { class: "content",
                match current_page() {
                    Page::Dashboard => rsx! { Dashboard {} },
                    Page::Events => rsx! { EventsPage {} },
                    Page::Register => rsx! { RegisterPage {} },
                    Page::Settings => rsx! { Settings {} },
                }
            }
        }
    }
}

#[component]
fn Dashboard() -> Element {
    let identity_db = load_identity_db();
    let event_log = load_event_log();
    let identities = identity_db.get_all();
    let events_data = event_log.get_recent(20);

    let mut detections = use_signal(|| vec![]);
    let mut tracks = use_signal(|| vec![]);
    let mut frame_count = use_signal(|| 0);
    let mut fps = use_signal(|| 0.0);
    let mut faces_detected = use_signal(|| 0);
    let mut last_time = use_signal(|| js_sys::Date::now());
    let mut tracker = use_signal(|| tracking::Tracker::new(0.4, 3000));
    let mut _interval_handle = use_signal::<Option<Interval>>(|| None);
    let mut camera_ready = use_signal(|| false);

    // Start camera
    use_effect(move || {
        spawn_local(async move {
            match start_camera("camera-feed").await {
                Ok(_) => {
                    log!("✓ Camera started successfully");
                    camera_ready.set(true);
                }
                Err(err) => {
                    log!("✗ Camera error: {:?}", err);
                }
            }
        });
    });

    // Frame processing loop - only start after camera is ready
    use_effect(move || {
        if !camera_ready() {
            return;
        }

        log!("Starting frame processing loop...");

        let interval = Interval::new(100, move || {
            frame_count.set(frame_count() + 1);

            let now = js_sys::Date::now();
            let mut last = last_time();
            
            // Calculate FPS
            let delta = (now - last) / 1000.0;
            if delta > 0.0 && delta < 1.0 {
                fps.set(1.0 / delta);
            }
            last_time.set(now);

            let current_count = frame_count();
            
            // Try to detect faces from video
            match detect_faces_from_video("camera-feed", "temp-canvas") {
                Some(dets) => {
                    if current_count % 5 == 0 {
                        log!("[Frame {}] ✓ Got image data, detected {} raw faces", current_count, dets.len());
                    }

                    let filtered_dets = detection::apply_nms(dets, 0.3);  // Lower IOU threshold = more aggressive filtering
                    if current_count % 5 == 0 {
                        log!("[Frame {}] After NMS (IOU=0.3): {} faces", current_count, filtered_dets.len());
                    }

                    let timestamp = now as u64;
                    let mut t = tracker.write();
                    let active_tracks = t.update(filtered_dets.clone(), timestamp);
                    drop(t);

                    faces_detected.set(filtered_dets.len());
                    detections.set(filtered_dets);
                    tracks.set(active_tracks.clone());

                    // Log unknown face events
                    let identity_db = load_identity_db();
                    let mut event_log = load_event_log();
                    
                    for track in &active_tracks {
                        // Only log tracks that are currently being detected
                        if timestamp - track.last_seen < 1000 {  // Within last second
                            // Check if we already logged this track recently
                            let already_logged = event_log.get_all().iter().any(|e| {
                                e.event_type == events::EventType::UnknownFace && 
                                e.track_id == Some(track.track_id) &&
                                (js_sys::Date::now() as u64 - e.timestamp) < 5000  // Within last 5 seconds
                            });
                            
                            if !already_logged {
                                event_log.add_event(
                                    events::EventType::UnknownFace,
                                    "Unknown".to_string(),
                                    track.detection.confidence,
                                    Some(track.track_id),
                                );
                                log!("Event: Unknown face detected (Track #{})", track.track_id);
                            }
                        }
                    }
                    
                    save_event_log(&event_log);

                    draw_detections_and_tracks("camera-feed", "overlay-canvas", &tracks());
                }
                None => {
                    if current_count % 30 == 0 {
                        log!("[Frame {}] ✗ Failed to read video frame or extract image data", current_count);
                    }
                }
            }
        });

        _interval_handle.set(Some(interval));
        log!("Frame processing loop started");
    });

    rsx! {
        div { class: "page",
            div { class: "dashboard-grid",
                div { class: "video-panel",
                    div { class: "card video-card",
                        div { class: "section-title",
                            h3 { "Live Feed" }
                            span { class: "pill live", "● Active" }
                        }
                        div { class: "video-container",
                            video {
                                id: "camera-feed",
                                autoplay: true,
                                playsinline: true,
                                muted: true,
                                controls: false,
                                style: "width: 100%; height: 100%; border-radius: 12px; object-fit: cover;"
                            }
                            canvas {
                                id: "overlay-canvas",
                                class: "video-overlay",
                            }
                            canvas {
                                id: "temp-canvas",
                                style: "display: none;",
                            }
                        }
                    }
                }
                
                div { class: "card",
                    div { class: "section-title",
                        h3 { "System Status" }
                        span { 
                            class: if camera_ready() { 
                                if faces_detected() > 0 { "pill live" } else { "pill warning" }
                            } else { 
                                "pill error" 
                            },
                            if !camera_ready() { 
                                "⏳ Starting camera..."
                            } else if faces_detected() > 0 { 
                                "✓ Detecting" 
                            } else { 
                                "⚠ No Faces" 
                            }
                        }
                    }
                    
                    div { class: "stats",
                        div { class: "stat", 
                            span { "Frames" }
                            strong { "{frame_count}" }
                        }
                        div { class: "stat", 
                            span { "FPS" }
                            strong { "{fps:.1}" }
                        }
                        div { class: "stat", 
                            span { "Faces" }
                            strong { "{faces_detected()}" }
                        }
                        div { class: "stat", 
                            span { "Tracks" }
                            strong { "{tracks().len()}" }
                        }
                    }
                    
                    h4 { style: "margin-top: 16px; margin-bottom: 8px;", "Tracked Faces" }
                    ul { class: "list",
                        for track in tracks().iter() {
                            li {
                                "#{track.track_id} · {track.frames_tracked}f · {(track.detection.confidence*100.0):.0}%"
                            }
                        }
                        if tracks().is_empty() {
                            li { class: "muted", "No active tracks" }
                        }
                    }
                    
                    h4 { style: "margin-top: 12px; margin-bottom: 8px;", "Identities" }
                    ul { class: "list",
                        for ident in identities.iter().take(5) {
                            li { "{ident.name}" }
                        }
                        if identities.is_empty() {
                            li { class: "muted", "No identities registered" }
                        }
                    }
                    
                    h4 { style: "margin-top: 12px; margin-bottom: 8px;", "Recent Events" }
                    ul { class: "list",
                        for evt in events_data.iter().take(5) {
                            li { "#{evt.id}" }
                        }
                        if events_data.is_empty() {
                            li { class: "muted", "No events logged" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RegisterPage() -> Element {
    let mut name = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());
    let mut captured_image = use_signal::<Option<String>>(|| None);
    let mut capture_status = use_signal(|| String::from("Ready"));

    let capture_face = {
        move |_| {
            spawn_local(async move {
                if let Some(image_data) = capture_face_from_video("camera-feed", "capture-canvas") {
                    captured_image.set(Some(image_data));
                    capture_status.set(String::from("✓ Face captured"));
                    log!("Face captured successfully");
                } else {
                    capture_status.set(String::from("✗ Capture failed"));
                    log!("Failed to capture face");
                }
            });
        }
    };

    let save_identity = {
        move |_| {
            let n = name().trim().to_string();
            if n.is_empty() {
                capture_status.set(String::from("✗ Name required"));
                return;
            }
            
            let mut db = load_identity_db();
            db.add_identity(n.clone(), None);
            save_identity_db(&db);
            
            log!("Saved identity: {}", n);
            capture_status.set(String::from("✓ Identity saved!"));
            name.set(String::new());
            notes.set(String::new());
            captured_image.set(None);
        }
    };

    rsx! {
        div { class: "page",
            div { class: "card registration-form",
                h2 { "Register New Identity" }
                
                div { class: "form-group",
                    label { "Person Name *" }
                    input {
                        r#type: "text",
                        placeholder: "Enter full name",
                        value: "{name()}",
                        oninput: move |e| name.set(e.value()),
                    }
                }
                
                div { class: "form-group",
                    label { "Capture Face" }
                    p { class: "muted", style: "margin: 0 0 8px 0; font-size: 13px;", 
                        "Make sure your face is clearly visible in the video feed"
                    }
                    button { onclick: capture_face, "Capture Face" }
                    if let Some(img) = captured_image() {
                        img {
                            src: "{img}",
                            class: "capture-preview",
                            alt: "Captured face"
                        }
                    }
                }
                
                div { class: "form-group",
                    label { "Notes (Optional)" }
                    textarea {
                        placeholder: "Additional information",
                        value: "{notes()}",
                        oninput: move |e| notes.set(e.value()),
                    }
                }
                
                div { class: "controls",
                    button { onclick: save_identity, "Save Identity" }
                    span { class: "pill", "{capture_status()}" }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EventFilter {
    All,
    Alerts,
    Unknowns,
}

#[component]
fn EventsPage() -> Element {
    let mut filter = use_signal(|| EventFilter::All);
    let event_log = load_event_log();
    let events_data = event_log.get_all();

    rsx! {
        div { class: "page",
            div { class: "card",
                h2 { "Security Events" }
                
                div { class: "controls",
                    label { "Filter:" }
                    select {
                        value: format!("{:?}", filter()),
                        onchange: move |e| {
                            match e.value().as_str() {
                                "All" => filter.set(EventFilter::All),
                                "Alerts" => filter.set(EventFilter::Alerts),
                                "Unknowns" => filter.set(EventFilter::Unknowns),
                                _ => {}
                            }
                        },
                        option { value: "All", selected: filter() == EventFilter::All, "All Events" }
                        option { value: "Alerts", selected: filter() == EventFilter::Alerts, "Alerts" }
                        option { value: "Unknowns", selected: filter() == EventFilter::Unknowns, "Unknown Faces" }
                    }
                }
                
                ul { class: "list",
                    for event in filter_events(filter(), &events_data).iter().take(50) {
                        li { 
                            "#{event.id} · {event.name} · conf {event.confidence:.2} · {event.timestamp}"
                        }
                    }
                    if filter_events(filter(), &events_data).is_empty() {
                        li { class: "muted", "No events for this filter" }
                    }
                }
            }
        }
    }
}

#[component]
fn Settings() -> Element {
    let db = load_identity_db();
    let log = load_event_log();
    let identity_count = db.get_all().len();
    let event_count = log.get_all().len();
    
    rsx! {
        div { class: "page",
            div { class: "card",
                h2 { "Settings & System Info" }
                
                h3 { "System Information" }
                ul { class: "list",
                    li { "Platform: WebAssembly (Dioxus)" }
                    li { "Version: 0.1.0-dev" }
                    li { "Storage: LocalStorage (JSON)" }
                    li { "Detection: Brightness-based (placeholder)" }
                    li { "Build Date: 2026-01-01" }
                }
                
                h3 { style: "margin-top: 16px;", "Database Status" }
                ul { class: "list",
                    li { "Identities: {identity_count}" }
                    li { "Events Logged: {event_count}" }
                }
                
                h3 { style: "margin-top: 16px;", "Actions" }
                div { class: "controls",
                    button { 
                        onclick: move |_| {
                            let _ = LocalStorage::delete(IDENTITY_DB_KEY);
                            let _ = LocalStorage::delete(EVENT_LOG_KEY);
                            log!("Database cleared");
                        },
                        class: "secondary",
                        "Clear All Data"
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
            EventFilter::Alerts => {
                event.confidence < 0.4
                    || matches!(event.event_type, events::EventType::Blacklisted | events::EventType::AfterHours)
            }
            EventFilter::Unknowns => event.event_type == events::EventType::UnknownFace,
        })
        .collect()
}

// Persistence helpers
fn load_identity_db() -> recognition::IdentityDatabase {
    LocalStorage::get(IDENTITY_DB_KEY).unwrap_or_else(|_| {
        log!("Creating new identity database");
        recognition::IdentityDatabase::new()
    })
}

fn save_identity_db(db: &recognition::IdentityDatabase) {
    if let Err(e) = LocalStorage::set(IDENTITY_DB_KEY, db) {
        log!("Error saving identity database: {:?}", e);
    } else {
        log!("Identity database saved");
    }
}

fn load_event_log() -> events::EventLog {
    LocalStorage::get(EVENT_LOG_KEY).unwrap_or_else(|_| {
        log!("Creating new event log");
        events::EventLog::new(1000)
    })
}

fn save_event_log(event_log: &events::EventLog) {
    if let Err(e) = LocalStorage::set(EVENT_LOG_KEY, event_log) {
        log!("Failed to save event log: {:?}", e);
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
    log!("Camera started");
    Ok(())
}

fn detect_faces_from_video(
    video_id: &str,
    canvas_id: &str,
) -> Option<Vec<detection::FaceDetection>> {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window, js_name = detectFacesMediaPipe)]
        fn detect_faces_mediapipe(video_id: &str) -> JsValue;
    }
    
    // Try MediaPipe first
    let js_result = detect_faces_mediapipe(video_id);
    
    if !js_result.is_null() && !js_result.is_undefined() {
        if let Ok(detections_js) = js_result.dyn_into::<js_sys::Array>() {
            let mut detections = Vec::new();
            
            for i in 0..detections_js.length() {
                if let Some(det_obj) = detections_js.get(i).dyn_into::<js_sys::Object>().ok() {
                    if let (Ok(x_val), Ok(y_val), Ok(w_val), Ok(h_val), Ok(score_val)) = (
                        js_sys::Reflect::get(&det_obj, &JsValue::from_str("x")),
                        js_sys::Reflect::get(&det_obj, &JsValue::from_str("y")),
                        js_sys::Reflect::get(&det_obj, &JsValue::from_str("width")),
                        js_sys::Reflect::get(&det_obj, &JsValue::from_str("height")),
                        js_sys::Reflect::get(&det_obj, &JsValue::from_str("score")),
                    ) {
                        if let (Some(x), Some(y), Some(w), Some(h), Some(score)) = (
                            x_val.as_f64(),
                            y_val.as_f64(),
                            w_val.as_f64(),
                            h_val.as_f64(),
                            score_val.as_f64(),
                        ) {
                            detections.push(detection::FaceDetection::new(
                                i,
                                x as f32,
                                y as f32,
                                w as f32,
                                h as f32,
                                score as f32,
                            ));
                        }
                    }
                }
            }
            
            if !detections.is_empty() {
                log!("MediaPipe: detected {} faces", detections.len());
                return Some(detections);
            }
        }
    }
    
    // Fallback to simple detection if MediaPipe not available
    log!("Falling back to simple edge detection");
    
    let window = web_sys::window()?;
    let document = window.document()?;

    let video: HtmlVideoElement = document.get_element_by_id(video_id)?.dyn_into().ok()?;
    let canvas: HtmlCanvasElement = document.get_element_by_id(canvas_id)?.dyn_into().ok()?;

    let video_width = video.video_width();
    let video_height = video.video_height();
    
    if video_width == 0 || video_height == 0 {
        return None;
    }

    canvas.set_width(video_width);
    canvas.set_height(video_height);

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .ok()??
        .dyn_into()
        .ok()?;

    ctx.draw_image_with_html_video_element(&video, 0.0, 0.0).ok()?;
    let image_data = ctx.get_image_data(0.0, 0.0, video_width as f64, video_height as f64).ok()?;

    Some(simple_face_detection(&image_data, video_width, video_height))
}

fn capture_face_from_video(
    video_id: &str,
    canvas_id: &str,
) -> Option<String> {
    let window = web_sys::window()?;
    let document = window.document()?;

    let video: HtmlVideoElement = document.get_element_by_id(video_id)?.dyn_into().ok()?;
    let canvas: HtmlCanvasElement = document.get_element_by_id(canvas_id)?.dyn_into().ok()?;

    let video_width = video.video_width();
    let video_height = video.video_height();

    if video_width == 0 || video_height == 0 {
        return None;
    }

    canvas.set_width(video_width);
    canvas.set_height(video_height);

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .ok()??
        .dyn_into()
        .ok()?;

    ctx.draw_image_with_html_video_element(&video, 0.0, 0.0).ok()?;

    canvas.to_data_url().ok()
}

fn simple_face_detection(
    image_data: &ImageData,
    width: u32,
    height: u32,
) -> Vec<detection::FaceDetection> {
    let data = image_data.data();
    let mut detections = Vec::new();
    let mut detection_id = 1;

    let grid_size = 8;  // Coarser grid for larger region detection
    let step_x = (width / grid_size).max(1);
    let step_y = (height / grid_size).max(1);

    log!("Detection: scanning {}x{} grid ({}x{} pixels per cell) for edge density", grid_size, grid_size, step_x, step_y);

    let mut edge_cells = Vec::new();
    let mut max_density = 0.0_f32;
    let mut densities = Vec::new();

    for gy in 0..grid_size {
        for gx in 0..grid_size {
            let x = (gx * step_x) as u32;
            let y = (gy * step_y) as u32;
            let w = step_x;
            let h = step_y;

            // Calculate edge density (how many high-contrast transitions)
            let edge_density = calculate_edge_density(&data, x, y, w, h, width);
            densities.push(edge_density);
            max_density = max_density.max(edge_density);
            
            // Faces have features (eyes, nose, mouth) = high edge density
            // Uniform areas (wall, plain background) = low edge density
            if edge_density > 0.04 {  // Lowered from 0.08
                edge_cells.push((gx, gy, edge_density));
            }
        }
    }

    let avg_density = if !densities.is_empty() { densities.iter().sum::<f32>() / densities.len() as f32 } else { 0.0 };
    log!("Detection: found {} cells with edge density > 0.04 (avg={:.3}, max={:.3})", edge_cells.len(), avg_density, max_density);

    // Group connected high-edge cells into face regions
    let mut visited = std::collections::HashSet::new();
    
    for (gx, gy, density) in edge_cells.iter() {
        if visited.contains(&(*gx, *gy)) {
            continue;
        }

        // Flood fill to find connected region
        let mut region = Vec::new();
        let mut queue = vec![(*gx, *gy)];
        
        while let Some((cx, cy)) = queue.pop() {
            if visited.contains(&(cx, cy)) || cx >= grid_size || cy >= grid_size {
                continue;
            }
            visited.insert((cx, cy));
            region.push((cx, cy));

            // Check 4 neighbors
            for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = (cx as i32 + dx) as u32;
                let ny = (cy as i32 + dy) as u32;
                if nx < grid_size && ny < grid_size && !visited.contains(&(nx, ny)) {
                    // Check if neighbor is also high edge
                    if edge_cells.iter().any(|(gx, gy, _)| *gx == nx && *gy == ny) {
                        queue.push((nx, ny));
                    }
                }
            }
        }

        // If region is large enough, treat as a face
        if region.len() >= 2 {
            let min_x = region.iter().map(|(x, _)| x).min().unwrap();
            let min_y = region.iter().map(|(_, y)| y).min().unwrap();
            let max_x = region.iter().map(|(x, _)| x).max().unwrap();
            let max_y = region.iter().map(|(_, y)| y).max().unwrap();

            let x = (*min_x * step_x) as f32;
            let y = (*min_y * step_y) as f32;
            let w = ((*max_x - *min_x + 1) * step_x) as f32;
            let h = ((*max_y - *min_y + 1) * step_y) as f32;

            let avg_density = region.iter()
                .map(|(gx, gy)| edge_cells.iter().find(|(egx, egy, _)| *egx == *gx && *egy == *gy).map(|(_, _, d)| d).unwrap_or(&0.0))
                .sum::<f32>() / region.len() as f32;
            
            let confidence = (avg_density * 1.5).min(0.95).max(0.4);

            log!("Detection: found face region {} cells, density={:.3}, confidence={:.2}", region.len(), avg_density, confidence);

            detections.push(detection::FaceDetection::new(
                detection_id,
                x,
                y,
                w,
                h,
                confidence,
            ));
            detection_id += 1;
        }
    }

    log!("Detection: final {} face regions detected", detections.len());
    detections
}

fn calculate_edge_density(data: &[u8], x: u32, y: u32, w: u32, h: u32, img_width: u32) -> f32 {
    let mut edge_count = 0;
    let mut total_count = 0;

    // Sample edges by looking at pixel differences (both horizontal and vertical)
    for dy in 0..h.saturating_sub(1) {
        for dx in 0..w.saturating_sub(1) {
            let px = x + dx;
            let py = y + dy;

            if px + 1 < img_width && py + 1 < img_width {
                let idx1 = ((py * img_width + px) * 4) as usize;
                let idx2_h = ((py * img_width + (px + 1)) * 4) as usize; // horizontal
                let idx2_v = (((py + 1) * img_width + px) * 4) as usize; // vertical

                // Check horizontal edge
                if idx1 + 2 < data.len() && idx2_h + 2 < data.len() {
                    let b1 = (data[idx1] as f32 + data[idx1 + 1] as f32 + data[idx1 + 2] as f32) / 3.0;
                    let b2 = (data[idx2_h] as f32 + data[idx2_h + 1] as f32 + data[idx2_h + 2] as f32) / 3.0;

                    if (b1 - b2).abs() > 15.0 {  // Lowered from 30.0
                        edge_count += 1;
                    }
                    total_count += 1;
                }
                
                // Check vertical edge
                if idx1 + 2 < data.len() && idx2_v + 2 < data.len() {
                    let b1 = (data[idx1] as f32 + data[idx1 + 1] as f32 + data[idx1 + 2] as f32) / 3.0;
                    let b2 = (data[idx2_v] as f32 + data[idx2_v + 1] as f32 + data[idx2_v + 2] as f32) / 3.0;

                    if (b1 - b2).abs() > 15.0 {  // Lowered from 30.0
                        edge_count += 1;
                    }
                    total_count += 1;
                }
            }
        }
    }

    if total_count > 0 {
        edge_count as f32 / total_count as f32
    } else {
        0.0
    }
}

fn sample_brightness(data: &[u8], x: u32, y: u32, w: u32, h: u32, img_width: u32) -> f32 {
    let mut total = 0.0;
    let mut count = 0;

    let sample_step = w / 4;
    let sample_step = sample_step.max(1);

    for dy in (0..h.min(20)).step_by(sample_step as usize) {
        for dx in (0..w.min(20)).step_by(sample_step as usize) {
            let px = x + dx;
            let py = y + dy;
            
            if px < img_width && py < img_width {
                let idx = ((py * img_width + px) * 4) as usize;
                if idx + 2 < data.len() {
                    let r = data[idx] as f32;
                    let g = data[idx + 1] as f32;
                    let b = data[idx + 2] as f32;
                    total += (r + g + b) / 3.0 / 255.0;
                    count += 1;
                }
            }
        }
    }

    if count > 0 {
        total / count as f32
    } else {
        0.0
    }
}

fn draw_detections_and_tracks(
    video_id: &str,
    canvas_id: &str,
    tracks: &[tracking::Track],
) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    let video: HtmlVideoElement = match document
        .get_element_by_id(video_id)
        .and_then(|el| el.dyn_into().ok())
    {
        Some(v) => v,
        None => return,
    };

    let canvas: HtmlCanvasElement = match document
        .get_element_by_id(canvas_id)
        .and_then(|el| el.dyn_into().ok())
    {
        Some(c) => c,
        None => return,
    };

    let video_width = video.video_width();
    let video_height = video.video_height();

    if video_width == 0 || video_height == 0 {
        return;
    }

    canvas.set_width(video_width);
    canvas.set_height(video_height);

    let ctx: CanvasRenderingContext2d = match canvas
        .get_context("2d")
        .ok()
        .and_then(|v| v)
        .and_then(|v| v.dyn_into().ok())
    {
        Some(c) => c,
        None => return,
    };

    ctx.clear_rect(0.0, 0.0, video_width as f64, video_height as f64);

    ctx.set_stroke_style_str("#4caf50");
    ctx.set_line_width(2.0);
    ctx.set_font("12px monospace");

    for track in tracks {
        let (x, y, w, h) = track.detection.bbox;
        
        ctx.stroke_rect(x as f64, y as f64, w as f64, h as f64);
        
        let label = format!("#{} {:.0}%", track.track_id, track.detection.confidence * 100.0);
        let text_y = if y > 20.0 { y - 5.0 } else { y + h + 15.0 };
        
        ctx.set_fill_style_str("rgba(76, 175, 80, 0.9)");
        ctx.fill_rect(x as f64, (text_y - 15.0) as f64, 80.0, 16.0);
        
        ctx.set_fill_style_str("#ffffff");
        let _ = ctx.fill_text(&label, x as f64 + 3.0, text_y as f64);
    }
}
