pub struct ExtensionResult {
    pub success: bool,
    pub value: Option<String>,
    pub error: Option<String>,
}

impl ExtensionResult {
    pub fn success(value: Option<String>) -> Self {
        Self {
            success: true,
            value,
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            value: None,
            error: Some(message),
        }
    }
}