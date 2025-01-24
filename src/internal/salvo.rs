use std::vec;

use tera::Tera;

pub fn global() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        (
            "Cargo.toml",
            include_str!("../../template/salvo/Cargo.tera"),
        ),
        (
            ".dockerignore",
            include_str!("../../template/dockerignore.tera"),
        ),
        (".gitignore", include_str!("../../template/gitignore.tera")),
        (
            "README.md",
            include_str!("../../template/salvo/README.tera"),
        ),
    ])
    .unwrap();
    tera
}

pub fn docker() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![(
        "Dockerfile",
        include_str!("../../template/Dockerfile.tera"),
    )])
    .unwrap();
    tera
}

pub fn other() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        ("dockerun.sh", include_str!("../../template/dockerun.tera")),
        ("config.toml", include_str!("../../template/config.tera")),
    ])
    .unwrap();
    tera
}

pub fn internal() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // lib.rs
        (
            "lib.rs",
            include_str!("../../template/salvo/internal/lib.tera"),
        ),
        // core
        (
            "core/mod.rs",
            include_str!("../../template/salvo/internal/core/mod.tera"),
        ),
        (
            "core/cache.rs",
            include_str!("../../template/salvo/internal/core/cache.tera"),
        ),
        (
            "core/config.rs",
            include_str!("../../template/salvo/internal/core/config.tera"),
        ),
        (
            "core/db.rs",
            include_str!("../../template/salvo/internal/core/db.tera"),
        ),
        (
            "core/logger.rs",
            include_str!("../../template/salvo/internal/core/logger.tera"),
        ),
        (
            "core/manager.rs",
            include_str!("../../template/salvo/internal/core/manager.tera"),
        ),
        // crypto
        (
            "crypto/mod.rs",
            include_str!("../../template/salvo/internal/crypto/mod.tera"),
        ),
        (
            "crypto/aes.rs",
            include_str!("../../template/salvo/internal/crypto/aes.tera"),
        ),
        (
            "crypto/hash.rs",
            include_str!("../../template/salvo/internal/crypto/hash.tera"),
        ),
        // middleware
        (
            "middleware/mod.rs",
            include_str!("../../template/salvo/internal/middleware/mod.tera"),
        ),
        (
            "middleware/catch_panic.rs",
            include_str!("../../template/salvo/internal/middleware/catch_panic.tera"),
        ),
        (
            "middleware/log.rs",
            include_str!("../../template/salvo/internal/middleware/log.tera"),
        ),
        (
            "middleware/trace.rs",
            include_str!("../../template/salvo/internal/middleware/trace.tera"),
        ),
        // result
        (
            "result/mod.rs",
            include_str!("../../template/salvo/internal/result/mod.tera"),
        ),
        (
            "result/code.rs",
            include_str!("../../template/salvo/internal/result/code.tera"),
        ),
        (
            "result/reply.rs",
            include_str!("../../template/salvo/internal/result/reply.tera"),
        ),
        // util
        (
            "util/mod.rs",
            include_str!("../../template/salvo/internal/util/mod.tera"),
        ),
        (
            "util/helper.rs",
            include_str!("../../template/salvo/internal/util/helper.tera"),
        ),
        (
            "util/identity.rs",
            include_str!("../../template/salvo/internal/util/identity.tera"),
        ),
        (
            "util/mutex.rs",
            include_str!("../../template/salvo/internal/util/mutex.tera"),
        ),
        (
            "util/xtime.rs",
            include_str!("../../template/salvo/internal/util/xtime.tera"),
        ),
    ])
    .unwrap();
    tera
}

pub fn app() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // main.rs
        (
            "main.rs",
            include_str!("../../template/salvo/app/main.tera"),
        ),
        // api
        (
            "api/mod.rs",
            include_str!("../../template/salvo/app/api/mod.tera"),
        ),
        (
            "api/greeter.rs",
            include_str!("../../template/salvo/app/api/greeter.tera"),
        ),
        // cmd
        (
            "cmd/mod.rs",
            include_str!("../../template/salvo/app/cmd/mod.tera"),
        ),
        (
            "cmd/hello.rs",
            include_str!("../../template/salvo/app/cmd/hello.tera"),
        ),
        (
            "cmd/serve.rs",
            include_str!("../../template/salvo/app/cmd/serve.tera"),
        ),
        // router
        (
            "router/mod.rs",
            include_str!("../../template/salvo/app/router/mod.tera"),
        ),
        (
            "router/route.rs",
            include_str!("../../template/salvo/app/router/route.tera"),
        ),
        // service
        (
            "service/mod.rs",
            include_str!("../../template/salvo/app/service/mod.tera"),
        ),
        (
            "service/greeter.rs",
            include_str!("../../template/salvo/app/service/greeter.tera"),
        ),
    ])
    .unwrap();
    tera
}
