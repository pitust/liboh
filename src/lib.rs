#![no_std]
#![feature(global_asm)]
#![feature(alloc_prelude)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(unboxed_closures)]

extern crate alloc;

// todo: init allocation and stuff
mod entry;
// done
pub mod syscall;
// done
pub mod prelude;
// todo: specific services
// done: requesting
pub mod service;
// todo
// pub mod mux;
// done
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

#[macro_export]
macro_rules! println {
    ($($tail:tt)*) => { writeln!(liboh::klog::KLog, $($tail)*).unwrap(); }
}

pub fn exec(p: alloc::vec::Vec<alloc::string::String>) {
    let tmp = alloc::format!("/bin/{}", p[0]);
    let bin = tmp.as_bytes();
    // write in the argvblob
    service::write_buf(p);
    // swap 'em
    syscall::sys_swapbuffers(); 
    // and write the kernel blob (this is a little awkward but meh)
    service::write_raw(bin);
    // now we can sys_exec
    syscall::sys_exec();
    // and done
}