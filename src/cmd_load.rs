#[allow(unused_imports)]
use log::{debug, error, info, warn};
use redis::{Client, Commands};
use std::{fs::File, io::Read};

use crate::{
	data::{BinData, JsonData, ValueBinType, ValueJsonType},
	utils::AtysResult,
};

pub fn load_from_json(redis_url: &str, file_path: &str) -> AtysResult<()>
{
	// 读取JSON文件
	let mut file = File::open(file_path)?;
	let mut json_data = String::new();
	file.read_to_string(&mut json_data)?;

	// 反序列化JSON回Rust结构体
	let deserialized_data: Vec<JsonData> = serde_json::from_str(&json_data)?;
	let key_size = deserialized_data.len();

	// 连接到Redis服务器
	let client = Client::open(redis_url)?;
	let mut con = client.get_connection()?;

	// 将反序列化的数据重新保存到Redis中
	for item in deserialized_data
	{
		match con.del::<_, ()>(&item.key)
		{
			Ok(_) =>
			{}
			Err(e) => error!("Failed to del: {}, error: {}", &item.key, e),
		}
		match item.value
		{
			ValueJsonType::Unknown =>
			{
				warn!("Skip unknown key type: {}", &item.key);
				continue;
			}
			ValueJsonType::String(val) => match con.set::<_, _, ()>(&item.key, &val)
			{
				Ok(_) =>
				{}
				Err(e) => error!("Failed to set value: {}, error: {}", &item.key, e),
			},

			ValueJsonType::List(vals) =>
			{
				for val in vals
				{
					match con.rpush::<_, _, ()>(&item.key, &val)
					{
						Ok(_) =>
						{}
						Err(e) => error!("Failed push value to list: {}, error: {}", &item.key, e),
					}
				}
			}
			ValueJsonType::Set(vals) =>
			{
				for val in vals
				{
					match con.sadd::<_, _, ()>(&item.key, &val)
					{
						Ok(_) =>
						{}
						Err(e) => error!("Failed add value to set: {}, error: {}", &item.key, e),
					}
				}
			}
			ValueJsonType::Hash(vals) =>
			{
				for (name, val) in vals
				{
					match con.hset::<_, _, _, ()>(&item.key, &name, &val)
					{
						Ok(_) =>
						{}
						Err(e) => error!("Failed set value to hash: {}, error: {}", &item.key, e),
					}
				}
			}
			ValueJsonType::ZSet(vals) =>
			{
				for (name, score) in vals
				{
					match con.zadd::<_, _, _, ()>(&item.key, &name, score)
					{
						Ok(_) =>
						{}
						Err(e) => error!("Failed add value to zset: {}, error: {}", &item.key, e),
					}
				}
			}
			ValueJsonType::Stream(vals) =>
			{
				for (key, vals) in vals
				{
					match con.xadd_map::<_, _, _, ()>(&item.key, &key, &vals)
					{
						Ok(_) =>
						{}
						Err(e) => error!("Failed add value to stream: {}, error: {}", &item.key, e),
					}
				}
			}
		}

		// 正整数：表示剩余有效期（秒）
		// -1：表示键存在但没有 TTL（永久）
		// -2：表示键不存在（已过期或未设置）
		if let Some(ttl) = item.ttl
		{
			if ttl > 0
			{
				match con.expire::<_, ()>(&item.key, ttl)
				{
					Ok(_) =>
					{}
					Err(e) => error!("Failed to expire: {}, error: {}", &item.key, e),
				}
			}
			else if ttl < -1
			{
				// 如果是-2，说明已经过期，这里设置成0快点删除
				match con.expire::<_, ()>(&item.key, 0)
				{
					Ok(_) =>
					{}
					Err(e) => error!("Failed to expire: {}, error: {}", &item.key, e),
				}
			}
		}
	}

	println!("Loaded {} keys from {}", key_size, file_path);
	Ok(())
}

pub fn load_from_bin(redis_url: &str, file_path: &str) -> AtysResult<()>
{
	// 读取数据文件
	let mut file = File::open(file_path)?;
	let mut bin_data = Vec::new();
	file.read_to_end(&mut bin_data)?;
	debug!("Loaded {} bytes from file", bin_data.len());

	// 反序列化
	let deserialized_data: Vec<BinData> = serde_cbor::from_slice(&bin_data)?;
	let key_size = deserialized_data.len();

	// 连接到Redis服务器
	let client = Client::open(redis_url)?;
	let mut con = client.get_connection()?;

	// 将反序列化的数据重新保存到Redis中
	for item in deserialized_data
	{
		// 将键转换为字符串
		let key_str = match String::from_utf8(item.key.clone())
		{
			Ok(s) => s,
			Err(_) => item
				.key
				.iter()
				.map(|byte| format!("{:02x}", byte))
				.collect(),
		};

		match con.del::<_, ()>(&item.key)
		{
			Ok(_) =>
			{}
			Err(e) => error!("Failed to del: {}, error: {}", &key_str, e),
		}
		match item.value
		{
			ValueBinType::Unknown =>
			{
				warn!("Skip unknown key type: {}", &key_str);
				continue;
			}
			ValueBinType::String(val) => match con.set::<_, _, ()>(&item.key, &val)
			{
				Ok(_) =>
				{}
				Err(e) => error!("Failed to set value: {}, error: {}", &key_str, e),
			},

			ValueBinType::List(vals) =>
			{
				for val in vals
				{
					match con.rpush::<_, _, ()>(&item.key, &val)
					{
						Ok(_) =>
						{}
						Err(e) => error!("Failed push value to list: {}, error: {}", &key_str, e),
					}
				}
			}
			ValueBinType::Set(vals) =>
			{
				for val in vals
				{
					match con.sadd::<_, _, ()>(&item.key, &val)
					{
						Ok(_) =>
						{}
						Err(e) => error!("Failed add value to set: {}, error: {}", &key_str, e),
					}
				}
			}
			ValueBinType::Hash(vals) =>
			{
				for (name, val) in vals
				{
					match con.hset::<_, _, _, ()>(&item.key, &name, &val)
					{
						Ok(_) =>
						{}
						Err(e) => error!("Failed set value to hash: {}, error: {}", &key_str, e),
					}
				}
			}
			ValueBinType::ZSet(vals) =>
			{
				for (name, score) in vals
				{
					match con.zadd::<_, _, _, ()>(&item.key, &name, score)
					{
						Ok(_) =>
						{}
						Err(e) => error!("Failed add value to zset: {}, error: {}", &key_str, e),
					}
				}
			}
			ValueBinType::Stream(vals) =>
			{
				for (key, vals) in vals
				{
					match con.xadd_map::<_, _, _, ()>(&item.key, &key, &vals)
					{
						Ok(_) =>
						{}
						Err(e) => error!("Failed add value to stream: {}, error: {}", &key_str, e),
					}
				}
			}
		}

		// 正整数：表示剩余有效期（秒）
		// -1：表示键存在但没有 TTL（永久）
		// -2：表示键不存在（已过期或未设置）
		if let Some(ttl) = item.ttl
		{
			if ttl > 0
			{
				match con.expire::<_, ()>(&item.key, ttl)
				{
					Ok(_) =>
					{}
					Err(e) => error!("Failed to expire: {}, error: {}", &key_str, e),
				}
			}
			else if ttl < -1
			{
				// 如果是-2，说明已经过期，这里设置成0快点删除
				match con.expire::<_, ()>(&item.key, 0)
				{
					Ok(_) =>
					{}
					Err(e) => error!("Failed to expire: {}, error: {}", &key_str, e),
				}
			}
		}
	}

	println!("Loaded {} keys from {}", key_size, file_path);
	Ok(())
}

#[cfg(test)]
mod tests
{
	use crate::utils::init_log;

	#[allow(unused_imports)]
	use super::*;
	#[allow(unused_imports)]
	use log::{debug, error, info, warn};

	#[test]
	fn test_redis_restore_1() -> AtysResult<()>
	{
		init_log();
		load_from_json("redis://localhost:6379/15", "F:\\temp\\redis_data.json")?;
		Ok(())
	}
}
