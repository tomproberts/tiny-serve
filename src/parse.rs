use std::fs;
use crate::{DEFAULT_PORT, ServeConfig};
use crate::ServeContent::{PageContent, RawContent};

#[derive(Debug, serde::Deserialize)]
struct ExternalConfig {
    port: Option<u16>,
    files: Option<Vec<RoutedFile>>,
    raw: Option<Vec<RoutedRawContent>>,
}

#[derive(Debug, PartialEq, serde::Deserialize, Clone)]
struct RoutedFile {
    route: String,
    file: String,
    #[serde(rename(deserialize = "type"))]
    content_type: Option<String>,
}

#[derive(Debug, PartialEq, serde::Deserialize, Clone)]
struct RoutedRawContent {
    route: String,
    content: String,
    #[serde(rename(deserialize = "type"))]
    content_type: Option<String>,
    status: Option<u16>,
}

impl ExternalConfig {
    pub fn to_serve_config(&self) -> ServeConfig {
        ServeConfig { content: RawContent(String::from("content")), port: self.port.unwrap_or(DEFAULT_PORT) }
    }
}

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
                    None => return Err("No port specified with -p.")
                }
            } else if arg == "-f" {
                // All values should be files
                serve_files = true
            } else if arg == "-c" {
                // Config file specified
                return match args.next() {
                    Some(path) => {
                        match fs::read_to_string(&path) {
                            Ok(conf) => {
                                let parsed: Result<ExternalConfig, _> = serde_yaml::from_str(&conf);
                                match parsed {
                                    Ok(success) => {
                                        println!("Reading from config file '{}', ignoring other arguments...", &path);
                                        Ok(success.to_serve_config())
                                    }
                                    Err(_) => Err("Failed to read config file")
                                }
                            }
                            Err(_) => Err("Could not find config file")
                        }
                    }
                    None => Err("No port specified.")
                };
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

        Ok(ServeConfig { content, port })
    }
}

#[cfg(test)]
mod tests {
    use crate::DEFAULT_PORT;
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

        assert!(config.is_serving_files(), "should be serving files when using -f option");
        assert_eq!(PageContent(vec![String::from("chapter1.html")]), config.content);
        assert_eq!(DEFAULT_PORT, config.port);

        let args = to_args!["chapter1.html", "chapter2.html", "-f"];
        let config = ServeConfig::build(args).unwrap();

        assert!(config.is_serving_files(), "should be serving files when using -f option");
        assert_eq!(PageContent(
            vec![String::from("chapter1.html"), String::from("chapter2.html")]
        ), config.content);
        assert_eq!(DEFAULT_PORT, config.port);
    }

    #[test]
    fn parse_config_yaml() {
        let yml = r#"
            port: 4500
            files:
              - route: "/chapter1"
                file: "./chapter1.html"
              - route: "/chapter4.html"
                file: "chapter4.html"
            raw:
              - route: "/"
                content: "<h1>content</h1>"
                type: "text/html"
                status: 200
        "#;

        let parsed: ExternalConfig = serde_yaml::from_str(yml).unwrap();
        assert_eq!(Some(4500), parsed.port);

        let files = parsed.files.clone().expect("Should be files");
        assert_eq!(2, files.len(), "should be 2 files specified");
        assert_eq!(RoutedFile {
            route: String::from("/chapter1"),
            file: String::from("./chapter1.html"),
            content_type: None,
        }, files[0]);
        assert_eq!(RoutedFile {
            route: String::from("/chapter4.html"),
            file: String::from("chapter4.html"),
            content_type: None,
        }, files[1]);

        let raw = &parsed.raw.clone().expect("Should be raw content");
        assert_eq!(1, raw.len(), "should be 1 raw content route specified");
        assert_eq!(RoutedRawContent {
            route: String::from("/"),
            content: String::from("<h1>content</h1>"),
            content_type: Some(String::from("text/html")),
            status: Some(200),
        }, raw[0]);

        let config = &parsed.to_serve_config();

        assert_eq!(4500, config.port);
        assert_eq!(RawContent(String::from("content")), config.content);
    }
}
