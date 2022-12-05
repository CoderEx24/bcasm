use std::collections::HashMap;

type ParsedAsm = (HashMap<String, (u16, bool)>, Vec<String>);

/// # parse_value
/// parses a string that contains a numerical value, and returns it as a 2-byte number
pub fn parse_value(str_val: &str) -> Result<u16, Box<dyn std::error::Error>> {
    use std::u16;

    if str_val.starts_with('\'') && str_val.ends_with('\'') {
        return Ok(str_val.as_bytes()[1] as u16);
    }

    let val_lower = str_val.to_ascii_lowercase();
    let val = val_lower.as_str();

    Ok(if val.len() > 2 && &val[0..2] == "0x" {
        let hex_value = &val[2..].to_uppercase();

        u16::from_str_radix(hex_value, 16)?
    } else if val.len() > 1 && &val[0..1] == "0" {
        let octal_value = &val[1..];

        u16::from_str_radix(octal_value, 8)?
    } else {
        val.parse()?
    })
}

/// # parse_asm
/// reads an assembly file, and produces a symbol table (represented as a hashmap)
/// and a the code as a vector
pub fn parse_asm(contents: String) -> Result<ParsedAsm, String> {
    let mut data = HashMap::new();
    let mut code = Vec::new();
    let mut is_data = true;

    let mut addr = 0u16;
    for line in contents
        .to_ascii_lowercase()
        .lines()
        .filter(|v| !v.trim().starts_with("//"))
        .map(|v| v.trim_end_matches("//"))
        .map(|v| {
            if v.contains(':') {
                v.split_inclusive(':').collect()
            } else {
                vec![v]
            }
        })
        .flatten()
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.trim())
    {
        if line.to_ascii_lowercase() == "data:" {
            is_data = true;
            continue;
        } else if line.to_ascii_lowercase() == "text:" {
            is_data = false;
            continue;
        } else if line.ends_with(':') {
            let label = line.trim_end_matches(':');
            data.insert(String::from(label.trim()), ((addr + 1) as u16, true));
            continue;
        }

        if is_data {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let label = parts[0];
            let value = parse_value(parts[1])
                .expect(format!("[ERROR] unparsable value '{}'", parts[1]).as_str());

            data.insert(String::from(label), (value, false));
        } else {
            code.push(String::from(line.trim()));
        }
        addr += 1;
    }

    Ok((data, code))
}

/// # produce_machine_code
/// takes the data and code sections and produces equivlent machine code
pub fn produce_machine_code((data, code): ParsedAsm) -> Result<Vec<u16>, &'static str> {
    if data.len() + code.len() > 4096 {
        return Err("[ERROR] The program exceeds the max size allowed (4096 16-bit words)");
    }

    let mut machine_code = Vec::with_capacity(1 + data.len() + code.len());
    let translation_table = HashMap::from([
        ("and", 0x0),
        ("add", 0x1),
        ("lda", 0x2),
        ("sta", 0x3),
        ("bun", 0x4),
        ("bsa", 0x5),
        ("isz", 0x6),
        ("cla", 0x7800),
        ("cle", 0x7400),
        ("cma", 0x7200),
        ("cme", 0x7100),
        ("cir", 0x7080),
        ("cil", 0x7040),
        ("inc", 0x7020),
        ("spa", 0x7010),
        ("sna", 0x7008),
        ("sza", 0x7004),
        ("sze", 0x7002),
        ("hlt", 0x7001),
        ("inp", 0xf800),
        ("out", 0xf400),
        ("ski", 0xf200),
        ("sko", 0xf100),
        ("ion", 0xf080),
        ("iof", 0xf040),
    ]);

    let mri = ["add", "and", "lda", "sta", "bun", "bsa", "isz"];

    let constants: Vec<u16> = data
        .iter()
        .filter(|(_, (_, is_label))| !*is_label)
        .map(|(_, (v, _))| *v)
        .collect();

    let labels: HashMap<String, u16> = data
        .iter()
        .enumerate()
        .map(|(i, (l, (v, b)))| (l.to_owned(), if *b { *v } else { 1 + i as u16 }))
        .collect();

    machine_code.push(0x4000 | (1 + constants.len() as u16));

    machine_code.extend(constants);

    machine_code.extend(code.iter().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if mri.contains(&parts[0]) {
            let opcode = translation_table.get(parts[0]).unwrap();
            let address = labels.get(parts[1]).cloned().unwrap_or_else(|| {
                parse_value(parts[1]).expect(
                    format!("[ERROR] {} is undefined and not a valid value", parts[1]).as_str(),
                )
            });
            let indirect = if parts.len() == 3 && parts[2].to_lowercase() == "i" {
                1
            } else {
                0
            };

            address | (opcode << 12) | (indirect << 15)
        } else {
            translation_table
                .get(parts[0])
                .cloned()
                .expect(format!("[ERROR] {} is not a recgonized instruction", parts[0]).as_str())
        }
    }));

    Ok(machine_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!(12, parse_value("12").unwrap());
        assert_eq!(12, parse_value("014").unwrap());
        assert_eq!(12, parse_value("0xC").unwrap());
        assert_eq!(12, parse_value("0xc").unwrap());
    }

    #[test]
    fn test_parse_asm() {
        let contents = "data:    \n \
                a 1   \n \
                b 2   \n \
             text:    \n \
                lda a \n \
                add b \n \
                out   \n \
                hlt";

        let (data, code) = parse_asm(contents.into()).unwrap();

        assert_eq!(2, data.len());
        assert_eq!(4, code.len());

        assert_eq!("lda a", code[0]);
        assert_eq!("add b", code[1]);
        assert_eq!("out", code[2]);
        assert_eq!("hlt", code[3]);

        assert_eq!((1, false), *data.get("a").unwrap());
        assert_eq!((2, false), *data.get("b").unwrap());

        println!("output: {:?}\n{:?}", data, code);
    }

    #[test]
    fn test_produce_machine_code() {
        let data = HashMap::from([("a".to_string(), (1, false)), ("b".to_string(), (2, false))]);

        let code = vec![
            "lda a".to_string(),
            "add b".to_string(),
            "out".to_string(),
            "hlt".to_string(),
        ];

        let machine_code = produce_machine_code((data, code)).unwrap();

        assert_eq!(7, machine_code.len());

        assert_eq!(0x4003, machine_code[0]); // 0000 bun 0003
        assert_eq!(0x0001, machine_code[1]); // 0001        1
        assert_eq!(0x0002, machine_code[2]); // 0002        2
        assert_eq!(0x2001, machine_code[3]); // 0003 lda 0001
        assert_eq!(0x1002, machine_code[4]); // 0004 add 0002
        assert_eq!(0xf400, machine_code[5]); // 0005 out
        assert_eq!(0x7001, machine_code[6]); // 0006 hlt
    }
}
