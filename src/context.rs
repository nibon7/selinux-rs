use crate::ffi;
use crate::selinux_macros::{wrap_context_get, wrap_context_set};
use std::ffi::{CStr, CString};
use std::fmt;

pub struct Context(ffi::context_t);

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

    wrap_context_get!(user);
    wrap_context_get!(role);
    wrap_context_get!(type);
    wrap_context_get!(range);

    wrap_context_set!(user);
    wrap_context_set!(role);
    wrap_context_set!(type);
    wrap_context_set!(range);
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::context_free(self.0) }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_str() {
            Some(s) => write!(f, "{}", s),
            None => Err(fmt::Error),
        }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_str() {
            Some(s) => write!(f, "{}", s),
            None => Err(fmt::Error),
        }
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
        assert_eq!(
            ctx.set_user("user_u").and_then(|c| c.get_user()),
            Some("user_u")
        );
    }

    #[test]
    fn test_set_role() {
        let c = Context::new(s);
        assert!(c.is_some());

        let mut ctx = c.unwrap();
        assert_eq!(
            ctx.set_role("user_r").and_then(|c| c.get_role()),
            Some("user_r")
        );
    }

    #[test]
    fn test_set_type() {
        let c = Context::new(s);
        assert!(c.is_some());

        let mut ctx = c.unwrap();
        assert_eq!(
            ctx.set_type("user_t").and_then(|c| c.get_type()),
            Some("user_t")
        );
    }

    #[test]
    fn test_set_range() {
        let c = Context::new(s);
        assert!(c.is_some());

        let mut ctx = c.unwrap();
        assert_eq!(ctx.set_range("s0").and_then(|c| c.get_range()), Some("s0"));
    }
}
