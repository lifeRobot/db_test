#[tokio::main]
async fn main() {
    init().unwrap();

    // 测试sqlite编译和查询等
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    let result = conn.prepare("select * from sqlite_master");
    log::info!("result is {:?}",result);

    log::logger().flush();
}

/// 日志和数据库的初始化等
fn init() -> anyhow::Result<()> {
    let log_conf = fast_log::Config::default().console();
    fast_log::init(log_conf)?;

    Ok(())
}
