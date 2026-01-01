# FaceGuard Documentation Index

Welcome to the FaceGuard documentation. This index provides links and context for all major documents:

- [ARCHITECTURE.md](ARCHITECTURE.md): System architecture, hardware targets, and design principles.
- [FEATURES.md](FEATURES.md): Complete feature list, planned features, and non-goals.
- [PLANS.md](PLANS.md): Project goals, milestones, and roadmap.
- [CHANGELOG.md](CHANGELOG.md): Release notes and change history.
- [FEATURE_PROGRESS.md](FEATURE_PROGRESS.md): Current status of feature development.
- [ISSUES.md](ISSUES.md): **NEW** - Active bug tracking, known issues, and resolutions.

---

## Current Status (2026-01-01)

### ✅ Working Features
- Real-time camera capture and display (WebRTC)
- Face detection with tracking (edge-density fallback algorithm)
- Identity registration and LocalStorage persistence
- Unknown face event logging
- Multi-page UI (Dashboard, Register, Events, Settings)
- FPS and performance metrics

### 🔴 Known Issues
- **MediaPipe Integration Not Loading** - See [ISSUES.md](ISSUES.md) #1
- **Bounding Box Position Offset** - See [ISSUES.md](ISSUES.md) #2

### 🔄 In Progress
- MediaPipe BlazeFace integration (implemented, troubleshooting)
- Face recognition with embedding matching

---

**Next Steps:**
- See [PLANS.md](PLANS.md) for upcoming goals and milestones.
- Track progress in [FEATURE_PROGRESS.md](FEATURE_PROGRESS.md).
- Report bugs or check status in [ISSUES.md](ISSUES.md).

