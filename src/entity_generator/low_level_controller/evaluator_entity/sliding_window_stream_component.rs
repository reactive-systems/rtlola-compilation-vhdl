use crate::entity_generator::GenerateVhdlCode;
use crate::ir_extension::ExtendedRTLolaIR;
use crate::vhdl_wrapper::expression_and_statement_serialize::*;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct SlidingWindowComponent<'a> {
    pub(crate) sliding_window: &'a SlidingWindow,
    pub(crate) ir: &'a RtLolaMir,
}

impl<'a> SlidingWindowComponent<'a> {
    pub(crate) fn new(sliding_window: &'a SlidingWindow, ir: &'a RtLolaMir) -> SlidingWindowComponent<'a> {
        SlidingWindowComponent { sliding_window, ir }
    }
}

impl GenerateVhdlCode for SlidingWindowComponent<'_> {
    fn template_name(&self) -> String {
        "sliding_window_stream_component.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        panic!("should not happen.")
    }
}

impl Serialize for SlidingWindowComponent<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("OutputStream", 3)?;
        s.serialize_field(
            "name",
            &format!(
                "{}_{}_{}",
                self.ir.get_name_for_stream_ref(self.sliding_window.target),
                get_str_for_sw_op(self.sliding_window.op),
                self.sliding_window.reference.idx()
            ),
        )?;
        s.serialize_field("in_ty", &get_vhdl_type(self.ir.get_ty_for_stream_ref(self.sliding_window.target)))?;
        s.serialize_field("sw_ret_ty", &get_vhdl_type(&self.sliding_window.ty))?;
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
        let example_file_content = "input a : Int8\noutput b @2Hz := a.aggregate(over: 1s, using: sum)";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let sw = &lola_instance.sliding_windows[0];
        let stream = SlidingWindowComponent::new(&sw, &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        VHDLGenerator::generate_and_create(&stream, &tera, &PathBuf::from("target/test_files/"));
    }

    #[test]
    fn non_panic_test() {
        //build test stream
        let example_file_content = "input a : Int8\noutput b @2Hz := a.aggregate(over: 1s, using: sum)";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let sw = &lola_instance.sliding_windows[0];
        let stream = SlidingWindowComponent::new(&sw, &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/components_and_entities/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        //take entity declaration
        let first_pos = result.find("component").expect("expected component declaration");
        let last_pos = result.find("end").expect("expected end component");
        let result = &result[first_pos..last_pos];
        let result: Vec<&str> = result.split("\n").collect();
        //check lines
        assert_eq!(result[0].trim(), "component a_sum_0_sliding_window_entity");
        assert_eq!(result[5].trim(), "data_in : in signed(7 downto 0);");
        assert_eq!(result[6].trim(), "data_out : out signed(7 downto 0);");
    }
}
