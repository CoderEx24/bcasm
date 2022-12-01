use std::collections::HashMap;

type ParsedAsm = (HashMap<String, u16>, Vec<String>);

/// # parse_value
/// parses a string that contains a numerical value, and returns it as a 2-byte number
pub fn parse_value(str_val: &str) -> Result<u16, &str> {
    use std::u16;
    
    if str_val.starts_with('\'') && str_val.ends_with('\'') {
        return Ok(str_val.as_bytes()[1] as u16);

    }


    let mut value: u16 = 0;

    let val_lower = str_val.to_ascii_lowercase();
    let val = val_lower.as_str();

    if &val[0..2] == "0x" {
        let hex_value = &val[2..].to_uppercase();

        value = u16::from_str_radix(hex_value, 16).expect(
            format!("[ERROR] failed to parse {}", val).as_str()
        );

    } else if &val[0..1] == "0" {
        let octal_value = &val[1..];
        
        value = u16::from_str_radix(octal_value, 8).expect(
            format!("[ERROR] failed to parse {}", val).as_str()
        );
    } else {
        value = val.parse().expect(
            format!("[ERROR] failed to parse {}", val).as_str()
        );
    }

    Ok(value)
}

pub fn parse_asm(filepath: &str) -> Result<ParsedAsm, String> {
    use std::fs::{ read_to_string, metadata };

    let metadata = metadata(filepath).expect(
        format!("[ERROR] failed to get metadata of {}", filepath).as_str()
    );

    if !metadata.is_file() {
        return Err(
            format!("[ERROR] {} is not a file", filepath)
        );
    }

    let contents = read_to_string(filepath).expect(
        format!("[ERROR] couldn't read {}", filepath).as_str()
    );

    let mut data: HashSet<String, u16>;
    let mut code: Vec<String>;
    let mut is_data = true;


    for line in contents.split('\n').into_iter() {
        if line.trim_end_matches(':').to_lowercase() == "data" {
            is_data = true;

        } else if line.trim_end_matches(':').to_lowercase() == "text" {
            is_data = false;

        }

        if is_data {
            let parts: Vec<&str> = line.split(' ').collect();
            let label = parts[0];
            let value = parts[1];
        }

    }

    Err("".into())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!(Ok(12), parse_value("12"));
        assert_eq!(Ok(12), parse_value("014"));
        assert_eq!(Ok(12), parse_value("0xC"));
        assert_eq!(Ok(12), parse_value("0xc"));
        
    }
}
