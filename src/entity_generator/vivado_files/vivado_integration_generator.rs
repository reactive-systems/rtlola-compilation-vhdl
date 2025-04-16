use crate::entity_generator::vivado_files::RegisterStatistic;
use crate::entity_generator::GenerateVhdlCode;
use crate::vhdl_wrapper::type_serialize::{
    get_values_for_register_mapping, get_vhdl_initial_type, RegisterMappingEnum,
};
use rtlola_frontend::RtLolaMir;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct VivadoIntegration<'a> {
    pub(crate) ir: &'a RtLolaMir,
    pub(crate) regs: &'a RegisterStatistic,
}

impl<'a> VivadoIntegration<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir, regs: &'a RegisterStatistic) -> VivadoIntegration<'a> {
        VivadoIntegration { ir, regs }
    }
}

impl GenerateVhdlCode for VivadoIntegration<'_> {
    fn template_name(&self) -> String {
        "vivado_integration.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "vivado_integration.vhdl".to_string()
    }
}

impl Serialize for VivadoIntegration<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_vivado_integration_setup();
        let mut s = serializer.serialize_struct("VIVADO_Integration", 9)?;
        s.serialize_field("output_values_signal_declaration", &setup.output_values_signal_declaration.concat())?;
        s.serialize_field(
            "input_values_in_component_declaration",
            &setup.input_values_in_component_declaration.concat(),
        )?;
        s.serialize_field(
            "output_values_in_component_declaration",
            &setup.output_values_in_component_declaration.concat(),
        )?;
        s.serialize_field(
            "process_declaration_for_write_output_values",
            &setup.process_declaration_for_write_output_values.concat(),
        )?;
        s.serialize_field("input_registers", &setup.input_registers.concat())?;
        s.serialize_field("output_registers", &setup.output_registers.concat())?;
        s.serialize_field("fill_registers", &setup.fill_registers.concat())?;
        s.serialize_field(
            "input_values_in_component_instantiation",
            &setup.input_values_in_component_instantiation.concat(),
        )?;
        s.serialize_field(
            "output_values_in_component_instantiation",
            &setup.output_values_in_component_instantiation.concat(),
        )?;
        s.end()
    }
}

struct VivadoIntegrationSetup {
    output_values_signal_declaration: Vec<String>,
    input_values_in_component_declaration: Vec<String>,
    output_values_in_component_declaration: Vec<String>,
    process_declaration_for_write_output_values: Vec<String>,
    input_registers: Vec<String>,
    output_registers: Vec<String>,
    fill_registers: Vec<String>,
    input_values_in_component_instantiation: Vec<String>,
    output_values_in_component_instantiation: Vec<String>,
}

impl VivadoIntegrationSetup {
    fn new() -> VivadoIntegrationSetup {
        VivadoIntegrationSetup {
            output_values_signal_declaration: Vec::new(),
            input_values_in_component_declaration: Vec::new(),
            output_values_in_component_declaration: Vec::new(),
            process_declaration_for_write_output_values: Vec::new(),
            input_registers: Vec::new(),
            output_registers: Vec::new(),
            fill_registers: Vec::new(),
            input_values_in_component_instantiation: Vec::new(),
            output_values_in_component_instantiation: Vec::new(),
        }
    }
}

impl VivadoIntegration<'_> {
    fn generate_vivado_integration_setup(&self) -> VivadoIntegrationSetup {
        let mut setup = VivadoIntegrationSetup::new();
        let mut num_registers_input_values = 0;
        let mut num_registers_output_values = 0;
        let mut num_input_pos = 0;
        let num_bit_for_register_rep = log_2(i32::from(self.regs.total_num_registers)) + 1;
        setup.fill_registers.push(format!(
            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= slv_reg0;",
            num_as_fix_bit_rep(0, num_bit_for_register_rep)
        ));
        setup.fill_registers.push(format!(
            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= slv_reg1;",
            num_as_fix_bit_rep(1, num_bit_for_register_rep)
        ));
        setup.fill_registers.push(format!(
            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= slv_reg2;",
            num_as_fix_bit_rep(2, num_bit_for_register_rep)
        ));
        for i in 0..self.regs.total_num_registers_new_input {
            setup.fill_registers.push(format!(
                "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= slv_reg{};",
                num_as_fix_bit_rep(3 + i, num_bit_for_register_rep),
                3 + i
            ));
        }
        let time_reg = 1 + self.regs.total_num_registers_new_input + self.regs.total_num_registers_input_values;
        setup.output_registers.push(format!(
            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= time_stream(31 downto 0);",
            num_as_fix_bit_rep(time_reg, num_bit_for_register_rep),
        ));
        setup.output_registers.push(format!(
            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= time_stream(63 downto 32);",
            num_as_fix_bit_rep(time_reg + 1, num_bit_for_register_rep),
        ));
        self.ir.inputs.iter().for_each(|cur| {
            if cur.name.as_str() != "time" {
                let vhdl_init_type = get_vhdl_initial_type(&cur.ty);
                let input_reg_number = 3 + self.regs.total_num_registers_new_input + num_registers_input_values;
                let output_reg_number = 1
                    + self.regs.total_num_registers_new_input
                    + self.regs.total_num_registers_input_values
                    + num_registers_output_values
                    + 2;

                setup.input_values_in_component_declaration.push(format!(
                    "\n\t\t\t{}_data_in : in {};\n\t\t\t{}_data_in_new_input : in std_logic;",
                    cur.name, vhdl_init_type, cur.name
                ));
                setup
                    .output_values_in_component_declaration
                    .push(format!("\n\t\t\t{}_stream : out {};", cur.name, vhdl_init_type));
                setup
                    .output_values_signal_declaration
                    .push(format!("\n\tsignal {}_stream : {};", cur.name, vhdl_init_type));
                match get_values_for_register_mapping(&cur.ty) {
                    RegisterMappingEnum::BoolRegister => {
                        setup.input_values_in_component_instantiation.push(format!(
                            "\n\t\t\t{}_data_in => slv_reg{}(0),",
                            cur.name,
                            num_registers_input_values + self.regs.total_num_registers_new_input + 3,
                        ));
                        setup.input_registers.push(format!(
                            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= slv_reg{};",
                            num_as_fix_bit_rep(input_reg_number, num_bit_for_register_rep),
                            input_reg_number
                        ));
                        setup.output_registers.push(format!(
                            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out(0) <= {}_stream;\n\t\t\treg_data_out(31 downto 1) <= (others => '0');",
                            num_as_fix_bit_rep(output_reg_number, num_bit_for_register_rep),
                            cur.name,
                        ));
                        num_registers_input_values += 1;
                        num_registers_output_values += 1;
                    },
                    RegisterMappingEnum::ReducedIntRegister(range1, range2) => {
                        setup.input_values_in_component_instantiation.push(format!(
                            "\n\t\t\t{}_data_in => slv_reg{}{},",
                            cur.name,
                            num_registers_input_values + self.regs.total_num_registers_new_input + 3,
                            range1
                        ));
                        setup.input_registers.push(format!(
                            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= slv_reg{};",
                            num_as_fix_bit_rep(input_reg_number, num_bit_for_register_rep),
                            input_reg_number
                        ));
                        setup.output_registers.push(format!(
                            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out{} <= {}_stream;\n\t\t\treg_data_out{} <= (others => '0');",
                            num_as_fix_bit_rep(output_reg_number, num_bit_for_register_rep),
                            range1,
                            cur.name,
                            range2,
                        ));
                        num_registers_input_values += 1;
                        num_registers_output_values += 1;
                    }
                    RegisterMappingEnum::WholeIntRegister | RegisterMappingEnum::FloatRegister => {
                        setup.input_values_in_component_instantiation.push(format!(
                            "\n\t\t\t{}_data_in => slv_reg{},",
                            cur.name,
                            num_registers_input_values + self.regs.total_num_registers_new_input + 3,
                        ));
                        setup.input_registers.push(format!(
                            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= slv_reg{};",
                            num_as_fix_bit_rep(input_reg_number, num_bit_for_register_rep),
                            input_reg_number
                        ));
                        setup.output_registers.push(format!(
                            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= {}_stream;",
                            num_as_fix_bit_rep(output_reg_number, num_bit_for_register_rep),
                            cur.name,
                        ));
                        num_registers_input_values += 1;
                        num_registers_output_values += 1;
                    },
                    RegisterMappingEnum::TwoIntRegisters | RegisterMappingEnum::DoubleRegister => {
                        setup.input_values_in_component_instantiation.push(format!(
                            "\n\t\t\t{}_data_in(31 downto 0) => slv_reg{},\n\t\t\t{}_data_in(63 downto 32) => slv_reg{},",
                            cur.name,
                            num_registers_input_values + self.regs.total_num_registers_new_input + 3,
                            cur.name,
                            num_registers_input_values + self.regs.total_num_registers_new_input + 3 + 1
                        ));
                        setup.input_registers.push(format!(
                            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= slv_reg{};",
                            num_as_fix_bit_rep(input_reg_number, num_bit_for_register_rep),
                            input_reg_number
                        ));
                        setup.input_registers.push(format!(
                            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= slv_reg{};",
                            num_as_fix_bit_rep(input_reg_number + 1, num_bit_for_register_rep),
                            input_reg_number + 1
                        ));
                        setup.output_registers.push(format!(
                            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= {}_stream(31 downto 0);",
                            num_as_fix_bit_rep(output_reg_number, num_bit_for_register_rep),
                            cur.name,
                        ));
                        setup.output_registers.push(format!(
                            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= {}_stream(63 downto 32);",
                            num_as_fix_bit_rep(output_reg_number + 1, num_bit_for_register_rep),
                            cur.name,
                        ));
                        num_registers_input_values += 2;
                        num_registers_output_values += 2;
                    }
                }
                setup.input_values_in_component_instantiation.push(format!(
                    "\n\t\t\t{}_data_in_new_input => slv_reg{}({}),",
                    cur.name,
                    num_input_pos / 32 + 3,
                    num_input_pos % 32
                ));
                num_input_pos += 1;
                setup
                    .output_values_in_component_instantiation
                    .push(format!("\n\t\t\t{}_stream => {}_stream,", cur.name, cur.name));
                setup.process_declaration_for_write_output_values.push(format!(" {}_stream,", cur.name));
            }
        });
        self.ir.outputs.iter().for_each(|cur| {
            let vhdl_init_type = get_vhdl_initial_type(&cur.ty);
            let output_reg_number = 3
                + self.regs.total_num_registers_new_input
                + self.regs.total_num_registers_input_values
                + num_registers_output_values;
            setup
                .output_values_in_component_declaration
                .push(format!("\n\t\t\t{}_stream : out {};", cur.name, vhdl_init_type));
            setup
                .output_values_signal_declaration
                .push(format!("\n\tsignal {}_stream : {};", cur.name, vhdl_init_type));
            setup
                .output_values_in_component_instantiation
                .push(format!("\n\t\t\t{}_stream => {}_stream,", cur.name, cur.name));
            setup.process_declaration_for_write_output_values.push(format!("{}_stream,", cur.name));
            match get_values_for_register_mapping(&cur.ty) {
                RegisterMappingEnum::BoolRegister => {
                    num_registers_output_values += 1;
                    setup.output_registers.push(format!(
                        "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out(0) <= {}_stream;\n\t\t\treg_data_out(31 downto 1) <= (others => '0');",
                        num_as_fix_bit_rep(output_reg_number, num_bit_for_register_rep),
                        cur.name,
                    ));
                }
                RegisterMappingEnum::ReducedIntRegister(range1,range2) => {
                    num_registers_output_values += 1;
                    setup.output_registers.push(format!(
                        "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out{} <= {}_stream;\n\t\t\treg_data_out{} <= (others => '0');",
                        num_as_fix_bit_rep(output_reg_number, num_bit_for_register_rep),
                        range1,
                        cur.name,
                        range2
                    ));
                }
                RegisterMappingEnum::WholeIntRegister | RegisterMappingEnum::FloatRegister => {
                    num_registers_output_values += 1;
                    setup.output_registers.push(format!(
                        "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= {}_stream;",
                        num_as_fix_bit_rep(output_reg_number, num_bit_for_register_rep),
                        cur.name,
                    ));
                }
                RegisterMappingEnum::TwoIntRegisters | RegisterMappingEnum::DoubleRegister => {
                    num_registers_output_values += 2;
                    setup.output_registers.push(format!(
                        "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= {}_stream(31 downto 0);",
                        num_as_fix_bit_rep(output_reg_number, num_bit_for_register_rep),
                        cur.name,
                    ));
                    setup.output_registers.push(format!(
                        "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out <= {}_stream(63 downto 32);",
                        num_as_fix_bit_rep(output_reg_number + 1, num_bit_for_register_rep),
                        cur.name,
                    ));
                }
            }
        });
        setup.output_registers.push(format!(
            "\n\t\twhen b\"{}\" => \n\t\t\treg_data_out(0) <= lost_data;",
            num_as_fix_bit_rep(self.regs.total_num_registers - 1, num_bit_for_register_rep),
        ));
        setup
    }
}

const fn num_bits<T>() -> usize {
    size_of::<T>() * 8
}

fn log_2(x: i32) -> u32 {
    assert!(x > 0);
    num_bits::<i32>() as u32 - x.leading_zeros() - 1
}

fn num_as_fix_bit_rep(num: u16, num_bits: u32) -> String {
    let mut res = String::new();
    let mut num = num;
    for _i in 0..num_bits {
        let bit = num % 2;
        num /= 2;
        res = format!("{}{}", res, bit)
    }
    res.chars().rev().collect::<String>()
}

#[cfg(test)]
mod vivado_integration_file_tests {
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
            "input bool_val: Bool\ninput a : Int8\n input b :Int8\noutput c @1Hz := a.hold().defaults(to:0) + 3\noutput d @2Hz := a.hold().defaults(to:0) + 6\noutput e := a + b\noutput fal: Bool := bool_val && ! bool_val";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let reg_stat = RegisterStatistic::new(&lola_instance);
        let implementation = VivadoIntegration::new(&lola_instance, &reg_stat);
        let tera: Tera = compile_templates!("templates/vivado_file_changes/*");
        VHDLGenerator::generate_and_create(&implementation, &tera, &PathBuf::from("target/test_files"))
    }
}
