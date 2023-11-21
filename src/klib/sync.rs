// Some structures in the kernel are going to be accessed or used only in
// single thread/core scenarios, some will even be used with CPU interrupts
// disabled. One good example is the device driver manager, which has a
// single instance (static) during the entire kernel lifetime and is accessed
// at the start of the kernel to initialize builtin modules.
//
// Such static structures are forced to be thread-safe by the Rust compiler,
// which looks for the implementation of both Send and Sync traits to check
// its safety, however considering the scenario we mentioned earlier, no
// multiple threads/cores, we we're fine to make them "thread-safe".

use core::cell::UnsafeCell;

pub struct SafeStaticData<T>
where
    T: ?Sized,
{
    pub data: UnsafeCell<T>,
}

impl<T> SafeStaticData<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            data: UnsafeCell::new(inner),
        }
    }

    pub fn inner(&self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }

    pub fn set_inner(&self, inner: T) {
        unsafe { *self.data.get() = inner };
    }
}

// Make SafeStaticData thread-safe to the compiler
unsafe impl<T> Send for SafeStaticData<T> {}
unsafe impl<T> Sync for SafeStaticData<T> {}
