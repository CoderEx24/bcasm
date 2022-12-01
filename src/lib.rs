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

    if val.len() > 2 && &val[0..2] == "0x" {
        let hex_value = &val[2..].to_uppercase();

        value = u16::from_str_radix(hex_value, 16).expect(
            format!("[ERROR] failed to parse {}", val).as_str()
        );

    } else if val.len() > 1 && &val[0..1] == "0" {
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

/// # parse_asm
/// reads an assembly file, and produces a symbol table (represented as a hashmap)
/// and a the code as a vector
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

    let mut data = HashMap::new();
    let mut code = Vec::new();
    let mut is_data = true;


    for (i, line) in contents
                        .to_ascii_lowercase()
                        .trim()
                        .split('\n')
                        .into_iter()
                        .filter(|v| !v.is_empty())
                        .map(|v| v.trim_start().trim_end())
                        .enumerate() {

        if line.trim_end_matches(':').to_lowercase() == "data" {
            is_data = true;
            continue;

        } else if line.trim_end_matches(':').to_lowercase() == "text" {
            is_data = false;
            continue;

        } else if line.ends_with(':') {
            let label = line.trim_end_matches(':');
            data.insert(String::from(label), (i + 1) as u16);
            continue;

        }

        if is_data {
            let parts: Vec<&str> = line.split(' ').collect();
            let label = parts[0].trim();
            let value = parse_value(parts[1]).unwrap();

            data.insert(String::from(label), value);

        } else if ! line.ends_with(':') {
            code.push(String::from(line.trim()));

        }

    }

    Ok((data, code))
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

    #[test]
    fn test_parse_asm() {
        use std::fs::write;
        
        let content = 
            "data:    \n \
                a 1   \n \
                b 2   \n \
             text:    \n \
                lda a \n \
                add b \n \
                out   \n \
                hlt";

        write("./test.asm", content);
        let (data, code) = parse_asm("./test.asm").unwrap();

        assert_eq!("lda a", code[0]);
        assert_eq!("add b", code[1]);
        assert_eq!("out",   code[2]);
        assert_eq!("hlt",   code[3]);

        assert_eq!(1, *data.get("a").unwrap());
        assert_eq!(2, *data.get("b").unwrap());

        println!("output: {:?}\n{:?}", data, code);
    }
}
