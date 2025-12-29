use anyhow::Result;

#[derive(Debug, Clone)]
pub struct FaceEvent {
    pub id: u32,
    pub name: String,
    pub confidence: f32,
}

pub fn detect_faces() -> Result<Vec<FaceEvent>> {
    // Fake data for now
    Ok(vec![
        FaceEvent {
            id: 1,
            name: "Anish".into(),
            confidence: 0.93,
        }
    ])
}
