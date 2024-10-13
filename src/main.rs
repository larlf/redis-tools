use clap::Parser;
use data::{Args, CommandType};
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use utils::{init_log, AtysResult};

pub mod cmd_load;
pub mod cmd_save;
pub mod data;
pub mod utils;

fn main()
{
	if cfg!(debug_assertions)
	{
		// 启用 RUST_BACKTRACE 环境变量
		std::env::set_var("RUST_BACKTRACE", "1");
	}

	init_log();

	// 解析输入参数
	let args = Args::parse();
	debug!("Args: {:?}", args);

	// 执行命令，并处理错误
	match execute_command(&args)
	{
		Ok(_) =>
		{}
		Err(e) => println!("Error: {:?}", e),
	};
}

/// 执行命令函数，根据传入的参数执行相应的操作
fn execute_command(args: &Args) -> AtysResult<()>
{
	// 确定输出文件的名称
	let filename = if args.file.contains("*")
	{
		match args.mode
		{
			data::ModeType::Json => args.file.replace("*", "json"),
			data::ModeType::Bin => args.file.replace("*", "bin"),
		}
	}
	else
	{
		args.file.clone()
	};

	// 检查Redis的URL
	let redis_url = if args.url.starts_with("redis://") { args.url.clone() } else { format!("redis://{}", args.url) };

	// 匹配命令参数，执行相应的操作
	match args.command
	{
		// 保存命令
		CommandType::Save =>
		{
			match args.mode
			{
				data::ModeType::Json => cmd_save::save_to_json(&redis_url, &filename)?,
				data::ModeType::Bin => cmd_save::save_to_bin(&redis_url, &filename)?,
			};
		}
		// 加载命令
		CommandType::Load =>
		{
			match args.mode
			{
				data::ModeType::Json => cmd_load::load_from_json(&redis_url, &filename)?,
				data::ModeType::Bin => cmd_load::load_from_bin(&redis_url, &filename)?,
			};
		}
	};

	// 操作成功
	Ok(())
}
