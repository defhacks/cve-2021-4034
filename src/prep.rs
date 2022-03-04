use std::fs;
use std::io::Write;
use std::path::Path;

/// This could've been a shell script, but we're optimizing for learning
fn main() -> std::io::Result<()> {
    // Note: used this env invocation to look for nicer ways to see if we could grab
    // libpwnkit.so from our corresponsing PROFILE, but only found /target/debug listed
    // in LD_LIBRARY_PATH

    // if let Ok(output) = std::process::Command::new("env").output() {
    //     println!("{}", String::from_utf8_lossy(&output.stdout));
    // }

    if !Path::new("playground/pwnkit").is_dir() {
        fs::create_dir("playground/pwnkit")?;
    }
    if !Path::new("playground/pwnkit/pwnkit.so").exists() {
        fs::copy("target/debug/libpwnkit.so", "playground/pwnkit/pwnkit.so")?;
    }
    if !Path::new("playground/pwnkit/gconv-modules").exists() {
        let mut file =
            fs::File::create("playground/pwnkit/gconv-modules").expect("could not create file");
        file.write_all(b"module UTF-8// PWNKIT// pwnkit 2\n")
            .expect("failed writing to file");
    }
    if !Path::new("playground/GCONV_PATH=.").is_dir() {
        fs::create_dir("playground/GCONV_PATH=.")?;
    }
    if !Path::new("playground/GCONV_PATH=./pwnkit").exists() {
        fs::copy("/bin/true", "playground/GCONV_PATH=./pwnkit")?;
    }

    Ok(())
}
