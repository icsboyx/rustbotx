use colored::*;

pub struct LocalConsole {
   pub prefix: &'static str,
   pub prefix_color: Color,
}

impl LocalConsole {
    pub fn new(prefix: &'static str, prefix_color: Color) -> Self {
        LocalConsole {
            prefix,
            prefix_color,
        }
    }

    pub fn console_println(&self, message: impl Into<String>) {
        let colored_prefix = match self.prefix_color {
            Color::Black => self.prefix.black(),
            Color::Red => self.prefix.red(),
            Color::Green => self.prefix.green(),
            Color::Yellow => self.prefix.yellow(),
            Color::Blue => self.prefix.blue(),
            Color::Magenta => self.prefix.magenta(),
            Color::Cyan => self.prefix.cyan(),
            Color::White => self.prefix.white(),
            Color::BrightBlack => self.prefix.bright_black(),
            Color::BrightRed => self.prefix.bright_red(),
            Color::BrightGreen => self.prefix.bright_green(),
            Color::BrightYellow => self.prefix.bright_yellow(),
            Color::BrightBlue => self.prefix.bright_blue(),
            Color::BrightMagenta => self.prefix.bright_magenta(),
            Color::BrightCyan => self.prefix.bright_cyan(),
            Color::BrightWhite => self.prefix.bright_white(),
        };

        println!(
            "[{}]: {}",
            colored_prefix,
            message.into()
        );
    }
}

impl Default for LocalConsole {
    fn default() -> Self {
        LocalConsole {
            prefix: "",
            prefix_color: Color::White,
        }
    }
}
