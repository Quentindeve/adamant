use stivale_boot::v2::StivaleStruct;

use self::{handover::Handover, stivale2_to_handover::_stivale2_to_handover};

pub mod handover;
pub mod stivale2_to_handover;

/// Converts a Stivale2Struct to a Handover.
/// Note that handover is a mutable reference; because Handover is fat, we want it on a static place, not on the stack
pub fn stivale2_to_handover(stivale2_struct: &StivaleStruct, handover: &'static mut Handover) {
    _stivale2_to_handover(stivale2_struct, handover);
}
