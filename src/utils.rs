use selinux_sys;

pub fn enabled() -> Option<bool> {
    match unsafe { selinux_sys::is_selinux_enabled() } {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

pub fn mls_enabled() -> Option<bool> {
    match unsafe { selinux_sys::is_selinux_mls_enabled() } {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enabled() {
        assert!(enabled().is_some());
    }

    #[test]
    fn test_mls_enabled() {
        assert!(mls_enabled().is_some());
    }
}
