use crate::entity_generator::vivado_files::RegisterStatistic;
use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::{get_values_for_register_mapping, RegisterMappingEnum};
use rtlola_frontend::RtLolaMir;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct Macros<'a> {
    pub(crate) ir: &'a RtLolaMir,
    pub(crate) regs: &'a RegisterStatistic,
}

impl<'a> Macros<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir, regs: &'a RegisterStatistic) -> Macros<'a> {
        Macros { ir, regs }
    }
}

impl GenerateVhdlCode for Macros<'_> {
    fn template_name(&self) -> String {
        "macros.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "macros.h".to_string()
    }
}

impl Serialize for Macros<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_macros_setup();
        let mut s = serializer.serialize_struct("Macros", 3)?;
        s.serialize_field("new_input", &setup.new_input.concat())?;
        s.serialize_field("input_values", &setup.input_values.concat())?;
        s.serialize_field("output_values", &setup.output_values.concat())?;
        s.end()
    }
}

struct MacrosSetup {
    new_input: Vec<String>,
    input_values: Vec<String>,
    output_values: Vec<String>,
}

impl MacrosSetup {
    fn new() -> MacrosSetup {
        MacrosSetup { new_input: Vec::new(), input_values: Vec::new(), output_values: Vec::new() }
    }
}

impl Macros<'_> {
    fn generate_macros_setup(&self) -> MacrosSetup {
        let mut setup = MacrosSetup::new();
        let mut num_registers_input_values = 0;
        let mut num_registers_output_values = 0;
        let mut num_input_pos = 0;
        num_registers_input_values += 2; //time register
        for i in 0..self.regs.total_num_registers_new_input {
            setup.new_input.push(format!("\n#define NEW_INPUT_SINGLE_STREAMS_REG_{} REG_{}", i, i + 3));
        }
        let time_reg = self.regs.total_setup_reg
            + self.regs.total_num_registers_new_input
            + self.regs.total_num_registers_input_values;
        setup.output_values.push(format!("\n#define TIME_STREAM_REG_LOW REG_{}", time_reg));
        setup.output_values.push(format!("\n#define TIME_STREAM_REG_HIGH REG_{}", time_reg + 1));
        self.ir.inputs.iter().for_each(|cur| {
            if cur.name.as_str() != "time" {
                let input_reg_number =
                    self.regs.total_setup_reg + self.regs.total_num_registers_new_input + num_registers_input_values;
                let output_reg_number = self.regs.total_setup_reg
                    + self.regs.total_num_registers_new_input
                    + self.regs.total_num_registers_input_values
                    + num_registers_output_values
                    + 2;
                setup.input_values.push(format!("\n#define {}_NEW_INPUT_POS {}", cur.name, num_input_pos % 32));
                num_input_pos += 1;
                match get_values_for_register_mapping(&cur.ty) {
                    RegisterMappingEnum::BoolRegister
                    | RegisterMappingEnum::ReducedIntRegister(_, _)
                    | RegisterMappingEnum::WholeIntRegister
                    | RegisterMappingEnum::FloatRegister => {
                        setup.input_values.push(format!("\n#define {}_REG REG_{}", cur.name, input_reg_number));
                        setup
                            .output_values
                            .push(format!("\n#define {}_STREAM_REG REG_{}", cur.name, output_reg_number));
                        num_registers_input_values += 1;
                        num_registers_output_values += 1;
                    }
                    RegisterMappingEnum::DoubleRegister | RegisterMappingEnum::TwoIntRegisters => {
                        setup.input_values.push(format!("\n#define {}_LOW_REG REG_{}", cur.name, input_reg_number));
                        setup.input_values.push(format!(
                            "\n#define {}_HIGH_REG REG_{}",
                            cur.name,
                            input_reg_number + 1
                        ));
                        setup
                            .output_values
                            .push(format!("\n#define {}_STREAM_LOW_REG REG_{}", cur.name, output_reg_number));
                        setup.output_values.push(format!(
                            "\n#define {}_STREAM_HIGH_REG REG_{}",
                            cur.name,
                            output_reg_number + 1
                        ));
                        num_registers_input_values += 2;
                        num_registers_output_values += 2;
                    }
                }
            }
        });
        self.ir.outputs.iter().for_each(|cur| {
            let output_reg_number = 3
                + self.regs.total_num_registers_new_input
                + self.regs.total_num_registers_input_values
                + num_registers_output_values;
            match get_values_for_register_mapping(&cur.ty) {
                RegisterMappingEnum::BoolRegister
                | RegisterMappingEnum::ReducedIntRegister(_, _)
                | RegisterMappingEnum::WholeIntRegister
                | RegisterMappingEnum::FloatRegister => {
                    setup.output_values.push(format!("\n#define {}_STREAM_REG REG_{}", cur.name, output_reg_number));
                    num_registers_output_values += 1;
                }
                RegisterMappingEnum::DoubleRegister | RegisterMappingEnum::TwoIntRegisters => {
                    setup
                        .output_values
                        .push(format!("\n#define {}_STREAM_LOW_REG REG_{}", cur.name, output_reg_number));
                    setup.output_values.push(format!(
                        "\n#define {}_STREAM_HIGH_REG REG_{}",
                        cur.name,
                        output_reg_number + 1
                    ));
                    num_registers_output_values += 2;
                }
            }
        });
        setup
    }
}

#[cfg(test)]
mod macros_file_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use std::path::PathBuf;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn generate_implementation_file() {
        let example_file_content =
            "input a : Int8 input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let reg_stat = RegisterStatistic::new(&lola_instance);
        let implementation = Macros::new(&lola_instance, &reg_stat);
        let tera: Tera = compile_templates!("templates/vivado_file_changes/*");
        VHDLGenerator::generate_and_create(&implementation, &tera, &PathBuf::from("target/test_files"))
    }
}
