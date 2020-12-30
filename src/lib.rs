use std::env;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::c_char;
use std::panic;

extern crate base64;
extern crate serde_json;
extern crate ureq;
use serde_json::from_slice;
use std::collections::HashMap;

fn cs(s: Vec<u8>) -> *const c_char {
  let c_str = CString::new(s).unwrap();
  let ptr = c_str.as_ptr();
  std::mem::forget(c_str);
  return ptr;
}

#[no_mangle]
pub extern "C" fn get(c: *const c_char) -> *const c_char {
  let nak: Vec<u8> = vec![21];
  panic::set_hook(Box::new(move |_| eprintln!("panic: fastkapow.get()")));
  let d = match env::var("KAPOW_DATA_URL") {
    Ok(d) => d,
    Err(_) => return cs(nak),
  };
  let i = match env::var("KAPOW_HANDLER_ID") {
    Ok(i) => i,
    Err(_) => return cs(nak),
  };
  let cb = unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() };
  let req = format!("{}/handlers/{}{}", d, i, cb);
  let get = ureq::get(&req).call();
  let mut bytes = vec![];
  if get.ok() {
    let mut reader = get.into_reader();
    let _ = reader.read_to_end(&mut bytes);
  } else {
    bytes = nak;
  }
  return cs(bytes);
}

#[no_mangle]
pub extern "C" fn b64_get(c: *const c_char) -> *const c_char {
  let nak: Vec<u8> = vec![21];
  panic::set_hook(Box::new(move |_| eprintln!("panic: fastkapow.get()")));
  let d = match env::var("KAPOW_DATA_URL") {
    Ok(d) => d,
    Err(_) => return cs(nak),
  };
  let i = match env::var("KAPOW_HANDLER_ID") {
    Ok(i) => i,
    Err(_) => return cs(nak),
  };
  let cb = unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() };
  let req = format!("{}/handlers/{}{}", d, i, cb);
  let get = ureq::get(&req).call();
  let mut strings = vec![];
  if get.ok() {
    let mut reader = get.into_reader();
    let _ = reader.read_to_end(&mut strings);
    return cs(base64::encode(strings).as_bytes().to_vec());
  }
  return cs(nak);
}

#[no_mangle]
pub extern "C" fn set(c: *const c_char) -> *const c_char {
  let ack: Vec<u8> = vec![6];
  let nak: Vec<u8> = vec![21];
  panic::set_hook(Box::new(move |_| eprintln!("panic: fastkapow.set()")));
  let d = match env::var("KAPOW_DATA_URL") {
    Ok(d) => d,
    Err(_) => return cs(nak),
  };
  let i = match env::var("KAPOW_HANDLER_ID") {
    Ok(i) => i,
    Err(_) => return cs(nak),
  };
  let cb = unsafe { CStr::from_ptr(c).to_bytes() };
  let v: HashMap<String, String> = from_slice(cb).unwrap();
  let req = format!("{}/handlers/{}{}", d, i, v["resource"]);
  let put = ureq::put(&req).send_string(&v["data"]);
  if put.ok() {
    return cs(ack);
  } else {
    return cs(nak);
  }
}
