#[derive(Debug, Clone)]
pub struct ConverterConfig {
    pub max_direct_conversion_size: usize,

    pub strict_validation: bool,

    pub pretty_output: bool,

    pub encoding: String,

    pub conversion_timeout_ms: u64,
}

impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            max_direct_conversion_size: 10 * 1024 * 1024, // 10MB
            strict_validation: true,
            pretty_output: false,
            encoding: "UTF-8".to_string(),
            conversion_timeout_ms: 30_000, // 30 segundos
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonConverterConfig {
    pub base: ConverterConfig,
    
    pub allow_comments: bool,
    
    pub allow_trailing_commas: bool,
    
    pub indent_size: usize,
}

impl Default for JsonConverterConfig {
    fn default() -> Self {
        Self {
            base: ConverterConfig::default(),
            allow_comments: false,
            allow_trailing_commas: false,
            indent_size: 2,
        }
    }
}