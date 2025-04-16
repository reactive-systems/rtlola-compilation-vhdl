use crate::entity_generator::vivado_files::RegisterStatistic;
use crate::entity_generator::GenerateVhdlCode;
use crate::static_constants::{FLOAT_32_NUMBER_AFTER_POINT_POW_TEN, FLOAT_64_NUMBER_AFTER_POINT_POW_TEN};
use crate::vhdl_wrapper::type_serialize::{
    get_atomic_ty, get_c_type, get_format_string_for_ty, get_value_for_Ty, get_values_for_register_mapping,
    RegisterMappingEnum,
};
use rtlola_frontend::mir::{IntTy, Type};
use rtlola_frontend::RtLolaMir;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct FPGAHighLevelCommunication<'a> {
    pub(crate) ir: &'a RtLolaMir,
    pub(crate) regs: &'a RegisterStatistic,
    pub(crate) mode: bool,
}

impl<'a> FPGAHighLevelCommunication<'a> {
    pub(crate) fn new(ir: &'a RtLolaMir, regs: &'a RegisterStatistic, mode: bool) -> FPGAHighLevelCommunication<'a> {
        FPGAHighLevelCommunication { ir, regs, mode }
    }
}

impl GenerateVhdlCode for FPGAHighLevelCommunication<'_> {
    fn template_name(&self) -> String {
        "fpga_high_level_communication.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        "fpga_high_level_communication.h".to_string()
    }
}

impl Serialize for FPGAHighLevelCommunication<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let setup = self.generate_fpga_high_level_communication();
        let mut s = serializer.serialize_struct("FPGAHighLevelcommunication", 21)?;
        s.serialize_field("input_line_new_inputs", &setup.input_line_new_inputs.concat())?;
        s.serialize_field("input_line_input_values", &setup.input_line_input_values.concat())?;
        s.serialize_field("output_line_input_streams", &setup.output_line_input_streams.concat())?;
        s.serialize_field("output_line_output_streams", &setup.output_line_output_streams.concat())?;
        s.serialize_field("set_new_input_values", &setup.set_new_input_values.concat())?;
        s.serialize_field("set_input_values", &setup.set_input_values.concat())?;
        s.serialize_field("get_input_streams", &setup.get_input_streams.concat())?;
        s.serialize_field("get_output_streams", &setup.get_output_streams.concat())?;
        s.serialize_field("input_float_conversions", &setup.input_float_conversions.concat())?;
        s.serialize_field("input_line_format_strings", &setup.input_line_format_strings.concat())?;
        s.serialize_field("input_line_values", &setup.input_line_values.concat())?;
        s.serialize_field("input_line_header", &setup.input_line_header.concat())?;
        s.serialize_field("output_float_conversions", &setup.output_float_conversions.concat())?;
        s.serialize_field("output_line_format_strings", &setup.output_line_format_strings.concat())?;
        s.serialize_field("output_line_values", &setup.output_line_values.concat())?;
        s.serialize_field("output_line_header", &setup.output_line_header.concat())?;
        s.serialize_field("input_line_initialize", &setup.input_line_initialize.concat())?;
        s.serialize_field("output_line_initialize", &setup.output_line_initialize.concat())?;

        let mode_setup =
            if self.mode { "unset_bit_for_reg(REG_0, OFFLINE);" } else { "set_bit_for_reg(REG_0, OFFLINE);" };
        s.serialize_field("mode", mode_setup)?;
        s.serialize_field("input_line_as_bytes", &setup.input_line_as_bytes.concat())?;
        s.serialize_field("output_line_as_bytes", &setup.output_line_as_bytes.concat())?;
        s.end()
    }
}

struct FPGAHighLevelCommunicationSetup {
    input_line_new_inputs: Vec<String>,
    input_line_input_values: Vec<String>,
    output_line_input_streams: Vec<String>,
    output_line_output_streams: Vec<String>,
    set_new_input_values: Vec<String>,
    set_input_values: Vec<String>,
    get_input_streams: Vec<String>,
    get_output_streams: Vec<String>,
    input_float_conversions: Vec<String>,
    input_line_format_strings: Vec<String>,
    input_line_values: Vec<String>,
    input_line_header: Vec<String>,
    output_float_conversions: Vec<String>,
    output_line_format_strings: Vec<String>,
    output_line_values: Vec<String>,
    output_line_header: Vec<String>,
    input_line_initialize: Vec<String>,
    output_line_initialize: Vec<String>,
    input_line_as_bytes: Vec<String>,
    output_line_as_bytes: Vec<String>,
}

impl FPGAHighLevelCommunicationSetup {
    fn new() -> FPGAHighLevelCommunicationSetup {
        FPGAHighLevelCommunicationSetup {
            input_line_new_inputs: Vec::new(),
            input_line_input_values: Vec::new(),
            output_line_input_streams: Vec::new(),
            output_line_output_streams: Vec::new(),
            set_new_input_values: Vec::new(),
            set_input_values: Vec::new(),
            get_input_streams: Vec::new(),
            get_output_streams: Vec::new(),
            input_float_conversions: Vec::new(),
            input_line_format_strings: Vec::new(),
            input_line_values: Vec::new(),
            input_line_header: Vec::new(),
            output_float_conversions: Vec::new(),
            output_line_format_strings: Vec::new(),
            output_line_values: Vec::new(),
            output_line_header: Vec::new(),
            input_line_initialize: Vec::new(),
            output_line_initialize: Vec::new(),
            input_line_as_bytes: Vec::new(),
            output_line_as_bytes: Vec::new(),
        }
    }
}

impl FPGAHighLevelCommunication<'_> {
    fn generate_fpga_high_level_communication(&self) -> FPGAHighLevelCommunicationSetup {
        let mut setup = FPGAHighLevelCommunicationSetup::new();
        let offset = "\t";
        for i in 0..self.regs.total_num_registers_new_input {
            setup.input_line_new_inputs.push(format!("\n\tXuint32 new_inputs_{};", i));
            setup
                .set_new_input_values
                .push(format!("\n\tset_input_for_reg(NEW_INPUT_SINGLE_STREAMS_REG_{}, line->new_inputs_{});", i, i));
        }
        self.ir.inputs.iter().for_each(|cur| {
            let c_type = get_c_type(&cur.ty);
            match get_values_for_register_mapping(&cur.ty) {
                RegisterMappingEnum::BoolRegister
                | RegisterMappingEnum::ReducedIntRegister(_,_)
                | RegisterMappingEnum::WholeIntRegister => {
                    let c_type = c_type.1;
                    setup.input_line_input_values.push(format!("\n\t{} {};", c_type, cur.name));
                    setup.output_line_input_streams.push(format!("\n\t{} {};", c_type, cur.name));
                    setup
                        .set_input_values
                        .push(format!("\n\tset_input_for_reg({}_REG, line->{});", cur.name, cur.name));
                    setup.get_input_streams.push(format!("\n\tline->{} = get_reg({}_STREAM_REG);", cur.name, cur.name));
                    let needed_cast = match get_atomic_ty(&cur.ty) {
                        Type::Int(IntTy::Int8) => "(signed char) ",
                        _ => ""
                    };
                    setup.input_line_values.push(format!(", {}line->{}", needed_cast,cur.name));
                    setup.input_line_format_strings.push(format!("{}, ", get_format_string_for_ty(&cur.ty)));
                    setup.input_line_header.push(format!("{}, ", cur.name));
                    setup.input_line_initialize.push(format!("\n\tline->{} = 0;", cur.name));
                    setup.input_line_as_bytes.push(format!("\n\twrite_{}_to_buff(buff,pos,line->{});\n\tpos += sizeof({});", c_type,cur.name,c_type));
                }
                RegisterMappingEnum::FloatRegister => {
                    let c_type = c_type.1;
                    let size = get_value_for_Ty(&cur.ty) + 1;
                    setup.input_line_input_values.push(format!("\n\t{} {};", c_type, cur.name));
                    setup.output_line_input_streams.push(format!("\n\t{} {};", c_type, cur.name));
                    setup
                        .set_input_values
                        .push(format!("\n\tset_float{}_input_for_reg({}_REG, line->{});", size,cur.name, cur.name));
                    setup
                        .get_input_streams
                        .push(format!("\n\tline->{} = get_reg_as_float{}({}_STREAM_REG);", cur.name, size,cur.name));
                    setup.input_float_conversions.push(format!(
                        "{}Xint32 {}_whole = (int) line->{};\n{}Xint32 {}_before_abs = (line->{} - (float) {}_whole) * {};\n{}Xuint32 {}_after = {}_before_abs < 0 ? -1 * {}_before_abs : {}_before_abs;\n",
                        offset,
                        cur.name,
                        cur.name,
                        offset,
                        cur.name,
                        cur.name,
                        cur.name,
                        FLOAT_32_NUMBER_AFTER_POINT_POW_TEN,
                        offset,
                        cur.name,
                        cur.name,
                        cur.name,
                        cur.name
                    ));
                    setup.input_line_values.push(format!(", {}_whole, {}_after", cur.name, cur.name));
                    setup.input_line_format_strings.push(format!("{}, ", get_format_string_for_ty(&cur.ty)));
                    setup.input_line_header.push(format!("{}, ", cur.name));
                    setup.input_line_initialize.push(format!("\n\tline->{} = 0.0;", cur.name));
                    setup.input_line_as_bytes.push(format!("\n\twrite_{}_to_buff(buff,pos,line->{});\n\tpos += sizeof({});", c_type,cur.name,c_type));
                }
                RegisterMappingEnum::TwoIntRegisters => {
                    setup.input_line_input_values.push(format!("\n\t{} {}_low;", c_type.0, cur.name));
                    setup.input_line_input_values.push(format!("\n\t{} {}_up;", c_type.1, cur.name));
                    setup.output_line_input_streams.push(format!("\n\t{} {}_low;", c_type.0, cur.name));
                    setup.output_line_input_streams.push(format!("\n\t{} {}_up;", c_type.1, cur.name));
                    setup
                        .set_input_values
                        .push(format!("\n\tset_input_for_reg({}_LOW_REG, line->{}_low);", cur.name, cur.name));
                    setup
                        .set_input_values
                        .push(format!("\n\tset_input_for_reg({}_HIGH_REG, line->{}_up);", cur.name, cur.name));
                    setup
                        .get_input_streams
                        .push(format!("\n\tline->{}_low = get_reg({}_STREAM_LOW_REG);", cur.name, cur.name));
                    setup
                        .get_input_streams
                        .push(format!("\n\tline->{}_up = get_reg({}_STREAM_HIGH_REG);", cur.name, cur.name));
                    setup.input_line_values.push(format!(", line->{}_low", cur.name));
                    setup.input_line_values.push(format!(", line->{}_up", cur.name));
                    setup.input_line_format_strings.push(format!("{}, ", get_format_string_for_ty(&cur.ty)));
                    setup.input_line_format_strings.push(format!("{}, ", get_format_string_for_ty(&cur.ty)));
                    setup.input_line_header.push(format!("{}_low, ", cur.name));
                    setup.input_line_header.push(format!("{}_up, ", cur.name));
                    setup.input_line_initialize.push(format!("\n\tline->{}_low = 0;", cur.name));
                    setup.input_line_initialize.push(format!("\n\tline->{}_up = 0;", cur.name));
                    setup.input_line_as_bytes.push(format!("\n\twrite_{}_to_buff(buff,pos,line->{}_low);\n\tpos += sizeof({});", c_type.0,cur.name,c_type.0));
                    setup.input_line_as_bytes.push(format!("\n\twrite_{}_to_buff(buff,pos,line->{}_up);\n\tpos += sizeof({});", c_type.1,cur.name,c_type.1));
                }
                RegisterMappingEnum::DoubleRegister => {
                    let c_type = c_type.1;
                    setup.input_line_input_values.push(format!("\n\t{} {};", c_type, cur.name));
                    setup.output_line_input_streams.push(format!("\n\t{} {};", c_type, cur.name));
                    setup.set_input_values.push(format!(
                        "\n\tset_double_input_for_reg({}_LOW_REG, {}_HIGH_REG, line->{});",
                        cur.name, cur.name, cur.name
                    ));
                    setup.get_input_streams.push(format!(
                        "\n\tline->{} = get_regs_as_double({}_STREAM_LOW_REG, {}_STREAM_HIGH_REG);",
                        cur.name, cur.name, cur.name
                    ));
                    setup.input_float_conversions.push(format!(
                        "{}Xint32 {}_whole = (int) line->{};\n{}Xint32 {}_before_abs = (line->{} - (double) {}_whole) * {};\n{}Xuint32 {}_after = {}_before_abs < 0 ? -1 * {}_before_abs : {}_before_abs;\n",
                        offset,
                        cur.name,
                        cur.name,
                        offset,
                        cur.name,
                        cur.name,
                        cur.name,
                        FLOAT_64_NUMBER_AFTER_POINT_POW_TEN,
                        offset,
                        cur.name,
                        cur.name,
                        cur.name,
                        cur.name
                    ));
                    setup.input_line_values.push(format!(", {}_whole, {}_after", cur.name, cur.name));
                    setup.input_line_format_strings.push(format!("{}, ", get_format_string_for_ty(&cur.ty)));
                    setup.input_line_header.push(format!("{}, ", cur.name));
                    setup.input_line_initialize.push(format!("\n\tline->{} = 0.0;", cur.name));
                    setup.input_line_as_bytes.push(format!("\n\twrite_{}_to_buff(buff,pos,line->{});\n\tpos += sizeof({});", c_type,cur.name,c_type));
                }
            }
        });
        self.ir.outputs.iter().for_each(|cur| {
            let c_type = get_c_type(&cur.ty);
            match get_values_for_register_mapping(&cur.ty) {
                RegisterMappingEnum::BoolRegister
                | RegisterMappingEnum::ReducedIntRegister(_,_)
                | RegisterMappingEnum::WholeIntRegister => {
                    let c_type = c_type.1;
                    setup.output_line_output_streams.push(format!("\n\t{} {};", c_type, cur.name));
                    setup
                        .get_output_streams
                        .push(format!("\n\tline->{} = get_reg({}_STREAM_REG);", cur.name, cur.name));
                    let needed_cast = match get_atomic_ty(&cur.ty) {
                        Type::Int(IntTy::Int8) => "(signed char) ",
                        _ => ""
                    };
                    setup.output_line_values.push(format!(", {}line->{}", needed_cast,cur.name));
                    setup.output_line_format_strings.push(format!("{}, ", get_format_string_for_ty(&cur.ty)));
                    setup.output_line_header.push(format!("{}, ", cur.name));
                    setup.output_line_initialize.push(format!("\n\tline->{} = 0;", cur.name));
                    setup.output_line_as_bytes.push(format!("\n\twrite_{}_to_buff(buff,pos,line->{});\n\tpos += sizeof({});", c_type,cur.name,c_type));
                }
                RegisterMappingEnum::FloatRegister  => {
                    let c_type = c_type.1;
                    let size = get_value_for_Ty(&cur.ty) + 1;
                    setup.output_line_output_streams.push(format!("\n\t{} {};", c_type, cur.name));
                    setup
                        .get_output_streams
                        .push(format!("\n\tline->{} = get_reg_as_float{}({}_STREAM_REG);", cur.name, size,cur.name));
                    setup.output_float_conversions.push(format!(
                        "{}Xint32 {}_whole = (int) line->{};\n{}Xint32 {}_before_abs = (line->{} - (float) {}_whole) * {};\n{}Xuint32 {}_after = {}_before_abs < 0 ? -1 * {}_before_abs : {}_before_abs;\n",
                        offset,
                        cur.name,
                        cur.name,
                        offset,
                        cur.name,
                        cur.name,
                        cur.name,
                        FLOAT_32_NUMBER_AFTER_POINT_POW_TEN,
                        offset,
                        cur.name,
                        cur.name,
                        cur.name,
                        cur.name
                    ));
                    setup.output_line_values.push(format!(", {}_whole, {}_after", cur.name, cur.name));
                    setup.output_line_format_strings.push(format!("{}, ", get_format_string_for_ty(&cur.ty)));
                    setup.output_line_header.push(format!("{}, ", cur.name));
                    setup.output_line_initialize.push(format!("\n\tline->{} = 0.0;", cur.name));
                    setup.output_line_as_bytes.push(format!("\n\twrite_{}_to_buff(buff,pos,line->{});\n\tpos += sizeof({});", c_type,cur.name,c_type));
                }
                RegisterMappingEnum::TwoIntRegisters => {
                    setup.output_line_output_streams.push(format!("\n\t{} {}_low;", c_type.0, cur.name));
                    setup.output_line_output_streams.push(format!("\n\t{} {}_up;", c_type.1, cur.name));
                    setup
                        .get_output_streams
                        .push(format!("\n\tline->{}_low = get_reg({}_STREAM_LOW_REG);", cur.name, cur.name));
                    setup
                        .get_output_streams
                        .push(format!("\n\tline->{}_up = get_reg({}_STREAM_HIGH_REG);", cur.name, cur.name));
                    setup.output_line_values.push(format!(", line->{}_low", cur.name));
                    setup.output_line_values.push(format!(", line->{}_up", cur.name));
                    setup.output_line_format_strings.push(format!("{}, ", get_format_string_for_ty(&cur.ty)));
                    setup.output_line_format_strings.push(format!("{}, ", get_format_string_for_ty(&cur.ty)));
                    setup.output_line_header.push(format!("{}_low, ", cur.name));
                    setup.output_line_header.push(format!("{}_up, ", cur.name));
                    setup.output_line_initialize.push(format!("\n\tline->{}_low = 0;", cur.name));
                    setup.output_line_initialize.push(format!("\n\tline->{}_up = 0;", cur.name));
                    setup.output_line_as_bytes.push(format!("\n\twrite_{}_to_buff(buff,pos,line->{}_low);\n\tpos += sizeof({});", c_type.0,cur.name,c_type.0));
                    setup.output_line_as_bytes.push(format!("\n\twrite_{}_to_buff(buff,pos,line->{}_up);\n\tpos += sizeof({});", c_type.1,cur.name,c_type.1));
                }
                RegisterMappingEnum::DoubleRegister => {
                    let c_type = c_type.1;
                    setup.output_line_output_streams.push(format!("\n\t{} {};", c_type, cur.name));
                    setup.get_output_streams.push(format!(
                        "\n\tline->{} = get_regs_as_double({}_STREAM_LOW_REG, {}_STREAM_HIGH_REG);",
                        cur.name, cur.name, cur.name
                    ));
                    setup.output_float_conversions.push(format!(
                        "{}Xint32 {}_whole = (int) line->{};\n{}Xint32 {}_before_abs = (line->{} - (double) {}_whole) * {};\n{}Xuint32 {}_after = {}_before_abs < 0 ? -1 * {}_before_abs : {}_before_abs;\n",
                        offset,
                        cur.name,
                        cur.name,
                        offset,
                        cur.name,
                        cur.name,
                        cur.name,
                        FLOAT_64_NUMBER_AFTER_POINT_POW_TEN,
                        offset,
                        cur.name,
                        cur.name,
                        cur.name,
                        cur.name
                    ));
                    setup.output_line_values.push(format!(", {}_whole, {}_after", cur.name, cur.name));
                    setup.output_line_format_strings.push(format!("{}, ", get_format_string_for_ty(&cur.ty)));
                    setup.output_line_header.push(format!("{}, ", cur.name));
                    setup.output_line_initialize.push(format!("\n\tline->{} = 0.0;", cur.name));
                    setup.output_line_as_bytes.push(format!("\n\twrite_{}_to_buff(buff,pos,line->{});\n\tpos += sizeof({});", c_type,cur.name,c_type));
                }
            }
        });
        setup
    }
}

#[cfg(test)]
mod fpga_high_level_communication_file_tests {
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
            "input a : Float32 input b :Float32\noutput c @1Hz := a.hold().defaults(to:0.0) + 3.0\noutput d @2Hz := a.hold().defaults(to:0.0) + 6.0\noutput e := a + b";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let reg_stat = RegisterStatistic::new(&lola_instance);
        let implementation = FPGAHighLevelCommunication::new(&lola_instance, &reg_stat, true);
        let tera: Tera = compile_templates!("templates/vivado_file_changes/*");
        VHDLGenerator::generate_and_create(&implementation, &tera, &PathBuf::from("target/test_files"))
    }
}
