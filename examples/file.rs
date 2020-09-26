use selinux::{Context, Getcon, Setcon};
use std::fs::File;

fn main() {
    let f = File::open("data/dummy").expect("failed to open file");

    let mut con = f.getcon();
    println!("orig context: {:?}", con);

    let new_con = Context::new("user_u:user_r:user_home_t:s0").unwrap();

    f.setcon(new_con.as_ref()).unwrap();

    con = f.getcon();
    println!("new conext: {:?}", con);
}
