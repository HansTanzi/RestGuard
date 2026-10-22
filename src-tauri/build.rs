fn main() {
    // tauri-build only reruns when tauri.conf.json changes, but icon changes must also be re-embedded into the exe
    println!("cargo:rerun-if-changed=icons");
    tauri_build::build()
}
