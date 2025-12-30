# FaceGuard — Feature List

This document describes **current**, **planned**, and **future** features of FaceGuard.

---

## 1. Core Security Features

### Face Detection
- Real-time multi-face detection
- GPU-accelerated inference
- Low-light tolerant models
- Adjustable confidence thresholds

---

### Face Recognition
- High-accuracy embeddings
- Multiple faces per identity
- Cosine similarity matching
- Fast vector search (FAISS planned)

---

### Identity Management
- User enrollment from camera
- Multiple samples per user
- Identity re-training
- Encrypted storage

---

## 2. Camera & Input Support

- USB cameras
- CSI cameras (Jetson)
- RTSP streams
- Multi-camera support (planned)
- Frame rate control

---

## 3. Event & Alert System

- Unknown face detection
- Blacklisted identity detection
- After-hours access alerts
- Custom rules engine
- Webhook notifications
- Local system notifications

---

## 4. User Interface

- Modern Dioxus UI
- Live camera preview
- Recognition timeline
- User management dashboard
- Dark mode
- Low-latency rendering (60 FPS target)

---

## 5. Performance & Optimization

- GPU inference (CUDA / TensorRT)
- INT8 / FP16 model optimization
- Frame skipping & batching
- Power-aware execution (Jetson)

---

## 6. Privacy & Security

- Fully offline operation
- No cloud dependency
- Encrypted face embeddings
- Secure boot support (Jetson)
- Minimal attack surface (Tauri)

---

## 7. Platform Support

| Platform | Status |
|-------|-------|
| Linux Desktop | ✅ |
| Windows Desktop | ✅ |
| NVIDIA Jetson Orin Nano | ✅ |
| Docker | Optional |
| Kubernetes | ❌ |

---

## 8. Planned Features

- Liveness detection (anti-spoofing)
- Mask / sunglasses handling
- Multi-camera fusion
- Mobile companion app
- Cloud sync (opt-in)
- Voice alerts
- Access control integration

---

## 9. Non-Goals (Intentional)

- ❌ Cloud-only inference
- ❌ Electron-based UI
- ❌ Centralized biometric storage
- ❌ Vendor lock-in

---

## 10. Target Performance

| Metric | Target |
|------|-------|
| Detection FPS | 25–30 |
| Recognition latency | <40 ms |
| Power usage (Jetson) | <10 W |
| Cold boot time | <3 seconds |
