#[macro_export]
macro_rules! cli_panic {
    ($($arg:tt)*) => {
        <$crate::Cli as ::clap::CommandFactory>::command().error(::clap::error::ErrorKind::ValueValidation, format!($($arg)*)).exit()
    };
}

pub fn rainbow_text(text: &str) -> String {
    const COLORS: [char; 7] = ['c', '6', 'e', 'a', 'b', '9', 'd'];
    const ESCAPE: char = '§';

    let mut output = String::with_capacity(text.len() * 3);
    let mut idx = 0;

    for ch in text.chars() {
        if ch.is_whitespace() {
            output.push(ch);
            continue;
        }
        let color: char = COLORS[idx];
        idx += 1;
        idx %= COLORS.len();
        output.push(ESCAPE);
        output.push(color);
        output.push(ch);
        if ch == ESCAPE {
            output.push(ESCAPE);
        }
    }

    output
}
