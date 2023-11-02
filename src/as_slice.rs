
pub trait AsStrSlice {
    fn as_slice(&self) -> &[&str];
}

impl AsStrSlice for &str {
    fn as_slice(&self) -> &[&str] {
        std::slice::from_ref(self)
    }
}

impl<const N: usize> AsStrSlice for [&str; N] {
    fn as_slice(&self) -> &[&str] {
        self
    }
}

impl AsStrSlice for &[&str] {
    fn as_slice(&self) -> &[&str] {
        self
    }
}