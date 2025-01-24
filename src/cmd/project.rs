use std::{env, fs};

use crate::internal::{self, is_empty_dir};

pub fn run(name: String, axum: bool, apps: Vec<String>) {
    // 获取当前目录
    let dir = env::current_dir().unwrap().canonicalize().unwrap();
    // 项目跟目录
    let root = dir.join(&name);

    // 判断目录是否为空
    if !is_empty_dir(&root) {
        println!("👿 目录({:?})不为空，请确认！", root);
        return;
    }
    // 创建项目目录
    fs::create_dir_all(root.clone()).unwrap();

    // 创建项目
    if axum {
        internal::build_axum_project(&root, name, apps);
    } else {
        internal::build_salvo_project(&root, name, apps);
    }

    println!("🍺 项目创建完成！请阅读README")
}
