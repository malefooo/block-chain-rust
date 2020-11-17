use std::fmt::Write;

pub fn u8_vec_to_string(v: Vec<u8>) -> String {
    let mut result = String::new();
    for num in v {
        write!(result, "{:02x}", num).expect("---ERROR---");
    }

    result
}

/// 字符串前置位补0
/// str:需补零的字符串
/// width:共多少位
pub fn leading_zero_to_string(str:String,width:u32) -> String{
    let mut leading_zero = width - (str.len() as u32);
    let mut leading_zero_str = String::new();

    loop {
        if leading_zero != 0 {
            leading_zero = leading_zero - 1;

            leading_zero_str.push_str("0");
        } else { break; }
    }

    leading_zero_str.push_str(str.as_str());
    leading_zero_str
}