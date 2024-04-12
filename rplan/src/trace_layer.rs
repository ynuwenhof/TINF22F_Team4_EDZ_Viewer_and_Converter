use axum::http::header::USER_AGENT;
use axum::http::Request;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use tower_http::trace;
use tracing::{error_span, Span};

#[derive(Clone)]
pub struct MakeSpan;

impl<T> trace::MakeSpan<T> for MakeSpan {
    fn make_span(&mut self, req: &Request<T>) -> Span {
        match req.headers().get(USER_AGENT).and_then(|h| h.to_str().ok()) {
            None => {
                error_span!(
                    "request",
                    method = %req.method(),
                    uri = %req.uri(),
                    version = ?req.version(),
                )
            }
            Some(user_agent) => {
                error_span!(
                    "request",
                    method = %req.method(),
                    uri = %req.uri(),
                    version = ?req.version(),
                    %user_agent,
                )
            }
        }
    }
}

#[derive(Clone)]
pub struct OnFailure;

impl<T> trace::OnFailure<T> for OnFailure
where
    T: Display,
{
    fn on_failure(&mut self, _failure_classification: T, _latency: Duration, _span: &Span) {}
}

pub struct Elapsed(pub Duration);

impl Display for Elapsed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}ms", self.0.as_millis())
    }
}
