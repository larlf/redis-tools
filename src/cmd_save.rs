use crate::{
	data::{BinData, JsonData, ValueBinType, ValueJsonType},
	utils::AtysResult,
};
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use redis::{Client, Commands};
use std::{collections::HashMap, fs::File, io::Write};

/// 将Redis中的数据保存为JSON文件
pub fn save_to_json(url: &str, filename: &str) -> AtysResult<()>
{
	// 连接到Redis服务器
	let client = Client::open(url)?;
	let mut con = client.get_connection()?;

	// 取得所有的键值
	let keys: Vec<String> = con.keys("*")?;
	let key_count = keys.len();

	// 生成要保存的数据
	let mut data: Vec<JsonData> = Vec::new();
	for key in keys
	{
		// 取得键值的类型
		let key_type: String = con.key_type::<_, String>(&key)?;

		// 获取不同类型的值
		let value: ValueJsonType = match key_type.as_str()
		{
			"string" =>
			{
				let data = match con.get(&key)
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting string value: {}, error: {}", &key, err);
						continue;
					}
				};
				ValueJsonType::String(data)
			}
			"list" =>
			{
				let data = match con.lrange::<_, Vec<String>>(&key, 0, -1)
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting list value: {}, error: {}", &key, err);
						continue;
					}
				};
				ValueJsonType::List(data)
			}
			"set" =>
			{
				let data = match con.smembers(&key)
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting set value: {}, error: {}", &key, err);
						continue;
					}
				};
				ValueJsonType::Set(data)
			}
			"hash" =>
			{
				let data = match con.hgetall::<_, HashMap<String, String>>(&key)
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting hash value: {}, error: {}", &key, err);
						continue;
					}
				};
				ValueJsonType::Hash(data)
			}
			"zset" =>
			{
				let data: Vec<(String, f64)> = match con.zrange_withscores(&key, 0, -1)
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting zset value: {}, error: {}", &key, err);
						continue;
					}
				};
				let map: HashMap<String, f64> = data.into_iter().collect();
				ValueJsonType::ZSet(map)
			}
			"stream" =>
			{
				let data = match con.xrange::<_, _, _, Vec<(String, HashMap<String, String>)>>(&key, "-", "+")
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting stream value: {}, error: {}", &key, err);
						continue;
					}
				};
				ValueJsonType::Stream(data)
			}
			"none" =>
			{
				// 说明这个键已经不存在了
				continue;
			}
			_ =>
			{
				error!("Unknown key type: {}", key_type);
				ValueJsonType::Unknown
			}
		};

		// 获取键的过期时间
		let ttl: Option<i64> = match con.ttl(&key)
		{
			Ok(ttl) => ttl,
			Err(_) => None,
		};

		// 将数据添加到向量中
		data.push(JsonData { key, value, ttl });
	}

	// 序列化为JSON并写入文件
	let json_data = serde_json::to_string_pretty(&data)?;
	let mut file = File::create(filename)?;
	file.write_all(json_data.as_bytes())?;
	println!("Save {} keys to {}", key_count, filename);

	Ok(())
}

/// 将Redis中的数据保存为二进制文件
pub fn save_to_bin(url: &str, filename: &str) -> AtysResult<()>
{
	// 连接到Redis服务器
	let client = Client::open(url)?;
	let mut con = client.get_connection()?;

	// 取得所有的键值
	let keys: Vec<Vec<u8>> = con.keys("*")?;
	let key_count = keys.len();

	// 生成要保存的数据
	let mut data: Vec<BinData> = Vec::new();
	for key in keys
	{
		// 将键转换为字符串
		let key_str = match String::from_utf8(key.clone())
		{
			Ok(s) => s,
			Err(_) => key.iter().map(|byte| format!("{:02x}", byte)).collect(),
		};

		// 取得键值的类型
		let key_type: String = con.key_type::<_, String>(&key)?;

		// 获取不同类型的值
		let value: ValueBinType = match key_type.as_str()
		{
			"string" =>
			{
				let data = match con.get(&key)
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting string value: {}, error: {}", &key_str, err);
						continue;
					}
				};
				ValueBinType::String(data)
			}
			"list" =>
			{
				let data = match con.lrange::<_, Vec<Vec<u8>>>(&key, 0, -1)
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting list value: {}, error: {}", &key_str, err);
						continue;
					}
				};
				ValueBinType::List(data)
			}
			"set" =>
			{
				let data = match con.smembers(&key)
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting set value: {}, error: {}", &key_str, err);
						continue;
					}
				};
				ValueBinType::Set(data)
			}
			"hash" =>
			{
				let data = match con.hgetall::<_, HashMap<Vec<u8>, Vec<u8>>>(&key)
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting hash value: {}, error: {}", &key_str, err);
						continue;
					}
				};
				ValueBinType::Hash(data)
			}
			"zset" =>
			{
				let data: Vec<(Vec<u8>, f64)> = match con.zrange_withscores(&key, 0, -1)
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting zset value: {}, error: {}", &key_str, err);
						continue;
					}
				};
				let map: HashMap<Vec<u8>, f64> = data.into_iter().collect();
				ValueBinType::ZSet(map)
			}
			"stream" =>
			{
				let data = match con.xrange::<_, _, _, Vec<(Vec<u8>, HashMap<Vec<u8>, Vec<u8>>)>>(&key, "-", "+")
				{
					Ok(data) => data,
					Err(err) =>
					{
						error!("Error getting stream value: {}, error: {}", &key_str, err);
						continue;
					}
				};
				ValueBinType::Stream(data)
			}
			"none" =>
			{
				// 说明这个键已经不存在了
				continue;
			}
			_ =>
			{
				error!("Unknown key type: {}", key_type);
				ValueBinType::Unknown
			}
		};

		// 获取键的过期时间
		let ttl: Option<i64> = match con.ttl(&key)
		{
			Ok(ttl) => ttl,
			Err(_) => None,
		};

		// 将数据添加到向量中
		data.push(BinData { key, value, ttl });
	}

	// 序列化为JSON并写入文件
	let bin_data = serde_cbor::to_vec(&data)?;
	let mut file = File::create(filename)?;
	file.write_all(&bin_data)?;
	println!("Save {} keys to {}", key_count, filename);

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
	fn test_redis_save_1() -> AtysResult<()>
	{
		init_log();
		save_to_json("redis://localhost:6379/15", "F:\\temp\\redis_data_1.json")?;
		Ok(())
	}
}
