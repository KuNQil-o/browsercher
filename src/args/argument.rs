use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(short, long, required = false)]
    pub text: Option<String>,
    pub url: Option<String>,
}

/// 子命令
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 向配置文件中添加网站配置
    Set(SetStruct),
    /// 展示所有配置
    List
}

#[derive(Parser, Debug)]
pub struct SetStruct {
    #[arg(short, long, required = false,default_value = "")]
    pub name: String,
    #[arg(short, long, required = true)]
    pub url: String,
    #[arg(short, long, required = true ,value_delimiter=' ')]
    pub alias: Vec<String>,
    #[arg(short, long, required = false,default_value = "")]
    pub rules: String
}


