#![feature(core_intrinsics)]

use std::fmt::{Debug, Formatter, Result};
use std::intrinsics::type_name;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WrapDebug<T>(pub T);

impl<T> WrapDebug<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for WrapDebug<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for WrapDebug<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Debug for WrapDebug<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(unsafe { type_name::<T>() })
    }
}

#[cfg(test)]
mod tests {
    use WrapDebug;
    #[test]
    fn test() {
        let value: String = "asdf".into();
        assert_eq!(format!("{:?}", WrapDebug(value)), "std::string::String");
    }
}
