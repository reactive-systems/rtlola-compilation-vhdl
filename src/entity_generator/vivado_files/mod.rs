use crate::entity_generator::VHDLGenerator;
use crate::vhdl_wrapper::type_serialize::{get_values_for_register_mapping, RegisterMappingEnum};
use crate::Config;
use rtlola_frontend::RtLolaMir;

pub(crate) mod communication_and_board_setup_generator;
pub(crate) mod convert_bytes_to_ctypes_generator;
pub(crate) mod convert_ctypes_to_bytes_generator;
pub(crate) mod fpga_high_level_communication_generator;
pub(crate) mod fpga_low_level_communication_generator;
pub(crate) mod macros_generator;
pub(crate) mod main_cfile_generator;
pub(crate) mod upd_communication_generator;
pub(crate) mod vivado_integration_generator;

pub(crate) struct RegisterStatistic {
    pub(crate) total_setup_reg: u16,
    pub(crate) total_num_registers_new_input: u16,
    pub(crate) total_num_registers_input_values: u16,
    pub(crate) total_num_registers: u16,
}

impl RegisterStatistic {
    pub(crate) fn new(ir: &RtLolaMir) -> RegisterStatistic {
        let total_setup_reg = 1;
        let total_num_registers_new_input =
            (&ir.inputs.len() / 32 + (if ir.inputs.len() % 32 != 0 { 1 } else { 0 })) as u16;
        let mut num_registers_input_values = 2;
        let mut num_registers_output_values = 2;
        ir.inputs.iter().for_each(|cur| {
            if cur.name.as_str() != "time" {
                match get_values_for_register_mapping(&cur.ty) {
                    RegisterMappingEnum::DoubleRegister | RegisterMappingEnum::TwoIntRegisters => {
                        num_registers_input_values += 2;
                        num_registers_output_values += 2;
                    }
                    _ => {
                        num_registers_input_values += 1;
                        num_registers_output_values += 1;
                    }
                }
            }
        });
        ir.outputs.iter().for_each(|cur| match get_values_for_register_mapping(&cur.ty) {
            RegisterMappingEnum::DoubleRegister | RegisterMappingEnum::TwoIntRegisters => {
                num_registers_output_values += 2;
            }
            _ => num_registers_output_values += 1,
        });
        let total_num_registers =
            1 + total_num_registers_new_input + num_registers_input_values + num_registers_output_values + 1;
        println!("Number setup register: {}", total_setup_reg);
        println!("Number new input register: {}", total_num_registers_new_input);
        println!("Number input value register: {}", num_registers_input_values);
        println!("Number output values register: {}", num_registers_output_values);
        println!("Number total register: {}", total_num_registers);
        RegisterStatistic {
            total_setup_reg,
            total_num_registers_new_input,
            total_num_registers,
            total_num_registers_input_values: num_registers_input_values,
        }
    }
}

pub(crate) fn generate_vivado_files(config: &Config) {
    let mut target = config.target.clone();
    target.push("vivado_files/");
    let tera_files = config.templates.clone() + "/vivado_file_changes/*";
    let tera = tera::compile_templates!(&tera_files);
    let reg_stat = RegisterStatistic::new(&config.ir);
    VHDLGenerator::generate_and_create(
        &communication_and_board_setup_generator::CommunicationAndBoardSetup::new(),
        &tera,
        &target,
    );
    VHDLGenerator::generate_and_create(&convert_bytes_to_ctypes_generator::ConvertBytesToCTypes::new(), &tera, &target);
    VHDLGenerator::generate_and_create(&convert_ctypes_to_bytes_generator::ConvertCTypesToBytes::new(), &tera, &target);
    VHDLGenerator::generate_and_create(
        &fpga_high_level_communication_generator::FPGAHighLevelCommunication::new(&config.ir, &reg_stat, config.mode),
        &tera,
        &target,
    );
    VHDLGenerator::generate_and_create(
        &fpga_low_level_communication_generator::FPGALowLevelCommunication::new(),
        &tera,
        &target,
    );
    VHDLGenerator::generate_and_create(&macros_generator::Macros::new(&config.ir, &reg_stat), &tera, &target);
    VHDLGenerator::generate_and_create(&main_cfile_generator::MainCFileGenerator::new(), &tera, &target);
    VHDLGenerator::generate_and_create(&upd_communication_generator::UDPCommunication::new(), &tera, &target);
    VHDLGenerator::generate_and_create(
        &vivado_integration_generator::VivadoIntegration::new(&config.ir, &reg_stat),
        &tera,
        &target,
    );
}
