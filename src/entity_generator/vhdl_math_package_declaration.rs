use crate::entity_generator::GenerateVhdlCode;
use crate::static_constants::serialize_bit_range_representation;
use serde::ser::{Serialize, Serializer};

/// This struct generates the math package in VHDL from the my_math_package template. Currently,
/// this package builds for every numeric datatype in RTLola, an sqrt-function. For this, we use the
/// square root implementation from https://vhdlguru.blogspot.com/2010/03/vhdl-function-for-finding-square-root.html
/// and adapt it for the other numeric types. In this adaption, we interpret the bit representation
/// as an integer to use the square root of this type. Because of the fix-point representation, this
/// is possible for all numeric types.
///
/// The realization of the stream expressions calls these functions in the LLC if the stream
/// expression contains an sqrt-function.
///
/// The serializer fills the gaps for the float arrays (16/32/64 bits) to bound the integer and
/// fraction bits to the constants defined in static_constants.rs.
pub(crate) struct MathPackageVHDL {}

impl MathPackageVHDL {
    pub(crate) fn new() -> MathPackageVHDL {
        MathPackageVHDL {}
    }
}

impl GenerateVhdlCode for MathPackageVHDL {
    fn template_name(&self) -> String {
        "my_math_package.tmpl".to_string()
    }
    fn file_name(&self) -> String {
        "my_math_package.vhdl".to_string()
    }
}

impl Serialize for MathPackageVHDL {
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
        let pack = MathPackageVHDL::new();
        let tera: Tera = compile_templates!("templates/*");
        VHDLGenerator::generate_and_create(&pack, &tera, &PathBuf::from("target/test_files"));
    }
}
