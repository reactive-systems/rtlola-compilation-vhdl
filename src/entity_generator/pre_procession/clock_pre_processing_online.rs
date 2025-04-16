use crate::entity_generator::GenerateVhdlCode;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct ClockPreProcessingOnline {
    pub(crate) number_cycles: u16,
}

impl ClockPreProcessingOnline {
    pub(crate) fn new(number_cycles: u16) -> ClockPreProcessingOnline {
        ClockPreProcessingOnline { number_cycles }
    }

    pub(crate) fn clock_cycles_changed_in_rising_edge(&self) -> u16 {
        (self.number_cycles * 3 + 1) * 2
    }
    pub(crate) fn clock_cycles_changed_in_falling_edge(&self) -> u16 {
        self.number_cycles * 3 + 1
    }
}

impl GenerateVhdlCode for ClockPreProcessingOnline {
    fn template_name(&self) -> String {
        "clock_pre_processing_online.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "clock_pre_processing.vhdl".to_string()
    }
}

impl Serialize for ClockPreProcessingOnline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("count_cycles", 6)?;
        let rising = self.clock_cycles_changed_in_rising_edge();
        let falling = self.clock_cycles_changed_in_falling_edge();
        s.serialize_field("rising", &rising)?;
        s.serialize_field("falling", &falling)?;
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
        let pack = ClockPreProcessingOnline::new(16);
        let tera: Tera = compile_templates!("templates/pre_processing/*");
        VHDLGenerator::generate_and_create(&pack, &tera, &PathBuf::from("target/test_files"));
    }
}
