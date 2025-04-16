use crate::entity_generator::GenerateVhdlCode;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct TimeUnitOffline {}

impl TimeUnitOffline {
    pub(crate) fn new() -> TimeUnitOffline {
        TimeUnitOffline {}
    }
}

impl GenerateVhdlCode for TimeUnitOffline {
    fn template_name(&self) -> String {
        "time_unit_offline.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "time_unit.vhdl".to_string()
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
        let pack = TimeUnitOffline::new();
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        VHDLGenerator::generate_and_create(&pack, &tera, &PathBuf::from("target/test_files"));
    }
}
