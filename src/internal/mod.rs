pub mod axum;
pub mod salvo;

use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use tera::Context;

pub enum AppMode {
    Single,
    Multi,
}

#[derive(serde::Serialize)]
pub struct App {
    pub name: String,
    pub mainfile: String,
}

pub fn is_empty_dir(path: &Path) -> bool {
    match path.read_dir() {
        Ok(entries) => entries.count() == 0,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => true,
            _ => panic!("{}", e),
        },
    }
}

pub fn build_axum_project(root: &Path, name: String, apps: Vec<String>) {
    let template = (axum::global(), axum::internal());
    let (mode, bins) = build_project(root, name, apps, template);
    build_app(
        root,
        bins,
        mode,
        (axum::app(), axum::docker(), axum::other()),
    );
}

pub fn build_axum_app(root: &Path, apps: Vec<App>, mode: AppMode) {
    build_app(
        root,
        apps,
        mode,
        (axum::app(), axum::docker(), axum::other()),
    );
}

pub fn build_salvo_project(root: &Path, name: String, apps: Vec<String>) {
    let template = (salvo::global(), salvo::internal());
    let (mode, bins) = build_project(root, name, apps, template);
    build_app(
        root,
        bins,
        mode,
        (salvo::app(), salvo::docker(), salvo::other()),
    );
}

pub fn build_salvo_app(root: &Path, apps: Vec<App>, mode: AppMode) {
    build_app(
        root,
        apps,
        mode,
        (salvo::app(), salvo::docker(), salvo::other()),
    );
}

fn build_project(
    root: &Path,
    name: String,
    apps: Vec<String>,
    template: (tera::Tera, tera::Tera),
) -> (AppMode, Vec<App>) {
    let src_dir = root.join("src");

    let mut bins = Vec::<App>::new();

    let mode = if apps.is_empty() {
        bins.push(App {
            name: name.clone(),
            mainfile: String::from("src/app/main.rs"),
        });
        AppMode::Single
    } else {
        for appname in apps {
            bins.push(App {
                name: appname.clone(),
                mainfile: format!("src/app/{}/main.rs", appname),
            });
        }
        AppMode::Multi
    };

    let (tera_global, tera_internal) = template;

    let mut ctx = Context::new();
    ctx.insert("name", &name);
    ctx.insert("apps", &bins);

    // 创建项目
    println!("🍺 create project: {}", name);

    // global
    for filename in tera_global.get_template_names() {
        let content = tera_global.render(filename, &ctx).unwrap();
        let path = root.join(filename);
        // 创建文件
        let mut file = File::create(path).unwrap();
        // 将内容写入文件
        file.write_all(content.as_bytes()).unwrap();
        println!("internal/{}", filename)
    }

    // internal
    let internal_dir = src_dir.join("internal");
    for filename in tera_internal.get_template_names() {
        let content = tera_internal.render(filename, &ctx).unwrap();
        let path = internal_dir.join(filename);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).unwrap();
        }
        // 创建文件
        let mut file = File::create(path).unwrap();
        // 将内容写入文件
        file.write_all(content.as_bytes()).unwrap();
        println!("internal/{}", filename)
    }

    (mode, bins)
}

fn build_app(
    root: &Path,
    apps: Vec<App>,
    mode: AppMode,
    template: (tera::Tera, tera::Tera, tera::Tera),
) {
    let (tera_app, tera_docker, tera_other) = template;

    let src_dir = root.join("src");

    // 创建app
    for app in &apps {
        let mut ctx = Context::new();
        ctx.insert("app_name", &app.name);

        // 模式
        let (app_dir, app_prefix) = match mode {
            AppMode::Single => {
                ctx.insert("dockerfile", "Dockerfile");
                ctx.insert("cfgfile", "config.toml");
                (src_dir.join("app"), String::from("app"))
            }
            AppMode::Multi => {
                ctx.insert("dockerfile", format!("{}.dockerfile", &app.name).as_str());
                ctx.insert("cfgfile", format!("{}_config.toml", &app.name).as_str());
                (
                    src_dir.join("app").join(&app.name),
                    format!("app/{}", &app.name),
                )
            }
        };
        if !is_empty_dir(&app_dir) {
            println!("👿 目录({:?})不为空，请确认！", app_dir);
            return;
        }

        println!("🍺 create app: {}", &app.name);

        // app
        for filename in tera_app.get_template_names() {
            let content = tera_app.render(filename, &ctx).unwrap();
            let path = app_dir.join(filename);
            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir).unwrap();
            }
            // 创建文件
            let mut file = File::create(path).unwrap();
            // 将内容写入文件
            file.write_all(content.as_bytes()).unwrap();
            println!("{}/{}", app_prefix, filename)
        }

        // dockerfile
        for filename in tera_docker.get_template_names() {
            let content = tera_docker.render(filename, &ctx).unwrap();
            let path = match mode {
                AppMode::Single => root.join(filename),
                AppMode::Multi => {
                    root.join(format!("{}.{}", &app.name, filename.to_lowercase()).as_str())
                }
            };
            // 创建文件
            let mut file = File::create(path).unwrap();
            // 将内容写入文件
            file.write_all(content.as_bytes()).unwrap();
            println!("{}", filename)
        }

        // config.toml
        for filename in tera_other.get_template_names() {
            let content = tera_other.render(filename, &ctx).unwrap();
            let path = match mode {
                AppMode::Single => root.join(filename),
                AppMode::Multi => root.join(format!("{}_{}", &app.name, filename).as_str()),
            };
            // 创建文件
            let mut file = File::create(path).unwrap();
            // 将内容写入文件
            file.write_all(content.as_bytes()).unwrap();
            println!("{}", filename)
        }
    }
}
