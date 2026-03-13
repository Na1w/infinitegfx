#[macro_export]
macro_rules! font_char {
    ($($line:expr),* $(,)?) => {{
        let mut b0 = 0u32;
        let mut b1 = 0u32;
        let mut row = 0;
        $(
            {
                let bytes = $line.as_bytes();
                let mut col = 0;
                while col < 8 && col < bytes.len() {
                    if bytes[col] != b' ' {
                        let bit_idx = col + row * 8;
                        if bit_idx < 32 {
                            b0 |= 1u32 << bit_idx;
                        } else {
                            b1 |= 1u32 << (bit_idx - 32);
                        }
                    }
                    col += 1;
                }
                row += 1;
            }
        )*
        let _ = row;
        [b0, b1]
    }};
}

/// Default font for text rendering.
pub const DEFAULT_FONT: [[u32; 2]; 27] = [
    font_char!(
        "  XXXX  ", " XX  XX ", " XX  XX ", " XXXXXX ", " XX  XX ", " XX  XX ", " XX  XX ",
        "        ",
    ),
    font_char!(
        " XXXXX  ", " XX  XX ", " XX  XX ", " XXXXX  ", " XX  XX ", " XX  XX ", " XXXXX  ",
        "        ",
    ),
    font_char!(
        "  XXXX  ", " XX  XX ", " XX     ", " XX     ", " XX     ", " XX  XX ", "  XXXX  ",
        "        ",
    ),
    font_char!(
        " XXXXX  ", " XX  XX ", " XX  XX ", " XX  XX ", " XX  XX ", " XX  XX ", " XXXXX  ",
        "        ",
    ),
    font_char!(
        " XXXXXX ", " XX     ", " XX     ", " XXXXX  ", " XX     ", " XX     ", " XXXXXX ",
        "        ",
    ),
    font_char!(
        " XXXXXX ", " XX     ", " XX     ", " XXXXX  ", " XX     ", " XX     ", " XX     ",
        "        ",
    ),
    font_char!(
        "  XXXX  ", " XX  XX ", " XX     ", " XX XXX ", " XX  XX ", " XX  XX ", "  XXXX  ",
        "        ",
    ),
    font_char!(
        " XX  XX ", " XX  XX ", " XX  XX ", " XXXXXX ", " XX  XX ", " XX  XX ", " XX  XX ",
        "        ",
    ),
    font_char!(
        "  XXXX  ", "   XX   ", "   XX   ", "   XX   ", "   XX   ", "   XX   ", "  XXXX  ",
        "        ",
    ),
    font_char!(
        "  XXXXX ", "     XX ", "     XX ", "     XX ", "     XX ", " XX  XX ", "  XXXX  ",
        "        ",
    ),
    font_char!(
        " XX  XX ", " XX XX  ", " XXXX   ", " XXX    ", " XXXX   ", " XX XX  ", " XX  XX ",
        "        ",
    ),
    font_char!(
        " XX     ", " XX     ", " XX     ", " XX     ", " XX     ", " XX     ", " XXXXXX ",
        "        ",
    ),
    font_char!(
        " XX  XXX", " XXX XXX", " XXXXXXX", " XXXXXXX", " XX X XX", " XX   XX", " XX   XX",
        "        ",
    ),
    font_char!(
        " XX   XX", " XXX  XX", " XXX  XX", " XX X XX", " XX  XXX", " XX  XXX", " XX   XX",
        "        ",
    ),
    font_char!(
        "  XXXX  ", " XX  XX ", " XX  XX ", " XX  XX ", " XX  XX ", " XX  XX ", "  XXXX  ",
        "        ",
    ),
    font_char!(
        " XXXXX  ", " XX  XX ", " XX  XX ", " XXXXX  ", " XX     ", " XX     ", " XX     ",
        "        ",
    ),
    font_char!(
        "  XXXX  ", " XX  XX ", " XX  XX ", " XX  XX ", " XX XXX ", " XX  XX ", "  XXXX X",
        "      XX",
    ),
    font_char!(
        " XXXXX  ", " XX  XX ", " XX  XX ", " XXXXX  ", " XXXX   ", " XX XX  ", " XX  XX ",
        "        ",
    ),
    font_char!(
        "  XXXX  ", " XX  XX ", "  XX    ", "   XX   ", "    XX  ", " XX  XX ", "  XXXX  ",
        "        ",
    ),
    font_char!(
        " XXXXXX ", "   XX   ", "   XX   ", "   XX   ", "   XX   ", "   XX   ", "   XX   ",
        "        ",
    ),
    font_char!(
        " XX  XX ", " XX  XX ", " XX  XX ", " XX  XX ", " XX  XX ", " XX  XX ", "  XXXX  ",
        "        ",
    ),
    font_char!(
        " XX  XX ", " XX  XX ", " XX  XX ", " XX  XX ", "  XXXX  ", "   XX   ", "   XX   ",
        "        ",
    ),
    font_char!(
        " XX   XX", " XX   XX", " XX   XX", " XX X XX", " XXXXXXX", " XXXXXXX", "  XX XX ",
        "        ",
    ),
    font_char!(
        " XX  XX ", " XX  XX ", "  XXXX  ", "   XX   ", "  XXXX  ", " XX  XX ", " XX  XX ",
        "        ",
    ),
    font_char!(
        " XX  XX ", " XX  XX ", "  XXXX  ", "   XX   ", "   XX   ", "   XX   ", "   XX   ",
        "        ",
    ),
    font_char!(
        " XXXXXX ", "     XX ", "    XX  ", "   XX   ", "  XX    ", " XX     ", " XXXXXX ",
        "        ",
    ),
    font_char!(
        "        ", "        ", "        ", "        ", "        ", "        ", "        ",
        "        ",
    ),
];
