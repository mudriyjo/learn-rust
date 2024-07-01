use std::alloc::{self, dealloc, Layout};

/// # Safety
/// 
/// This function is unsafe because.....
unsafe fn doing_somethin_unsafe() {

}

fn allocate_memory_with_rust() {
    unsafe {
        // Allocate memory with Rust. it's safer to force alignment
        let layout = Layout::new::<u16>();
        let ptr = alloc::alloc(layout);

        //Set the allocated vatiable - dereference pointer and set to 42 
        *ptr = 42;
        assert_eq!(*ptr, 42);

        // Deallocate memory
        dealloc(ptr, layout);
    }
}

fn main() {
    allocate_memory_with_rust();
}
