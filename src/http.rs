use crate::{error::Result, jobs};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::time::Duration;
use ureq::OrAnyStatus;

// ----------------------------------------------------------------------------
// Interface

#[derive(Deserialize)]
struct RequestOptions {
    #[serde(default)]
    output_filename: Option<String>,
    #[serde(default)]
    body_filename: Option<String>,
}

#[derive(Serialize)]
struct Response<'a> {
    status_code: u16,
    headers: HashMap<String, String>,
    body: Option<&'a str>,
}

// If the response can be deserialized -> success.
// If the response can't be deserialized -> failure.
byond_fn!(fn http_request_blocking(method, url, body, headers, options) {
    let req = match construct_request(method, url, body, headers, options) {
        Ok(r) => r,
        Err(e) => return Some(e.to_string())
    };

    match submit_request(req) {
        Ok(r) => Some(r),
        Err(e) => Some(e.to_string())
    }
});

// Returns new job-id.
byond_fn!(fn http_request_async(method, url, body, headers, options) {
    let req = match construct_request(method, url, body, headers, options) {
        Ok(r) => r,
        Err(e) => return Some(e.to_string())
    };

    Some(jobs::start(move || {
        match submit_request(req) {
            Ok(r) => r,
            Err(e) => e.to_string()
        }
    }))
});

// If the response can be deserialized -> success.
// If the response can't be deserialized -> failure or WIP.
byond_fn!(fn http_check_request(id) {
    Some(jobs::check(id))
});

// ----------------------------------------------------------------------------
// Shared HTTP client state

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

thread_local! {
    pub static HTTP_CLIENT: RefCell<Option<ureq::Agent>> = RefCell::new(Some(ureq::agent()));
}

// ----------------------------------------------------------------------------
// Request construction and execution

struct RequestPrep {
    req: ureq::Request,
    output_filename: Option<String>,
    body: Vec<u8>,
}

fn construct_request(
    method: &str,
    url: &str,
    body: &str,
    headers: &str,
    options: &str,
) -> Result<RequestPrep> {
    HTTP_CLIENT.with(|cell| {
        let borrow = cell.borrow_mut();
        match &*borrow {
            Some(client) => {
                let mut req = match method {
                    "post" => client.post(url),
                    "put" => client.put(url),
                    "patch" => client.patch(url),
                    "delete" => client.delete(url),
                    "head" => client.head(url),
                    _ => client.get(url),
                }
                .set("User-Agent", &format!("{PKG_NAME}/{VERSION}"))
                .timeout(Duration::from_secs(30));

                let mut final_body = body.as_bytes().to_vec();

                if !headers.is_empty() {
                    let headers: BTreeMap<&str, &str> = serde_json::from_str(headers)?;
                    for (key, value) in headers {
                        req = req.set(key, value);
                    }
                }

                let mut output_filename = None;
                if !options.is_empty() {
                    let options: RequestOptions = serde_json::from_str(options)?;
                    output_filename = options.output_filename;
                    if let Some(fname) = options.body_filename {
                        final_body = std::fs::read(fname)?;
                    }
                }

                Ok(RequestPrep {
                    req,
                    output_filename,
                    body: final_body,
                })
            }

            // If we got here we royally fucked up
            None => {
                let client = ureq::agent();
                let req = client.get("");
                let output_filename = None;
                Ok(RequestPrep {
                    req,
                    output_filename,
                    body: Vec::new(),
                })
            }
        }
    })
}

fn submit_request(prep: RequestPrep) -> Result<String> {
    let response = prep.req.send_bytes(&prep.body).or_any_status()?;

    let body;
    let mut resp = Response {
        status_code: response.status(),
        headers: HashMap::new(),
        body: None,
    };

    for key in response.headers_names() {
        let Some(value) = response.header(&key) else {
            continue;
        };

        resp.headers.insert(key, value.to_owned());
    }

    if let Some(output_filename) = prep.output_filename {
        let mut writer = std::io::BufWriter::new(std::fs::File::create(output_filename)?);
        std::io::copy(&mut response.into_reader(), &mut writer)?;
        writer.flush()?;
    } else {
        body = response.into_string()?;
        resp.body = Some(&body);
    }

    Ok(serde_json::to_string(&resp)?)
}

byond_fn!(
    fn start_http_client() {
        HTTP_CLIENT.with(|cell| cell.replace(Some(ureq::agent())));
        Some("")
    }
);

byond_fn!(
    fn shutdown_http_client() {
        HTTP_CLIENT.with(|cell| cell.replace(None));
        Some("")
    }
);
