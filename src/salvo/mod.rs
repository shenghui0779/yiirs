use std::vec;

use tera::Tera;

pub fn new() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // main
        (
            "src/main.rs",
            include_str!("../../template/salvo/main.tera"),
        ),
        // app
        (
            "src/app/mod.rs",
            include_str!("../../template/salvo/app/mod.tera"),
        ),
        // app/api
        (
            "src/app/api/mod.rs",
            include_str!("../../template/salvo/app/api/mod.tera"),
        ),
        (
            "src/app/api/greeter.rs",
            include_str!("../../template/salvo/app/api/greeter.tera"),
        ),
        // app/cmd
        (
            "src/app/cmd/mod.rs",
            include_str!("../../template/salvo/app/cmd/mod.tera"),
        ),
        (
            "src/app/cmd/hello.rs",
            include_str!("../../template/salvo/app/cmd/hello.tera"),
        ),
        // app/router
        (
            "src/app/router/mod.rs",
            include_str!("../../template/salvo/app/router/mod.tera"),
        ),
        (
            "src/app/router/route.rs",
            include_str!("../../template/salvo/app/router/route.tera"),
        ),
        // app/service
        (
            "src/app/service/mod.rs",
            include_str!("../../template/salvo/app/service/mod.tera"),
        ),
        (
            "src/app/service/greeter.rs",
            include_str!("../../template/salvo/app/service/greeter.tera"),
        ),
        // internal
        (
            "src/internal/mod.rs",
            include_str!("../../template/salvo/internal/mod.tera"),
        ),
        // internal/core
        (
            "src/internal/core/mod.rs",
            include_str!("../../template/salvo/internal/core/mod.tera"),
        ),
        (
            "src/internal/core/cache.rs",
            include_str!("../../template/salvo/internal/core/cache.tera"),
        ),
        (
            "src/internal/core/config.rs",
            include_str!("../../template/salvo/internal/core/config.tera"),
        ),
        (
            "src/internal/core/db.rs",
            include_str!("../../template/salvo/internal/core/db.tera"),
        ),
        (
            "src/internal/core/logger.rs",
            include_str!("../../template/salvo/internal/core/logger.tera"),
        ),
        (
            "src/internal/core/manager.rs",
            include_str!("../../template/salvo/internal/core/manager.tera"),
        ),
        // internal/crypto
        (
            "src/internal/crypto/mod.rs",
            include_str!("../../template/salvo/internal/crypto/mod.tera"),
        ),
        (
            "src/internal/crypto/aes.rs",
            include_str!("../../template/salvo/internal/crypto/aes.tera"),
        ),
        (
            "src/internal/crypto/hash.rs",
            include_str!("../../template/salvo/internal/crypto/hash.tera"),
        ),
        // internal/middleware
        (
            "src/internal/middleware/mod.rs",
            include_str!("../../template/salvo/internal/middleware/mod.tera"),
        ),
        (
            "src/internal/middleware/catch_panic.rs",
            include_str!("../../template/salvo/internal/middleware/catch_panic.tera"),
        ),
        (
            "src/internal/middleware/log.rs",
            include_str!("../../template/salvo/internal/middleware/log.tera"),
        ),
        (
            "src/internal/middleware/trace.rs",
            include_str!("../../template/salvo/internal/middleware/trace.tera"),
        ),
        // internal/result
        (
            "src/internal/result/mod.rs",
            include_str!("../../template/salvo/internal/result/mod.tera"),
        ),
        (
            "src/internal/result/code.rs",
            include_str!("../../template/salvo/internal/result/code.tera"),
        ),
        (
            "src/internal/result/reply.rs",
            include_str!("../../template/salvo/internal/result/reply.tera"),
        ),
        // internal/util
        (
            "src/internal/util/mod.rs",
            include_str!("../../template/salvo/internal/util/mod.tera"),
        ),
        (
            "src/internal/util/helper.rs",
            include_str!("../../template/salvo/internal/util/helper.tera"),
        ),
        (
            "src/internal/util/identity.rs",
            include_str!("../../template/salvo/internal/util/identity.tera"),
        ),
        (
            "src/internal/util/mutex.rs",
            include_str!("../../template/salvo/internal/util/mutex.tera"),
        ),
        (
            "src/internal/util/xtime.rs",
            include_str!("../../template/salvo/internal/util/xtime.tera"),
        ),
        // root
        (".dockerignore", include_str!("../../template/dockerignore")),
        (".gitignore", include_str!("../../template/gitignore")),
        (
            "Cargo.toml",
            include_str!("../../template/salvo/Cargo.toml"),
        ),
        ("config.toml", include_str!("../../template/config.toml")),
        ("Dockerfile", include_str!("../../template/Dockerfile")),
        ("dockerun.sh", include_str!("../../template/dockerun.sh")),
        ("README.md", include_str!("../../template/salvo/README.md")),
    ])
    .unwrap();
    tera
}
