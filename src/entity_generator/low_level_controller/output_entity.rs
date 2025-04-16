use crate::entity_generator::GenerateVhdlCode;
use crate::ir_extension::ExtendedRTLolaIR;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub(crate) struct OutputStreamVHDL<'a> {
    pub(crate) output: &'a OutputStream,
    pub(crate) ir: &'a RtLolaMir,
}

impl<'a> OutputStreamVHDL<'a> {
    pub(crate) fn new(output: &'a OutputStream, ir: &'a RtLolaMir) -> OutputStreamVHDL<'a> {
        OutputStreamVHDL { output, ir }
    }
}

impl GenerateVhdlCode for OutputStreamVHDL<'_> {
    fn template_name(&self) -> String {
        "output_stream.tmpl".to_string()
    }

    fn file_name(&self) -> String {
        format!("{}_output_stream_entity.vhdl", self.output.name)
    }
}

impl Serialize for OutputStreamVHDL<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("OutputStream", 11)?;
        s.serialize_field("name", &format!("{}_output_stream", self.output.name))?;
        s.serialize_field("ty", &get_vhdl_type(&self.output.ty))?;
        let mem_bound = self.output.values_to_memorize().unwrap() - 1;
        s.serialize_field("array_size", &(mem_bound))?;
        s.serialize_field("array_ty", &generate_vhdl_array_type_downwards(&self.output.ty, mem_bound))?;
        s.serialize_field("input_streams", &self.generate_vhdl_dependencies(true))?;
        s.serialize_field("default_init", &generate_vhdl_type_default_initialisation(&self.output.ty))?;
        s.serialize_field("default_array_init", &generate_vhdl_array_default_initialisation(&self.output.ty))?;
        s.serialize_field("default_shift_init", &generate_vhdl_shift_default(&self.output.ty))?;
        let (temp_types, exp, expr_as_string) = self.generate_vhdl_expression_and_temporaries();
        s.serialize_field("temporaries_declaration", &temp_types)?;
        s.serialize_field("expr", &exp)?;
        let mut ac_as_strings = Vec::new();
        let mut first = true;
        self.output.accessed_by.iter().for_each(|(cur, _)| {
            let name = self.ir.stream(*cur).name();
            if first {
                first = false;
                ac_as_strings.push(name.to_string());
            } else {
                ac_as_strings.push(format!(", {}", name));
            }
        });
        s.serialize_field(
            "print_stream",
            &format!("output {} : {} := {}", self.output.name, self.output.ty, expr_as_string),
        )?;
        s.serialize_field("output_dependencies_in_dg", &self.generate_output_dependencies_annotations())?;
        s.serialize_field(
            "input_dependencies_in_dg",
            &self.ir.get_input_dependencies_for_stream_as_annotation(self.output.reference),
        )?;
        s.end()
    }
}

#[cfg(test)]
mod output_tests {
    use super::*;
    use crate::entity_generator::VHDLGenerator;
    use tera::{compile_templates, Tera};

    fn parse(spec: &str) -> Result<RtLolaMir, String> {
        rtlola_frontend::parse(&rtlola_frontend::ParserConfig::for_string(spec.to_string()))
            .map_err(|e| format!("{e:?}"))
    }

    #[ignore] // fix lowering
    #[test]
    fn const_stream_test() {
        let example_file_content = "output constantStreamTest : Int8 := 5";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let outputs = &lola_instance.outputs;
        let stream = OutputStreamVHDL::new(&outputs[0], &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        let result = VHDLGenerator::generate(&stream, &tera);

        let temporal_first_pos = result.find("temporal").expect("expected temporal declaration");
        let temporal_last_pos = result.find("updt").expect("expected updt declaration");
        let temporal_declaration_result = &result[temporal_first_pos..temporal_last_pos];
        let temporal_declaration_result: Vec<&str> = temporal_declaration_result.split("\n").collect();
        assert_eq!(temporal_declaration_result[1].trim(), "variable temp_0: signed(7 downto 0) := (others => '0');");

        let updt_first_pos = result.find("update").expect("expected update declaration");
        let updt_last_pos = result.find("register").expect("expected register declaration");
        let updt_result = &result[updt_first_pos..updt_last_pos];
        let updt_result: Vec<&str> = updt_result.split("\n").collect();
        assert_eq!(updt_result[1].trim(), "temp_0 := to_signed(5, 8);");
        assert_eq!(updt_result[2].trim(), "updt := temp_0;");
    }

    #[test]
    fn addition_stream_test() {
        let example_file_content = "input in : Int8 \n output add5 : Int8 := in + 5 \n output add7 : Int8 := add5 + 7";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let outputs = &lola_instance.outputs;
        let out_stream_0 = OutputStreamVHDL::new(&outputs[0], &lola_instance);
        let _out_stream_1 = OutputStreamVHDL::new(&outputs[1], &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        let result = VHDLGenerator::generate(&out_stream_0, &tera);

        let entity_first_pos = result.find("entity").expect("expected entity declaration");
        let entity_last_pos = result.find("architecture").expect("expected entity declaration");
        let entity_result = &result[entity_first_pos..entity_last_pos];
        let entity_result: Vec<&str> = entity_result.split("\n").collect();
        assert_eq!(entity_result[0].trim(), "entity add5_output_stream_entity is");
        assert_eq!(entity_result[3].trim(), "in_0 : in signed(7 downto 0);");
        assert_eq!(entity_result[4].trim(), "in_data_valid_0 : in std_logic;");
        assert_eq!(entity_result[5].trim(), "data_out : out signed8_array(0 downto 0);");
        assert_eq!(entity_result[6].trim(), "data_valid_out : out bit_array(0 downto 0);");

        let temporal_first_pos = result.find("temporal").expect("expected temporal declaration");
        let temporal_last_pos = result.find("updt").expect("expected updt declaration");
        let temporal_declaration_result = &result[temporal_first_pos..temporal_last_pos];
        let temporal_declaration_result: Vec<&str> = temporal_declaration_result.split("\n").collect();
        assert_eq!(temporal_declaration_result[1].trim(), "variable temp_0: signed(7 downto 0) := (others => '0');");
        assert_eq!(temporal_declaration_result[2].trim(), "variable temp_1: signed(7 downto 0) := (others => '0');");
        assert_eq!(temporal_declaration_result[3].trim(), "variable temp_2: signed(7 downto 0) := (others => '0');");

        let updt_first_pos = result.find("Evaluation").expect("expected pseudo evaluation declaration");
        let updt_last_pos = result.find("Register").expect("expected register update declaration");
        let updt_result = &result[updt_first_pos..updt_last_pos];
        let updt_result: Vec<&str> = updt_result.split("\n").collect();
        assert_eq!(updt_result[7].trim(), "temp_0 := in_0;");
        assert_eq!(updt_result[8].trim(), "temp_1 := to_signed(5, 8);");
        assert_eq!(updt_result[10].trim(), "temp_2 := temp_0 + temp_1;");
        assert_eq!(updt_result[11].trim(), "updt := temp_2;");
    }

    #[test]
    fn lookup_stream_test() {
        let example_file_content = "input a : Int8 \n output b : Int8 := a.offset(by:-1).defaults(to:3) + 5";
        let lola_instance = parse(example_file_content).unwrap_or_else(|e| panic!("spec is invalid: {}", e));
        let outputs = &lola_instance.outputs;
        let b_stream = OutputStreamVHDL::new(&outputs[0], &lola_instance);
        let tera: Tera = compile_templates!("templates/low_level_controller/*");
        let result = VHDLGenerator::generate(&b_stream, &tera);

        let entity_first_pos = result.find("entity").expect("expected entity declaration");
        let entity_last_pos = result.find("architecture").expect("expected entity declaration");
        let entity_result = &result[entity_first_pos..entity_last_pos];
        let entity_result: Vec<&str> = entity_result.split("\n").collect();
        assert_eq!(entity_result[0].trim(), "entity b_output_stream_entity is");
        assert_eq!(entity_result[3].trim(), "a_neg1 : in signed(7 downto 0);");
        assert_eq!(entity_result[4].trim(), "a_data_valid_neg1 : in std_logic;");
        assert_eq!(entity_result[5].trim(), "data_out : out signed8_array(0 downto 0);");
        assert_eq!(entity_result[6].trim(), "data_valid_out : out bit_array(0 downto 0);");

        let temporal_first_pos = result.find("temporal").expect("expected temporal declaration");
        let temporal_last_pos = result.find("updt").expect("expected updt declaration");
        let temporal_declaration_result = &result[temporal_first_pos..temporal_last_pos];
        let temporal_declaration_result: Vec<&str> = temporal_declaration_result.split("\n").collect();
        assert_eq!(temporal_declaration_result[1].trim(), "variable temp_0: signed(7 downto 0) := (others => '0');");
        assert_eq!(temporal_declaration_result[2].trim(), "variable temp_1: signed(7 downto 0) := (others => '0');");
        assert_eq!(temporal_declaration_result[3].trim(), "variable temp_2: signed(7 downto 0) := (others => '0');");
        assert_eq!(temporal_declaration_result[4].trim(), "variable temp_3: signed(7 downto 0) := (others => '0');");

        let updt_first_pos = result.find("Evaluation").expect("expected pseudo evaluation declaration");
        let updt_last_pos = result.find("Register").expect("expected register declaration");
        let updt_result = &result[updt_first_pos..updt_last_pos];
        let updt_result: Vec<&str> = updt_result.split("\n").collect();
        assert_eq!(updt_result[7].trim(), "temp_0 := a_neg1;");
        assert_eq!(updt_result[8].trim(), "temp_1 := to_signed(3, 8);");
        assert_eq!(updt_result[10].trim(), "temp_2 := sel(temp_0, temp_1, a_data_valid_neg1);");
        assert_eq!(updt_result[11].trim(), "temp_3 := to_signed(5, 8);");
        assert_eq!(updt_result[13].trim(), "temp_4 := temp_2 + temp_3;");
        assert_eq!(updt_result[14].trim(), "updt := temp_4;");
    }
}
