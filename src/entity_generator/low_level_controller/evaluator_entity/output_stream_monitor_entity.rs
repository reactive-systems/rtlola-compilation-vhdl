use crate::entity_generator::low_level_controller::output_entity::OutputStreamVHDL;
use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct OutputStreamMonitorEntity<'a> {
    pub(crate) output_stream: &'a OutputStreamVHDL<'a>,
}

impl<'a> OutputStreamMonitorEntity<'a> {
    pub(crate) fn new(output_stream: &'a OutputStreamVHDL) -> OutputStreamMonitorEntity<'a> {
        OutputStreamMonitorEntity { output_stream }
    }
}

impl GenerateVhdlCode for OutputStreamMonitorEntity<'_> {
    fn template_name(&self) -> String {
        "output_stream_entity.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        panic!("should not happen.")
    }
}

impl Serialize for OutputStreamMonitorEntity<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("OutputStream", 5)?;
        s.serialize_field("name", &self.output_stream.output.name)?;
        s.serialize_field("ty", &get_vhdl_type(&self.output_stream.output.ty))?;
        let mut data = "".to_string();
        let mut valid = "".to_string();
        for i in 0..self.output_stream.output.values_to_memorize().unwrap() {
            let cur_data = format!("\t\t\tdata_out({}) => {}_entity_data_{},", i, self.output_stream.output.name, i);
            data = format!("{}{}\n", data, cur_data);
            let cur_valid =
                format!("\t\t\tdata_valid_out({}) => {}_entity_data_valid_{},", i, self.output_stream.output.name, i);
            valid = format!("{}{}\n", valid, cur_valid);
        }
        let data = data;
        let valid = valid;
        s.serialize_field("data", &data)?;
        s.serialize_field("valid", &valid)?;
        s.serialize_field("input_streams", &self.output_stream.generate_vhdl_dependencies(false))?;
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
        let lola_copy = lola_instance.clone();
        let outputs = lola_instance.outputs;
        let output = OutputStreamVHDL::new(&outputs[0], &lola_copy);
        let stream = OutputStreamMonitorEntity::new(&output);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        VHDLGenerator::generate_and_create(&stream, &tera, &PathBuf::from("target/test_files"));
    }

    #[test]
    fn non_panic_test() {
        //build test stream
        let example_file_content = "input a : Int32 \n output b : Int32 := a + a[-1].defaults(to:0)";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let lola_copy = lola_instance.clone();
        let outputs = lola_instance.outputs;
        let output = OutputStreamVHDL::new(&outputs[0], &lola_copy);
        let stream = OutputStreamMonitorEntity::new(&output);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        let result: Vec<&str> = result.split("\n").collect();
        //check lines
        assert_eq!(result[0].trim(), "b_entity_instance: b_output_stream_entity");
        assert_eq!(result[6].trim(), "a_0 => a_entity_data_0,");
        assert_eq!(result[7].trim(), "a_data_valid_0 => a_entity_data_valid_0,");
        assert_eq!(result[10].trim(), "data_out(0) => b_entity_data_0,");
        assert_eq!(result[11].trim(), "data_valid_out(0) => b_entity_data_valid_0,");
        assert_eq!(result[12].trim(), "pe_done_out => b_pe_done,");
    }
}
