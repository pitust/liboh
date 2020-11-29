#![no_std]
#![feature(global_asm)]
#![feature(alloc_prelude)]
#![feature(asm)]
#![feature(naked_functions)]
extern crate alloc;

// todo: init allocation and stuff
mod entry;
// done
pub mod syscall;
// done
pub mod prelude;
// todo
mod alloction;
pub mod mutex;
pub mod klog {
    pub struct KLog;
    impl core::fmt::Write for KLog {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            crate::syscall::sys_klog(s);
            Ok(())
        }
    }
}
