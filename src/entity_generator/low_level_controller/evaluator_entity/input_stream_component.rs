use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct InputStreamComponent<'a> {
    pub(crate) input_stream: &'a InputStream,
}

impl<'a> InputStreamComponent<'a> {
    pub(crate) fn new(input_stream: &'a InputStream) -> InputStreamComponent<'a> {
        InputStreamComponent { input_stream }
    }
}

impl GenerateVhdlCode for InputStreamComponent<'_> {
    fn template_name(&self) -> String {
        "input_stream_component.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        panic!("should not happen.")
    }
}

impl Serialize for InputStreamComponent<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("InputStream", 5)?;
        s.serialize_field("name", &self.input_stream.name)?;
        s.serialize_field("ty", &get_vhdl_type(&self.input_stream.ty))?;
        s.serialize_field("array_size", &(self.input_stream.values_to_memorize().unwrap() - 1))?;
        s.serialize_field(
            "array_ty",
            &generate_vhdl_array_type_downwards(
                &self.input_stream.ty,
                self.input_stream.values_to_memorize().unwrap() - 1,
            ),
        )?;
        s.end()
    }
}

#[cfg(test)]
mod tests {
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
        let example_file_content = "input boolTest : Bool";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let inputs = lola_instance.inputs;
        let stream = InputStreamComponent::new(&inputs[0]);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        VHDLGenerator::generate_and_create(&stream, &tera, &PathBuf::from("target/test_files/"));
    }

    #[test]
    fn non_panic_test() {
        //build test stream
        let example_file_content = "input ter_stream : Bool";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let inputs = lola_instance.inputs;
        let stream = InputStreamComponent::new(&inputs[0]);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        //take entity declaration
        let first_pos = result.find("component").expect("expected component declaration");
        let last_pos = result.find("end").expect("expected end component");
        let result = &result[first_pos..last_pos];
        let result: Vec<&str> = result.split("\n").collect();
        //check lines
        assert_eq!(result[0].trim(), "component ter_stream_input_stream_entity");
        assert_eq!(result[3].trim(), "data_in : in std_logic;");
        assert_eq!(result[4].trim(), "data_out : out bit_array(0 downto 0);");
        assert_eq!(result[5].trim(), "data_valid_out : out bit_array(0 downto 0);");
    }
}
