use clap::{command, Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 结构体Args，用于解析命令行参数
#[derive(Debug)]
#[derive(Parser)]
#[command(name = "Redis Tools")]
#[command(version = "0.1.2")]
#[command(author = "Larlf Wang <larlf.wang@gmail.com>")]
#[command(about = "Some tools for redis")]
pub struct Args
{
	// 指定操作类型
	#[arg(required = true)]
	pub command: CommandType,
	// Redis服务器的地址
	#[arg(short, long, default_value = "redis://localhost:6379/0")]
	pub url: String,
	// 数据文件的名称
	#[arg(short, long, default_value = "data.*")]
	pub file: String,
	// 数据文件类型
	#[arg(short, long, default_value = "json")]
	pub mode: ModeType,
}

/// 当前支持的命令类型
#[derive(Debug, Clone, ValueEnum)]
pub enum CommandType
{
	Save,
	Load,
}

/// 生成或导入的数据类型
#[derive(Debug, Clone, ValueEnum)]
pub enum ModeType
{
	Json,
	Bin,
}

/// Json数据类型
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum ValueJsonType
{
	Unknown,
	String(String),
	ZSet(HashMap<String, f64>),
	List(Vec<String>),
	Set(Vec<String>),
	Hash(HashMap<String, String>),
	Stream(Vec<(String, HashMap<String, String>)>),
}

/// Json数据
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonData
{
	pub key: String,
	pub value: ValueJsonType,
	pub ttl: Option<i64>,
}

/// 二进制数据类型
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum ValueBinType
{
	Unknown,
	String(Vec<u8>),
	ZSet(HashMap<Vec<u8>, f64>),
	List(Vec<Vec<u8>>),
	Set(Vec<Vec<u8>>),
	Hash(HashMap<Vec<u8>, Vec<u8>>),
	Stream(Vec<(Vec<u8>, HashMap<Vec<u8>, Vec<u8>>)>),
}

/// 二进制数据
#[derive(Serialize, Deserialize, Debug)]
pub struct BinData
{
	pub key: Vec<u8>,
	pub value: ValueBinType,
	pub ttl: Option<i64>,
}
