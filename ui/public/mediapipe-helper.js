// MediaPipe Face Detection Integration
let faceDetector = null;
let isMediaPipeReady = false;

// Initialize MediaPipe when script loads
async function initMediaPipe() {
    try {
        // Load MediaPipe Face Detection
        const vision = await import('https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.2/vision_bundle.js');
        
        const { FaceDetector, FilesetResolver } = vision;
        
        const filesetResolver = await FilesetResolver.forVisionTasks(
            "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.2/wasm"
        );
        
        faceDetector = await FaceDetector.createFromOptions(filesetResolver, {
            baseOptions: {
                modelAssetPath: 'https://storage.googleapis.com/mediapipe-models/face_detector/blaze_face_short_range/float16/1/blaze_face_short_range.tflite',
                delegate: "GPU"
            },
            runningMode: "VIDEO",
            minDetectionConfidence: 0.5
        });
        
        isMediaPipeReady = true;
        console.log("✓ MediaPipe Face Detector initialized");
    } catch (error) {
        console.warn("Failed to initialize MediaPipe:", error);
        isMediaPipeReady = false;
    }
}

// Detect faces in video element
window.detectFacesMediaPipe = function(videoId) {
    if (!isMediaPipeReady || !faceDetector) {
        return null;
    }
    
    try {
        const video = document.getElementById(videoId);
        if (!video || video.readyState < 2) {
            return null;
        }
        
        const startTimeMs = performance.now();
        const detections = faceDetector.detectForVideo(video, startTimeMs);
        
        if (!detections || !detections.detections || detections.detections.length === 0) {
            return [];
        }
        
        // Convert MediaPipe detections to our format
        const results = detections.detections.map(detection => {
            const bbox = detection.boundingBox;
            return {
                x: bbox.originX,
                y: bbox.originY,
                width: bbox.width,
                height: bbox.height,
                score: detection.categories[0]?.score || 0.5
            };
        });
        
        return results;
    } catch (error) {
        console.error("MediaPipe detection error:", error);
        return null;
    }
};

// Initialize on load
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initMediaPipe);
} else {
    initMediaPipe();
}
