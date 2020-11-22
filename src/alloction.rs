use core::ptr::NonNull;

use linked_list_allocator::LockedHeap;

// #[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[global_allocator]
static WRAP_ALLOC: WrapAlloc = WrapAlloc {};
struct WrapAlloc {}
unsafe impl alloc::alloc::GlobalAlloc for WrapAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        while match ALLOCATOR.lock().allocate_first_fit(layout) {
            Ok(o) => {
                return o.as_ptr();
            }
            Err(e) => { true }
        } {
            extend(layout.size());
        }
        unreachable!();
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        ALLOCATOR.lock().deallocate(NonNull::new(ptr).unwrap(), layout);
    }
}

pub(crate) fn init() {
    unsafe {
        ALLOCATOR
            .lock()
            .init((crate::syscall::sys_sbrk(4096) - 4096) as usize, 4096);
    }
}
fn extend(sz: usize) {
    crate::syscall::sys_sbrk(sz as u64);
    unsafe { ALLOCATOR.lock().extend(sz) }
}
