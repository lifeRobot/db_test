#[tokio::main]
async fn main() {
    init().unwrap();

    let flags =
        rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE |
            rusqlite::OpenFlags::SQLITE_OPEN_CREATE |
            rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX |
            rusqlite::OpenFlags::SQLITE_OPEN_URI;
    log::info!("flags is {:?}",flags);
    // let path = rusqlite::path_to_cstring(":memory:".as_ref());
    // log::info!("path is {:?}",path);
    let i = unsafe { libsqlite3_sys::sqlite3_threadsafe() };
    log::info!("i is {}",i);

    // version ok [cargo build --target=armv7-unknown-linux-gnueabihf] version is 3039002
    let version = rusqlite::version_number();
    log::info!("version is {}",version);

    // TODO this code( libsqlite3_sys::sqlite3_mutex_alloc(0) ) will be build fail
    //  you can comment this and run [cargo build --target=armv7-unknown-linux-gnueabihf]
    // let mutex_ptr = unsafe { libsqlite3_sys::sqlite3_mutex_alloc(0) };
    // log::info!("mutes ptr is {:?}",mutex_ptr);
    // const SQLITE_SINGLETHREADED_MUTEX_MAGIC: usize = 8;
    // let is_singlethreaded = mutex_ptr as usize == SQLITE_SINGLETHREADED_MUTEX_MAGIC;
    // log::info!("is_singlethreaded is {}",is_singlethreaded);


    let conn = rusqlite::Connection::open_in_memory();
    log::info!("conn is {:?}",conn);

    // this is test code nodes
    // let a = rusqlite::inner_connection::ensure_safe_sqlite_threading_mode();
    // log::info!("a is {:?}",a);
    // 测试sqlite编译和查询等
    // let conn = rusqlite::InnerConnection::open_with_flags(&path.unwrap(), flags, None);
    // if conn.is_ok() {
    //     log::info!("conn ok");
    // } else {
    //     log::info!("conn error");
    // }
    // let conn = rusqlite::Connection::open_with_flags(":memory:", flags);
    // log::info!("conn is {:?}",conn);
    // log::info!("type id is {:?}",conn.type_id());
    // let result = conn.prepare("select * from sqlite_master");
    // log::info!("result is {:?}",result);

    log::logger().flush();
}

/// 日志和数据库的初始化等
fn init() -> anyhow::Result<()> {
    let log_conf = fast_log::Config::default().console();
    fast_log::init(log_conf)?;

    Ok(())
}
