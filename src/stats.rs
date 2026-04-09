use std::sync::{Arc, Mutex};
use std::time::Instant;

const MAX_LINE_LENGTHS: usize = 10_000;
const MAX_SPARKLINE_ENTRIES: usize = 60;

#[derive(Debug, Clone)]
pub struct StatsSnapshot {
    pub elapsed_secs: f64,
    pub total_lines: u64,
    pub total_bytes: u64,
    pub throughput_lines: f64,
    pub throughput_bytes: f64,
    pub sparkline: Vec<f64>,
    pub line_lengths: Vec<u64>,
}

impl StatsSnapshot {
    /// Effective throughput: use tick-based value if available,
    /// otherwise fall back to total/elapsed (useful before first tick).
    pub fn effective_throughput_lines(&self) -> f64 {
        if self.throughput_lines > 0.0 {
            self.throughput_lines
        } else if self.elapsed_secs > 0.0 {
            self.total_lines as f64 / self.elapsed_secs
        } else {
            0.0
        }
    }

    pub fn effective_throughput_bytes(&self) -> f64 {
        if self.throughput_bytes > 0.0 {
            self.throughput_bytes
        } else if self.elapsed_secs > 0.0 {
            self.total_bytes as f64 / self.elapsed_secs
        } else {
            0.0
        }
    }
}

#[derive(Debug)]
struct StatsState {
    start_time: Instant,
    total_lines: u64,
    total_bytes: u64,
    lines_since_last_tick: u64,
    bytes_since_last_tick: u64,
    throughput_lines: f64,
    throughput_bytes: f64,
    sparkline: Vec<f64>,
    line_lengths: Vec<u64>,
}

impl StatsState {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_lines: 0,
            total_bytes: 0,
            lines_since_last_tick: 0,
            bytes_since_last_tick: 0,
            throughput_lines: 0.0,
            throughput_bytes: 0.0,
            sparkline: Vec::new(),
            line_lengths: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct StatsCollector {
    inner: Arc<Mutex<StatsState>>,
}

impl Default for StatsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsCollector {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(StatsState::new())),
        }
    }

    /// Record a line, incrementing counters and storing its byte length.
    pub fn record_line(&self, byte_len: u64) {
        let mut state = self.inner.lock().unwrap();
        state.total_lines += 1;
        state.total_bytes += byte_len;
        state.lines_since_last_tick += 1;
        state.bytes_since_last_tick += byte_len;

        // Store byte_len in line_lengths, capped at MAX_LINE_LENGTHS
        if state.line_lengths.len() >= MAX_LINE_LENGTHS {
            state.line_lengths.remove(0);
        }
        state.line_lengths.push(byte_len);
    }

    /// Called by TUI on each tick. Computes throughput and updates sparkline.
    pub fn tick(&self, interval_secs: f64) {
        let mut state = self.inner.lock().unwrap();

        // Compute throughput from since_last_tick counters
        state.throughput_lines = state.lines_since_last_tick as f64 / interval_secs;
        state.throughput_bytes = state.bytes_since_last_tick as f64 / interval_secs;

        // Push throughput_lines to sparkline, capped at MAX_SPARKLINE_ENTRIES
        let throughput_lines = state.throughput_lines;
        if state.sparkline.len() >= MAX_SPARKLINE_ENTRIES {
            state.sparkline.remove(0);
        }
        state.sparkline.push(throughput_lines);

        // Reset since_last_tick counters
        state.lines_since_last_tick = 0;
        state.bytes_since_last_tick = 0;
    }

    /// Return a snapshot of current state for rendering.
    pub fn snapshot(&self) -> StatsSnapshot {
        let state = self.inner.lock().unwrap();
        let elapsed_secs = state.start_time.elapsed().as_secs_f64();

        StatsSnapshot {
            elapsed_secs,
            total_lines: state.total_lines,
            total_bytes: state.total_bytes,
            throughput_lines: state.throughput_lines,
            throughput_bytes: state.throughput_bytes,
            sparkline: state.sparkline.clone(),
            line_lengths: state.line_lengths.clone(),
        }
    }

    /// Clone the Arc handle for sharing across threads.
    pub fn clone_handle(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_line_increments_totals() {
        let stats = StatsCollector::new();
        stats.record_line(100);
        stats.record_line(50);

        let snap = stats.snapshot();
        assert_eq!(snap.total_lines, 2);
        assert_eq!(snap.total_bytes, 150);
    }

    #[test]
    fn tick_computes_throughput() {
        let stats = StatsCollector::new();
        stats.record_line(100);
        stats.record_line(100);
        stats.record_line(100);
        stats.tick(1.0);

        let snap = stats.snapshot();
        assert!((snap.throughput_lines - 3.0).abs() < 0.01);
        assert!((snap.throughput_bytes - 300.0).abs() < 0.01);
    }

    #[test]
    fn tick_resets_counters() {
        let stats = StatsCollector::new();
        stats.record_line(100);
        stats.tick(1.0);
        stats.tick(1.0);

        let snap = stats.snapshot();
        // After second tick with no new data, throughput should be 0
        assert!((snap.throughput_lines - 0.0).abs() < 0.01);
        assert!((snap.throughput_bytes - 0.0).abs() < 0.01);
    }

    #[test]
    fn sparkline_accumulates() {
        let stats = StatsCollector::new();
        stats.record_line(100);
        stats.record_line(100);
        stats.record_line(100);
        stats.tick(1.0);

        stats.record_line(50);
        stats.record_line(50);
        stats.tick(1.0);

        let snap = stats.snapshot();
        assert_eq!(snap.sparkline.len(), 2);
        assert!((snap.sparkline[0] - 3.0).abs() < 0.01); // First tick: 3 lines
        assert!((snap.sparkline[1] - 2.0).abs() < 0.01); // Second tick: 2 lines
    }

    #[test]
    fn line_lengths_recorded() {
        let stats = StatsCollector::new();
        stats.record_line(100);
        stats.record_line(200);
        stats.record_line(50);

        let snap = stats.snapshot();
        assert_eq!(snap.line_lengths.len(), 3);
        assert_eq!(snap.line_lengths[0], 100);
        assert_eq!(snap.line_lengths[1], 200);
        assert_eq!(snap.line_lengths[2], 50);
    }
}
