use error_chain::error_chain;
use serde::Deserialize;
use std::collections::HashMap;

// #[derive(Debug, Deserialize)]
// pub struct SumsubContentError {
//     pub errors: HashMap<String, String>,
// }

#[derive(Default, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SumsubContentError {
    //#[serde(rename = "description")]
    pub description: String,
    //#[serde(rename = "code")]
    pub code: i64,
    //#[serde(rename = "correlationId")]
    pub correlation_id: String,
    //#[serde(rename = "errorCode")]
    pub error_code: i64,
    //#[serde(rename = "errorName")]
    pub error_name: String,
}

error_chain! {
    errors {
        SumsubError(response: SumsubContentError)
    }
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        ReqError(reqwest::Error);
        InvalidHeaderError(reqwest::header::InvalidHeaderValue);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        UrlParserError(url::ParseError);
        Json(serde_json::Error);
        TimestampError(std::time::SystemTimeError);
    }
}
