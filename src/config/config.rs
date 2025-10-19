use std::fs;
use std::path::Path;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::context::const_context;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ConfigRoot {
    pub config: Config,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub website: Vec<Website>,
}

#[derive(Debug, Serialize, Deserialize,Clone,Builder)]
#[builder(build_fn(name = "private_build"))]
pub struct Website {
    name: String,
    url: String,
    aliases: Vec<String>,
    rules: String,
}

impl WebsiteBuilder {
    pub fn build(&self) -> Result<Website, String> {
        let mut build = self.private_build().map_err(|e| e.to_string())?;
        if build.name.is_empty() {
            build.name = build.url.clone();
        }
        if build.rules.is_empty() {
            build.rules = "".to_string();
        }
        Ok(build)
    }
}

impl Website {
}

impl Config {
    /// 根据别名查找匹配的网址
    pub fn find_url_by_alias(&self, context: &str) -> Option<&str> {
        for website in &self.website {
            for alias in &website.aliases {
                if alias == context {
                    return Some(&website.url);
                }
            }
            if website.url == context {
                return Some(&website.url);
            }
        }
        None
    }

    pub fn find_rule_by_url(&self, url: &str) -> Option<&str> {
        for website in &self.website {
            if url == website.url {
                return Some(&website.rules);
            }
        }
        None
    }
}

impl ConfigRoot {
    /// 从文件读取配置
    pub fn load() -> Result<ConfigRoot,Box<dyn std::error::Error>> {
        let path = const_context::CFG_PATH_STR.as_str();
        if !Path::new(path).exists() {
            let default = ConfigRoot { config: Config { website: vec![] } };
            fs::write(path, serde_yaml::to_string(&default).unwrap()).unwrap();
            return Ok(default);
        }
        let yaml_str = fs::read_to_string(path)?;
        let root: ConfigRoot = serde_yaml::from_str(&yaml_str)?;
        Ok(root)
    }

    /// 写入配置文件
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let yaml_str = serde_yaml::to_string(self)?;
        fs::write(const_context::CFG_PATH_STR.as_str(), yaml_str)?;
        Ok(())
    }

    /// 添加网站配置
    pub fn add_website(website: Website) -> Result<(),Box<dyn std::error::Error>> {
        let mut root = ConfigRoot::load()?;
        if root.config.website.iter().any(|w| w.url == website.url) {
            println!("网站 [{}] 已存在", website.name);
            return Ok(());
        }
        root.config.website.push(website);
        root.save()?;
        println!("添加成功！");
        Ok(())
    }

    pub fn list() {
        let root = ConfigRoot::load().unwrap();
        let website = root.config.website;
        for site in &website {
            println!("name: {}",site.name);
            println!("url: {}",site.url);
            println!("rules: {}",site.rules);
            println!("aliases:{}",site.aliases.join(","));
        }
    }
}
