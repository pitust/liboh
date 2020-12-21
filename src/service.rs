use core::alloc::Layout;

pub fn request<'a, T: serde::Serialize, U: serde::Deserialize<'a> + Clone>(to: &str, t: T) -> U {
    write_buf(t);
    crate::syscall::sys_send(to);
    read_buf()
}

pub fn read_buf<'a, U: serde::Deserialize<'a> + Clone>() -> U {
    let l = crate::syscall::sys_getbufferlen();
    let a = unsafe { alloc::alloc::alloc(Layout::from_size_align(l as usize, 8).unwrap()) };
    let slc = unsafe { core::slice::from_raw_parts_mut(a, l as usize) };
    crate::syscall::sys_readbuffer(slc);
    let x: U = postcard::from_bytes::<'a, U>(slc).unwrap().clone();
    unsafe { alloc::alloc::dealloc(a, Layout::from_size_align(l as usize, 8).unwrap()) }
    x
}

pub fn write_buf<T: serde::Serialize>(t: T) {
    let d = postcard::to_allocvec(&t).unwrap();
    crate::syscall::sys_bindbuffer(d.as_slice());
}

pub fn write_raw(d: &[u8]) {
    crate::syscall::sys_bindbuffer(d);
}