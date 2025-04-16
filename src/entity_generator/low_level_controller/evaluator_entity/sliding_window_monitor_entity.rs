use crate::entity_generator::GenerateVhdlCode;
use crate::ir_extension::ExtendedRTLolaIR;
use crate::vhdl_wrapper::expression_and_statement_serialize::*;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct SlidingWindowMonitorEntity<'a> {
    pub(crate) sliding_window: &'a SlidingWindow,
    pub(crate) ir: &'a RtLolaMir,
}

impl<'a> SlidingWindowMonitorEntity<'a> {
    pub(crate) fn new(sliding_window: &'a SlidingWindow, ir: &'a RtLolaMir) -> SlidingWindowMonitorEntity<'a> {
        SlidingWindowMonitorEntity { sliding_window, ir }
    }
}

impl GenerateVhdlCode for SlidingWindowMonitorEntity<'_> {
    fn template_name(&self) -> String {
        "sliding_window_entity.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        panic!("should not happen.")
    }
}

impl Serialize for SlidingWindowMonitorEntity<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("OutputStream", 4)?;
        s.serialize_field(
            "name",
            &format!(
                "{}_{}_{}",
                self.ir.get_name_for_stream_ref(self.sliding_window.target),
                get_str_for_sw_op(self.sliding_window.op),
                self.sliding_window.reference.idx()
            ),
        )?;
        s.serialize_field("ty", &get_vhdl_type(&self.sliding_window.ty))?;
        s.serialize_field("sw_type", &get_str_for_sw_op(self.sliding_window.op))?;
        s.serialize_field("input_data", &self.ir.get_name_for_stream_ref(self.sliding_window.target))?;
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
        let example_file_content = "input a : Int8\noutput b @ 2Hz := a.aggregate(over: 1s, using: sum)";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let sw = &lola_instance.sliding_windows[0];
        let stream = SlidingWindowMonitorEntity::new(sw, &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        VHDLGenerator::generate_and_create(&stream, &tera, &PathBuf::from("target/test_files"));
    }

    #[test]
    fn non_panic_test() {
        //build test stream
        let example_file_content = "input a : Int8\noutput b @ 2Hz := a.aggregate(over: 1s, using: sum)";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let sw = &lola_instance.sliding_windows[0];
        let stream = SlidingWindowMonitorEntity::new(sw, &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        let result: Vec<&str> = result.split("\n").collect();
        //check lines
        assert_eq!(result[0].trim(), "a_sum_0_sliding_window_entity_instance: a_sum_0_sliding_window_entity");
        assert_eq!(result[4].trim(), "evict => a_sum_0_evict,");
        assert_eq!(result[8].trim(), "data_in => a_entity_data_0,");
        assert_eq!(result[9].trim(), "data_out => a_sum_0_entity_data,");
    }
}
