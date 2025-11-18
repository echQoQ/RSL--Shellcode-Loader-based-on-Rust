extern crate embed_resource;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // 当 ICON_PATH 环境变量变化时，强制重新运行构建脚本
    println!("cargo:rerun-if-env-changed=ICON_PATH");
    // 当使用 fallback 的 icon.rc 时，如文件变化也应重新构建
    println!("cargo:rerun-if-changed=icon.rc");

    // 可通过环境变量 ICON_PATH 指定 ico 路径；否则回退到仓库自带的 icon.rc
    if let Ok(icon_path) = env::var("ICON_PATH") {
        let out = PathBuf::from(env::var("OUT_DIR").unwrap());
        let rc = out.join("icon_cfg.rc");
        let content = format!("iconName ICON \"{}\"\n", icon_path.replace('\\', "/"));
        fs::write(&rc, content).expect("write icon_cfg.rc failed");
        embed_resource::compile(rc);
    } else {
        embed_resource::compile("icon.rc");
    }
}