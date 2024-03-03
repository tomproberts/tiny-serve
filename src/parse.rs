use crate::{DEFAULT_PORT, ServeConfig};
use crate::ServeContent::{PageContent, RawContent};

impl ServeConfig {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<ServeConfig, &'static str> {
        args.next(); // skip "tiny-serve"

        let mut contents: Vec<String> = vec![];
        let mut port = DEFAULT_PORT;
        let mut serve_files = false;

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
            } else if arg == "-f" {
                // All values should be files
                serve_files = true
            } else {
                contents.push(arg)
            }
        }

        if contents.is_empty() {
            return Err("Usage: tiny-serve [-p <PORT>] [-f] <content|filename>...");
        }

        let content = {
            if serve_files { PageContent(contents) } else {
                let strings = contents.join("\n");
                RawContent(strings)
            }
        };

        Ok(ServeConfig { content, port, serve_files })
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

    #[test]
    fn parse_files() {
        let args = to_args!["-f", "chapter1.html"];
        let config = ServeConfig::build(args).unwrap();

        assert!(config.serve_files, "should be serving files when using -f option");
        assert_eq!(PageContent(vec![String::from("chapter1.html")]), config.content);
        assert_eq!(DEFAULT_PORT, config.port);

        let args = to_args!["chapter1.html", "chapter2.html", "-f"];
        let config = ServeConfig::build(args).unwrap();

        assert!(config.serve_files, "should be serving files when using -f option");
        assert_eq!(PageContent(
            vec![String::from("chapter1.html"), String::from("chapter2.html")]
        ), config.content);
        assert_eq!(DEFAULT_PORT, config.port);
    }
}