use axum::{
    body::Body,
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct Logger<'a> {
    logs: Vec<Log<'a>>,
    start: Instant,
    lap: Instant,
}

#[derive(Clone)]
pub enum Log<'a> {
    Started {
        method: Method,
        path: String,
        query: String,
    },
    Message {
        message: &'a str,
        duration: Duration,
    },
    Report {
        check: Check,
        message: &'a str,
        duration: Duration,
    },
    Error {
        error: Error,
        id: &'a str,
        message: &'a str,
        details: Option<String>,
    },
    Completed {
        status_code: StatusCode,
    },
}

#[derive(Clone, Debug)]
pub enum Check {
    RequestIntegrity,
    ResourceConflict,
    ResourceExistence,
}

#[derive(Clone, Debug)]
pub enum Error {
    RequestIntegrity,
    ResponseIntegrity,

    // ResourceConflict,
    ResourceCreation,
    ResourceDeletion,
    ResourceExistence,
    ResourceMove,
    ResourceRead,

    DatabaseQuery,
    DatabaseInsertion,
    DatabaseDeletion,

    WebSocketParse,
    WebSocketSend,
}

impl<'a> Logger<'a> {
    pub fn start(method: Method, path: String, query: String) -> Self {
        let time = Instant::now();

        Self {
            logs: vec![Log::Started {
                method,
                path,
                query,
            }],
            start: time,
            lap: time,
        }
    }

    // Log a message.
    pub fn log(&mut self, message: &'a str) {
        self.logs.push(Log::Message {
            message,
            duration: self.lap.elapsed(),
        });
        self.lap = Instant::now();
    }

    // Report that a check has passed.
    pub fn report(&mut self, check: Check, message: &'a str) {
        self.logs.push(Log::Report {
            check,
            message,
            duration: self.lap.elapsed(),
        });
        self.lap = Instant::now();
    }

    // Handler returned with a success.
    pub fn success(&mut self, status_code: StatusCode, message: &'a str) -> Response<Body> {
        self.end(status_code);

        (status_code, message.to_string()).into_response()
    }

    // Handler returned with an error.
    pub fn error(
        &mut self,
        status_code: StatusCode,
        error: Error,
        id: &'a str,
        message: &'a str,
        details: Option<anyhow::Error>,
    ) -> Response<Body> {
        self.logs.push(Log::Error {
            error: error.clone(),
            id,
            message,
            details: details.map(|x| x.to_string()),
        });

        // Only end on error if not websocket related.
        match error {
            Error::WebSocketSend => {}
            Error::WebSocketParse => {}
            _ => self.end(status_code),
        }

        (status_code, message.to_string()).into_response()
    }

    fn end(&mut self, status_code: StatusCode) {
        let total_duration = self.start.elapsed();
        self.logs.push(Log::Completed { status_code });

        for log in &self.logs {
            match log {
                Log::Started {
                    method,
                    path,
                    query,
                } => {
                    #[cfg(feature = "log.console")]
                    println!("\nStarted {method} {path}");

                    #[cfg(feature = "log.console")]
                    if !query.is_empty() {
                        // Format query so that it is in the form {key: value}, {key: value}, ...
                        let params: HashMap<_, _> = query
                            .split('&')
                            .filter_map(|pair| {
                                let mut parts = pair.split('=');
                                Some((parts.next()?, parts.next()?))
                            })
                            .collect();

                        let formatted_params = params
                            .iter()
                            .map(|(key, value)| format!("{key}: {value}"))
                            .collect::<Vec<String>>()
                            .join(", ");

                        println!("  Params: {{{formatted_params}}}");
                    }
                }
                Log::Message { message, duration } => {
                    #[cfg(feature = "log.console")]
                    println!("  LOG ({duration:?}) {message}");
                }
                Log::Report {
                    check,
                    message,
                    duration,
                } => {
                    #[cfg(feature = "log.console")]
                    println!("  CHECK {check:?} ({duration:?})  PASSED. {message}");
                }
                Log::Error {
                    error,
                    id,
                    message,
                    details,
                } => {
                    #[cfg(feature = "log.console")]
                    println!("  >>> ERROR {error:?} ({id})  {message}");
                    if let Some(details) = details {
                        #[cfg(feature = "log.console")]
                        println!("            {details}");
                    }
                }
                Log::Completed { status_code } => {
                    #[cfg(feature = "log.console")]
                    println!("Completed {status_code} in {total_duration:?}");
                }
            }
        }
    }
}
