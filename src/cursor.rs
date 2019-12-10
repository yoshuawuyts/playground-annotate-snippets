use std::io::Cursor;

/// A cursor that wraps
pub struct Seeker<T> {
    inner: Cursor<T>,
}

impl<T> Seeker<T> {
    /// Create a new instance.
    pub fn new(cursor: Cursor<T>) -> Self {
        Self { inner: cursor }
    }
}
