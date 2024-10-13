use log::LevelFilter;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use log4rs::{
	append::console::ConsoleAppender,
	config::{Appender, Logger, Root},
	encode::pattern::PatternEncoder,
	Config,
};

/// 统一的考虑到错误处理的返回类型
pub type AtysResult<T> = anyhow::Result<T>;

/// 定义一个宏，用于快速生成错误
#[allow(unused_macros)]
#[macro_export]
macro_rules! atys_error {
	($($arg:tt)*) => {
        Err(anyhow::anyhow!($($arg)*))
    };
}

/// 用于在函数返回值上快速生成一个Ok，以助返回值推断
#[allow(unused_macros)]
#[macro_export]
macro_rules! atys_result {
	($return_type:ty) => {
		Ok::<$return_type, anyhow::Error>(Default::default())
	};
}

/// 初始化日志系统
pub fn init_log()
{
	let encoder = PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S.%3f)} {h([{l}])} {m} ... {f}:{L}\n");
	let stdout = ConsoleAppender::builder()
		.encoder(Box::new(encoder))
		.build();

	let log_level = if cfg!(debug_assertions) { LevelFilter::Debug } else { LevelFilter::Info };
	let config = Config::builder()
		.appender(Appender::builder().build("stdout", Box::new(stdout)))
		.logger(Logger::builder().build("redis_tools", log_level))
		.build(Root::builder().appender("stdout").build(LevelFilter::Warn))
		.unwrap();

	log4rs::init_config(config).unwrap();
}

#[cfg(test)]
mod tests
{
	use crate::utils::init_log;

	#[allow(unused_imports)]
	use super::*;
	#[allow(unused_imports)]
	use log::{debug, error, info, warn};
	use redis::Commands;

	#[test]
	fn test_redis_1() -> AtysResult<()>
	{
		init_log();

		// 连接到Redis服务器
		let client = redis::Client::open("redis://192.168.0.24/")?;
		let mut con = client.get_connection()?;

		// 设置一个键值对
		let _: () = con.set("my_key", 42)?;

		// 获取键的值
		let value: i32 = con.get("my_key")?;
		println!("The value of 'my_key' is: {}", value);

		// 获取所有键
		let keys: Vec<String> = con.keys("*")?;
		for key in keys
		{
			let key_type = con.key_type::<_, String>(&key)?;
			println!("{} : {:?}", key, key_type);
		}

		Ok(())
	}

	#[test]
	fn test_log_1()
	{
		init_log();
		debug!("This is debug");
		info!("This is info");
	}
}
