/// Drops CSI and OSC escape sequences and keeps everything else.
///
/// A hyperlink loses its wrapper but keeps its visible text. An `ESC` followed by
/// anything else is left alone.
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c != '\x1b' {
            result.push(c);
            continue;
        }

        match chars.peek() {
            // CSI: ESC [ params intermediates final, where final is 0x40..=0x7e
            Some('[') => {
                chars.next();

                for c in chars.by_ref() {
                    if matches!(c, '\x40'..='\x7e') {
                        break;
                    }
                }
            }

            // OSC: ESC ] payload terminator, where terminator is BEL, ST or 8-bit ST
            Some(']') => {
                chars.next();

                while let Some(c) = chars.next() {
                    match c {
                        '\x07' | '\u{9c}' => break,
                        '\x1b' => {
                            if chars.peek() == Some(&'\\') {
                                chars.next();
                            }

                            break;
                        }
                        _ => {}
                    }
                }
            }

            _ => result.push(c),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::strip_ansi;

    #[test]
    fn plain_text_is_unchanged() {
        assert_eq!(strip_ansi("no escapes here"), "no escapes here");
    }

    #[test]
    fn strips_foreground_and_reset() {
        assert_eq!(strip_ansi("\x1b[32m[INFO]\x1b[0m ready"), "[INFO] ready");
    }

    #[test]
    fn strips_multi_parameter_sequences() {
        assert_eq!(strip_ansi("\x1b[38;2;255;105;180mpink\x1b[0m"), "pink");
        assert_eq!(strip_ansi("\x1b[38;5;208mfixed\x1b[0m"), "fixed");
    }

    #[test]
    fn strips_nested_styles() {
        assert_eq!(
            strip_ansi("\x1b[1m\x1b[32mbold green\x1b[0m\x1b[0m"),
            "bold green"
        );
    }

    #[test]
    fn keeps_lone_escape_without_bracket() {
        assert_eq!(strip_ansi("a\x1bb"), "a\x1bb");
    }

    #[test]
    fn keeps_non_ascii_payload() {
        assert_eq!(strip_ansi("\x1b[33mcaffè ☕\x1b[0m"), "caffè ☕");
    }

    #[test]
    fn strips_osc_hyperlink_with_st() {
        assert_eq!(
            strip_ansi("see \x1b]8;;https://example.com\x1b\\the docs\x1b]8;;\x1b\\ now"),
            "see the docs now"
        );
    }

    #[test]
    fn strips_osc_terminated_by_bel() {
        assert_eq!(strip_ansi("\x1b]0;window title\x07ready"), "ready");
    }

    #[test]
    fn strips_osc_terminated_by_8bit_st() {
        assert_eq!(strip_ansi("\x1b]8;;https://example.com\u{9c}link"), "link");
    }

    #[test]
    fn osc_payload_may_contain_semicolons_and_brackets() {
        assert_eq!(strip_ansi("\x1b]8;;https://a.dev/x?y=1;z=[2]\x07ok"), "ok");
    }

    #[test]
    fn unterminated_osc_consumes_the_rest() {
        assert_eq!(strip_ansi("keep\x1b]8;;never closed"), "keep");
    }

    #[test]
    fn strips_csi_with_non_alphabetic_final_byte() {
        assert_eq!(strip_ansi("\x1b[?25lhidden\x1b[?25h"), "hidden");
        assert_eq!(strip_ansi("\x1b[0@pad"), "pad");
    }

    #[test]
    fn keeps_escape_followed_by_other_bytes() {
        assert_eq!(strip_ansi("a\x1b(Bb"), "a\x1b(Bb");
    }

    #[test]
    fn hyperlink_and_color_together() {
        assert_eq!(
            strip_ansi("\x1b[32m\x1b]8;;file:///tmp/log\x07app.log\x1b]8;;\x07\x1b[0m"),
            "app.log"
        );
    }
}
