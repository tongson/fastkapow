use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io::Read;
use std::env;
use std::panic;

extern crate ureq;

fn cs(s: Vec<u8>) -> *const c_char {
  let c_str = CString::new(s).unwrap();
  let ptr = c_str.as_ptr();
  std::mem::forget(c_str);
  return ptr
}

#[no_mangle]
pub extern "C" fn get(c: *const c_char) -> *const c_char {
  panic::set_hook(Box::new(move |_| eprintln!("panic: fkapow.get()")));
  let d = match env::var("KAPOW_DATA_URL") {
    Ok(d) => d,
    Err(_) => return cs(b"\x04".to_vec()),
  };
  let i = match env::var("KAPOW_HANDLER_ID") {
    Ok(i) => i,
    Err(_) => return cs(b"\x04".to_vec()),
  };
  let cb = unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() };
  let req = format!("{}/handlers/{}{}", d, i, cb);
  let get = ureq::get(&req).call();
  // Process response
  let mut bytes = vec![];
  if get.status().to_string() == "200" {
    let mut reader = get.into_reader();
    let _ = reader.read_to_end(&mut bytes);
  } else {
    bytes = b"\x04".to_vec();
  }
  return cs(bytes)
}

