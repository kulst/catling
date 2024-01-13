use bitvec::array::BitArray;
use defmt::Format;
use device_driver::{implement_registers, AsyncRegisterDevice, Register};
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

pub mod enums;
pub mod registers;

pub struct Esc {}
