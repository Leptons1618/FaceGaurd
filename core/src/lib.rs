use anyhow::Result;

pub mod camera {
    #[cfg(target_arch = "wasm32")]
    pub fn ingest() {
        // Web: Use browser APIs (getUserMedia) via JS interop (to be implemented)
        // For now, just log to console
        web_sys::console::log_1(&"Camera ingest called (web)".into());
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn ingest() {
        // Native: Use GStreamer/OpenCV (to be implemented)
        println!("Camera ingest called (native)");
    }
}

pub mod detection {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FaceDetection {
        pub id: u32,
        pub bbox: (f32, f32, f32, f32), // (x, y, w, h)
        pub confidence: f32,
        pub landmarks: Option<Vec<(f32, f32)>>, // Optional facial landmarks
    }

    impl FaceDetection {
        pub fn new(id: u32, x: f32, y: f32, w: f32, h: f32, confidence: f32) -> Self {
            Self {
                id,
                bbox: (x, y, w, h),
                confidence,
                landmarks: None,
            }
        }

        pub fn area(&self) -> f32 {
            self.bbox.2 * self.bbox.3
        }

        pub fn center(&self) -> (f32, f32) {
            (self.bbox.0 + self.bbox.2 / 2.0, self.bbox.1 + self.bbox.3 / 2.0)
        }

        /// Calculate Intersection over Union with another detection
        pub fn iou(&self, other: &FaceDetection) -> f32 {
            let (x1, y1, w1, h1) = self.bbox;
            let (x2, y2, w2, h2) = other.bbox;

            let x_left = x1.max(x2);
            let y_top = y1.max(y2);
            let x_right = (x1 + w1).min(x2 + w2);
            let y_bottom = (y1 + h1).min(y2 + h2);

            if x_right < x_left || y_bottom < y_top {
                return 0.0;
            }

            let intersection = (x_right - x_left) * (y_bottom - y_top);
            let union = self.area() + other.area() - intersection;

            intersection / union
        }
    }

    pub fn detect_faces() -> Vec<FaceDetection> {
        // This will be called from WASM with actual video frame data
        vec![]
    }

    /// Process detections with NMS (Non-Maximum Suppression)
    pub fn apply_nms(mut detections: Vec<FaceDetection>, iou_threshold: f32) -> Vec<FaceDetection> {
        detections.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        let mut keep = Vec::new();
        while !detections.is_empty() {
            let best = detections.remove(0);
            keep.push(best.clone());
            
            detections.retain(|det| best.iou(det) < iou_threshold);
        }
        keep
    }
}

pub mod recognition {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FaceIdentity {
        pub id: u32,
        pub name: String,
        pub confidence: f32,
        pub embedding: Option<Vec<f32>>, // 128 or 512-dim face embedding
        pub created_at: u64,
        pub last_seen: u64,
    }

    impl FaceIdentity {
        pub fn new(id: u32, name: String) -> Self {
            let now = js_sys::Date::now() as u64;
            Self {
                id,
                name,
                confidence: 0.0,
                embedding: None,
                created_at: now,
                last_seen: now,
            }
        }

        pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
            self.embedding = Some(embedding);
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct IdentityDatabase {
        identities: Vec<FaceIdentity>,
        next_id: u32,
    }

    impl Default for IdentityDatabase {
        fn default() -> Self {
            Self::new()
        }
    }

    impl IdentityDatabase {
        pub fn new() -> Self {
            Self {
                identities: Vec::new(),
                next_id: 1,
            }
        }

        pub fn add_identity(&mut self, name: String, embedding: Option<Vec<f32>>) -> FaceIdentity {
            let mut identity = FaceIdentity::new(self.next_id, name);
            self.next_id += 1;
            
            if let Some(emb) = embedding {
                identity = identity.with_embedding(emb);
            }
            
            self.identities.push(identity.clone());
            identity
        }

        pub fn get_all(&self) -> Vec<FaceIdentity> {
            self.identities.clone()
        }

        pub fn find_by_embedding(&self, query_embedding: &[f32], threshold: f32) -> Option<(FaceIdentity, f32)> {
            let mut best_match = None;
            let mut best_similarity = threshold;

            for identity in &self.identities {
                if let Some(ref emb) = identity.embedding {
                    let similarity = cosine_similarity(query_embedding, emb);
                    if similarity > best_similarity {
                        best_similarity = similarity;
                        best_match = Some((identity.clone(), similarity));
                    }
                }
            }

            best_match
        }

        pub fn update_last_seen(&mut self, id: u32) {
            if let Some(identity) = self.identities.iter_mut().find(|i| i.id == id) {
                identity.last_seen = js_sys::Date::now() as u64;
            }
        }
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if mag_a == 0.0 || mag_b == 0.0 {
            return 0.0;
        }

        dot / (mag_a * mag_b)
    }

    // Legacy function for compatibility
    pub fn recognize_faces() -> Vec<FaceIdentity> {
        vec![]
    }
}

pub mod tracking {
    use super::detection::FaceDetection;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Track {
        pub track_id: u32,
        pub detection: FaceDetection,
        pub frames_tracked: u32,
        pub last_seen: u64, // timestamp
        pub identity_id: Option<u32>,
    }

    impl Track {
        pub fn new(track_id: u32, detection: FaceDetection, timestamp: u64) -> Self {
            Self {
                track_id,
                detection,
                frames_tracked: 1,
                last_seen: timestamp,
                identity_id: None,
            }
        }

        pub fn update(&mut self, detection: FaceDetection, timestamp: u64) {
            self.detection = detection;
            self.frames_tracked += 1;
            self.last_seen = timestamp;
        }
    }

    pub struct Tracker {
        tracks: Vec<Track>,
        next_id: u32,
        iou_threshold: f32,
        max_age: u64, // milliseconds
    }

    impl Tracker {
        pub fn new(iou_threshold: f32, max_age_ms: u64) -> Self {
            Self {
                tracks: Vec::new(),
                next_id: 1,
                iou_threshold,
                max_age: max_age_ms,
            }
        }

        pub fn update(&mut self, detections: Vec<FaceDetection>, timestamp: u64) -> Vec<Track> {
            // Remove stale tracks
            self.tracks.retain(|t| timestamp - t.last_seen < self.max_age);

            let mut unmatched_detections = detections;
            let mut matched_tracks = Vec::new();

            // Match detections to existing tracks using IOU
            for track in &mut self.tracks {
                let mut best_match_idx = None;
                let mut best_iou = self.iou_threshold;

                for (idx, det) in unmatched_detections.iter().enumerate() {
                    let iou = track.detection.iou(det);
                    if iou > best_iou {
                        best_iou = iou;
                        best_match_idx = Some(idx);
                    }
                }

                if let Some(idx) = best_match_idx {
                    let detection = unmatched_detections.remove(idx);
                    track.update(detection, timestamp);
                    matched_tracks.push(track.clone());
                }
            }

            // Create new tracks for unmatched detections
            for detection in unmatched_detections {
                let track = Track::new(self.next_id, detection, timestamp);
                self.next_id += 1;
                self.tracks.push(track.clone());
                matched_tracks.push(track);
            }

            matched_tracks
        }

        pub fn get_active_tracks(&self) -> Vec<Track> {
            self.tracks.clone()
        }
    }

    // Legacy function for compatibility
    pub fn track_faces() -> Vec<Track> {
        vec![]
    }
}

pub mod events {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum EventType {
        FaceDetected,
        FaceRecognized,
        UnknownFace,
        LowConfidence,
        Blacklisted,
        AfterHours,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FaceEvent {
        pub id: u32,
        pub event_type: EventType,
        pub name: String,
        pub confidence: f32,
        pub timestamp: u64,
        pub track_id: Option<u32>,
    }

    impl FaceEvent {
        pub fn new(id: u32, event_type: EventType, name: String, confidence: f32) -> Self {
            Self {
                id,
                event_type,
                name,
                confidence,
                timestamp: js_sys::Date::now() as u64,
                track_id: None,
            }
        }

        pub fn with_track(mut self, track_id: u32) -> Self {
            self.track_id = Some(track_id);
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EventLog {
        events: Vec<FaceEvent>,
        next_id: u32,
        max_events: usize,
    }

    impl Default for EventLog {
        fn default() -> Self {
            Self::new(1000)
        }
    }

    impl EventLog {
        pub fn new(max_events: usize) -> Self {
            Self {
                events: Vec::new(),
                next_id: 1,
                max_events,
            }
        }

        pub fn add_event(&mut self, event_type: EventType, name: String, confidence: f32, track_id: Option<u32>) -> FaceEvent {
            let mut event = FaceEvent::new(self.next_id, event_type, name, confidence);
            self.next_id += 1;
            
            if let Some(tid) = track_id {
                event = event.with_track(tid);
            }
            
            self.events.push(event.clone());
            
            // Keep only recent events
            if self.events.len() > self.max_events {
                self.events.drain(0..self.events.len() - self.max_events);
            }
            
            event
        }

        pub fn get_all(&self) -> Vec<FaceEvent> {
            self.events.clone()
        }

        pub fn get_recent(&self, count: usize) -> Vec<FaceEvent> {
            let start = if self.events.len() > count {
                self.events.len() - count
            } else {
                0
            };
            self.events[start..].to_vec()
        }

        pub fn filter_by_type(&self, event_type: EventType) -> Vec<FaceEvent> {
            self.events.iter()
                .filter(|e| e.event_type == event_type)
                .cloned()
                .collect()
        }
    }

    pub fn log_event(_event: &FaceEvent) {
        // Compatibility function
    }

    // Legacy function for compatibility
    pub fn generate_events() -> Vec<FaceEvent> {
        vec![]
    }
}

pub fn detect_faces() -> Result<Vec<events::FaceEvent>> {
    // Legacy compatibility function - use modules directly instead
    Ok(vec![])
}
