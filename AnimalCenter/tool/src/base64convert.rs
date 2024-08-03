use rustc_serialize::base64::{ToBase64, MIME};

pub fn vectobase64(vec: &Vec<u8>, t: &str) -> String {
    let base64 = vec.to_base64(MIME);
    format!("data:image/{};base64,{}", t, base64.replace("\r\n", ""))
}
