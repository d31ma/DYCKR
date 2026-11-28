pub struct PageNavigation;

impl PageNavigation {
    pub const FETCH_MODE: &'static str = "sec-fetch-mode";
    pub const SHARED_PATH_VARY: &'static str = "Sec-Fetch-Mode, Accept";
    const PAGE_METHODS: [&'static str; 2] = ["GET", "HEAD"];
    const NAVIGATE: &'static str = "navigate";
    const HTML: &'static str = "text/html";

    pub fn is_navigation(method: &str, accept: Option<&str>, fetch_mode: Option<&[u8]>) -> bool {
        let reads = Self::reads(method);
        let asks_for_document = Self::asks_for_document(accept, fetch_mode);
        [reads, asks_for_document] == [true, true]
    }

    pub fn reads(method: &str) -> bool {
        Self::PAGE_METHODS.contains(&method)
    }

    fn asks_for_document(accept: Option<&str>, fetch_mode: Option<&[u8]>) -> bool {
        let accepts_html = accept.is_some_and(|accept| accept.contains(Self::HTML));
        fetch_mode.map_or(accepts_html, |mode| mode == Self::NAVIGATE.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::PageNavigation;

    #[test]
    fn a_navigating_get_or_head_asks_for_the_page() {
        assert!(PageNavigation::is_navigation(
            "GET",
            None,
            Some(b"navigate")
        ));
        assert!(PageNavigation::is_navigation(
            "HEAD",
            Some("application/json"),
            Some(b"navigate")
        ));
    }

    #[test]
    fn without_a_fetch_mode_an_html_accept_asks_for_the_page() {
        assert!(PageNavigation::is_navigation(
            "GET",
            Some("text/html,*/*"),
            None
        ));
        assert!(!PageNavigation::is_navigation(
            "GET",
            Some("application/json"),
            None
        ));
        assert!(!PageNavigation::is_navigation("GET", None, None));
    }

    #[test]
    fn a_fetch_mode_other_than_navigate_wins_over_an_html_accept() {
        assert!(!PageNavigation::is_navigation(
            "GET",
            Some("text/html"),
            Some(b"cors")
        ));
        assert!(!PageNavigation::is_navigation(
            "GET",
            Some("text/html"),
            Some(b"Navigate")
        ));
    }

    #[test]
    fn only_get_and_head_read_a_page() {
        assert!(!PageNavigation::is_navigation(
            "POST",
            Some("text/html"),
            Some(b"navigate")
        ));
        assert!(!PageNavigation::is_navigation(
            "get",
            Some("text/html"),
            Some(b"navigate")
        ));
        assert!(PageNavigation::reads("HEAD"));
        assert!(!PageNavigation::reads("OPTIONS"));
    }
}
