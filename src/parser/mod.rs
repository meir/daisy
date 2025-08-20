pub fn parse_string(input: &str) -> String {
    let input = input[1..input.len() - 1].to_string();

    escape_characters(&input)
}

pub fn parse_multiline_string(input: &str) -> String {
    let s = input.to_string();
    let inner = &s[2..s.len() - 2];

    let lines: Vec<&str> = inner.lines().collect();
    let indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);
    let lines = lines
        .into_iter()
        .map(|l| if l.len() >= indent { &l[indent..] } else { l })
        .collect::<Vec<_>>();
    let lines = lines[1..lines.len() - 1].join("\n");

    escape_characters(&lines)
}

fn escape_characters(input: &str) -> String {
    let mut result = String::new();
    let mut escape = false;
    for c in input.chars() {
        if escape {
            match c {
                'n' => result.push('\n'),
                't' => result.push('\t'),
                '\\' => result.push('\\'),
                '"' => result.push('"'),
                _ => result.push(c), // Just add the character as is
            }
            escape = false;
        } else if c == '\\' {
            escape = true;
        } else {
            result.push(c);
        }
    }
    result
}
