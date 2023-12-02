mod j2;
mod j1;


use std::alloc::System;

#[allow(unused)]
use tracking_allocator::{
    AllocationGroupId, AllocationRegistry, AllocationTracker, Allocator,
};

#[global_allocator]
static GLOBAL: Allocator<System> = tracking_allocator::Allocator::system();

// #[global_allocator]
// static GLOBAL: Allocator<System> = Allocator::system();

struct StdoutTracker;

impl AllocationTracker for StdoutTracker {
    fn allocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        group_id: AllocationGroupId,
    ) {
        println!(
            "allocation -> addr=0x{:0x} object_size={} wrapped_size={} group_id={:?}",
            addr, object_size, wrapped_size, group_id
        );
    }

    fn deallocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        source_group_id: AllocationGroupId,
        current_group_id: AllocationGroupId,
    ) {
        println!(
            "deallocation -> addr=0x{:0x} object_size={} wrapped_size={} source_group_id={:?} current_group_id={:?}",
            addr, object_size, wrapped_size, source_group_id, current_group_id
        );
    }
}

fn main() {
    AllocationRegistry::set_global_tracker(StdoutTracker)
        .expect("no other global tracker should be set yet");

    println!("STARTING DAYS COMPUTATIONS");

    AllocationRegistry::enable_tracking();
    println!("J1 -----------------------------------------------------");
    println!("p1");
    println!("{}", j1::p1());
    println!("p2");
    println!("{}", j1::p2());
    println!("J2 -----------------------------------------------------");
    println!("p1");
    println!("{}", j2::p1());
    println!("p2");
    println!("{}", j2::p2());
    AllocationRegistry::disable_tracking();
}