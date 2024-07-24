use std::str::FromStr;

use regex::Regex;
use url::Url;

struct URIUtils {
    board_key_regex: Regex,
}

impl URIUtils {
    const URI_PATH_SEPARATOR: char = '/';
    const BOARD_KEY_LENGTH: usize = 12;
    const BOARD_KEY_REGEX: &'static str =
        r"([A-Za-z0-9\-_]{4}){2}([A-Za-z0-9\-_]{4}|[A-Za-z0-9\-_]{3}=|[A-Za-z0-9\-_]{2}==)?";

    pub fn new() -> Self {
        // this is one of the rare cases where unwrapping is ok.
        // if this fails it is the fault of the author of the regex
        let board_key_regex = Regex::new(Self::BOARD_KEY_REGEX).unwrap();
        Self { board_key_regex }
    }

    pub fn parse_board_key(&self, inbound: &Url) -> Result<String, String> {
        let trimmed_path = inbound
            .path()
            .trim_matches(Self::URI_PATH_SEPARATOR)
            .to_string();
        let board_key = trimmed_path
            .split(Self::URI_PATH_SEPARATOR)
            .last()
            .unwrap_or("");

        if board_key.len() != Self::BOARD_KEY_LENGTH {
            return Err(format!(
                "Incorrect path param, {} should consist of {} characters, full path {}",
                board_key,
                Self::BOARD_KEY_LENGTH,
                trimmed_path
            ));
        }

        if !self.board_key_regex.is_match(board_key) {
            return Err(format!(
                "Incorrect path param, {} isn't proper board key in URI Base64, full path {}",
                board_key, trimmed_path
            ));
        }

        Ok(board_key.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn should_parse_board_key() {
        let test_cases = vec![
            ("wss://miro.com/ws/uXjVNlDh1GM=?user_id=1", "uXjVNlDh1GM="),
            ("wss://miro.com/ws/uXjVNlDh1GM=/", "uXjVNlDh1GM="),
            ("wss://miro.com/ws/uXjVNlDh1G%3D%3D", "uXjVNlDh1G=="),
        ];

        let utils = URIUtils::new();

        for (uri_string, expected) in test_cases {
            let uri = Url::parse(uri_string).expect("Failed to parse URL");
            assert_eq!(utils.parse_board_key(&uri).unwrap(), expected);
        }
    }

    #[test]
    fn should_error_on_parse_board_key() {
        let invalid_uris = vec![
            "",
            "wss://miro.com/",
            "wss://miro.com/ws/u==/",
            "wss://miro.com/ws/uX==",
        ];

        for uri_string in invalid_uris {
            let uri = Url::parse(uri_string);
            let utils = URIUtils::new();
            match uri {
                Ok(uri) => assert!(
                    utils.parse_board_key(&uri).is_err(),
                    "Expected error for URI: {}",
                    uri_string
                ),
                Err(_) => assert!(true, "Invalid URL: {}", uri_string),
            }
        }
    }
}
