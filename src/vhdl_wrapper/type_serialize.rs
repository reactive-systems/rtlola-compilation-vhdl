use crate::static_constants::{
    FLOAT_16_HIGH, FLOAT_16_LOW, FLOAT_16_NUMBER_AFTER_POINT, FLOAT_32_HIGH, FLOAT_32_LOW, FLOAT_32_NUMBER_AFTER_POINT,
    FLOAT_64_HIGH, FLOAT_64_LOW, FLOAT_64_NUMBER_AFTER_POINT,
};
use rtlola_frontend::ir::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

//--------------------Type Serialize----------------------------------------------------------------

pub(crate) fn get_vhdl_type(ty: &Type) -> String {
    match ty {
        Type::Int(size_ty) => {
            let max_num = get_value_for_IntTy(*size_ty);
            format!("signed({} downto 0)", max_num)
        }
        Type::UInt(size_ty) => {
            let max_num = get_value_for_UIntTy(*size_ty);
            format!("unsigned({} downto 0)", max_num)
        }
        Type::Bool => "std_logic".to_string(),
        Type::Option(ty) => get_vhdl_type(ty),
        Type::Float(size_ty) => {
            let (high, low) = get_float_range(*size_ty);
            format!("sfixed({} downto {})", high, low)
        }
        Type::Function(_args, ret) => get_vhdl_type(ret),
        _ => {
            unimplemented!("{:?}", ty);
        }
    }
}

pub(crate) fn get_larger_ty(ty: &Type) -> Type {
    match ty {
        Type::Int(size_ty) => Type::Int(get_larget_IntTy(*size_ty)),
        Type::UInt(size_ty) => Type::UInt(get_larget_UIntTy(*size_ty)),
        Type::Option(op_ty) => get_larger_ty(op_ty),
        Type::Function(_args, ret) => get_larger_ty(ret),
        Type::Float(size_ty) => Type::Float(get_larget_FloatTy(*size_ty)),
        _ => unimplemented!("Type: {} not implemented", ty),
    }
}

pub(crate) fn get_vhdl_initial_type(ty: &Type) -> String {
    match ty {
        Type::Int(size_ty) => {
            let length_vec = get_value_for_IntTy(*size_ty);
            format!("std_logic_vector({} downto 0)", length_vec)
        }
        Type::UInt(size_ty) => {
            let length_vec = get_value_for_UIntTy(*size_ty);
            format!("std_logic_vector({} downto 0)", length_vec)
        }
        Type::Bool => "std_logic".to_string(),
        Type::Option(ty) => get_vhdl_initial_type(ty),
        Type::Float(size_ty) => {
            let length_vec = get_value_for_FloatTy(*size_ty);
            format!("std_logic_vector({} downto 0)", length_vec)
        }
        Type::Function(_args, ret) => get_vhdl_initial_type(ret),
        _ => {
            unimplemented!("{:?}", ty);
        }
    }
}

pub(crate) fn get_vhdl_initial_type_cast(ty: &Type, name: String) -> String {
    match ty {
        Type::Int(_size_ty) => format!("std_logic_vector({})", name),
        Type::UInt(_size_ty) => format!("std_logic_vector({})", name),
        Type::Bool => name,
        Type::Option(op_ty) => get_vhdl_initial_type_cast(op_ty, name),
        Type::Float(_size_ty) => format!("to_slv({})", name),
        _ => {
            unimplemented!("{:?}", ty);
        }
    }
}

pub(crate) enum RegisterMappingEnum {
    BoolRegister,
    ReducedIntRegister(String, String),
    WholeIntRegister,
    TwoIntRegisters,
    ReducedFloatRegister,
    FloatRegister,
    DoubleRegister,
}

pub(crate) fn get_values_for_register_mapping(ty: &Type) -> RegisterMappingEnum {
    match ty {
        Type::Int(IntTy::I8) | Type::UInt(UIntTy::U8) => {
            RegisterMappingEnum::ReducedIntRegister("(7 downto 0)".to_string(), "(31 downto 8)".to_string())
        }
        Type::Int(IntTy::I16) | Type::UInt(UIntTy::U16) => {
            RegisterMappingEnum::ReducedIntRegister("(15 downto 0)".to_string(), "(31 downto 16)".to_string())
        }
        Type::Int(IntTy::I32) | Type::UInt(UIntTy::U32) => RegisterMappingEnum::WholeIntRegister,
        Type::Int(IntTy::I64) | Type::UInt(UIntTy::U64) => RegisterMappingEnum::TwoIntRegisters,

        //        Type::Int(size_ty) => match size_ty {
        //            IntTy::I8 => RegisterMappingEnum::ReducedIntRegister("(7 downto 0)".to_string()),
        //            IntTy::I16 => RegisterMappingEnum::ReducedIntRegister("(15 downto 0)".to_string()),
        //            IntTy::I32 => RegisterMappingEnum::WholeIntRegister,
        //            IntTy::I64 => RegisterMappingEnum::TwoIntRegisters,
        //        },
        //        Type::UInt(size_ty) => match size_ty {
        //            IntTy::U8 => RegisterMappingEnum::ReducedIntRegister("(7 downto 0)".to_string()),
        //            IntTy::U16 => RegisterMappingEnum::ReducedIntRegister("(15 downto 0)".to_string()),
        //            IntTy::U32 => RegisterMappingEnum::WholeIntRegister,
        //            IntTy::U64 => RegisterMappingEnum::TwoIntRegisters,
        //        },
        Type::Bool => RegisterMappingEnum::BoolRegister,
        Type::Float(size_ty) => match size_ty {
            FloatTy::F16 => RegisterMappingEnum::ReducedFloatRegister,
            FloatTy::F32 => RegisterMappingEnum::FloatRegister,
            FloatTy::F64 => RegisterMappingEnum::DoubleRegister,
        },
        Type::Option(op_ty) => get_values_for_register_mapping(op_ty),
        _ => unimplemented!(""),
    }
}

pub(crate) fn get_c_type(ty: &Type) -> (String, String) {
    match ty {
        Type::Bool => (String::new(), "Xuint8".to_string()),
        Type::Int(size_ty) => match size_ty {
            IntTy::I8 => (String::new(), "Xint8".to_string()),
            IntTy::I16 => (String::new(), "Xint16".to_string()),
            IntTy::I32 => (String::new(), "Xint32".to_string()),
            IntTy::I64 => ("Xint32".to_string(), "Xuint32".to_string()),
        },
        Type::UInt(size_ty) => match size_ty {
            UIntTy::U8 => (String::new(), "Xuint8".to_string()),
            UIntTy::U16 => (String::new(), "Xuint16".to_string()),
            UIntTy::U32 => (String::new(), "Xuint32".to_string()),
            UIntTy::U64 => ("Xuint32".to_string(), "Xuint32".to_string()),
        },
        Type::Float(size_ty) => match size_ty {
            FloatTy::F16 => (String::new(), "float".to_string()),
            FloatTy::F32 => (String::new(), "float".to_string()),
            FloatTy::F64 => (String::new(), "double".to_string()),
        },
        Type::Option(op_ty) => get_c_type(op_ty),
        _ => unimplemented!(""),
    }
}

pub(crate) fn get_format_string_for_ty(ty: &Type) -> String {
    match ty {
        Type::Bool => "%u".to_string(),
        Type::Int(IntTy::I8) | Type::Int(IntTy::I16) => "%d".to_string(),
        Type::Int(IntTy::I32) | Type::Int(IntTy::I64) => "%ld".to_string(),
        Type::UInt(UIntTy::U8) | Type::UInt(UIntTy::U16) => "%u".to_string(),
        Type::UInt(UIntTy::U32) | Type::UInt(UIntTy::U64) => "%lu".to_string(),
        Type::Float(FloatTy::F16) => format!("%ld.%{}lu", FLOAT_16_NUMBER_AFTER_POINT),
        Type::Float(FloatTy::F32) => format!("%ld.%{}lu", FLOAT_32_NUMBER_AFTER_POINT),
        Type::Float(FloatTy::F64) => format!("%ld.%{}lu", FLOAT_64_NUMBER_AFTER_POINT),
        Type::Option(op_ty) => get_format_string_for_ty(op_ty),
        _ => unimplemented!(""),
    }
}

#[allow(non_snake_case)]
pub(crate) fn get_value_for_Ty(ty: &Type) -> u16 {
    match ty {
        Type::Int(size_ty) => get_value_for_IntTy(*size_ty),
        Type::UInt(size_ty) => get_value_for_UIntTy(*size_ty),
        Type::Option(op_ty) => get_value_for_Ty(op_ty),
        Type::Float(size_ty) => get_value_for_FloatTy(*size_ty),
        Type::Function(_args, ret) => get_value_for_Ty(ret),
        _ => panic!(),
    }
}

//signed(7 downto 0) --> Zahlenbereich: -2**7 bis 2**7-1
#[allow(non_snake_case)]
pub(crate) fn get_value_for_IntTy(ty: IntTy) -> u16 {
    match ty {
        IntTy::I8 => 7,
        IntTy::I16 => 15,
        IntTy::I32 => 31,
        IntTy::I64 => 63,
    }
}

//unsigned(7 downto 0) --> Zahlenbereich: 0 bis 2**8-1
#[allow(non_snake_case)]
pub(crate) fn get_value_for_UIntTy(ty: UIntTy) -> u16 {
    match ty {
        UIntTy::U8 => 7,
        UIntTy::U16 => 15,
        UIntTy::U32 => 31,
        UIntTy::U64 => 63,
    }
}

#[allow(non_snake_case)]
pub(crate) fn get_value_for_FloatTy(ty: FloatTy) -> u16 {
    match ty {
        FloatTy::F16 => 15,
        FloatTy::F32 => 31,
        FloatTy::F64 => 63,
    }
}

#[allow(non_snake_case)]
pub(crate) fn get_float_range(ty: FloatTy) -> (i16, i16) {
    match ty {
        FloatTy::F16 => (FLOAT_16_HIGH, FLOAT_16_LOW),
        FloatTy::F32 => (FLOAT_32_HIGH, FLOAT_32_LOW),
        FloatTy::F64 => (FLOAT_64_HIGH, FLOAT_64_LOW),
    }
}

#[allow(non_snake_case)]
pub(crate) fn get_larget_FloatTy(ty: FloatTy) -> FloatTy {
    match ty {
        FloatTy::F16 => FloatTy::F64,
        FloatTy::F32 => FloatTy::F64,
        FloatTy::F64 => panic!(),
    }
}

#[allow(non_snake_case)]
pub(crate) fn get_larget_IntTy(ty: IntTy) -> IntTy {
    match ty {
        IntTy::I8 => IntTy::I16,
        IntTy::I16 => IntTy::I32,
        IntTy::I32 => IntTy::I64,
        IntTy::I64 => panic!(),
    }
}

#[allow(non_snake_case)]
pub(crate) fn get_larget_UIntTy(ty: UIntTy) -> UIntTy {
    match ty {
        UIntTy::U8 => UIntTy::U16,
        UIntTy::U16 => UIntTy::U32,
        UIntTy::U32 => UIntTy::U64,
        UIntTy::U64 => panic!(),
    }
}

// pub(crate) fn is_32_bit_ty(ty: Type) -> bool {
//     match ty {
//         Type::Bool => false,
//         Type::Int(size) => size == IntTy::I32,
//         Type::UInt(size) => size == UIntTy::U32,
//         Type::Float(size) => size == FloatTy::F32,
//         Type::Tuple(_) | Type::String => unimplemented!(),
//         Type::Option(ty) => is_32_bit_ty(*ty),
//         Type::Function(_args,)
//     }
// }

//#[allow(non_snake_case)]
//pub(crate) fn get_ISize_as_USize(ty: IntTy) -> UIntTy {
//    match ty {
//        IntTy::I8 => UIntTy::U8,
//        IntTy::I16 => UIntTy::U16,
//        IntTy::I32 => UIntTy::U32,
//        IntTy::I64 => UIntTy::U64,
//    }
//}
//
//#[allow(non_snake_case)]
//pub(crate) fn get_FSize_as_USize(ty: FloatTy) -> UIntTy {
//    match ty {
//        FloatTy::F32 => UIntTy::U32,
//        FloatTy::F64 => UIntTy::U64,
//    }
//}
//
//
//#[allow(non_snake_case)]
//pub(crate) fn get_UTypeSize_for_ty(ty: Type) -> UIntTy {
//    match ty {
//        Type::Int(size) => get_ISize_as_USize(size),
//        Type::Option(ty) => get_UTypeSize_for_ty(*ty),
//        Type::Float(size) => get_FSize_as_USize(size),
//        _ => unimplemented!("")
//    }
//}

pub(crate) fn generate_vhdl_array_type(ty: &Type, size: u16, downwards_dir: bool) -> String {
    let dir = if downwards_dir { format!("{} downto 0", size) } else { format!("0 to {}", size) };
    match ty {
        Type::Int(int_size) => format!("signed{}_array({})", get_value_for_IntTy(*int_size) + 1, dir),
        Type::UInt(uint_size) => format!("unsigned{}_array({})", get_value_for_UIntTy(*uint_size) + 1, dir),
        Type::Bool => format!("bit_array({})", dir),
        Type::Option(op_ty) => generate_vhdl_array_type(op_ty, size, downwards_dir),
        Type::Float(int_size) => format!("sfixed{}_array({})", get_value_for_FloatTy(*int_size) + 1, dir),
        _ => unimplemented!(),
    }
}

pub(crate) fn generate_vhdl_array_type_downwards(ty: &Type, size: u16) -> String {
    generate_vhdl_array_type(ty, size, true)
}

pub(crate) fn generate_vhdl_type_default_initialisation(ty: &Type) -> String {
    match ty {
        Type::UInt(_) | Type::Int(_) => "(others => '0')".to_string(),
        Type::Bool => "'0'".to_string(),
        Type::Option(op_ty) => generate_vhdl_type_default_initialisation(op_ty),
        Type::Float(_) => "(others => '0')".to_string(),
        Type::Function(_args, ret) => generate_vhdl_type_default_initialisation(ret),
        _ => unimplemented!("{:?}", ty),
    }
}

pub(crate) fn generate_vhdl_array_default_initialisation(ty: &Type) -> String {
    match ty {
        Type::UInt(_) | Type::Int(_) => "(others => (others => '0'))".to_string(),
        Type::Bool => "(others => '0')".to_string(),
        Type::Option(op_ty) => generate_vhdl_array_default_initialisation(op_ty),
        Type::Float(_) => "(others => (others => '0'))".to_string(),
        _ => unimplemented!("{:?}", ty),
    }
}

pub(crate) fn generate_vhdl_type_default(ty: &Type, variable: String) -> String {
    match ty {
        Type::Int(_) => format!("to_signed(0, {}'length)", variable),
        Type::UInt(_) => format!("to_unsigned(0, {}'length)", variable),
        Type::Bool => "'0'".to_string(),
        Type::Option(op_ty) => generate_vhdl_type_default(op_ty, variable),
        Type::Float(size_ty) => {
            let (high, low) = get_float_range(*size_ty);
            format!("to_sfixed(0.0, {}, {})", high, low)
        }
        _ => unimplemented!("{:?}", ty),
    }
}

pub(crate) fn generate_vhdl_shift_default(ty: &Type) -> String {
    generate_vhdl_type_default(ty, "updt".to_string())
}

pub(crate) fn is_float_type(ty: &Type) -> Option<FloatTy> {
    match ty {
        Type::Bool => None,
        Type::Int(_) => None,
        Type::UInt(_) => None,
        Type::Float(size_ty) => Some(*size_ty),
        Type::Function(_args, ret) => is_float_type(ret),
        Type::Option(ty) => is_float_type(ty),
        Type::String => None,
        _ => unimplemented!(),
    }
}

pub(crate) fn get_atomic_ty(ty: &Type) -> Type {
    match ty {
        Type::Bool => Type::Bool,
        Type::Int(int_ty) => Type::Int(*int_ty),
        Type::UInt(uint_ty) => Type::UInt(*uint_ty),
        Type::Float(size_ty) => Type::Float(*size_ty),
        Type::Function(_args, ret) => get_atomic_ty(ret),
        Type::Option(ty) => get_atomic_ty(ty),
        Type::String => Type::String,
        _ => unimplemented!(),
    }
}

pub(crate) fn get_sw_default_value_with_cast(ty: &Type, var: &str) -> String {
    match ty {
        Type::Int(_) => format!("to_signed(0, {}'length)", var),
        Type::UInt(_) => format!("to_unsigned(0, {}'length)", var),
        Type::Float(_) => format!("to_sfixed(0.0, {})", var),
        Type::Option(op_ty) => get_sw_default_value_with_cast(op_ty, var),
        _ => unimplemented!("get_sw_default_value_with_cast for type {} not implemented", ty),
    }
}

pub(crate) fn get_count_upd(ty: &Type, var: &str) -> String {
    match ty {
        Type::Int(_) => format!("to_signed(1, {}'length)", var),
        Type::UInt(_) => format!("to_unsigned(1, {}'length)", var),
        Type::Float(_) => format!("to_sfixed(1.0, {})", var),
        Type::Bool => panic!("count sliding window should not have type bool"),
        Type::Option(op) => get_count_upd(op, var),
        _ => unimplemented!("get_count_upd not implemented for type {}", ty),
    }
}

pub(crate) fn resize_float(target_ty: &Type, expr: String) -> String {
    match get_atomic_ty(&target_ty) {
        Type::Float(fl_ty) => {
            let (high, low) = get_float_range(fl_ty);
            format!("resize({}, {}, {});", expr, high, low)
        }
        Type::Int(_) | Type::UInt(_) => format!("{};", expr),
        _ => unimplemented!(),
    }
}
