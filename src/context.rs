use crate::ffi;
use std::ffi::{CStr, CString};

pub struct Context(ffi::context_t);

macro_rules! wrap_context_get {
    ($f1: ident, $f2:ident) => {
        pub fn $f1(&self) -> Option<&str> {
            unsafe {
                match ffi::$f2(self.0) {
                    p if !p.is_null() => CStr::from_ptr(p).to_str().ok(),
                    _ => None,
                }
            }
        }
    };
}

macro_rules! wrap_context_set {
    ($f1:ident, $f2:ident) => {
        pub fn $f1(&mut self, s: &str) -> Option<&mut Self> {
            let cs = CString::new(s).ok()?;

            match unsafe { ffi::$f2(self.0, cs.as_ptr() as *const i8) } {
                0 => Some(self),
                _ => None,
            }
        }
    };
}

impl Context {
    pub fn new(s: &str) -> Option<Self> {
        let cs = CString::new(s).ok()?;

        match unsafe { ffi::context_new(cs.as_ptr() as *const i8) } {
            p if !p.is_null() => Some(Self { 0: p }),
            _ => None,
        }
    }

    pub fn to_str(&self) -> Option<&str> {
        unsafe {
            match ffi::context_str(self.0) {
                p if !p.is_null() => CStr::from_ptr(p).to_str().ok(),
                _ => None,
            }
        }
    }

    wrap_context_get!(get_user, context_user_get);
    wrap_context_get!(get_role, context_role_get);
    wrap_context_get!(get_type, context_type_get);
    wrap_context_get!(get_range, context_range_get);

    wrap_context_set!(set_user, context_user_set);
    wrap_context_set!(set_role, context_role_set);
    wrap_context_set!(set_type, context_type_set);
    wrap_context_set!(set_range, context_range_set);
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::context_free(self.0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_upper_case_globals)]
    const s: &str = "unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023";

    #[test]
    fn test_new() {
        assert!(Context::new(s).is_some());
    }

    #[test]
    fn test_to_str() {
        let c = Context::new(s);
        assert!(c.is_some());

        let ctx = c.unwrap();
        assert_eq!(ctx.to_str(), Some(s));
    }

    #[test]
    fn test_get_user() {
        let c = Context::new(s);
        assert!(c.is_some());

        let ctx = c.unwrap();
        assert_eq!(ctx.get_user(), Some("unconfined_u"));
    }

    #[test]
    fn test_get_role() {
        let c = Context::new(s);
        assert!(c.is_some());

        let ctx = c.unwrap();
        assert_eq!(ctx.get_role(), Some("unconfined_r"));
    }

    #[test]
    fn test_get_type() {
        let c = Context::new(s);
        assert!(c.is_some());

        let ctx = c.unwrap();
        assert_eq!(ctx.get_type(), Some("unconfined_t"));
    }

    #[test]
    fn test_get_range() {
        let c = Context::new(s);
        assert!(c.is_some());

        let ctx = c.unwrap();
        assert_eq!(ctx.get_range(), Some("s0-s0:c0.c1023"));
    }

    #[test]
    fn test_set_user() {
        let c = Context::new(s);
        assert!(c.is_some());

        let mut ctx = c.unwrap();
        assert!(ctx.set_user("user_u").is_some());
        assert_eq!(ctx.get_user(), Some("user_u"));
    }

    #[test]
    fn test_set_role() {
        let c = Context::new(s);
        assert!(c.is_some());

        let mut ctx = c.unwrap();
        assert!(ctx.set_role("user_r").is_some());
        assert_eq!(ctx.get_role(), Some("user_r"));
    }

    #[test]
    fn test_set_type() {
        let c = Context::new(s);
        assert!(c.is_some());

        let mut ctx = c.unwrap();
        assert!(ctx.set_type("user_t").is_some());
        assert_eq!(ctx.get_type(), Some("user_t"));
    }

    #[test]
    fn test_set_range() {
        let c = Context::new(s);
        assert!(c.is_some());

        let mut ctx = c.unwrap();
        assert!(ctx.set_range("s0").is_some());
        assert_eq!(ctx.get_range(), Some("s0"));
    }
}
