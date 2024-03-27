
use std::fs::File;
use std::io::prelude::*;
use toml;

// 配置对象
pub static mut CONFIG: Option<&mut Conf> = None;

#[derive(Debug)]
pub struct Conf {
    file_path: String,
    settings: toml::Value,
}

impl Conf{
    fn new(file_path: String, settings: toml::Value) -> Conf{
        Conf{file_path, settings}
    }

    pub fn file_path(&self) -> String {
        String::from(&self.file_path)
    }
    pub fn settings(&self) -> &toml::Value {
        &self.settings
    }
}

/// # init 初始化解析 toml 配置文件
pub unsafe fn init(file_path: String) -> std::io::Result<()> {

    let mut file = File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let value = contents.parse::<toml::Value>().unwrap();

    let c:Conf = Conf::new(file_path.to_string(), value);

    unsafe {
        // 将`c`从内存中泄漏，变成`'static`生命周期
        let bc = Box::new(c);
        CONFIG = Some(Box::leak(bc));

        // println!("{:?}", CONFIG);
        // let s = CONFIG.as_ref().unwrap();
        // println!("{:?}", s);
        // println!("{}", s.file_path());
        // println!("{:?}", s.settings);
    }

    Ok(())
}

/// # get_str 读取字符串配置
/// # 参数
/// key1、key2
/// settings()[key1][key2].as_str()
pub unsafe fn get_str(key1: &str, key2: &str) -> String {
    let cf = CONFIG.as_ref().unwrap();
    cf.settings()[key1][key2].as_str().unwrap().to_string()
}

/// # get_int 读取 i64 配置
pub unsafe fn get_int(key1: &str, key2: &str) -> i64 {
    let cf = CONFIG.as_ref().unwrap();
    cf.settings()[key1][key2].as_integer().unwrap()
}

/// # get_float 读取 f64 配置
pub unsafe fn get_float(key1: &str, key2: &str) -> f64 {
    let cf = CONFIG.as_ref().unwrap();
    cf.settings()[key1][key2].as_float().unwrap()
}

/// # get_bool 读取 bool 配置
pub unsafe fn get_bool(key1: &str, key2: &str) -> bool {
    let cf = CONFIG.as_ref().unwrap();
    cf.settings()[key1][key2].as_bool().unwrap()
}