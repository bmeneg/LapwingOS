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

use core::ops::{Deref, DerefMut};

pub struct SingleThreadData<T>
where
    T: ?Sized,
{
    data: T,
}

impl<T> SingleThreadData<T> {
    pub const fn new(inner: T) -> Self {
        Self { data: inner }
    }
}

// Make SingleThreadData thread-safe to the compiler
unsafe impl<T> Send for SingleThreadData<T> {}
unsafe impl<T> Sync for SingleThreadData<T> {}

// Get the inner data when deferring SingleThreadData
impl<T> Deref for SingleThreadData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for SingleThreadData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
