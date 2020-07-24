#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#![feature(alloc)]
extern crate alloc;

extern crate uefi;

extern crate uefi_services;
extern crate compiler_builtins;


use uefi::prelude::*;

use crate::alloc::vec::Vec;
use uefi::table::boot::MemoryType;
use log::info;

const EFI_PAGE_SIZE: u64 = 0x1000;

fn memory_map(bt: &BootServices) {
    let map_size = bt.memory_map_size() * 2;
    info!("map_size: {}", map_size);

    let mut buffer = Vec::with_capacity(map_size);
    unsafe {
        buffer.set_len(map_size);
    }
    let (_k, mut desc_iter) = bt
        .memory_map(&mut buffer)
        .expect("Failed to retrive UEFI memory map")
        .expect("Operation is incomplete");

    assert!(desc_iter.len() > 0, "Memory map is empty");

    info!("efi: usable memory ranges ({} total)", desc_iter.len());
    for (j, descriptor) in desc_iter.enumerate() {
        match descriptor.ty {
            MemoryType::CONVENTIONAL => {
                let size = descriptor.page_count * EFI_PAGE_SIZE;
                let end_address = descriptor.phys_start + size;
                info!("> {:#x} - {:#x} ({} KB)", descriptor.phys_start, end_address, size)
            }
            _ => {},
        }
    }
}

#[entry]
 fn efi_main(_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    // Initialize logging.
    uefi_services::init(&system_table);

// Print out the UEFI revision number
    {
        let rev = system_table.uefi_revision();
        let (major, minor) = (rev.major(), rev.minor());

        info!("UEFI {}.{}", major, minor);

        memory_map(&system_table.boot_services());
    }
    loop {}
}
