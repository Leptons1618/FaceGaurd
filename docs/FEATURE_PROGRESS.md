# Feature Development Progress

## Completed
- Project scaffolding
- Initial documentation and architecture/feature references
- Prototype Dioxus dashboard that renders detections, tracks, identities, and event lists
- Event filtering and CSV/JSON export from the UI mock data

## In Progress
- Core engine scaffolds: camera ingestion (native/web), detection, recognition, tracking, and event generation returning stub data
- Wiring UI to core crate types and sample events; Tauri backend IPC prep
- Camera ingestion backends (GStreamer/OpenCV native; getUserMedia for web)
- Event logging/rules engine and identity management integration

## Upcoming
- Replace stubs with real inference models (RetinaFace/ArcFace), tracking pipeline, and data persistence
- WASM/web support and build pipeline
- Edge device optimization (Jetson, CUDA/TensorRT)
- Plugin system and advanced recognition features (liveness, multi-camera fusion, mobile companion app)
