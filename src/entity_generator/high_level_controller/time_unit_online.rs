use crate::entity_generator::GenerateVhdlCode;
use crate::static_constants::{get_period_of_system_clock, get_start_time};
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct TimeUnitOnline {}

impl TimeUnitOnline {
    pub(crate) fn new() -> TimeUnitOnline {
        TimeUnitOnline {}
    }
}

impl GenerateVhdlCode for TimeUnitOnline {
    fn template_name(&self) -> String {
        "time_unit_online.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "time_unit.vhdl".to_string()
    }
}

impl Serialize for TimeUnitOnline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("PeriodicManager", 2)?;
        s.serialize_field("start_time", &(get_start_time().as_nanos() as u64))?;
        s.serialize_field("time_per_cycle", &(get_period_of_system_clock().as_nanos() as u64))?;
        let frequency_in_mhz = 1.0 / (get_period_of_system_clock().as_nanos() as f64) * 1000.0;
        s.serialize_field("clock_freq_as_string", &format!("{} MHZ", frequency_in_mhz))?;

        s.serialize_field("period_as_string", &format!("{} ns", get_period_of_system_clock().as_nanos()))?;
        s.end()
    }
}

#[cfg(test)]
mod time_unit_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    #[test]
    fn generate_entity() {
        let pack = TimeUnitOnline::new();
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        VHDLGenerator::generate_and_create(&pack, &tera, &PathBuf::from("target/test_files"));
    }
}
