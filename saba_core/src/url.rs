use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self.clone())
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }

    fn is_http(&mut self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        false
    }

    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        // ?があればそれより前の部分だけを使用
        let host_and_port = url_parts[0].split('?').next().unwrap_or(url_parts[0]);

        if let Some(index) = host_and_port.find(':') {
            host_and_port[..index].to_string()
        } else {
            host_and_port.to_string()
        }
    }

    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        // ?があればそれより前の部分だけを使用
        let host_and_port = url_parts[0].split('?').next().unwrap_or(url_parts[0]);

        if let Some(index) = host_and_port.find(':') {
            host_and_port[index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }

    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        path_and_searchpart[0].to_string()
    }

    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            // パス部分がない場合、ホスト部分に?が含まれているかチェック
            if let Some(index) = url_parts[0].find('?') {
                return url_parts[0][index + 1..].to_string();
            } else {
                return "".to_string();
            }
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        if path_and_searchpart.len() < 2 {
            "".to_string()
        } else {
            path_and_searchpart[1].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_url_creation() {
        let url = Url::new("http://example.com".to_string());
        assert_eq!(url.url, "http://example.com");
        assert_eq!(url.host, "");
        assert_eq!(url.port, "");
        assert_eq!(url.path, "");
        assert_eq!(url.searchpart, "");
    }

    #[test]
    fn test_parse_simple_url() {
        let mut url = Url::new("http://example.com".to_string());
        let result = url.parse();

        assert!(result.is_ok());
        let parsed_url = result.unwrap();
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "80"); // デフォルトポート
        assert_eq!(parsed_url.path(), "");
        assert_eq!(parsed_url.searchpart(), "");
    }

    #[test]
    fn test_parse_url_with_port() {
        let mut url = Url::new("http://example.com:8080".to_string());
        let result = url.parse();

        assert!(result.is_ok());
        let parsed_url = result.unwrap();
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "8080");
        assert_eq!(parsed_url.path(), "");
        assert_eq!(parsed_url.searchpart(), "");
    }

    #[test]
    fn test_parse_url_with_path() {
        let mut url = Url::new("http://example.com/path/to/resource".to_string());
        let result = url.parse();

        assert!(result.is_ok());
        let parsed_url = result.unwrap();
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "80");
        assert_eq!(parsed_url.path(), "path/to/resource");
        assert_eq!(parsed_url.searchpart(), "");
    }

    #[test]
    fn test_parse_url_with_search_params() {
        let mut url = Url::new("http://example.com/search?q=rust&lang=ja".to_string());
        let result = url.parse();

        assert!(result.is_ok());
        let parsed_url = result.unwrap();
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "80");
        assert_eq!(parsed_url.path(), "search");
        assert_eq!(parsed_url.searchpart(), "q=rust&lang=ja");
    }

    #[test]
    fn test_parse_complex_url() {
        let mut url =
            Url::new("http://api.example.com:3000/v1/users?limit=10&offset=20".to_string());
        let result = url.parse();

        assert!(result.is_ok());
        let parsed_url = result.unwrap();
        assert_eq!(parsed_url.host(), "api.example.com");
        assert_eq!(parsed_url.port(), "3000");
        assert_eq!(parsed_url.path(), "v1/users");
        assert_eq!(parsed_url.searchpart(), "limit=10&offset=20");
    }

    #[test]
    fn test_parse_url_with_only_search_params() {
        let mut url = Url::new("http://example.com?query=test".to_string());
        let result = url.parse();

        assert!(result.is_ok());
        let parsed_url = result.unwrap();
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "80");
        assert_eq!(parsed_url.path(), "");
        assert_eq!(parsed_url.searchpart(), "query=test");
    }

    #[test]
    fn test_parse_non_http_scheme_error() {
        let mut url = Url::new("https://example.com".to_string());
        let result = url.parse();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Only HTTP scheme is supported.");
    }

    #[test]
    fn test_parse_ftp_scheme_error() {
        let mut url = Url::new("ftp://files.example.com".to_string());
        let result = url.parse();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Only HTTP scheme is supported.");
    }

    #[test]
    fn test_is_http_function() {
        let mut http_url = Url::new("http://example.com".to_string());
        assert!(http_url.is_http());

        let mut https_url = Url::new("https://example.com".to_string());
        assert!(!https_url.is_http());
    }

    #[test]
    fn test_extract_host_various_cases() {
        let url = Url::new("http://sub.domain.example.com:8080/path".to_string());
        assert_eq!(url.extract_host(), "sub.domain.example.com");

        let url_no_port = Url::new("http://example.com/path".to_string());
        assert_eq!(url_no_port.extract_host(), "example.com");
    }

    #[test]
    fn test_extract_port_various_cases() {
        let url_with_port = Url::new("http://example.com:8080/path".to_string());
        assert_eq!(url_with_port.extract_port(), "8080");

        let url_no_port = Url::new("http://example.com/path".to_string());
        assert_eq!(url_no_port.extract_port(), "80");
    }

    #[test]
    fn test_extract_path_various_cases() {
        let url_with_path = Url::new("http://example.com/api/v1/users".to_string());
        assert_eq!(url_with_path.extract_path(), "api/v1/users");

        let url_root = Url::new("http://example.com".to_string());
        assert_eq!(url_root.extract_path(), "");

        let url_root_slash = Url::new("http://example.com/".to_string());
        assert_eq!(url_root_slash.extract_path(), "");
    }

    #[test]
    fn test_extract_searchpart_various_cases() {
        let url_with_search = Url::new("http://example.com/search?q=test&type=all".to_string());
        assert_eq!(url_with_search.extract_searchpart(), "q=test&type=all");

        let url_no_search = Url::new("http://example.com/path".to_string());
        assert_eq!(url_no_search.extract_searchpart(), "");

        let url_empty_search = Url::new("http://example.com/path?".to_string());
        assert_eq!(url_empty_search.extract_searchpart(), "");
    }

    // 追加のエッジケーステスト
    #[test]
    fn test_url_with_port_and_search_params() {
        let mut url = Url::new("http://localhost:3000?debug=true&mode=dev".to_string());
        let result = url.parse();

        assert!(result.is_ok());
        let parsed_url = result.unwrap();
        assert_eq!(parsed_url.host(), "localhost");
        assert_eq!(parsed_url.port(), "3000");
        assert_eq!(parsed_url.path(), "");
        assert_eq!(parsed_url.searchpart(), "debug=true&mode=dev");
    }

    #[test]
    fn test_url_with_empty_path_and_trailing_slash() {
        let mut url = Url::new("http://example.com/".to_string());
        let result = url.parse();

        assert!(result.is_ok());
        let parsed_url = result.unwrap();
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "80");
        assert_eq!(parsed_url.path(), "");
        assert_eq!(parsed_url.searchpart(), "");
    }

    #[test]
    fn test_url_with_multiple_question_marks() {
        let mut url = Url::new("http://example.com/search?q=what?is?this&type=test".to_string());
        let result = url.parse();

        assert!(result.is_ok());
        let parsed_url = result.unwrap();
        assert_eq!(parsed_url.host(), "example.com");
        assert_eq!(parsed_url.port(), "80");
        assert_eq!(parsed_url.path(), "search");
        assert_eq!(parsed_url.searchpart(), "q=what?is?this&type=test");
    }

    #[test]
    fn test_url_with_special_port_numbers() {
        let mut url_443 = Url::new("http://secure.example.com:443/api".to_string());
        let result = url_443.parse();

        assert!(result.is_ok());
        let parsed_url = result.unwrap();
        assert_eq!(parsed_url.host(), "secure.example.com");
        assert_eq!(parsed_url.port(), "443");
        assert_eq!(parsed_url.path(), "api");
    }

    #[test]
    fn test_invalid_schemes() {
        let schemes = ["https://", "ftp://", "file://", "mailto:", ""];

        for scheme in schemes.iter() {
            let url_string = format!("{}example.com", scheme);
            let mut url = Url::new(url_string);
            let result = url.parse();

            if scheme == &"" {
                // 空のURLの場合はHTTPスキームがないためエラー
                assert!(result.is_err());
            } else if scheme != &"http://" {
                assert!(result.is_err());
                assert_eq!(result.unwrap_err(), "Only HTTP scheme is supported.");
            }
        }
    }
    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_unsupported_scheme() {
        let url = "https://example.com:8888/index.html".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }
}
