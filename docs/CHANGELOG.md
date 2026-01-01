# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - 2026-01-01

### Added
- Multi-page navigation system (Dashboard, Register, Events, Settings)
- Real-time face detection with 100ms frame processing pipeline
- WebRTC camera capture via getUserMedia API
- Canvas overlay for bounding box visualization
- Edge-density based fallback face detection algorithm
- Non-Maximum Suppression (NMS) for duplicate filtering (IOU < 0.3)
- IOU-based face tracking across frames
- FPS calculation and real-time display
- Identity registration with face capture functionality
- Unknown face event logging system with track association
- LocalStorage persistence for identities and events (JSON serialization)
- Event deduplication (5-second window per track ID)
- MediaPipe BlazeFace integration (implemented but not loading - see ISSUES.md #1)
- Status indicators (Active, Detecting, No Faces)
- Unified Dashboard layout without scrollbars
- Comprehensive debug logging for frame processing

### Changed
- Detection algorithm: brightness-based → edge-density based
- Grid size: 16x16 → 8x8 cells (80x60 pixels per cell)
- Edge detection threshold: 30.0 → 15.0 (pixel brightness difference)
- Edge density threshold: 0.08 → 0.04 (4% edge pixels required)
- NMS IOU threshold: 0.4 → 0.3 (more aggressive filtering)
- Added both horizontal and vertical edge detection
- Bounding box coordinate calculation: proper grid-to-pixel conversion
- Frame logging interval: every 10 frames → every 5 frames

### Fixed
- Frame processing interval now stored in signal to prevent dropping (#3)
- Camera readiness checks before frame extraction
- Over-detection issue (143 → 1-3 faces) via edge-density algorithm (#4)
- Unknown face events now properly logged and persisted (#5)
- Bounding box position offset (partial fix, pending MediaPipe) (#2)
- Removed unused `sample_variance` function

### Known Issues
- MediaPipe Face Detection not initializing correctly (using fallback) - See ISSUES.md #1
- Bounding box precision limited by fallback algorithm accuracy - See ISSUES.md #2

---

## [2025-12-31]
- Initial project structure and setup.
- Core and UI crates established.
- WASM build integration.
- Core engine scaffolds for camera/detection/recognition/tracking/events returning sample data for the UI.
- Prototype Dioxus dashboard with detections, tracks, identities, and events plus filtering and CSV/JSON export.
- Project documentation and planning initiated.

