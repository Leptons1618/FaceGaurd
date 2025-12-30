use dioxus::prelude::*;
use faceguard_core::camera;
use faceguard_core::detection;
use faceguard_core::recognition;

fn main() {
    launch(app);
}

fn app() -> Element {
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
            h2 { "Recognized Identities" }
            ul {
                for ident in identities {
                    li {
                        "{ident.name} (ID: {ident.id}, confidence: {ident.confidence})"
                    }
                }
            }
        }
    }
}
