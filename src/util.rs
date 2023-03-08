use core::fmt;
use std::fmt::Debug;

pub struct DebugToDisplay<T>(pub T)
where
    T: fmt::Display;

impl<T> Debug for DebugToDisplay<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
