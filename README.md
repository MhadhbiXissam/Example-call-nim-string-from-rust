# Example-call-nim-string-from-rust

`fn main() {
    let output = Command::new("nim")
        .arg("c")
        .arg("--noMain")
        .arg("--noLinking")
        .arg("--nimcache:nimcache")
        .arg("src/fib.nim")
        .output()
        .expect("Failed to invoke nim compiler");
    if !output.status.success() {
        let msg = String::from_utf8_lossy(output.stderr.as_slice());
        let _ = writeln!(io::stderr(), "\nerror occurred: {}\n", msg);
        std::process::exit(1);
    }

    cc::Build::new()
        .include("**$HOME.choosenim/toolchains/nim-1.4.8/lib**")
        .warnings(false)
        .file("**nimcache/@mfib.nim.c**")
        .file("nimcache/stdlib_system.nim.c")
        .compile("fib_nim");
}

`
