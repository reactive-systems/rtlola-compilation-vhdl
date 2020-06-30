use crate::entity_generator::*;
use crate::ir_extension::ExtendedRTLolaIR;
use crate::vhdl_wrapper::expression_and_statement_serialize;
use crate::vhdl_wrapper::expression_and_statement_serialize::get_str_for_sw_op;
use rtlola_frontend::ir::RTLolaIR;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::path::PathBuf;

pub(crate) struct BuildScript<'a> {
    pub(crate) online: bool,
    pub(crate) ir: &'a RTLolaIR,
    pub(crate) test: bool,
}

impl<'a> BuildScript<'a> {
    pub(crate) fn new(ir: &RTLolaIR, test: bool, online: bool) -> BuildScript {
        BuildScript { ir, test, online }
    }
}

impl<'a> GenerateVhdlCode for BuildScript<'a> {
    fn template_name(&self) -> String {
        "build_script.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "build_script_wip.py".to_string()
    }
}

impl<'a> Serialize for BuildScript<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let inputs: Vec<String> = self
            .ir
            .inputs
            .iter()
            .map(|cur| format!("\ncompile_file(\"llc/{}_input_stream_entity.vhdl\")", cur.name))
            .collect();
        let outputs: Vec<String> = self
            .ir
            .outputs
            .iter()
            .map(|cur| format!("\ncompile_file(\"llc/{}_output_stream_entity.vhdl\")", cur.name))
            .collect();
        let sw: Vec<String> = self
            .ir
            .sliding_windows
            .iter()
            .map(|cur| {
                format!(
                    "\ncompile_file(\"llc/{}_{}_{}_sliding_window_entity.vhdl\")",
                    self.ir.get_name_for_stream_ref(cur.target),
                    get_str_for_sw_op(cur.op),
                    cur.reference.idx()
                )
            })
            .collect();
        let mut s = serializer.serialize_struct("BuildScript", 3)?;
        s.serialize_field("online", &self.online)?;
        s.serialize_field("evaluator_input_streams", &inputs.concat())?;
        s.serialize_field("evaluator_output_streams", &outputs.concat())?;
        s.serialize_field("evaluator_sliding_windows", &sw.concat())?;
        let mode = if self.test { "test" } else { "impl" };
        s.serialize_field("run_mode", &format!("run_{}_wip", mode))?;
        s.end()
    }
}

#[cfg(test)]
mod build_script_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use rtlola_frontend::ir::*;
    use std::path::PathBuf;
    use tera::Tera;

    fn parse(spec: &str) -> Result<RTLolaIR, String> {
        rtlola_frontend::parse("stdin", spec, crate::CONFIG)
    }

    #[test]
    fn generate_build_script_file() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let build_scrip = BuildScript::new(&lola_instance, true, false);
        let tera: Tera = compile_templates!("templates/*");
        VHDLGenerator::generate_and_create(&build_scrip, &tera, &PathBuf::from("target/test_files"))
    }
}
