fn main() {
    // tauri-build 只在 tauri.conf.json 变化时重跑，图标改了也要重新嵌入 exe
    println!("cargo:rerun-if-changed=icons");
    tauri_build::build()
}
