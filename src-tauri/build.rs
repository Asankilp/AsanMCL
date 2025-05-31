use std::env;

fn main() {
    // 读取 MICROSOFT_CLIENT_ID 环境变量
    if let Ok(client_id) = env::var("MICROSOFT_CLIENT_ID") {
        println!("cargo:rustc-env=MICROSOFT_CLIENT_ID={}", client_id);
    } else {
        println!("cargo:warning=MICROSOFT_CLIENT_ID environment variable is not set, using default value.");
        // println!("cargo:info={:?}", env::vars())
        println!("cargo:rustc-env=MICROSOFT_CLIENT_ID=1145141919810"); // 默认值
    }

    tauri_build::build()
}
