use crate::entity_generator::GenerateVhdlCode;
use crate::ir_extension::ExtendedRTLolaIR;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct InputStreamVHDL<'a> {
    pub(crate) input: &'a InputStream,
    pub(crate) ir: &'a RtLolaMir,
}

impl<'a> InputStreamVHDL<'a> {
    pub(crate) fn new(input: &'a InputStream, ir: &'a RtLolaMir) -> InputStreamVHDL<'a> {
        InputStreamVHDL { input, ir }
    }
}

impl GenerateVhdlCode for InputStreamVHDL<'_> {
    fn template_name(&self) -> String {
        "input_stream.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        format!("{}_input_stream_entity.vhdl", self.input.name)
    }
}

impl Serialize for InputStreamVHDL<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("InputStream", 7)?;
        s.serialize_field("name", &format!("{}_input_stream", self.input.name))?;
        s.serialize_field("ty", &get_vhdl_type(&self.input.ty))?;
        let mem_bound = self.input.values_to_memorize().unwrap() - 1;
        s.serialize_field("array_size", &mem_bound)?;
        s.serialize_field("array_ty", &generate_vhdl_array_type_downwards(&self.input.ty, mem_bound))?;
        s.serialize_field("default_init", &generate_vhdl_array_default_initialisation(&self.input.ty))?;
        s.serialize_field("print_stream", &format!("input {} : {}", self.input.name, self.input.ty))?;
        s.serialize_field(
            "input_dependencies_in_dg",
            &self.ir.get_input_dependencies_for_stream_as_annotation(self.input.reference),
        )?;
        //        s.serilaize_field("input_offset_dependencies", self.ir.get_input_dependencies_for_stream_as_annotation(self.input.reference))?;
        //        s.serilaize_field("input_window_dependencies", self.ir.get_input_dependencies_for_stream_as_annotation(self.input.reference))?;
        s.end()
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_bool_entity() {
        let example_file_content = "input boolTest : Bool";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let inputs = lola_instance.clone().inputs;
        let stream = InputStreamVHDL::new(&inputs[0], &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        VHDLGenerator::generate_and_create(&stream, &tera, &PathBuf::from("target/test_files"));
    }

    #[test]
    fn generate_signed_entity() {
        let example_file_content = "input signedTest : Int8";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let inputs = lola_instance.clone().inputs;
        let stream = InputStreamVHDL::new(&inputs[0], &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        VHDLGenerator::generate_and_create(&stream, &tera, &PathBuf::from("target/test_files"));
    }

    #[test]
    fn bool_type_test() {
        //build test stream
        let example_file_content = "input ter_stream : Bool";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let inputs = lola_instance.clone().inputs;
        let stream = InputStreamVHDL::new(&inputs[0], &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        //take entity declaration
        let first_pos = result.find("entity").expect("expected entity declaration");
        let last_pos = result.find("architecture").expect("expected entity declaration");
        let result = &result[first_pos..last_pos];
        let result: Vec<&str> = result.split("\n").collect();
        //check lines
        assert_eq!(result[0].trim(), "entity ter_stream_input_stream_entity is");
        assert_eq!(result[3].trim(), "data_in : in std_logic;");
        assert_eq!(result[4].trim(), "data_out : out bit_array(0 downto 0);");
        assert_eq!(result[5].trim(), "data_valid_out : out bit_array(0 downto 0);");
    }

    #[test]
    fn signed_type_test_event() {
        //build test stream
        let example_file_content = "input id_stream : Int8\n output test := id_stream + 3";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let inputs = lola_instance.clone().inputs;
        let stream = InputStreamVHDL::new(&inputs[0], &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        //take entity declaration
        let first_pos = result.find("entity").expect("expected entity declaration");
        let last_pos = result.find("architecture").expect("expected entity declaration");
        let result = &result[first_pos..last_pos];
        let result: Vec<&str> = result.split("\n").collect();
        //check lines
        assert_eq!(result[0].trim(), "entity id_stream_input_stream_entity is");
        assert_eq!(result[3].trim(), "data_in : in signed(7 downto 0);");
        assert_eq!(result[4].trim(), "data_out : out signed8_array(0 downto 0);");
        assert_eq!(result[5].trim(), "data_valid_out : out bit_array(0 downto 0);");
    }

    #[test]
    fn signed_type_test_periodic() {
        //build test stream
        let example_file_content =
            "input id_stream : Int8\n output test @1Hz := id_stream.aggregate(over: 2s, using: count)";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let inputs = lola_instance.clone().inputs;
        let stream = InputStreamVHDL::new(&inputs[0], &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        //take entity declaration
        let first_pos = result.find("entity").expect("expected entity declaration");
        let last_pos = result.find("architecture").expect("expected entity declaration");
        let result = &result[first_pos..last_pos];
        let result: Vec<&str> = result.split("\n").collect();
        //check lines
        assert_eq!(result[0].trim(), "entity id_stream_input_stream_entity is");
        assert_eq!(result[3].trim(), "data_in : in signed(7 downto 0);");
        assert_eq!(result[4].trim(), "data_out : out signed8_array(0 downto 0);");
        assert_eq!(result[5].trim(), "data_valid_out : out bit_array(0 downto 0);");
    }
}
