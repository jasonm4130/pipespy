use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

const MAX_SAMPLES: usize = 20;

#[derive(Debug)]
struct BufferState {
    data: VecDeque<Vec<u8>>,
    total_bytes: usize,
    done: bool,
    samples: VecDeque<String>,
    max_samples: usize,
}

impl BufferState {
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
            total_bytes: 0,
            done: false,
            samples: VecDeque::new(),
            max_samples: MAX_SAMPLES,
        }
    }
}

#[derive(Clone)]
pub struct SharedBuffer {
    inner: Arc<(Mutex<BufferState>, Condvar)>,
    capacity: usize,
}

impl SharedBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Arc::new((Mutex::new(BufferState::new()), Condvar::new())),
            capacity,
        }
    }

    /// Push a line into the buffer, blocking if the buffer is at capacity.
    pub fn push(&self, line: Vec<u8>) {
        let (lock, cvar) = &*self.inner;
        let mut state = lock.lock().unwrap();

        // Block while buffer is full and not done
        while state.total_bytes + line.len() > self.capacity && !state.done {
            state = cvar.wait(state).unwrap();
        }

        let len = line.len();

        // Store a lossy UTF-8 sample
        let sample = String::from_utf8_lossy(&line).into_owned();
        if state.samples.len() >= state.max_samples {
            state.samples.pop_front();
        }
        state.samples.push_back(sample);

        state.data.push_back(line);
        state.total_bytes += len;

        cvar.notify_one();
    }

    /// Pop a line from the buffer, blocking if empty. Returns None when done and empty.
    pub fn pop(&self) -> Option<Vec<u8>> {
        let (lock, cvar) = &*self.inner;
        let mut state = lock.lock().unwrap();

        loop {
            if let Some(line) = state.data.pop_front() {
                state.total_bytes -= line.len();
                cvar.notify_one();
                return Some(line);
            }

            if state.done {
                return None;
            }

            state = cvar.wait(state).unwrap();
        }
    }

    /// Signal that no more data will be pushed.
    pub fn mark_done(&self) {
        let (lock, cvar) = &*self.inner;
        let mut state = lock.lock().unwrap();
        state.done = true;
        cvar.notify_all();
    }

    /// Return a snapshot of recent samples for TUI display.
    pub fn get_samples(&self) -> Vec<String> {
        let (lock, _) = &*self.inner;
        let state = lock.lock().unwrap();
        state.samples.iter().cloned().collect()
    }

    /// Clone the Arc handle for sharing across threads.
    pub fn clone_handle(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn push_pop_single_line() {
        let buf = SharedBuffer::new(1024);
        let line = b"hello world".to_vec();
        buf.push(line.clone());
        let result = buf.pop();
        assert_eq!(result, Some(line));
    }

    #[test]
    fn pop_returns_none_when_done() {
        let buf = SharedBuffer::new(1024);
        buf.push(b"first line".to_vec());
        buf.mark_done();
        let first = buf.pop();
        assert_eq!(first, Some(b"first line".to_vec()));
        let second = buf.pop();
        assert_eq!(second, None);
    }

    #[test]
    fn samples_are_stored() {
        let buf = SharedBuffer::new(1024);
        buf.push(b"line one".to_vec());
        buf.push(b"line two".to_vec());
        let samples = buf.get_samples();
        assert!(samples.contains(&"line one".to_string()));
        assert!(samples.contains(&"line two".to_string()));
    }

    #[test]
    fn fifo_ordering() {
        let buf = SharedBuffer::new(1024 * 1024);
        let n = 100usize;
        for i in 0..n {
            buf.push(format!("line {i}").into_bytes());
        }
        buf.mark_done();
        for i in 0..n {
            let popped = buf.pop().expect("expected a line");
            let s = String::from_utf8(popped).unwrap();
            assert_eq!(s, format!("line {i}"));
        }
        assert_eq!(buf.pop(), None);
    }

    #[test]
    fn concurrent_push_pop() {
        let buf = SharedBuffer::new(1024 * 1024);
        let writer_buf = buf.clone_handle();
        let n = 1000usize;

        let writer = thread::spawn(move || {
            for i in 0..n {
                writer_buf.push(format!("msg {i}").into_bytes());
            }
            writer_buf.mark_done();
        });

        let mut received = Vec::with_capacity(n);
        while let Some(line) = buf.pop() {
            received.push(String::from_utf8(line).unwrap());
        }

        writer.join().unwrap();

        assert_eq!(received.len(), n);
        for (i, msg) in received.iter().enumerate() {
            assert_eq!(msg, &format!("msg {i}"));
        }
    }
}
