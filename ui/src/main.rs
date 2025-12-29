use dioxus::prelude::*;
use core::detect_faces;

fn main() {
    launch(App);
}

fn App() -> Element {
    let faces = detect_faces().unwrap_or_default();

    rsx! {
        div { class: "p-4",
            h1 { "FaceGuard – Security Dashboard" }
            ul {
                for face in faces {
                    li {
                        "{face.name} ({face.confidence})"
                    }
                }
            }
        }
    }
}
