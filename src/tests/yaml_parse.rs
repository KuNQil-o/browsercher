use std::fs::File;
use std::io::Read;
use crate::config::config::ConfigRoot;

#[test]
fn test_yaml_parse(){
    let mut file = match File::open("/home/wkq/develop/code/rust/browsearcher/config.yaml") {
        Ok(file) => {
            file
        }
        Err(err) => {
            panic!("{}", err);
        }
    };
    // 创建一个字符串
    let mut contents = String::new();

    // 读取文件内容到字符串缓冲区
    file.read_to_string(&mut contents).unwrap();

    // 打印原始的YAML字符串
    println!("YAML file contents:\n{}", contents);

    let config: ConfigRoot = serde_yaml::from_str(&contents).unwrap();
    println!("{:?}", config);
}