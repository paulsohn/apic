#![no_std]
#![feature(nonzero_ops)]
#![feature(strict_provenance)]

use core::ptr::NonNull;
use volatile::VolatilePtr;
use volatile::access::{Access, ReadOnly, ReadWrite};

pub mod io_apic;
pub mod registers;

pub struct ApicBase {
    pub base_addr: NonNull<u8>,
}

impl ApicBase {
    /// base address must have 'static lifetime
    pub const unsafe fn new(base_addr: NonNull<u8>) -> Self {
        Self {
            base_addr,
        }
    }

    pub fn id<'a>(&'a self) -> VolatilePtr<'a, registers::Id> {
        unsafe { self.offset(Offset::Id, ReadWrite) }
    }

    pub fn version<'a>(&'a self) -> VolatilePtr<'a, registers::Version, ReadOnly> {
        unsafe { self.offset(Offset::Version, ReadOnly) }
    }

    pub fn extended_apic_feature<'a>(&'a self) -> VolatilePtr<'a, registers::ExtendedApicFeature> {
        unsafe { self.offset(Offset::ExtendedApicFeature, ReadWrite) }
    }

    pub fn extended_apic_control<'a>(&'a self) -> VolatilePtr<'a, registers::ExtendedApicControl> {
        unsafe { self.offset(Offset::ExtendedApicControl, ReadWrite) }
    }

    pub fn spurious_interrupt_vector<'a>(
        &'a self,
    ) -> VolatilePtr<'a, registers::SpuriousInterruptVector> {
        unsafe { self.offset(Offset::SpuriousInterruptVector, ReadWrite) }
    }

    pub fn timer_local_vector_table_entry<'a>(
        &'a self,
    ) -> VolatilePtr<'a, registers::TimerLocalVectorTableEntry> {
        unsafe { self.offset(Offset::TimerLocalVectorTableEntry, ReadWrite) }
    }

    pub fn timer_initial_count<'a>(&'a self) -> VolatilePtr<'a, registers::TimerInitialCount> {
        unsafe { self.offset(Offset::TimerInitialCount, ReadWrite) }
    }

    pub fn timer_divide_configuration<'a>(
        &'a self,
    ) -> VolatilePtr<'a, registers::TimerDivideConfiguration> {
        unsafe { self.offset(Offset::TimerDivideConfiguration, ReadWrite) }
    }

    pub fn end_of_interrupt<'a>(&self) -> &'static mut registers::EndOfInterrupt {
        unsafe {
            self.base_addr
                .map_addr(|addr| {
                    addr.unchecked_add(Offset::EndOfInterrupt as usize)
                })
                .cast::<registers::EndOfInterrupt>()
                .as_mut()
        }
    }

    unsafe fn offset<'a, T, A>(&self, offset: Offset, access: A) -> VolatilePtr<'a, T, A>
    where A: Access
    {
        VolatilePtr::new_restricted(
            access,
            self.base_addr
                .map_addr(|addr| {
                    addr.unchecked_add(offset as usize)
                })
                .cast()
        )
    }
}

#[repr(usize)]
pub enum Offset {
    Id = 0x20,
    Version = 0x30,
    TaskPriority = 0x80,
    ArbitrationPriority = 0x90,
    ProcessorPriority = 0xa0,
    EndOfInterrupt = 0xb0,
    RemoteRead = 0xc0,
    LocalDestination = 0xd0,
    DestinationFormat = 0xe0,
    SpuriousInterruptVector = 0xf0,
    InService = 0x100,
    TriggerMode = 0x180,
    InterruptRequest = 0x200,
    ErrorStatus = 0x280,
    InterruptCommand = 0x300,
    TimerLocalVectorTableEntry = 0x320,
    ThermalLocalVectorTableEntry = 0x330,
    PerformanceCounterLocalVectorTableEntry = 0x340,
    LocalInterrupt0VectorTableEntry = 0x350,
    LocalInterrupt1VectorTableEntry = 0x360,
    ErrorVectorTableEntry = 0x370,
    TimerInitialCount = 0x380,
    TimerCurrentCount = 0x390,
    TimerDivideConfiguration = 0x3e0,
    ExtendedApicFeature = 0x400,
    ExtendedApicControl = 0x410,
    SpecificEndOfInterrupt = 0x420,
    InterruptEnable = 0x480,
    ExtendedInterruptLocalVectorTable = 0x500,
}
