pub mod configs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works()  -> std::io::Result<()> {
        println!("Hello, world!");

        let file_path = String::from("E://irust//iconf//src//config.toml");

        unsafe {
            let _ = configs::init(file_path);
            println!("{:?}", configs::CONFIG);

            let c = configs::CONFIG.as_ref().unwrap();
            println!("{:?}", c);

            println!("{}", c.file_path());
            println!("{}", c.settings());

            let env = c.settings()["basics"]["env"].as_str().unwrap();
            let port = c.settings()["basics"]["port"].as_integer().unwrap();
            println!("env={} port={}", env, port);

            print_type_of(&port);

            println!("env = {}", configs::get_str("basics", "env"));
            println!("port = {}", configs::get_int("basics", "port"));
            println!("pi = {}", configs::get_float("basics", "pi"));
            println!("isOk = {}", configs::get_bool("basics", "isOk"));
            print_type_of(&configs::get_bool("basics", "isOk"));

        }

        Ok(())
    }
}

fn print_type_of<T>(_: &T) {
    println!("print_type_of 类型是 = {}", std::any::type_name::<T>())
}
