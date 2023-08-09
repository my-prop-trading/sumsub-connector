#[derive(Clone)]
pub enum SumsubEndpoint {
    AccessToken,
}

impl From<SumsubEndpoint> for String {
    fn from(item: SumsubEndpoint) -> Self {
        String::from(match item {
            SumsubEndpoint::AccessToken => "/resources/accessTokens",
        })
    }
}
