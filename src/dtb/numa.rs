use core::slice;

use fdt::Fdt;
use rmm::{FrameCount, PhysicalAddress};

use crate::{
    cpu_set::MAX_CPU_COUNT,
    numa::{self, NumaMemory},
};

pub fn init(fdt: &Fdt) {
    // todo
}
