use std::num::ParseIntError;
use std::str;

pub fn all_chars() -> Vec<String> {
    let mut result: Vec<String> = vec![];

    let one_byte = vec![0, 127];
    for i in one_byte[0]..one_byte[1] {
        let first = format!("{:02X}", i);
        if let Some(v) = output(&first) {
            result.push(v);
        }
    }

    let two_bytes = vec![192, 223, 128, 191];
    for i in two_bytes[0]..two_bytes[1] {
        for j in two_bytes[2]..two_bytes[3] {
            let first = format!("{:02X}", i);
            let second = format!("{:02X}", j);
            let hex = format!("{}{}", first, second);

            if let Some(v) = output(&hex) {
                result.push(v);
            }
        }
    }

    let three_bytes = vec![224, 239, 128, 191, 128, 191];
    for i in three_bytes[0]..three_bytes[1] {
        for j in three_bytes[2]..three_bytes[3] {
            for k in three_bytes[4]..three_bytes[5] {
                let first = format!("{:02X}", i);
                let second = format!("{:02X}", j);
                let third = format!("{:02X}", k);
                let hex = format!("{}{}{}", first, second, third);

                if let Some(v) = output(&hex) {
                    result.push(v);
                }
            }
        }
    }

    let four_bytes = vec![240, 247, 128, 191, 128, 191, 128, 191];
    for i in four_bytes[0]..four_bytes[1] {
        for j in four_bytes[2]..four_bytes[3] {
            for k in four_bytes[4]..four_bytes[5] {
                for l in four_bytes[6]..four_bytes[7] {
                    let first = format!("{:02X}", i);
                    let second = format!("{:02X}", j);
                    let third = format!("{:02X}", k);
                    let four = format!("{:02X}", l);
                    let hex = format!("{}{}{}{}", first, second, third, four);

                    if let Some(v) = output(&hex) {
                        result.push(v);
                    }
                }
            }
        }
    }

    result
}

pub fn output(hex: &str) -> Option<String> {
    match &decode_hex(&hex) {
        Ok(dh) => match str::from_utf8(dh) {
            Ok(v) => Some(v.to_string()),
            _ => None,
        },
        _ => None,
    }
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
}
