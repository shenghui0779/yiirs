use chrono::Local;
use std::env;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::{self, fmt::time::FormatTime};

// 用来格式化日志的输出时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub async fn init() -> WorkerGuard {
    // 直接初始化，采用默认的Subscriber，默认只输出INFO、WARN、ERROR级别的日志
    // tracing_subscriber::fmt::init();

    // 开发环境，日志输出到控制台
    if let Ok(v) = env::var("ENV") {
        if v == "dev" {
            let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
            tracing_subscriber::fmt()
                .with_max_level(Level::DEBUG)
                .with_file(true)
                .with_line_number(true)
                .with_timer(LocalTimer)
                .with_writer(non_blocking)
                .json()
                .flatten_event(true)
                .init();

            return guard;
        }
    }

    let log_path = match env::var("LOG_PATH") {
        Ok(v) => v,
        Err(_) => "logs".to_string(),
    };

    // 使用tracing_appender，指定日志的输出目标位置
    // 参考: https://docs.rs/tracing-appender/0.2.0/tracing_appender/
    // 如果不是在main函数中，guard必须返回到main()函数中，否则不输出任何信息到日志文件
    let file_appender = tracing_appender::rolling::daily(log_path, "tracing.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // 初始化并设置日志格式(定制和筛选日志)
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_file(true)
        .with_line_number(true) // 写入标准输出
        .with_ansi(false) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
        .with_timer(LocalTimer)
        .with_writer(non_blocking)
        .json()
        .flatten_event(true)
        .init(); // 初始化并将SubScriber设置为全局SubScriber

    guard
}
