# Issue Tracker

## Active Issues

### #1 - MediaPipe Face Detection Not Loading
**Status:** 🔴 Open  
**Priority:** High  
**Component:** Detection / UI  
**Created:** 2026-01-01

**Description:**  
MediaPipe BlazeFace model integration is implemented but not initializing correctly in the browser. The system falls back to a simple edge-density detection algorithm.

**Technical Details:**
- **Implementation Location:** `ui/src/main.rs` lines 320-390 (inline script in document head)
- **Model:** MediaPipe BlazeFace Short Range (TFLite)
- **CDN:** `@mediapipe/tasks-vision@0.10.2`
- **Detection Function:** `window.detectFacesMediaPipe(videoId)` via JS interop
- **Current Behavior:** Script loads but MediaPipe initialization likely fails silently
- **Fallback:** Custom edge-density algorithm (8x8 grid, edge threshold 15.0, density threshold 0.04)

**Symptoms:**
- No console message: `"✓ MediaPipe Face Detector initialized"`
- Console shows: `"Falling back to simple edge detection"`
- Bounding boxes are imprecise (fallback algorithm is not ML-based)

**Attempted Fixes:**
1. ✅ Embedded MediaPipe script inline in document head (vs external file)
2. ✅ Added JS interop via `wasm-bindgen` extern functions
3. ✅ Implemented graceful fallback to edge detection
4. ❌ Still not loading - may be CORS, async loading timing, or CDN issue

**Reproduction:**
1. Build and run: `cd ui && dx serve --release`
2. Open browser console at `http://localhost:8080`
3. Check for MediaPipe initialization message
4. Observe detection logs show fallback method

**Potential Solutions:**
- [ ] Use local MediaPipe WASM files instead of CDN
- [ ] Switch to ONNX Runtime Web with custom TFLite conversion
- [ ] Try alternative: `tensorflow.js` with BlazeFace model
- [ ] Add more detailed error logging in MediaPipe init
- [ ] Check browser compatibility (MediaPipe requires modern browsers)
- [ ] Investigate async timing - may need to delay detection until model ready

**Related Files:**
- `ui/src/main.rs` - Main detection pipeline
- `ui/public/mediapipe-helper.js` - Standalone JS (not currently used)

**Workaround:**  
System currently uses edge-density fallback detection. Works but less accurate.

---

### #2 - Bounding Box Position Offset
**Status:** 🟡 Partial Fix  
**Priority:** Medium  
**Component:** Detection  
**Created:** 2026-01-01

**Description:**  
When using fallback edge-density detection, bounding boxes are not precisely aligned with faces.

**Root Cause:**  
Edge-density algorithm groups grid cells (8x8, 80x60 pixels each) and creates boxes from connected regions. This is inherently less precise than ML model keypoint detection.

**Fix Applied:**
- Changed coordinate calculation from `(*min_x as f32) * (step_x as f32)` to `(*min_x * step_x) as f32`
- Improved but still not pixel-perfect due to algorithm limitations

**Resolution:**  
Will be resolved when MediaPipe integration (Issue #1) is fixed. ML models provide exact pixel coordinates.

---

## Closed Issues

### #3 - Frame Processing Not Running (0 FPS, 0 Frames)
**Status:** ✅ Closed  
**Resolved:** 2026-01-01  
**Component:** UI / State Management

**Description:**  
Frame processing interval wasn't being stored in signal, causing it to be dropped immediately after creation.

**Solution:**  
Added `_interval_handle` signal to keep `gloo_timers::Interval` alive across re-renders.

**Changed Files:**
- `ui/src/main.rs` - Added `_interval_handle: Signal<Option<Interval>>`

---

### #4 - Over-Detection (143 Faces Per Frame)
**Status:** ✅ Closed  
**Resolved:** 2026-01-01  
**Component:** Detection Algorithm

**Description:**  
Initial brightness-based detection was flagging all grid cells as faces because video background had uniform brightness (~0.43).

**Solution:**  
Replaced brightness-based detection with edge-density detection looking for high-contrast pixel transitions (facial features like eyes, nose, mouth).

**Algorithm Changes:**
- Metric: Brightness threshold → Edge density (pixel contrast)
- Grid: 16x16 → 8x8 (coarser)
- Threshold: Brightness 0.3-0.75 → Edge density > 0.04
- Edge detection: Horizontal + vertical pixel differences > 15.0
- Grouping: Flood-fill connected components

**Result:**  
Detection dropped from 143 false positives to 1-3 actual faces.

---

### #5 - No Unknown Face Events Being Logged
**Status:** ✅ Closed  
**Resolved:** 2026-01-01  
**Component:** Events / Persistence

**Description:**  
Detected faces weren't triggering `UnknownFace` event creation.

**Solution:**  
Added event logging in frame processing loop with deduplication (5-second window per track ID).

**Implementation:**
- Check active tracks after detection
- Log `EventType::UnknownFace` for new tracks
- Save to LocalStorage via `save_event_log()`
- Prevent duplicate events for same track within 5 seconds

**Changed Files:**
- `ui/src/main.rs` - Added event logging after tracker update (lines ~427-450)

---

## Issue Labels

- 🔴 **Open** - Active issue requiring attention
- 🟡 **Partial Fix** - Workaround in place, root cause remains
- ✅ **Closed** - Resolved and tested
- 🔵 **Enhancement** - Feature request, not a bug
- 🟣 **Investigation** - Root cause unknown, needs debugging

## Contributing

To report a new issue:
1. Add to "Active Issues" section with next available number
2. Include: Status, Priority, Component, Description, Technical Details
3. Update FEATURE_PROGRESS.md if it affects feature completion
4. Reference issue number in commit messages: `fix: bounding box calculation (#2)`
