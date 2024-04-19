use std::{env, path::Path};

fn main() {
    if env::var("TARGET")
        .expect("target")
        .ends_with("windows-msvc")
    {
        let mut manifest = Path::new("res/reliquary-archiver.exe.manifest")
            .canonicalize()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        const PREFIX: &str = r#"\\?\"#;
        if manifest.starts_with(PREFIX) {
            manifest = manifest[PREFIX.len()..].to_string();
        }
        println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}", manifest);
        println!("cargo:rerun-if-changed=res/reliquary-archiver.exe.manifest");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
