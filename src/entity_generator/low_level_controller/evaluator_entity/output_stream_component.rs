use crate::entity_generator::low_level_controller::output_entity::OutputStreamVHDL;
use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct OutputStreamComponent<'a> {
    pub(crate) output_stream: &'a OutputStreamVHDL<'a>,
}

impl<'a> OutputStreamComponent<'a> {
    pub(crate) fn new(output_stream: &'a OutputStreamVHDL) -> OutputStreamComponent<'a> {
        OutputStreamComponent { output_stream }
    }
}

impl GenerateVhdlCode for OutputStreamComponent<'_> {
    fn template_name(&self) -> String {
        "output_stream_component.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        panic!("should not happen.")
    }
}

impl Serialize for OutputStreamComponent<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("OutputStream", 5)?;
        s.serialize_field("name", &self.output_stream.output.name)?;
        s.serialize_field("ty", &get_vhdl_type(&self.output_stream.output.ty))?;
        s.serialize_field("array_size", &(self.output_stream.output.values_to_memorize().unwrap() - 1))?;
        s.serialize_field(
            "array_ty",
            &generate_vhdl_array_type_downwards(
                &self.output_stream.output.ty,
                self.output_stream.output.values_to_memorize().unwrap() - 1,
            ),
        )?;
        s.serialize_field("input_streams", &self.output_stream.generate_vhdl_dependencies(true))?;
        s.end()
    }
}

#[cfg(test)]
mod output_component_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    #[should_panic]
    fn panic_test() {
        let example_file_content = "input in : Int8\n\n output add5 : Int8 := in + 5";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let lola_copy = lola_instance.clone();
        let outputs = lola_instance.outputs;
        let stream = OutputStreamVHDL::new(&outputs[0], &lola_copy);
        let stream = OutputStreamComponent::new(&stream);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        VHDLGenerator::generate_and_create(&stream, &tera, &PathBuf::from("target/test_files/"));
    }

    #[test]
    fn non_panic_test() {
        //build test stream
        let example_file_content = "input in : Int8\n\n output add : Int8 := in + 5";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let lola_copy = lola_instance.clone();
        let outputs = lola_instance.outputs;
        let stream = OutputStreamVHDL::new(&outputs[0], &lola_copy);
        let stream = OutputStreamComponent::new(&stream);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        //take entity declaration
        let first_pos = result.find("component").expect("expected component declaration");
        let last_pos = result.find("end").expect("expected end component");
        let result = &result[first_pos..last_pos];
        let result: Vec<&str> = result.split("\n").collect();
        //check lines
        assert_eq!(result[0].trim(), "component add_output_stream_entity");
        assert_eq!(result[3].trim(), "in_0 : in signed(7 downto 0);");
        assert_eq!(result[4].trim(), "in_data_valid_0 : in std_logic;");
        assert_eq!(result[5].trim(), "data_out : out signed8_array(0 downto 0);");
        assert_eq!(result[6].trim(), "data_valid_out : out bit_array(0 downto 0);");
    }
}
