/// File with static constants, used in the FPGA compilation
use serde::ser::{SerializeStruct, Serializer};
use std::time::Duration;

/// Constants to slow the system clock, for the high-level controller and low-level controller
pub(crate) const NUM_CLOCK_CYCLES_PER_INPUT_CYCLE_OFFLINE: u16 = 4;
pub(crate) const NUM_CLOCK_CYCLES_PER_INPUT_CYCLE_ONLINE: u16 = 2;

/// Bit range for the fix-point representation of real number:
/// - positive number = bit range of integer part
/// - negative number = bit range of fractional part
pub(crate) const FLOAT_16_HIGH: i16 = 4;
pub(crate) const FLOAT_16_LOW: i16 = -11;
pub(crate) const FLOAT_32_HIGH: i16 = 8;
pub(crate) const FLOAT_32_LOW: i16 = -23;
pub(crate) const FLOAT_64_HIGH: i16 = 11;
pub(crate) const FLOAT_64_LOW: i16 = -52;

/// Function to serialize the previously described bit range
pub(crate) fn serialize_bit_range_representation<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut s = serializer.serialize_struct("BitRangeFixPointRepresentation", 6)?;
    s.serialize_field("lower_bound_16_bit", &FLOAT_16_LOW)?;
    s.serialize_field("upper_bound_16_bit", &FLOAT_16_HIGH)?;
    s.serialize_field("lower_bound_32_bit", &FLOAT_32_LOW)?;
    s.serialize_field("upper_bound_32_bit", &FLOAT_32_HIGH)?;
    s.serialize_field("lower_bound_64_bit", &FLOAT_64_LOW)?;
    s.serialize_field("upper_bound_64_bit", &FLOAT_64_HIGH)?;
    s.end()
}

// Number of places for the fractional part of a real number, used in the processing part of the hardware board

pub(crate) const FLOAT_32_NUMBER_AFTER_POINT: u16 = 4;
pub(crate) const FLOAT_32_NUMBER_AFTER_POINT_POW_TEN: u16 = 10000;

pub(crate) const FLOAT_64_NUMBER_AFTER_POINT: u16 = 4;
pub(crate) const FLOAT_64_NUMBER_AFTER_POINT_POW_TEN: u16 = 10000;

/// Starting time of monitor and period of the system clock; rust does not allow a constant durant -> encoded as constant function
pub(crate) fn get_start_time() -> Duration {
    Duration::new(0, 0)
}

pub(crate) fn get_period_of_system_clock() -> Duration {
    Duration::new(0, 40)
}

/// Number of inputs in the simulation environment, to test the resulting monitor
pub(crate) const NUM_TEST_INPUTS: u32 = 20;
