use std::fs;

pub fn read_doc_to_resp(path: String) -> String {
    let status_200 = "HTTP/1.1 200 OK";
    let status_404 = "HTTP/1.1 404 Not Found";

    let contents = fs::read_to_string(path);

    match contents {
        Ok(fc) => {
            let length = fc.len();
            return format!("{status_200}\r\nContent-Length: {length}\r\n\r\n{fc}");
        }
        Err(e) => {
            println!("Unable to read files :  {:?}", e);
            return format!("{status_404}\r\n");
        }
    }
}