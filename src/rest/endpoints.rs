#[derive(Clone)]
pub enum SumsubEndpoint {
    AccessTokens,
    Applicants,
}

impl From<SumsubEndpoint> for String {
    fn from(item: SumsubEndpoint) -> Self {
        String::from(match item {
            SumsubEndpoint::AccessTokens => "/resources/accessTokens",
            SumsubEndpoint::Applicants => "/resources/applicants",        })
    }
}
