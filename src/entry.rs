use core::panic::PanicInfo;
use core::fmt::Write;

extern {
    fn __main() -> ();
}

global_asm!("
_start:
    mov rsp, stack_bottom
    mov rdi, 11 ; sys_sbrk
    mov rsi, 0x10000 ; len
    mov r8, 0
    syscall
    mov rsp, rax
    jmp _liboh_entry
    do_syscall sys_klog, rax, 0
    jmp $
align 8
stack_top:
    resb 4096
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
