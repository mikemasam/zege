pub struct AppLogger {}

impl AppLogger {
    pub fn log(str: String) {
        println!("LOG: > {}", str);
    }
    pub fn debug(str: String) {
        println!("DEBUG: ? {}", str);
    }
    pub fn error(str: String) {
        println!("ERR: ! {}", str);
    }
}
