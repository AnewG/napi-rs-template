#![deny(clippy::all)]

use napi_derive::napi;
use v_htmlescape::escape;

#[napi]
pub fn escape_html(input: String) -> String {
  escape(&input).to_string()
}

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}
