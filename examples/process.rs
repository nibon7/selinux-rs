use selinux::Context;

fn main() {
    let con = Context::current();
    println!("getcon: {:?}", con);

    let con = Context::previous();
    println!("getprevcon: {:?}", con);

    let con = Context::execute();
    println!("getexeccon: {:?}", con);

    let con = Context::fs_create();
    println!("getfscreatecon: {:?}", con);

    let con = Context::key_create();
    println!("getkeycreatecon: {:?}", con);

    let con = Context::socket_create();
    println!("getsockcreatecon: {:?}", con);

    let context = Context::new("user_u:user_r:user_t:s0").unwrap();

    context.set_current().unwrap();
    context.set_exec().unwrap();
    context.set_fs_create().unwrap();
    context.set_key_create().unwrap();
    context.set_socket_create().unwrap();
}
