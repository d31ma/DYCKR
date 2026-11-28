
pub struct LexicalMask {
    pub code: Vec<bool>,
    pub error: Option<&'static str>,
    pub marker_problem: Option<(usize, &'static str)>,
}

impl LexicalMask {
    pub fn complete(code: Vec<bool>) -> Self {
        Self {
            code,
            error: None,
            marker_problem: None,
        }
    }

    pub fn malformed(mut code: Vec<bool>, start: usize, message: &'static str) -> Self {
        code[start..].fill(false);
        Self {
            code,
            error: Some(message),
            marker_problem: None,
        }
    }
}

