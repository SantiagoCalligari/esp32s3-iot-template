#![no_std]
#![deny(clippy::mem_forget, clippy::large_stack_frames)]
#![feature(impl_trait_in_assoc_type)]

pub mod dhcp;
pub mod http;
pub mod utils;
pub mod wifi;

// Re-exportar cosas comunes
pub use embassy_executor::Spawner;
pub use embassy_time::Timer;
pub use rtt_target::rprintln;
