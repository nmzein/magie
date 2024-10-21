use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::time::{Duration, Instant};

pub async fn logging_middleware(mut req: Request<Body>, next: Next) -> impl IntoResponse {
    // Extract information from the request
    let method = req.method().clone();
    let uri = req.uri().clone();
    let path: String = uri.path().to_string();
    let query: Vec<(String, String)> = uri
        .query()
        .unwrap_or_default()
        .split('&') // Split by '&' to get individual key-value pairs
        .filter_map(|pair| {
            // Split each pair by '=' and collect them into (key, value)
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?.to_string(); // The key part
            let value = parts.next().unwrap_or_default().to_string(); // The value part (default to empty if missing)
            Some((key, value))
        })
        .collect();

    let logger = Logger::start(method, path, query);

    // Pass the request information to the next middleware/handler
    req.extensions_mut().insert(logger);

    // Call the next middleware/handler.
    let response = next.run(req).await;

    response
}

#[derive(Clone)]
pub struct Logger {
    logs: Vec<Log>,
    start: Instant,
    lap: Instant,
}

#[derive(Clone)]
pub enum Log {
    Started(Method, String, Vec<(String, String)>),
    Message(String, Duration),
    Report(Check, String, Duration),
    Error(Error, String, String, String),
    Completed(StatusCode),
}

#[derive(Clone, Debug)]
pub enum Check {
    RequestIntegrityCheck,
    ResourceConflictCheck,
}

#[derive(Clone, Debug)]
pub enum Error {
    RequestIntegrityError,
    ResourceConflictError,
    DatabaseQueryError,
    DatabaseInsertionError,
    DirectoryCreationError,
}

impl Logger {
    fn start(method: Method, path: String, query: Vec<(String, String)>) -> Self {
        Self {
            logs: vec![Log::Started(method, path, query)],
            start: Instant::now(),
            lap: Instant::now(),
        }
    }

    // Log a message.
    pub fn log(&mut self, message: &str) {
        self.logs
            .push(Log::Message(message.to_string(), self.lap.elapsed()));
        self.lap = Instant::now();
    }

    // Report that a check has passed.
    pub fn report(&mut self, check: Check, message: &str) {
        self.logs
            .push(Log::Report(check, message.to_string(), self.lap.elapsed()));
        self.lap = Instant::now();
    }

    // Handler returned with a success.
    pub fn success(&mut self, status_code: StatusCode, message: &str) -> Response<Body> {
        self.end(status_code);

        (status_code, message.to_string()).into_response()
    }

    // Handler returned with an error.
    pub fn error(
        &mut self,
        status_code: StatusCode,
        error: Error,
        id: &str,
        message: &str,
        details: Option<String>,
    ) -> Response<Body> {
        self.logs.push(Log::Error(
            error,
            id.to_string(),
            message.to_string(),
            details.unwrap_or_default(),
        ));
        self.end(status_code);

        (status_code, message.to_string()).into_response()
    }

    fn end(&mut self, status_code: StatusCode) {
        let duration = self.start.elapsed();
        self.logs.push(Log::Completed(status_code));

        for log in &self.logs {
            match log {
                Log::Started(method, path, query) => {
                    #[cfg(feature = "log.console")]
                    println!("\nStarted {} {}", method, path);

                    // Format request_info.query so that it is in the form {key: value}, {key: value}, ...
                    let query = query
                        .iter()
                        .map(|(key, value)| format!("{key}: {value}"))
                        .collect::<Vec<String>>()
                        .join(", ");

                    #[cfg(feature = "log.console")]
                    println!("  Params: {{{query}}}");
                }
                Log::Message(message, msg_duration) => {
                    #[cfg(feature = "log.console")]
                    println!("  LOG ({msg_duration:?}) {message}");
                }
                Log::Report(check, message, msg_duration) => {
                    #[cfg(feature = "log.console")]
                    println!("  CHECK {check:?} ({msg_duration:?})  PASSED. {message}");
                }
                Log::Error(error, id, message, details) => {
                    #[cfg(feature = "log.console")]
                    println!("  >>> {error:?} ({id})  {message}");
                    #[cfg(feature = "log.console")]
                    println!("     {details}");
                }
                Log::Completed(status_code) => {
                    #[cfg(feature = "log.console")]
                    println!("Completed {status_code} in {duration:?}");
                }
            }
        }
    }
}
