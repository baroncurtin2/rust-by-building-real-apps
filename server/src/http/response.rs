use super::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            f,
            "HTTP/1.1 {status_code} {reason}\r\n\r\n{body}",
            status_code = self.status_code,
            reason = self.status_code.reason_phrase(),
            body = body
        )
    }
}