pub(crate) fn strip_ansi_codes(s: &str) -> String {
    if s.is_empty() || !s.contains('\x1b') {
        return s.to_string();
    }

    let mut buf = String::with_capacity(s.len());
    let mut in_escape = false;

    for c in s.chars() {
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            buf.push(c);
        }
    }

    // Shrink to fit actual size if there's significant difference
    if buf.capacity() > buf.len() + (buf.len() / 4) {
        buf.shrink_to_fit();
    }

    buf
}
