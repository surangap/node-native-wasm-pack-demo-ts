#![allow(non_snake_case)]
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub struct Resp {
  retString: String
}

#[wasm_bindgen]
impl Resp {
  pub fn new(val: String) -> Resp {
    Resp { retString: val}
  }
  pub fn getString(&self) -> String {
    return self.retString.clone();
  }

  pub fn setString(&mut self, val: String) {
    self.retString = val;
  }
}

#[wasm_bindgen]
pub fn sayHelloWithObject(s: &str) -> Resp {
  let r = String::from("hello ");
  return Resp{retString: r+s};
}

#[wasm_bindgen]
pub fn sayHelloWithString(s: &str) -> String {
  let r = String::from("hello ");
  return r+s;
}

