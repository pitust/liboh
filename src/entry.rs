use core::panic::PanicInfo;
use core::fmt::Write;

extern {
    fn __main() -> ();
}

global_asm!("
.intel_syntax noprefix
.align 8
stack_top:
    .space 0x400, 0x00
stack_bottom:
.att_syntax
");

#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    asm!(".intel_syntax noprefix
          lea rsp, [rip + stack_bottom]
          push rax
          mov rdi, 11
          mov rsi, 0x10000
          mov r8, 0
          push rcx
          push r11
          syscall
          pop r11
          pop rcx
          pop rbx
          mov rsp, rax
          push rbx
          pop rax
          .att_syntax");


    _liboh_entry();
}

#[macro_export]
macro_rules! main {
    ($fn: ident) => {
        #[no_mangle] extern "C" fn __main() -> () { $fn(); }
    };
}

#[no_mangle]
extern "C" fn _liboh_entry() -> ! {
    crate::alloction::init();
    unsafe { __main(); }
    crate::syscall::sys_exit();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write!(crate::klog::KLog, "{}", info).unwrap();
    loop {}
}
