use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct HeaderEntry {
  pub key: String,
  pub value: String,
}

#[napi(object)]
pub struct ParsedRequest {
  pub method: String,
  pub path: String,
  pub version: String,
  pub headers: Vec<HeaderEntry>,
}

#[napi]
pub fn parse_http(raw: Buffer) -> Result<ParsedRequest> {
  let raw_str = std::str::from_utf8(&raw).map_err(|e| Error::from_reason(e.to_string()))?;

  let mut lines = raw_str.lines();

  let request_line = lines.next().unwrap_or_default();
  let mut parts = request_line.split_whitespace();

  let method = parts.next().unwrap_or("").to_string();
  let path = parts.next().unwrap_or("").to_string();
  let version = parts.next().unwrap_or("").to_string();

  let mut headers = vec![];

  for line in lines {
    if line.is_empty() {
      break;
    }
    if let Some((k, v)) = line.split_once(":") {
      headers.push(HeaderEntry {
        key: k.trim().to_string(),
        value: v.trim().to_string(),
      });
    }
  }

  Ok(ParsedRequest {
    method,
    path,
    version,
    headers,
  })
}
