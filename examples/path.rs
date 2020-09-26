use selinux::{Context, Getcon, Setcon};
use std::path::Path;

fn main() {
    let p = Path::new("data/dummy");

    let mut con = p.getcon();
    println!("orig context: {:?}", con);

    let new_con = Context::new("user_u:user_r:user_home_t:s0").unwrap();

    p.setcon(new_con.as_ref()).unwrap();

    con = p.getcon();
    println!("new conext: {:?}", con);
}
