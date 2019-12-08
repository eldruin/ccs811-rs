//! The prelude is a collection of all the traits for the CCS811 in this crate
//!
//! The traits have been renamed to avoid collisions with other items when
//! performing a glob import.

pub use Ccs811AppMode as _ccs811_ccs811appmode;
pub use Ccs811BootMode as _ccs811_ccs811bootmode;
pub use Ccs811Device as _ccs811_ccs811device;
