#[derive(Clone, Debug)]
pub struct SumsubConfig {
    pub rest_api_host: String,
}

impl Default for SumsubConfig {
    fn default() -> Self {
        Self {
            rest_api_host: "https://api.sumsub.com".into(),
        }
    }
}

impl SumsubConfig {
    pub fn test_env() -> Self {
        Self {
            rest_api_host: "https://api.sumsub.com".into(),
        }
    }
}
