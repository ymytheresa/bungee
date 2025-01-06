use std::cell::Cell;

// Maximum recursion depth to prevent stack overflow
const MAX_RECURSION_DEPTH: u32 = 100;

thread_local! {
    static RECURSION_DEPTH: Cell<u32> = Cell::new(0);
}

/// A guard that tracks recursion depth and prevents stack overflow.
pub struct RecursionGuard;

impl RecursionGuard {
    /// Creates a new recursion guard, incrementing the recursion depth.
    /// Returns None if the maximum recursion depth is exceeded.
    pub fn new() -> Option<Self> {
        let mut exceeded = false;
        
        RECURSION_DEPTH.with(|depth| {
            let current = depth.get();
            if current >= MAX_RECURSION_DEPTH {
                exceeded = true;
            } else {
                depth.set(current + 1);
            }
        });

        if exceeded {
            None
        } else {
            Some(RecursionGuard)
        }
    }

    /// Returns the current recursion depth.
    pub fn current_depth() -> u32 {
        RECURSION_DEPTH.with(|depth| depth.get())
    }
}

impl Drop for RecursionGuard {
    fn drop(&mut self) {
        RECURSION_DEPTH.with(|depth| {
            let current = depth.get();
            if current > 0 {
                depth.set(current - 1);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursion_guard() {
        assert!(RecursionGuard::new().is_some());
        {
            let _guard = RecursionGuard::new().unwrap();
            assert!(RecursionGuard::new().is_some());
        }
        assert!(RecursionGuard::new().is_some());
    }

    #[test]
    fn test_max_recursion() {
        let mut guards = Vec::new();
        for _ in 0..MAX_RECURSION_DEPTH {
            guards.push(RecursionGuard::new().unwrap());
        }
        assert!(RecursionGuard::new().is_none());
        guards.clear();
        assert!(RecursionGuard::new().is_some());
    }
} 