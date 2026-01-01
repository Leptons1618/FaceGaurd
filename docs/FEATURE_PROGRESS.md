# Feature Development Progress

## Completed ✅
- Project scaffolding
- Initial documentation and architecture/feature references
- Multi-page UI with navigation (Dashboard, Register, Events, Settings)
- Webcam capture and live feed display (WebRTC getUserMedia)
- Canvas overlay for bounding box visualization
- Event logging system with LocalStorage persistence
- Identity registration with face capture
- Real-time frame processing pipeline (100ms interval)
- Face tracking with IOU-based matching
- Non-Maximum Suppression (NMS) filtering
- FPS calculation and display
- Unknown face event detection and logging
- Unified Dashboard layout (no scrollbars)
- Status indicators (Active, Detecting, No Faces)

## In Progress 🔄
- **Face detection model integration** (See ISSUES.md #1)
  - MediaPipe BlazeFace implemented but not loading correctly
  - Currently using fallback edge-density detection algorithm
- Face recognition with embedding matching
- Identity database CRUD operations UI

## Upcoming 📋
- Replace fallback detection with working ML model (MediaPipe/ONNX)
- Face embedding extraction for recognition
- Identity matching against detected faces
- Event filtering and search UI
- WASM/web ML model optimization
- Native camera backends (GStreamer/OpenCV for desktop)
- Edge device optimization (Jetson, CUDA/TensorRT)
- Plugin system and advanced features (liveness, multi-camera, mobile app)

## Known Issues
See [ISSUES.md](ISSUES.md) for detailed bug tracking and technical context.

