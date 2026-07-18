use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

pub struct ProgressTracker {
    downloaded: Arc<AtomicU64>,
    total: Option<u64>,
}

impl ProgressTracker {
    pub fn new(total: Option<u64>) -> Self {
        Self {
            downloaded: Arc::new(AtomicU64::new(0)),
            total,
        }
    }

    pub fn add_bytes(&mut self, bytes: u64) {
        self.downloaded.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn downloaded(&self) -> u64 {
        self.downloaded.load(Ordering::Relaxed)
    }

    pub fn total(&self) -> Option<u64> {
        self.total
    }

    pub fn percentage(&self) -> Option<f64> {
        self.total
            .map(|total| (self.downloaded() as f64 / total as f64) * 100.0)
    }
}
