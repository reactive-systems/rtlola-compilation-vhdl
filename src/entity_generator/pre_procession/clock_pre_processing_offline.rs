use crate::entity_generator::GenerateVhdlCode;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct ClockPreProcessingOffline {
    pub(crate) number_cycles: u16,
}

impl ClockPreProcessingOffline {
    pub(crate) fn new(number_cycles: u16) -> ClockPreProcessingOffline {
        ClockPreProcessingOffline { number_cycles }
    }

    pub(crate) fn clock_cycles_changed_in_rising_edge(&self) -> Vec<u16> {
        assert_eq!(self.number_cycles % 2, 0);
        vec![self.number_cycles, 3 * self.number_cycles + 2]
    }
    pub(crate) fn clock_cycles_changed_in_falling_edge(&self) -> Vec<u16> {
        assert_eq!(self.number_cycles % 2, 0);
        vec![self.number_cycles / 2, self.number_cycles + (self.number_cycles / 2) + 1]
    }
}

impl GenerateVhdlCode for ClockPreProcessingOffline {
    fn template_name(&self) -> String {
        "clock_pre_processing_offline.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "clock_pre_processing.vhdl".to_string()
    }
}

impl Serialize for ClockPreProcessingOffline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("count_cycles", 6)?;
        let rising = self.clock_cycles_changed_in_rising_edge();
        let falling = self.clock_cycles_changed_in_falling_edge();
        s.serialize_field("rising_tclk", &rising[0])?;
        s.serialize_field("rising_eclk", &rising[1])?;
        s.serialize_field("falling_tclk", &falling[0])?;
        s.serialize_field("falling_eclk", &falling[1])?;
        s.end()
    }
}

#[cfg(test)]
mod clock_pre_processing_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    #[test]
    fn generate_entity() {
        let pack = ClockPreProcessingOffline::new(16);
        let tera: Tera = compile_templates!("templates/pre_processing/*");
        VHDLGenerator::generate_and_create(&pack, &tera, &PathBuf::from("target/test_files"));
    }
}
