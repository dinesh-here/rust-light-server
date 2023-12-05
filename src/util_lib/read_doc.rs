use std::fs;

pub fn read_doc_to_resp(path: String) -> String {
    println!("Reading: {:?}", path);
    let status_200 = "HTTP/1.1 200 OK";
    let status_404 = "HTTP/1.1 404 Not Found";

    let contents = fs::read(path);

    match contents {
        Ok(fc) => {
            let length = fc.len();
            let ut8_tx= String::from_utf8_lossy(&fc);
            return format!("{status_200}\r\nContent-Length: {length}\r\n\r\n{ut8_tx}");
        }
        Err(e) => {
            println!("Unable to read files :  {:?}", e);
            return format!("{status_404}\r\n");
        }
    }
}