# FaceGuard — System Architecture

FaceGuard is a high-performance, on-device face detection and recognition system designed for
home and office security. It is optimized for **edge AI devices** (NVIDIA Jetson Orin Nano)
and built entirely in **Rust** for safety, performance, and long-term maintainability.

---

## 1. High-Level Overview

```

┌───────────────────────────────┐
│        Dioxus Frontend        │
│  (Web / WebAssembly UI)       │
│                               │
│  • Live camera view            │
│  • Face timeline               │
│  • User enrollment             │
│  • Alerts & logs               │
└───────────────┬───────────────┘
│ IPC (Tauri)
┌───────────────▼───────────────┐
│        Tauri Backend           │
│  (Native Rust Application)    │
│                               │
│  • Secure IPC                  │
│  • OS integration              │
│  • Permissions & sandboxing    │
└───────────────┬───────────────┘
│ Rust API calls
┌───────────────▼───────────────┐
│        Core Engine             │
│  (Pure Rust Workspace Crate)  │
│                               │
│  • Camera ingestion            │
│  • Face detection              │
│  • Face recognition            │
│  • Tracking & events           │
└───────────────┬───────────────┘
│ CUDA / TensorRT
┌───────────────▼───────────────┐
│   GPU / Accelerator Layer     │
│                               │
│  • NVIDIA CUDA                 │
│  • TensorRT (INT8 / FP16)      │
│  • Jetson DLA (future)         │
└───────────────────────────────┘

```

---

## 2. Repository Layout

```

faceguard/
├── Cargo.toml          # Workspace root
├── core/               # Vision + recognition engine
│   ├── src/
│   │   ├── camera/
│   │   ├── detection/
│   │   ├── recognition/
│   │   ├── tracking/
│   │   └── events/
│   └── Cargo.toml
│
├── ui/                 # Dioxus frontend
│   ├── src/
│   ├── assets/
│   ├── Cargo.toml
│   └── src-tauri/      # Tauri native backend
│       ├── src/
│       ├── tauri.conf.json
│       └── Cargo.toml
│
└── README_*.md

```

---

## 3. Core Engine Architecture (`core`)

### 3.1 Camera Pipeline
- USB / CSI / RTSP cameras
- GStreamer or OpenCV backend
- Zero-copy GPU buffers (Jetson)
- Frame batching support

```

Camera → Decoder → Frame Buffer → Detector

```

---

### 3.2 Face Detection
- Models: RetinaFace / MTCNN (ONNX)
- GPU-accelerated inference
- Bounding box + landmarks
- Multi-face support

---

### 3.3 Face Recognition
- Models: ArcFace / MobileFaceNet
- Embedding vector generation
- Cosine similarity matching
- Multiple embeddings per identity

```

Face Crop → Alignment → Embedding → Similarity Search

```

---

### 3.4 Tracking
- SORT / DeepSORT
- Temporal ID assignment
- Reduces duplicate recognition calls
- Improves FPS

---

### 3.5 Event Engine
- Rule-based triggers
- Confidence thresholds
- Unknown / blacklisted detection
- Alert dispatch

---

## 4. Frontend Architecture (Dioxus)

- WebAssembly (WASM)
- Reactive component model
- GPU-accelerated rendering
- Shared Rust types via workspace crate

### UI Components
- Live camera feed
- Recognition timeline
- User enrollment forms
- Event log viewer
- System health panel

---

## 5. IPC & Security (Tauri)

- Strongly-typed IPC commands
- No direct JS access to OS
- Sandboxed permissions
- Native notifications & tray support

---

## 6. Deployment Targets

| Platform | Support |
|-------|--------|
| Linux (x86_64) | ✅ |
| Windows | ✅ |
| Jetson Orin Nano (aarch64) | ✅ |
| Cloud dependency | ❌ (optional only) |

---

## 7. Design Principles

- **On-device first** (privacy)
- **Zero-copy where possible**
- **No Electron**
- **No cloud lock-in**
- **Deterministic performance**