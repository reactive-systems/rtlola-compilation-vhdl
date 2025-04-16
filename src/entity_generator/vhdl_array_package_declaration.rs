use crate::entity_generator::GenerateVhdlCode;
use crate::static_constants::serialize_bit_range_representation;
use serde::ser::{Serialize, Serializer};

/// This struct generates the array package in VHDL from the my_array_package template.
/// This package builds for every datatype in RTLola, a corresponding array type that is mostly used
/// in the LLC to store the offset values. Another application of the array type is the HLC to
/// represent the offset array in the Scheduler.
///
/// The serializer fills the gaps for the float arrays (16/32/64 bits) to bound the integer and
/// fraction bits to the constants defined in static_constants.rs.
pub(crate) struct ArrayPackageVHDL {}

impl ArrayPackageVHDL {
    pub(crate) fn new() -> ArrayPackageVHDL {
        ArrayPackageVHDL {}
    }
}

impl GenerateVhdlCode for ArrayPackageVHDL {
    fn template_name(&self) -> String {
        "my_array_package.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "my_array_package.vhdl".to_string()
    }
}

impl Serialize for ArrayPackageVHDL {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_bit_range_representation(serializer)
    }
}

#[cfg(test)]
mod package_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    #[test]
    fn generate_entity() {
        let pack = ArrayPackageVHDL::new();
        let tera: Tera = compile_templates!("templates/*");
        VHDLGenerator::generate_and_create(&pack, &tera, &PathBuf::from("target/test_files"));
    }
}
