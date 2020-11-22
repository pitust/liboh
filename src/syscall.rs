global_asm!(
    "
__sys:
    push rcx
    push r11
    syscall
    pop r11
    pop rcx
    ret
"
);
extern "C" {
    fn __sys(sysno: i64, arg1: u64, arg2: u64) -> i64;
}
pub fn syscall(sysno: i64, arg1: u64, arg2: u64) -> i64 {
    unsafe { __sys(sysno, arg1, arg2) }
}

pub const SYS_EXIT: i64 = 0;
pub const SYS_BINDBUFFER: i64 = 1;
pub const SYS_GETBUFFERLEN: i64 = 2;
pub const SYS_READBUFFER: i64 = 3;
pub const SYS_SWAPBUFFERS: i64 = 4;
pub const SYS_SEND: i64 = 5;
pub const SYS_LISTEN: i64 = 6;
pub const SYS_ACCEPT: i64 = 7;
pub const SYS_EXEC: i64 = 8;
pub const SYS_RESPOND: i64 = 9;
pub const SYS_KLOG: i64 = 10;
pub const SYS_SBRK: i64 = 11;

pub fn sys_klog(s: &str) {
    syscall(SYS_KLOG, s.as_bytes().as_ptr() as u64, s.as_bytes().len() as u64);
}
pub fn sys_exit() -> ! {
    syscall(SYS_EXIT, 0, 0);
    sys_klog("[$pid] Exit failed; spinning exit attempts...");
    loop {
        syscall(SYS_EXIT, 0, 0);
    }
}
pub fn sys_bindbuffer(buf: &[u8]) {
    let ptr = buf.as_ptr();
    let len = buf.len();
    syscall(SYS_BINDBUFFER, ptr as u64, len as u64);
}
pub fn sys_getbufferlen() -> i64 {
    syscall(SYS_GETBUFFERLEN, 0, 0)
}
pub fn sys_readbuffer(to: &mut [u8]) {
    syscall(SYS_READBUFFER, to.as_ptr() as u64, 0);
}
pub fn sys_swapbuffers() {
    syscall(SYS_SWAPBUFFERS, 0, 0);
}
pub fn sys_send(tgd: &str) {
    syscall(SYS_SEND, tgd.as_bytes().as_ptr() as u64, tgd.len() as u64);
}
pub fn sys_listen(tgd: &str) {
    syscall(SYS_LISTEN, tgd.as_bytes().as_ptr() as u64, tgd.len() as u64);
}
pub fn sys_accept(tgd: &str) {
    syscall(SYS_ACCEPT, tgd.as_bytes().as_ptr() as u64, tgd.len() as u64);
}
pub fn sys_exec() {
    syscall(SYS_EXEC, 0, 0);
}
pub fn sys_respond() {
    syscall(SYS_RESPOND, 0, 0);
}

pub fn sys_sbrk(len: u64) -> u64 {
    syscall(SYS_SBRK, len, 0) as u64
}
