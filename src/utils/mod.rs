use std::fmt::Write;

pub fn u8_vec_to_string(v:Vec<u8>) -> String{
    let mut result = String::new();
    for num in v {
        write!(result,"{:02x}",num);
    }

    result
}