#![no_std]
#![feature(start)]
#![feature(panic_info_message)]

#[macro_use]
mod print;
mod write_to;
mod factorial;

use factorial::*;
use print::println;
use core::panic::PanicInfo;
use volatile_register::RO;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  if let Some(args) = info.message() {
    print::write_args(args);
  }
  loop {}
}

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
pub extern "C" fn size() -> u16 {
  core::mem::size_of::<usize>() as u16
}



#[start]
fn main(_argc: isize, _args: *const *const u8) -> isize {
  println!("Hello from Rust! {} {:?}", "foo", 42u8);
  let f1 = factorial(20);
  let f2 = factorial2(20);
  assert!(f1 == f2);
  println!("foo {} 20! is {}", "bar", f1);
  loop {}
}
