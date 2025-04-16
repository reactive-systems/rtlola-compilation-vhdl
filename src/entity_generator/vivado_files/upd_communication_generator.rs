use serde::Serialize;

use crate::entity_generator::GenerateVhdlCode;

#[derive(Serialize)]
pub(crate) struct UDPCommunication {}

impl UDPCommunication {
    pub(crate) fn new() -> UDPCommunication {
        UDPCommunication {}
    }
}

impl GenerateVhdlCode for UDPCommunication {
    fn template_name(&self) -> String {
        "udp_communication.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "udp_communication.h".to_string()
    }
}

#[cfg(test)]
mod udpc_communication_file_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    #[test]
    fn generate_entity() {
        let cfile = UDPCommunication::new();
        let tera: Tera = compile_templates!("templates/vivado_file_changes/*");
        VHDLGenerator::generate_and_create(&cfile, &tera, &PathBuf::from("target/test_files"));
    }
}
