#[derive(Clone)]
pub enum SumsubEndpoint {
    AccessToken,
    ApplicantData,
}

impl From<SumsubEndpoint> for String {
    fn from(item: SumsubEndpoint) -> Self {
        String::from(match item {
            SumsubEndpoint::AccessToken => "/resources/accessTokens",
            SumsubEndpoint::ApplicantData => "/resources/applicants",
            //SumsubEndpoint::ApplicantData => "/resources/applicants/{applicantId}/one",
        })
    }
}
