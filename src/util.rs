pub(crate) fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;

    for c in s.chars() {
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            result.push(c);
        }
    }

    result
}
