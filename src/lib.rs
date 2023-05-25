// #![no_std]
// use core::panic::PanicInfo;
//
// #[panic_handler]
// pub fn panicker(info: &PanicInfo) -> ! {
//     {}
// }

#[no_mangle]
pub extern "C" fn rust_fn1() -> bool {
    return true;
}
