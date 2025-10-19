use std::fs;
use std::sync::LazyLock;
use crate::config::config::ConfigRoot;

// 字符串形式的路径
pub static CFG_PATH_STR: LazyLock<String> = LazyLock::new(|| {
    dirs::home_dir()
        .expect("无法获取用户目录")
        .join(".browsearcher.yaml")
        .to_str()
        .expect("路径转换失败")
        .to_string()
});

// 结构体类型的配置文件
pub static CFG_STRUCT: LazyLock<ConfigRoot> = LazyLock::new(|| {
    let file_content = fs::read_to_string(CFG_PATH_STR.as_str())
        .expect("无法读取配置文件");
    let parsed_cfg: ConfigRoot = serde_yaml::from_str(&file_content)
        .expect("配置文件解析失败，请检查配置文件格式");
    parsed_cfg
});