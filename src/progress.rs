pub struct ProgressTracker {
    downloaded: u64,
    total: Option<u64>,
}

impl ProgressTracker {
    pub fn new(total: Option<u64>) -> Self {
        Self {
            downloaded: 0,
            total,
        }
    }

    pub fn add_bytes(&mut self, bytes: u64) {
        self.downloaded += bytes;
    }

    pub fn downloaded(&self) -> u64 {
        self.downloaded
    }

    pub fn total(&self) -> Option<u64> {
        self.total
    }

    pub fn percentage(&self) -> Option<f64> {
        self.total
            .map(|total| (self.downloaded as f64 / total as f64) * 100.0)
    }
}
