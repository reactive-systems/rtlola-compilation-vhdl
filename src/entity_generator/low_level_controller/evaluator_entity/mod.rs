use crate::entity_generator::low_level_controller::evaluator_entity::sliding_window_monitor_entity::SlidingWindowMonitorEntity;
use crate::entity_generator::low_level_controller::evaluator_entity::sliding_window_stream_component::SlidingWindowComponent;
use crate::entity_generator::low_level_controller::output_entity::OutputStreamVHDL;
use crate::entity_generator::{GenerateVhdlCode, VHDLGenerator};
use crate::ir_extension::ExtendedRTLolaIR;
use crate::vhdl_wrapper::expression_and_statement_serialize::*;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use tera::Tera;

pub(crate) mod input_stream_component;
pub(crate) mod input_stream_monitor_entity;
pub(crate) mod output_stream_component;
pub(crate) mod output_stream_monitor_entity;
pub(crate) mod sliding_window_monitor_entity;
pub(crate) mod sliding_window_stream_component;

pub(crate) struct Evaluator<'a> {
    pub(crate) ir: &'a RtLolaMir,
    pub(crate) tera_files: String,
}

impl<'a> Evaluator<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir, tera_files: String) -> Evaluator<'a> {
        let tera_files = tera_files + "/low_level_controller/components_and_entities/*";
        Evaluator { ir, tera_files }
    }
}

impl GenerateVhdlCode for Evaluator<'_> {
    fn template_name(&self) -> String {
        "evaluator.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "evaluator.vhdl".to_string()
    }
}

impl Serialize for Evaluator<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_evaluator_setup();
        let mut s = serializer.serialize_struct("Monitor", 19)?;
        s.serialize_field("inputs", &setup.inputs.concat())?;
        s.serialize_field("outputs", &setup.outputs.concat())?;
        s.serialize_field("component_declaration", &setup.component_declaration.concat())?;
        s.serialize_field("signals", &setup.signals.concat())?;
        s.serialize_field("component_instantiation", &setup.component_instantiation.concat())?;
        s.serialize_field("disable_en_signals", &setup.disable_en_assignments.concat())?;
        s.serialize_field("upd_signal_assignments", &setup.upd_signal_assignments.concat())?;
        s.serialize_field("pe_signal_assignments", &setup.pe_signal_assignments.concat())?;
        s.serialize_field("evict_signal_assignments", &setup.evict_signal_assignments.concat())?;
        s.serialize_field("eval_signal_assignments", &setup.eval_signal_assignments.concat())?;
        s.serialize_field("sw_update_signal_assignments", &setup.sw_update_signal_assignments.concat())?;
        s.serialize_field("sw_request_signal_assignments", &setup.sw_request_signal_assignments.concat())?;
        s.serialize_field("output_port_assignments", &setup.output_port_assignments.concat())?;
        s.serialize_field(
            "valid_port_assignment",
            &format!("\n\t\t\t\t\tvalid_reg <= '1'{};", setup.valid_port_assignment.concat()),
        )?;
        s.serialize_field(
            "evaluator_done_assignment",
            &format!(
                "\n\t\t\t\t\tupd_and_pe_done <= '1'{};\n\t\t\t\t\tevaluator_done <= upd_and_pe_done{};",
                setup.udp_pe_done_port_assignment.concat(),
                setup.eval_done_port_assignment.concat()
            ),
        )?;
        let spec = format!("{}{}", setup.print_input_streams.concat(), setup.print_output_streams.concat());
        s.serialize_field("print_spec", &spec)?;
        s.serialize_field("print_input_stream_names", &setup.print_input_stream_names.concat())?;
        s.serialize_field("print_output_stream_names", &setup.print_output_stream_names.concat())?;
        s.serialize_field("print_sliding_windows", &setup.print_sliding_windows.concat())?;
        s.end()
    }
}

pub(crate) struct EvaluatorSetup {
    pub(crate) inputs: Vec<String>,
    pub(crate) outputs: Vec<String>,
    pub(crate) component_declaration: Vec<String>,
    pub(crate) signals: Vec<String>,
    pub(crate) component_instantiation: Vec<String>,
    pub(crate) disable_en_assignments: Vec<String>,
    pub(crate) upd_signal_assignments: Vec<String>,
    pub(crate) pe_signal_assignments: Vec<String>,
    pub(crate) evict_signal_assignments: Vec<String>,
    pub(crate) eval_signal_assignments: Vec<String>,
    pub(crate) sw_update_signal_assignments: Vec<String>,
    pub(crate) sw_request_signal_assignments: Vec<String>,
    pub(crate) valid_port_assignment: Vec<String>,
    pub(crate) udp_pe_done_port_assignment: Vec<String>,
    pub(crate) eval_done_port_assignment: Vec<String>,
    pub(crate) output_port_assignments: Vec<String>,
    pub(crate) print_input_streams: Vec<String>,
    pub(crate) print_input_stream_names: Vec<String>,
    pub(crate) print_output_streams: Vec<String>,
    pub(crate) print_output_stream_names: Vec<String>,
    pub(crate) print_sliding_windows: Vec<String>,
}

impl EvaluatorSetup {
    fn new() -> EvaluatorSetup {
        EvaluatorSetup {
            inputs: Vec::new(),
            outputs: Vec::new(),
            component_declaration: Vec::new(),
            signals: Vec::new(),
            component_instantiation: Vec::new(),
            disable_en_assignments: Vec::new(),
            upd_signal_assignments: Vec::new(),
            pe_signal_assignments: Vec::new(),
            evict_signal_assignments: Vec::new(),
            eval_signal_assignments: Vec::new(),
            sw_update_signal_assignments: Vec::new(),
            sw_request_signal_assignments: Vec::new(),
            valid_port_assignment: Vec::new(),
            udp_pe_done_port_assignment: Vec::new(),
            eval_done_port_assignment: Vec::new(),
            output_port_assignments: Vec::new(),
            print_input_streams: Vec::new(),
            print_input_stream_names: Vec::new(),
            print_output_streams: Vec::new(),
            print_output_stream_names: Vec::new(),
            print_sliding_windows: Vec::new(),
        }
    }
}

impl Evaluator<'_> {
    fn generate_evaluator_setup(&self) -> EvaluatorSetup {
        let mut setup = EvaluatorSetup::new();
        let tera: Tera = tera::compile_templates!(&self.tera_files);
        self.ir.inputs.iter().for_each(|cur| {
            let annotation = format!("input {} : {}", cur.name, cur.ty);
            if cur.name.as_str() != "time" {
                setup.inputs.push(format!(
                    "\n\t\t{} : in {};\n\t\t{}_en : in std_logic;",
                    cur.name,
                    get_vhdl_type(&cur.ty),
                    cur.name
                ));
                setup.upd_signal_assignments.push(format!("\n\t\t\t\t\t{}_upd <= {}_en;", cur.name, cur.name));
                setup.udp_pe_done_port_assignment.push(format!(" and (not {}_en or {}_upd_done)", cur.name, cur.name));
            } else {
                setup.upd_signal_assignments.push(format!("\n\t\t\t\t\t{}_upd <= '1';", cur.name));
                setup.udp_pe_done_port_assignment.push(format!(" and {}_upd_done", cur.name));
            }
            setup.component_declaration.push(format!(
                "\t--* {}\n{}",
                annotation,
                VHDLGenerator::generate(&input_stream_component::InputStreamComponent::new(cur), &tera)
            ));
            setup.component_instantiation.push(format!(
                "\t--* {}\n{}",
                annotation,
                VHDLGenerator::generate(&input_stream_monitor_entity::InputStreamMonitorEntity::new(cur), &tera)
            ));
            let mut res_signal =
                vec![format!("\tsignal {}_upd : std_logic;\n\tsignal {}_upd_done : std_logic;\n", cur.name, cur.name)];
            res_signal.extend((0..cur.values_to_memorize().unwrap()).map(|cur_bound| {
                format!(
                    "\tsignal {}_entity_data_{} : {};\n\tsignal {}_entity_data_valid_{} : std_logic;\n",
                    cur.name,
                    cur_bound,
                    get_vhdl_type(&cur.ty),
                    cur.name,
                    cur_bound
                )
            }));
            setup.signals.push(res_signal.concat());
            setup.disable_en_assignments.push(format!("\n\t\t\t\t{}_upd <= '0';", cur.name));
            setup.print_input_streams.push(format!("--* {}\n", annotation));
            setup.print_input_stream_names.push(format!("\n\t\t\t\t\t--* - {} ", cur.name));
        });
        self.ir.outputs.iter().for_each(|cur| {
            let (_,_,expr_as_string) = OutputStreamVHDL::new(cur, self.ir).generate_vhdl_expression_and_temporaries();
            let annotation = format!("output {} := {}", cur.name, expr_as_string);
            setup.inputs.push(format!("\n\t\t{}_en : in std_logic;", cur.name));
            setup.outputs.push(format!("\n\t\t{} : out {};", cur.name, get_vhdl_type(&cur.ty)));
            let cur_output = OutputStreamVHDL::new(cur, self.ir);
            setup.component_declaration.push(
                format!(
                    "\t--* {}\n{}",
                    annotation,VHDLGenerator::generate(
                &output_stream_component::OutputStreamComponent::new(&cur_output),
                &tera,
            )));
            setup.component_instantiation.push(
                format!(
                    "\t--* {}\n{}",
                    annotation,VHDLGenerator::generate(
                &output_stream_monitor_entity::OutputStreamMonitorEntity::new(&cur_output),
                &tera,
            )));
            let mut res_signal = vec![format!(
                "\tsignal {}_pe : std_logic;\n\tsignal {}_eval : std_logic;\n\tsignal {}_pe_done : std_logic;\n\tsignal {}_eval_done : std_logic;\n",
                cur.name, cur.name, cur.name, cur.name
            )];
            res_signal.extend((0..cur.values_to_memorize().unwrap()).map(|cur_bound| {
                format!(
                    "\tsignal {}_entity_data_{} : {};\n\tsignal {}_entity_data_valid_{} : std_logic;\n",
                    cur.name,
                    cur_bound,
                    get_vhdl_type(&cur.ty),
                    cur.name,
                    cur_bound
                )
            }));
            setup.signals.push(res_signal.concat());
            setup.disable_en_assignments.push(format!("\n\t\t\t\t{}_pe <= '0';\n\t\t\t\t{}_eval <= '0';", cur.name, cur.name));
                setup.pe_signal_assignments.push(format!(
                    "\n\t\t\t\t\t{}_pe <= {}_en;",
                    cur.name,
                    cur.name,));
            let sw_dependencies = self.ir.get_used_windows_in_expr(&cur.eval.clauses[0].expression);
            sw_dependencies.iter().for_each(|cur_window| {
                let window = self.ir.sliding_window(*cur_window);
                let sw_target = self.ir.get_name_for_stream_ref(window.target);
                let sw_window_ty = get_str_for_sw_op(window.op);
                let window_name = format!("{}_{}_{}", sw_target, sw_window_ty, cur_window.idx());
                let sw_exp = format!("{}.aggregate(over: {} s, using: {})", sw_target, window.duration.as_secs_f64(), sw_window_ty);
                setup.sw_request_signal_assignments.push(format!(
                    "\n\t\t\t\t\t--* {} has Source {}\n\t\t\t\t\t{}_request <= {}_en and upd_and_pe_done and (not {}_en or {}_upd_done);",
                    sw_exp,
                    cur.name,
                    window_name,
                    cur.name,
                    sw_target,
                    window_name));
                setup
                    .eval_done_port_assignment
                    .push(format!(" and (not {}_en or {}_request_done)", cur.name, window_name));
            });
            let stream_dependencies_from_expr = OutputStreamVHDL::generate_dependencies_in_expr(&cur.eval.clauses[0].expression);
            let no_zero_offset = ! cur.accesses.iter().any(|(stream, accesses)| {
                self.ir.is_output_reference(*stream) &&
                    accesses.iter().any(|(_,cur)|*cur == StreamAccessKind::Offset( Offset::Past(0)) || *cur == StreamAccessKind::Sync
                    )} && stream_dependencies_from_expr.iter().any(|cur_in_expr| *cur_in_expr == *stream));
            let pre_annotation = format!("\n\t\t\t\t\t--* {}\n\t\t\t\t\t--* Evaluation Phase of Output Stream {} is Influenced by", annotation, cur.name);
            if sw_dependencies.is_empty() && no_zero_offset {
                setup.eval_signal_assignments.push(format!(
                    "{} No Lookup\n\t\t\t\t\t{}_eval <= {}_en and upd_and_pe_done;",
                    pre_annotation,
                    cur.name,
                    cur.name));
            } else {
                let mut sw_dep_vhdl = Vec::new();
                let mut sw_dep_string = Vec::new();
                sw_dependencies.iter().for_each(|cur_window|{
                    let window = self.ir.sliding_window(*cur_window);
                    let sw_target = self.ir.get_name_for_stream_ref(window.target);
                    let sw_window_ty = get_str_for_sw_op(window.op);
                    let window_name = format!("{}_{}_{}", sw_target, sw_window_ty, cur_window.idx());
                    sw_dep_vhdl.push(format!(" and {}_request_done", window_name));
                    sw_dep_string.push(format!("\n\t\t\t\t\t--* - Window Lookup: {}.aggregate(over: {} s, using: {})", sw_target, window.duration.as_secs_f64(), sw_window_ty));
                });
                let mut stream_dep_vhdl = Vec::new();
                let mut stream_dep_string = Vec::new();
                cur.accesses.iter().for_each(|(stream, accesses)| {
                    let has_zero_offset = accesses.iter().any(|(_, cur)| *cur == StreamAccessKind::Hold || *cur == StreamAccessKind::Sync || *cur == StreamAccessKind::Offset( Offset::Past(0)));
                    match stream {
                        StreamReference::Out(_) => {
                            let is_not_hold = stream_dependencies_from_expr.iter().any(|cur| *cur == *stream);
                            if has_zero_offset {
                                if is_not_hold {
                                    let stream = self.ir.stream(*stream).name();
                                    stream_dep_vhdl.push(format!(" and {}_eval_done", stream));
                                    stream_dep_string.push(format!("\n\t\t\t\t\t--* - Synchronous Lookup: {}", stream));
                                } else {
                                    let stream = self.ir.stream(*stream).name();
                                    stream_dep_vhdl.push(format!(" and (not {}_en or {}_eval_done)", stream, stream));
                                    stream_dep_string.push(format!("\n\t\t\t\t\t--* - Sample & Hold Lookup: {}", stream));
                                }
                            }
                        }
                        StreamReference::In(_) => {}
                    }
                });
                setup.eval_signal_assignments.push(format!(
                    "{} the following Lookups: {}{}\n\t\t\t\t\t{}_eval <= {}_en and upd_and_pe_done{}{};",
                    pre_annotation,
                    sw_dep_string.concat(),
                    stream_dep_string.concat(),
                    cur.name,
                    cur.name,
                sw_dep_vhdl.concat(), stream_dep_vhdl.concat()));
            }
            setup.output_port_assignments.push(format!("\n\t{} <= {}_entity_data_0;", cur.name, cur.name));
            setup.valid_port_assignment.push(format!(" and {}_entity_data_valid_0", cur.name));
            setup.udp_pe_done_port_assignment.push(format!(" and (not {}_en or {}_pe_done)", cur.name, cur.name));
            setup.eval_done_port_assignment.push(format!(" and (not {}_en or {}_eval_done)", cur.name, cur.name));
            setup.print_output_stream_names.push(format!("\n\t\t\t\t\t--* - {}",cur.name));
            setup.print_input_streams.push(format!("--* {}\n", annotation));
        });
        self.ir.sliding_windows.iter().for_each(|cur| {
            let sw_target = self.ir.get_name_for_stream_ref(cur.target);
            let sw_ty = get_vhdl_type(&cur.ty);
            let sw_window_ty = get_str_for_sw_op(cur.op);
            let sw_name = format!("{}_{}_{}", sw_target, sw_window_ty, cur.reference.idx());
            let annotation =
                format!("{}.aggregate(over: {} s, using: {})", sw_target, cur.duration.as_secs_f64(), sw_window_ty);
            setup.component_declaration.push(format!(
                "\t--* {}\n{}",
                annotation,
                VHDLGenerator::generate(&SlidingWindowComponent::new(cur, self.ir), &tera)
            ));
            setup.component_instantiation.push(format!(
                "\t--* {}\n{}",
                annotation,
                VHDLGenerator::generate(&SlidingWindowMonitorEntity::new(cur, self.ir), &tera)
            ));
            setup.signals.push(format!("\tsignal {}_evict : std_logic;\n", sw_name));
            setup.signals.push(format!("\tsignal {}_upd : std_logic;\n", sw_name));
            setup.signals.push(format!("\tsignal {}_request : std_logic;\n", sw_name));
            setup.signals.push(format!("\tsignal {}_entity_data : {};\n", sw_name, sw_ty));
            setup.signals.push(format!("\tsignal {}_entity_data_valid : std_logic;\n", sw_name));
            setup.signals.push(format!("\tsignal {}_evict_done : std_logic;\n", sw_name));
            setup.signals.push(format!("\tsignal {}_upd_done : std_logic;\n", sw_name));
            setup.signals.push(format!("\tsignal {}_request_done : std_logic;\n", sw_name));
            setup.disable_en_assignments.push(format!(
                "\n\t\t\t\t{}_evict <= '0';\n\t\t\t\t{}_upd <= '0';\n\t\t\t\t{}_request <= '0';",
                sw_name, sw_name, sw_name
            ));
            setup.evict_signal_assignments.push(format!("\n\t\t\t\t\t{}_evict <= '1';", sw_name));
            let sw_en = if self.ir.is_output_reference(cur.target) {
                format!("{}_eval_done", sw_target)
            } else {
                format!("{}_upd_done", sw_target)
            };
            setup.sw_update_signal_assignments.push(format!(
                "\n\t\t\t\t\t--* - {} aggregates over {}\n\t\t\t\t\t{}_upd <= {} and upd_and_pe_done;",
                annotation, sw_target, sw_name, sw_en
            ));
            setup.udp_pe_done_port_assignment.push(format!(" and (not {}_en or {}_evict_done)", sw_target, sw_name));
            setup.eval_done_port_assignment.push(format!(" and (not {}_en or {}_upd_done)", sw_target, sw_name));
            setup.print_sliding_windows.push(format!("\n\t\t\t\t\t--* - {}", annotation))
        });
        setup
    }
}

#[cfg(test)]
mod monitor_test {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_evaluator_file() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let tera_files = "templates".to_string();
        let evaluator = Evaluator::new(&lola_instance, tera_files);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        VHDLGenerator::generate_and_create(&evaluator, &tera, &PathBuf::from("target/test_files"));
    }

    #[test]
    fn generate_single_input_monitor() {
        let example_file_content = "input boolTest : Bool";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let tera_files = "templates".to_string();
        let stream = Evaluator::new(&lola_instance, tera_files);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        //monitor entity
        let entity_first_pos = result.find("entity evaluator").expect("expected entity declaration");
        let entity_last_pos = result.find("end evaluator").expect("expected entity declaration");
        let entity_result = &result[entity_first_pos..entity_last_pos];
        let entity_result: Vec<&str> = entity_result.split("\n").collect();
        assert_eq!(entity_result[4].trim(), "boolTest : in std_logic;");
        assert_eq!(entity_result[6].trim(), "done : out std_logic;");
        assert_eq!(entity_result[7].trim(), "valid : out std_logic");
        // en signal assignments
        let en_first_pos = result.find("-- Input Stream Updates").expect("expected input update");
        let en_last_pos = result.find("-- Eval Phase").expect("expected eval phase");
        let en_result = &result[en_first_pos..en_last_pos];
        let en_result: Vec<&str> = en_result.split("\n").collect();
        assert_eq!(en_result[3].trim(), "boolTest_upd <= boolTest_en;");
        // output port assignment
        let out_port_first_pos = result.find("-- Eval Phase").expect("expected eval phase");
        let out_port_last_pos = result.find(" -- Valid Assignment").expect("expected valid assignment");
        let out_port_result = &result[out_port_first_pos..out_port_last_pos];
        let out_port_result: Vec<&str> = out_port_result.split("\n").collect();
        assert_eq!(out_port_result[1].trim(), "-- SW Update Phase");
        // valid port assignment
        let valid_port_first_pos = result.find(" -- Valid Assignment").expect("expected valid assignment");
        let valid_port_last_pos =
            result.find("-- Evaluator Done assignment").expect("expected low_level_controller done assignment");
        let valid_port_result = &result[valid_port_first_pos..valid_port_last_pos];
        let valid_port_result: Vec<&str> = valid_port_result.split("\n").collect();
        assert_eq!(valid_port_result[1].trim(), "valid_reg <= '1';");
    }

    #[test]
    fn generate_single_input_single_output_monitor() {
        let example_file_content = "input a : Int8\noutput b := a[-1].defaults(to:0) + 5";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let tera_files = "templates".to_string();
        let stream = Evaluator::new(&lola_instance, tera_files);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        let result = VHDLGenerator::generate(&stream, &tera);
        //monitor entity
        let entity_first_pos = result.find("entity evaluator").expect("expected entity declaration");
        let entity_last_pos = result.find("end evaluator").expect("expected entity declaration");
        let entity_result = &result[entity_first_pos..entity_last_pos];
        let entity_result: Vec<&str> = entity_result.split("\n").collect();
        assert_eq!(entity_result[4].trim(), "a : in signed(7 downto 0);");
        assert_eq!(entity_result[7].trim(), "b : out signed(7 downto 0);");
        assert_eq!(entity_result[8].trim(), "done : out std_logic;");
        assert_eq!(entity_result[9].trim(), "valid : out std_logic");
        // en signal assignments
        let en_first_pos = result.find("-- Input Stream Updates").expect("expected input update");
        let en_last_pos = result.find("-- Eval Phase").expect("expected eval phase");
        let en_result = &result[en_first_pos..en_last_pos];
        let en_result: Vec<&str> = en_result.split("\n").collect();
        assert_eq!(en_result[3].trim(), "a_upd <= a_en;");
        assert_eq!(en_result[7].trim(), "b_pe <= b_en;");
        // output port assignment
        let out_port_first_pos = result.find("-- Eval Phase").expect("expected eval phase");
        let out_port_last_pos = result.find(" -- Valid Assignment").expect("expected valid assignment");
        let out_port_result = &result[out_port_first_pos..out_port_last_pos];
        let out_port_result: Vec<&str> = out_port_result.split("\n").collect();
        assert_eq!(out_port_result[3].trim(), "b_eval <= b_en and upd_and_pe_done;");
        assert_eq!(out_port_result[4].trim(), "-- SW Update Phase");
        // valid port assignment
        let valid_port_first_pos = result.find(" -- Valid Assignment").expect("expected valid assignment");
        let valid_port_last_pos =
            result.find("-- Evaluator Done assignment").expect("expected low_level_controller done assignment");
        let valid_port_result = &result[valid_port_first_pos..valid_port_last_pos];
        let valid_port_result: Vec<&str> = valid_port_result.split("\n").collect();
        assert_eq!(valid_port_result[1].trim(), "valid_reg <= '1' and b_entity_data_valid_0;");
    }
}
