use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Once;

static PERFORMANCE_LOG: Mutex<Option<HashMap<String, (Duration, usize)>>> = Mutex::new(None);
static INIT_LOG: Once = Once::new();
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

/// A guard that measures the duration of a scope and logs performance metrics.
pub struct PerformanceGuard {
    start: Instant,
    operation: String,
}

impl PerformanceGuard {
    /// Creates a new performance guard for the given operation.
    pub fn new(operation: &str) -> Self {
        INIT_LOG.call_once(|| {
            *PERFORMANCE_LOG.lock().unwrap() = Some(HashMap::new());
        });

        CALL_COUNT.fetch_add(1, Ordering::SeqCst);

        Self {
            start: Instant::now(),
            operation: operation.to_string(),
        }
    }

    /// Logs the current performance metrics to a file.
    fn log_metrics(&self, duration: Duration) {
        if let Some(ref mut metrics) = *PERFORMANCE_LOG.lock().unwrap() {
            let entry = metrics
                .entry(self.operation.clone())
                .or_insert((Duration::new(0, 0), 0));
            entry.0 += duration;
            entry.1 += 1;

            // Log to file every 100 calls
            if CALL_COUNT.load(Ordering::SeqCst) % 100 == 0 {
                let log_path = Path::new("performance.log");
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(log_path)
                {
                    for (op, (total_duration, count)) in metrics.iter() {
                        let avg_duration = total_duration.as_micros() as f64 / *count as f64;
                        writeln!(
                            file,
                            "{}: {} calls, avg {} µs",
                            op, count, avg_duration
                        ).ok();
                    }
                    writeln!(file, "---").ok();
                }
            }
        }
    }
}

impl Drop for PerformanceGuard {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        self.log_metrics(duration);
    }
} 