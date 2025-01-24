use std::env;

use crate::internal::{self, App, AppMode};

pub fn run(name: Vec<String>, axum: bool) {
    // 获取当前目录
    let dir = env::current_dir().unwrap().canonicalize().unwrap();

    let mut apps = Vec::<App>::new();
    for v in name {
        apps.push(App {
            name: v.clone(),
            mainfile: format!("src/app/{}/main.rs", v),
        });
    }

    if axum {
        internal::build_axum_app(&dir, apps, AppMode::Multi);
        return;
    }
    internal::build_salvo_app(&dir, apps, AppMode::Multi);
}
