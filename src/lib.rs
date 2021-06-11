#![no_std]
mod write_to;
use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  loop {}
}


extern "C" {
  fn __putchar(c: u8);
}

use volatile_register::{RW, RO};


#[repr(C)]
pub struct Timer {
  pub t0: RO<u8>,
}


pub fn get_timer() -> &'static mut RO<u8> {
  unsafe {
    &mut *(20 as *mut RO<u8>)
  }
}


#[no_mangle]
pub extern "C" fn get_tick() -> u8 {
  let t = get_timer();
  t.read()
}



#[no_mangle]
pub extern "C" fn factorial(n: u32) -> u32 {
  match n {
    0 => 1,
    _ => n * factorial(n-1),
  }
}


#[no_mangle]
pub extern "C" fn factorial2(n: u32) -> u32 {
  (1..n+1).fold(1, |acc, v| acc * v)
}


#[no_mangle]
pub extern "C" fn size() -> u16 {
  core::mem::size_of::<usize>() as u16
}

const HELLO: &str = "Hello from Rust!\n";

pub fn write(text: &str) {
  text.bytes().for_each(|b| unsafe{__putchar(b)});
}

#[no_mangle]
pub extern "C" fn run() {
  let mut buf = [0u8; 64];
  // let mut dest = [0u8; 64];
  // dest[..].copy_from_slice(&buf);

  let s: &str = write_to::show(&mut buf, format_args!("foo, {} {} {} {}", "bar", 42, factorial(12), factorial2(12))).unwrap();
  write(s);
  loop {}
}
