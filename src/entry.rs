use core::panic::PanicInfo;
use core::fmt::Write;

extern {
    fn __main() -> ();
}

#[macro_export]
macro_rules! main {
    ($fn: ident) => {
        #[no_mangle] extern "C" fn __main() -> () { $fn(); }
    };
}

#[no_mangle]
extern "C" fn _start() -> ! {
    crate::alloction::init();
    unsafe { __main(); }
    crate::syscall::sys_exit();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write!(crate::klog::KLog, "{}", info);
    loop {}
}
