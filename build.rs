use std::process::Command;

const LINUX_LIB_PATH_DEBUG: &'static str = "./target/debug/libgodot_quic_connections.so";
const LINUX_ADDON_PATH_DEBUG: &'static str =
    "./addons/QuicServer/libgodot_quic_connections_debug.so";
const LINUX_LIB_PATH_RELEASE: &'static str = "./target/release/libgodot_quic_connections.so";
const LINUX_ADDON_PATH_RELEASE: &'static str = "../addons/QuicServer/libgodot_quic_connections.so";

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    #[cfg(target_os = "linux")]
    {
        Command::new("mv")
            .args([LINUX_LIB_PATH_DEBUG, LINUX_ADDON_PATH_DEBUG])
            .status()
            .unwrap();

        Command::new("mv")
            .args([LINUX_LIB_PATH_RELEASE, LINUX_ADDON_PATH_RELEASE])
            .status()
            .unwrap();
    }
}
