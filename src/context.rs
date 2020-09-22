use std::fmt::{Debug, Display, Formatter};
pub struct Context {
    user: String,
    role: String,
    _type: String,
    range: String,
}

macro_rules! context_access {
    ($field:ident, $setter:ident) => {
        pub fn $field(&self) -> &str {
            self.$field.as_str()
        }

        pub fn $setter(&mut self, $field: &str) {
            self.$field = $field.to_owned();
        }
    };
}

impl Context {
    pub fn new(context: &str) -> Option<Self> {
        let mut iter = context.split(":");
        let user = iter.next()?.to_owned();
        let role = iter.next()?.to_owned();
        let _type = iter.next()?.to_owned();
        let range = iter.collect::<Vec<&str>>().join(":");
        Some(Context {
            user,
            role,
            _type,
            range,
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}:{}:{}", self.user, self.role, self._type, self.range)
    }

    context_access!(user, set_user);
    context_access!(role, set_role);
    context_access!(_type, set_type);
    context_access!(range, set_range);
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Debug for Context {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTEXT: &str = "user_u:role_r:type_t:s0-s0:c0.c1023";
    const CONTEXT_WITHOUT_RANGE: &str = "user_u:role_r:type_t";

    #[test]
    fn context_basic() {
        let mut context = Context::new(CONTEXT).unwrap();
        assert_eq!(context.user(), "user_u");
        assert_eq!(context._type(), "type_t");
        assert_eq!(context.role(), "role_r");
        assert_eq!(context.range(), "s0-s0:c0.c1023");

        context.set_type("unconfined_t");
        assert_eq!(context._type(), "unconfined_t");

        context.set_range("s0");
        assert_eq!(context.range(), "s0");

        assert_eq!(context.to_string(), "user_u:role_r:unconfined_t:s0");

        let context = Context::new(CONTEXT_WITHOUT_RANGE).unwrap();
        assert_eq!(context.range(), "");
        assert_eq!(context._type(), "type_t");
    }

    #[test]
    fn test_formatter() {
        let context = Context::new(CONTEXT).unwrap();
        assert_eq!(format!("{}", context), CONTEXT);
        assert_eq!(format!("{:?}", context), CONTEXT);
    }
}
