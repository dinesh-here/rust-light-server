use std::{path::PathBuf, ops::Add};


pub fn get_file_path(str: String, path:PathBuf) -> String {
    let clean_path = clear_args(str);
    return match clean_path.as_str() {
        "/" => String::from(path.to_string_lossy().to_string() + "index.html"),
        _ => String::from(path.to_string_lossy().to_string()).add(&clean_path)
    };
}


fn clear_args(str: String) -> String {
    let idx_pos = str.find("?").unwrap_or(0);

    if idx_pos > 0 {
        return str.split_at(idx_pos).0.to_string();
    }

    return str;
}