use core::panic::PanicInfo;
use core::fmt::Write;

extern {
    fn __main() -> ();
}

global_asm!("
_start:
    lea rsp, [stack_bottom]
    mov rdi, 11 // sys_sbrk
    mov rsi, 0x10000 // len
    mov r8, QWORD 0
    syscall
    mov rsp, QWORD rax
    jmp _liboh_entry
.align 8
stack_top:
    .space 0x400, 0x00
stack_bottom:
");

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
    write!(crate::klog::KLog, "{}", info);
    loop {}
}
