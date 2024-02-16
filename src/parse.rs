use crate::{DEFAULT_PORT, ServeConfig};
use crate::ServeContent::RawContent;

impl ServeConfig {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<ServeConfig, &'static str> {
        args.next(); // skip "tiny-serve"

        let mut contents: Vec<String> = Vec::new();
        let mut port = DEFAULT_PORT;
        let are_files = false;

        while let Some(arg) = args.next() {
            if arg == "-p" {
                // Get port number
                match args.next() {
                    Some(text) => {
                        match text.parse() {
                            Ok(number) => port = number,
                            _ => return Err("Given port is invalid")
                        }
                    }
                    None => return Err("No port specified.")
                }
            } else {
                contents.push(arg)
            }
        }

        if contents.is_empty() {
            return Err("Usage: tiny-serve [-p <PORT>] [-f] <content|filename>...");
        }

        let content = match are_files {
            true => RawContent(String::from("no content")),
            false => {
                let strings = contents.join("\n");

                RawContent(strings)
            }
        };

        Ok(ServeConfig { content, port })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! to_args {
        [ $($x:expr),* ] => {
            ["tiny-serve", $($x,)*].iter().map(|s| s.to_string())
        };
    }

    #[test]
    fn parse_basic() {
        let args = to_args!["content"];
        let config = ServeConfig::build(args).unwrap();

        assert_eq!(RawContent(String::from("content")), config.content);
        assert_eq!(DEFAULT_PORT, config.port);
    }

    #[test]
    fn parse_port_successfully() {
        let args = to_args!["-p", "1200", "more_content"];
        let config = ServeConfig::build(args).unwrap();

        assert_eq!(1200, config.port);
        assert_eq!(RawContent(String::from("more_content")), config.content);

        let args = to_args!["initial content", "-p", "2700"];
        let config = ServeConfig::build(args).unwrap();

        assert_eq!(2700, config.port);
        assert_eq!(RawContent(String::from("initial content")), config.content);
    }
}