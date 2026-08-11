use crate::PhysicalAddress;

pub use self::{buddy::*, bump::*};

mod buddy;
mod bump;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct FrameCount(usize);

impl FrameCount {
    pub fn new(count: usize) -> Self {
        Self(count)
    }

    pub fn data(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct FrameUsage {
    used: FrameCount,
    total: FrameCount,
}

impl FrameUsage {
    pub fn new(used: FrameCount, total: FrameCount) -> Self {
        Self { used, total }
    }

    pub fn used(&self) -> FrameCount {
        self.used
    }

    pub fn free(&self) -> FrameCount {
        FrameCount(self.total.0 - self.used.0)
    }

    pub fn total(&self) -> FrameCount {
        self.total
    }
}

pub trait FrameAllocatorOutput {
    fn physical_address(&self) -> PhysicalAddress;
}

pub unsafe trait FrameAllocator {
    type O: FrameAllocatorOutput;

    fn allocate(&mut self, count: FrameCount) -> Option<Self::O>;

    unsafe fn free(&mut self, address: PhysicalAddress, count: FrameCount);

    fn allocate_one(&mut self) -> Option<Self::O> {
        self.allocate(FrameCount::new(1))
    }

    unsafe fn free_one(&mut self, address: PhysicalAddress) {
        unsafe {
            self.free(address, FrameCount::new(1));
        }
    }

    fn usage(&self) -> FrameUsage;
}

unsafe impl<T, V: FrameAllocatorOutput> FrameAllocator for &mut T
where
    T: FrameAllocator<O = V>,
{
    type O = V;

    fn allocate(&mut self, count: FrameCount) -> Option<Self::O> {
        T::allocate(self, count)
    }
    unsafe fn free(&mut self, address: PhysicalAddress, count: FrameCount) {
        unsafe { T::free(self, address, count) }
    }
    fn allocate_one(&mut self) -> Option<Self::O> {
        T::allocate_one(self)
    }
    unsafe fn free_one(&mut self, address: PhysicalAddress) {
        unsafe { T::free_one(self, address) }
    }
    fn usage(&self) -> FrameUsage {
        T::usage(self)
    }
}
