use crate::entity_generator::GenerateVhdlCode;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct InputStreamMonitorEntity<'a> {
    pub(crate) input_stream: &'a InputStream,
}

impl<'a> InputStreamMonitorEntity<'a> {
    pub(crate) fn new(input_stream: &'a InputStream) -> InputStreamMonitorEntity<'a> {
        InputStreamMonitorEntity { input_stream }
    }
}

impl GenerateVhdlCode for InputStreamMonitorEntity<'_> {
    fn template_name(&self) -> String {
        "input_stream_entity.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        panic!("should not happen.")
    }
}

impl Serialize for InputStreamMonitorEntity<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("InputStream", 4)?;
        s.serialize_field("name", &self.input_stream.name)?;
        if self.input_stream.name.as_str() == "time" {
            s.serialize_field("data_in", &"input_time")?;
        } else {
            s.serialize_field("data_in", &self.input_stream.name)?;
        }
        let mut data = "".to_string();
        let mut valid = "".to_string();
        for i in 0..self.input_stream.values_to_memorize().unwrap() {
            let cur_data = format!("\n\t\t\tdata_out({}) => {}_entity_data_{},", i, self.input_stream.name, i);
            data = format!("{}{}", data, cur_data);
            let cur_valid =
                format!("\n\t\t\tdata_valid_out({}) => {}_entity_data_valid_{},", i, self.input_stream.name, i);
            valid = format!("{}{}", valid, cur_valid);
        }
        let data = data;
        let valid = valid;
        s.serialize_field("data_out", &data)?;
        s.serialize_field("data_valid_out", &valid)?;
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
        let stream = InputStreamMonitorEntity::new(&inputs[0]);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        VHDLGenerator::generate_and_create(&stream, &tera, &PathBuf::from("target/test_files/"));
    }

    #[test]
    fn non_panic_test() {
        //build test stream
        let example_file_content = "input a : UInt32 \n output b : UInt32 := a + a[-1].defaults(to:0)";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let inputs = lola_instance.inputs;
        let stream = InputStreamMonitorEntity::new(&inputs[0]);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        let result: Vec<&str> = result.split("\n").collect();
        //check lines
        assert_eq!(result[0].trim(), "a_entity_instance: a_input_stream_entity");
        assert_eq!(result[5].trim(), "data_in => a,");
        assert_eq!(result[6].trim(), "data_out(0) => a_entity_data_0,");
        assert_eq!(result[8].trim(), "data_valid_out(0) => a_entity_data_valid_0,");
        assert_eq!(result[10].trim(), "done_out => a_upd_done");
    }
}
