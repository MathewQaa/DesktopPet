fn main() {
    println!("cargo::rustc-check-cfg=cfg(mobile)");
    println!("cargo:rerun-if-changed=tauri.conf.json");
    println!("cargo:rerun-if-changed=../dist");
}
