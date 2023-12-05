use std::path::PathBuf;


pub fn get_file_path(str: String, path:PathBuf) -> String {
    return match str.as_str() {
        "/" => String::from(path.to_string_lossy().to_string() + "index.html"),
        _ => String::from(str)
    };
}
