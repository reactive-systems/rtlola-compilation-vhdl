use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use streamlab_frontend::ir::*;

use streamlab_common::schedule::{Deadline, Schedule};

pub(crate) struct EvalController<'a> {
    pub(crate) schedule: &'a Schedule,
    pub(crate) ir: &'a RTLolaIR,
    pub(crate) event_driven_output: Vec<&'a OutputStream>,
    pub(crate) time_driven_output: Vec<&'a OutputStream>,
}

impl<'a> EvalController<'a> {
    pub(crate) fn new(schedule: &'a Schedule, ir: &'a RTLolaIR) -> EvalController<'a> {
        let mut event = Vec::new();
        let mut time = Vec::new();
        ir.outputs.iter().for_each(|cur| {
            if ir.event_driven.iter().any(|cur_event| cur_event.reference == cur.reference) {
                event.push(cur);
            } else {
                time.push(cur);
            }
        });
        EvalController { schedule, ir, event_driven_output: event, time_driven_output: time }
    }
}

impl<'a> GenerateVhdlCode for EvalController<'a> {
    fn template_name(&self) -> String {
        "eval_controller.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "eval_controller.vhdl".to_string()
    }
}

impl<'a> Serialize for EvalController<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EvalController", 10)?;
        s.serialize_field("inputs", &self.generate_inputs())?;
        s.serialize_field("outputs", &self.generate_outputs())?;
        s.serialize_field("registers", &self.generate_registers())?;
        s.serialize_field("defaults", &self.generate_defaults())?;
        s.serialize_field("deadline_len", &(self.schedule.deadlines.len()))?;
        s.serialize_field("deadline_handling", &self.generate_deadline_handling())?;
        s.serialize_field("event_condition", &self.generate_event_condition())?;
        s.serialize_field("event_handling", &self.generate_event_handling())?;
        s.serialize_field("en_no_stream", &self.generate_en_no_stream())?;
        s.serialize_field("final_mapping", &self.generate_final_mapping())?;
        s.end()
    }
}

impl<'a> EvalController<'a> {
    fn generate_inputs(&self) -> String {
        let res: Vec<String> = self
            .ir
            .inputs
            .iter()
            .map(|cur| {
                format!(
                    "\n\t\t{}_data_in : in {};\n\t\t{}_en_in : in std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                )
            })
            .collect();
        res.concat()
    }

    fn generate_outputs(&self) -> String {
        let inputs: Vec<String> = self
            .ir
            .inputs
            .iter()
            .map(|cur| {
                format!(
                    "\n\t\t{}_data_out : out {};\n\t\t{}_en_out : out std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                )
            })
            .collect();
        let outputs: Vec<String> =
            self.ir.outputs.iter().map(|cur| format!("\n\t\t{}_en_out : out std_logic;", cur.name)).collect();
        let mut res = inputs;
        res.extend(outputs);
        res.concat()
    }

    fn generate_registers(&self) -> String {
        let inputs: Vec<String> = self
            .ir
            .inputs
            .iter()
            .map(|cur| {
                format!(
                    "\nsignal {}_data_reg : {};\nsignal {}_en_reg: std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                )
            })
            .collect();

        let outputs: Vec<String> =
            self.ir.outputs.iter().map(|cur| format!("\nsignal {}_en_reg : std_logic;", cur.name)).collect();
        let deadline_en: Vec<String> = self
            .time_driven_output
            .iter()
            .map(|cur| {
                format!("\nsignal {}_en_array : bit_array({} downto 0);", cur.name, self.schedule.deadlines.len() - 1)
            })
            .collect();

        let mut res = inputs;
        res.extend(outputs);
        res.extend(deadline_en);
        res.concat()
    }

    fn generate_defaults(&self) -> String {
        let inputs: Vec<String> = self
            .ir
            .inputs
            .iter()
            .map(|cur| {
                format!(
                    "\n\t\t{}_data_reg <= {};\n\t\t{}_en_reg <= '0';",
                    cur.name,
                    generate_vhdl_type_default_initialisation(&cur.ty),
                    cur.name
                )
            })
            .collect();

        let outputs: Vec<String> =
            self.ir.outputs.iter().map(|cur| format!("\n\t\t{}_en_reg <= '0';", cur.name)).collect();

        let mut counter = -1;
        let deadlines_en: Vec<String> = self
            .schedule
            .deadlines
            .iter()
            .map(|cur_deadline| {
                counter += 1;
                self.time_driven_output
                    .iter()
                    .map(|cur_out| {
                        format!(
                            "\n\t\t{}_en_array({}) <= '{}';",
                            cur_out.name,
                            counter,
                            if cur_deadline.due.contains(&cur_out.reference.out_ix()) { "1" } else { "0" }
                        )
                    })
                    .collect()
            })
            .collect();

        let mut res = inputs;
        res.extend(outputs);
        res.extend(deadlines_en);
        res.concat()
    }

    fn generate_deadline_handling(&self) -> String {
        let inputs: Vec<String> =
            self.ir.inputs.iter().map(|cur| format!("\n\t\t\t{}_en_reg <= '0';", cur.name)).collect();

        let outputs_event: Vec<String> =
            self.event_driven_output.iter().map(|cur| format!("\n\t\t\t{}_en_reg <= '0';", cur.name)).collect();

        let outputs_time: Vec<String> = self
            .time_driven_output
            .iter()
            .map(|cur| format!("\n\t\t\t{}_en_reg <= {}_en_array(deadline_pos);", cur.name, cur.name))
            .collect();

        let mut res = inputs;
        res.extend(outputs_event);
        res.extend(outputs_time);
        res.concat()
    }

    fn generate_event_condition(&self) -> String {
        let cond: Vec<String> = self.ir.inputs.iter().map(|cur| format!(" or {}_en_in", cur.name)).collect();
        format!("('0'{}) = '1'", cond.concat())
    }

    fn generate_event_handling(&self) -> String {
        let inputs: Vec<String> = self
            .ir
            .inputs
            .iter()
            .map(|cur| {
                format!(
                    "\n\t\t\t{}_data_reg <= {}_data_in;\n\t\t\t{}_en_reg <= {}_en_in;",
                    cur.name, cur.name, cur.name, cur.name
                )
            })
            .collect();

        let outputs_event: Vec<String> = self
            .event_driven_output
            .iter()
            .map(|cur| {
                let input_dependencies: Vec<String> = cur
                    .input_dependencies
                    .iter()
                    .map(|cur| format!(" and {}_en_in", self.ir.get_in(*cur).name))
                    .collect();
                format!("\n\t\t\t{}_en_reg <= '1'{};", cur.name, input_dependencies.concat())
            })
            .collect();

        let outputs_time: Vec<String> =
            self.time_driven_output.iter().map(|cur| format!("\n\t\t\t{}_en_reg <= '0';", cur.name)).collect();

        let mut res = inputs;
        res.extend(outputs_event);
        res.extend(outputs_time);
        res.concat()
    }

    fn generate_en_no_stream(&self) -> String {
        let inputs: Vec<String> =
            self.ir.inputs.iter().map(|cur| format!("\n\t\t\t{}_en_reg <= '0';", cur.name)).collect();
        let outputs: Vec<String> =
            self.ir.outputs.iter().map(|cur| format!("\n\t\t\t{}_en_reg <= '0';", cur.name)).collect();
        let mut res = inputs;
        res.extend(outputs);
        res.concat()
    }

    fn generate_final_mapping(&self) -> String {
        let inputs: Vec<String> = self
            .ir
            .inputs
            .iter()
            .map(|cur| {
                format!(
                    "\n{}_data_out <= {}_data_reg;\n{}_en_out <= {}_en_reg;",
                    cur.name, cur.name, cur.name, cur.name
                )
            })
            .collect();

        let outputs: Vec<String> =
            self.ir.outputs.iter().map(|cur| format!("\n{}_en_out <= {}_en_reg;", cur.name, cur.name)).collect();

        let mut res = inputs;
        res.extend(outputs);
        res.concat()
    }
}

#[cfg(test)]
mod periodic_manager_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use streamlab_common::schedule::*;
    use streamlab_frontend::*;
    use tera::Tera;

    fn parse(spec: &str) -> Result<RTLolaIR, String> {
        streamlab_frontend::parse("stdin", spec, crate::CONFIG)
    }

    #[ignore] // fix spec
    #[test]
    fn generate_eval_controller_file() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a!0 + 3\noutput d @2Hz := a!0 + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let schedule = &Schedule::from(&lola_instance).unwrap_or_else(|e| panic!(e));
        let eval_controller = EvalController::new(schedule, &lola_instance);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        VHDLGenerator::generate_and_create(&eval_controller, &tera, &PathBuf::from("target/test_files"))
    }

    #[ignore] // fix spec
    #[test]
    fn eval_controller_test() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a!0 + 3\noutput d @2Hz := a!0 + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let schedule = &Schedule::from(&lola_instance).unwrap_or_else(|e| panic!(e));
        let eval_controller = EvalController::new(schedule, &lola_instance);
        let tera: Tera = compile_templates!("templates/high_level_controller/*");
        let result = VHDLGenerator::generate(&eval_controller, &tera);
        //take signal
        let first_pos = result.find("default values").expect("expected default handling");
        let last_pos = result.find("deadline handling").expect("expected deadline handling");
        let result_default_handling = &result[first_pos..last_pos];
        let result_default_handling: Vec<&str> = result_default_handling.split("\n").collect();
        //check lines
        assert_eq!(result_default_handling[10].trim(), "c_en_array(0) <= '0';");
        assert_eq!(result_default_handling[11].trim(), "d_en_array(0) <= '1';");
        assert_eq!(result_default_handling[12].trim(), "c_en_array(1) <= '1';");
        assert_eq!(result_default_handling[13].trim(), "d_en_array(1) <= '1';");
        //take signal
        let first_pos = result.find("deadline handling").expect("expected deadline handling");
        let last_pos = result.find("event handling").expect("expected event handling");
        let result_deadline_handling = &result[first_pos..last_pos];
        let result_deadline_handling: Vec<&str> = result_deadline_handling.split("\n").collect();
        //check lines
        assert_eq!(result_deadline_handling[3].trim(), "a_en_reg <= '0';");
        assert_eq!(result_deadline_handling[4].trim(), "b_en_reg <= '0';");
        assert_eq!(result_deadline_handling[5].trim(), "e_en_reg <= '0';");
        assert_eq!(result_deadline_handling[6].trim(), "c_en_reg <= c_en_array(deadline_pos);");
        assert_eq!(result_deadline_handling[7].trim(), "d_en_reg <= d_en_array(deadline_pos);");
        //take signal
        let first_pos = result.find("event handling").expect("expected event handling");
        let last_pos = result.find("no stream handling").expect("expected no stream handling");
        let result_event_handling = &result[first_pos..last_pos];
        let result_event_handling: Vec<&str> = result_event_handling.split("\n").collect();
        //check lines
        assert_eq!(result_event_handling[2].trim(), "a_data_reg <= a_data_in;");
        assert_eq!(result_event_handling[3].trim(), "a_en_reg <= a_en_in;");
        assert_eq!(result_event_handling[4].trim(), "b_data_reg <= b_data_in;");
        assert_eq!(result_event_handling[5].trim(), "b_en_reg <= b_en_in;");
        assert_eq!(result_event_handling[6].trim(), "e_en_reg <= '1' and a_en_in and b_en_in;");
        assert_eq!(result_event_handling[7].trim(), "c_en_reg <= '0';");
        assert_eq!(result_event_handling[8].trim(), "d_en_reg <= '0';");
    }
}
