use crate::entity_generator::*;
use crate::vhdl_wrapper::expression_and_statement_serialize::get_str_for_sw_op;
use rtlola_frontend::RtLolaMir;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct BuildScript<'a> {
    pub(crate) online: bool,
    pub(crate) ir: &'a RtLolaMir,
    pub(crate) test: bool,
}

impl BuildScript<'_> {
    pub(crate) fn new(ir: &RtLolaMir, test: bool, online: bool) -> BuildScript {
        BuildScript { ir, test, online }
    }
}

impl GenerateVhdlCode for BuildScript<'_> {
    fn template_name(&self) -> String {
        "build_script.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "build_script_wip.py".to_string()
    }
}

impl Serialize for BuildScript<'_> {
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
                    self.ir.stream(cur.target).name(),
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
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
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
