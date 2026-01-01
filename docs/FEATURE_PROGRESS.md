# Feature Development Progress

## Completed
- Project scaffolding
- Initial documentation and architecture/feature references
- Multi-page UI with navigation (Dashboard, Live Feed, Events, Settings)
- Webcam capture and live feed display
- Canvas overlay for bounding box visualization
- Event filtering and CSV/JSON export

## In Progress
- Real video frame processing pipeline with detection inference
- Face detection backend integration (need to add actual ML models)
- Face recognition with identity management
- Object tracking across frames

## Upcoming
- Replace mock detections with actual model inference (RetinaFace/ArcFace)
- WASM/web ML model deployment
- Native camera backends (GStreamer/OpenCV for desktop)
- Edge device optimization (Jetson, CUDA/TensorRT)
- Plugin system and advanced features (liveness, multi-camera, mobile app)
