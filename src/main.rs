mod args;
mod config;
mod context;
mod tests;

use crate::args::argument::{Args, Commands};
use clap::Parser;
use std::process::exit;
use regex::Regex;
use crate::config::config::{ConfigRoot, WebsiteBuilder};
use crate::context::const_context;
use crate::context::context_func::invoke_browser;
// TODO 设置默认搜索

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match &args.command {
        // ---------------- 子命令: set ----------------
        Some(Commands::Set(set)) => {

            let website = WebsiteBuilder::default()
                .name(set.name.clone())
                .url(set.url.clone())
                .aliases(set.alias.clone())
                .rules(set.rules.clone())
                .build()?;
            ConfigRoot::add_website(website)?;
        }
        Some(Commands::List) => {
            ConfigRoot::list();
        }
        // ---------------- 主命令 ----------------
        None => {
            let url = match &args.url {
                Some(u) => u,
                None => {
                    eprintln!("❌ 请输入网址或别名！");
                    exit(1);
                }
            };

            let config = &const_context::CFG_STRUCT.config;

            // 1. 尝试在配置文件中查找别名对应的网址
            if let Some(found_url) = config.find_url_by_alias(url) {
                invoke_browser(
                    found_url,
                    args.text.as_deref(),
                    config.find_rule_by_url(found_url),
                )?;
                return Ok(()); // 不需要 exit(0)，自然结束即可
            }

            // 2. 如果不是别名，判断是否是合法网址
            let url_pattern = Regex::new(r"^https?://.*$")?;
            if url_pattern.is_match(url) {
                invoke_browser(url, None, None)?;
            } else {
                eprintln!("❌ 请输入正确的网址！");
            }
        }
    }

    Ok(())
}




// fn main() {
//     // 获取用户输入的命令行参数
//     let args = Args::parse();
//     let url = args.url.unwrap_or_default();
//     let text = args.text.unwrap_or_default();
//
//
//     // 解析子命令
//     match &args.command {
//         Some(Commands::Set(set)) => set.set_website(),
//         None => {
//             // 执行命令
//             // 读取外部配置文件
//             // 根据用户主目录获取配置文件路径
//             let config_path = std::env::home_dir()
//                 .expect("无法获取用户主目录")
//                 .join(".browsearcher.yaml");
//             // 获取配置文件
//             let config = match File::open(&config_path) {
//                 Ok(config_file) => config_file,
//                 Err(err) => {
//                     panic!("无法打开配置文件 {:?}: {}", config_path, err);
//                 }
//             };
//             // 解析配置文件
//             let root: ConfigRoot = serde_yaml::from_reader(config).unwrap();
//             let config = root.config;
//             // 根据别名获取配置文件中的路径
//             let config_url = match config.find_url_by_alias(&url) {
//                 None => {
//                     panic!("未找到该别名的网址，请进行配置！")
//                 }
//                 Some(url) => url,
//             };
//             // 打开浏览器
//             if !config_url.is_empty() {
//                 let rule = config.find_rule_by_url(config_url).unwrap_or_default();
//                 webbrowser::open(format!("{}/{}{}", config_url, rule, text).as_str()).unwrap();
//             }
//         }
//     }
// }
