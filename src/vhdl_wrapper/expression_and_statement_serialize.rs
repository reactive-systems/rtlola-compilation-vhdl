use crate::entity_generator::low_level_controller::output_entity::OutputStreamVHDL;
use crate::ir_extension::ExtendedRTLolaIR;
use crate::vhdl_wrapper::type_serialize::*;
use rtlola_frontend::mir::*;

const OFFSET: &str = "\t\t\t\t";

//--------------------Expression and Statement------------------------------------------------------

impl OutputStreamVHDL<'_> {
    pub(crate) fn generate_vhdl_expression_and_temporaries(&self) -> (String, String, String) {
        assert_eq!(1, self.output.eval.clauses.len());
        let (counter, types, expr_eval, expr_as_string) =
            self.generate_vhdl_expression(&self.output.eval.clauses[0].expression, 0);
        let temp = self.generate_vhdl_temporaries_declarations(types);
        (temp, format!("{}\n{}updt := temp_{};", expr_eval, OFFSET, counter - 1), expr_as_string)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_expression_with_cast(
        lhs_counter: u16,
        rhs_counter: u16,
        types: Vec<(Type, TemporaryTypeHelp)>,
        op: String,
        lhs: String,
        rhs: String,
        high_bound: i16,
        low_bound: i16,
        lhs_as_string: String,
        rhs_as_string: String,
    ) -> (u16, Vec<(Type, TemporaryTypeHelp)>, String, String) {
        let annotation = format!("{} {} {}", lhs_as_string, op, rhs_as_string);
        (
            rhs_counter + 2,
            types,
            format!(
                "{}{}\n{}--* temp_{} := {}\n{}temp_{} := temp_{} {} temp_{};\n{}temp_{} := temp_{}({} downto {});",
                lhs,
                rhs,
                OFFSET,
                rhs_counter + 1,
                annotation,
                OFFSET,
                rhs_counter,
                lhs_counter - 1,
                op,
                rhs_counter - 1,
                OFFSET,
                rhs_counter + 1,
                rhs_counter,
                high_bound,
                low_bound
            ),
            annotation,
        )
    }

    pub(crate) fn generate_vhdl_expression(
        &self,
        exp: &Expression,
        counter: u16,
    ) -> (u16, Vec<(Type, TemporaryTypeHelp)>, String, String) {
        use ExpressionKind::*;
        match &exp.kind {
            LoadConstant(constant) => match constant {
                Constant::Bool(val) => {
                    let val_as_string = if *val { "'1'" } else { "'0'" };
                    (
                        counter + 1,
                        vec![(exp.ty.clone(), TemporaryTypeHelp::Type)],
                        format!("\n{}temp_{} := {};", OFFSET, counter, val_as_string),
                        val_as_string.to_string(),
                    )
                }
                Constant::Int(val) => (
                    counter + 1,
                    vec![(exp.ty.clone(), TemporaryTypeHelp::Type)],
                    format!("\n{}temp_{} := to_signed({}, {});", OFFSET, counter, val, get_value_for_Ty(&exp.ty) + 1),
                    val.to_string(),
                ),
                Constant::UInt(val) => (
                    counter + 1,
                    vec![(exp.ty.clone(), TemporaryTypeHelp::Type)],
                    format!("\n{}temp_{} := to_unsigned({}, {});", OFFSET, counter, val, get_value_for_Ty(&exp.ty) + 1),
                    val.to_string(),
                ),
                Constant::Float(val) => match exp.ty {
                    Type::Float(fl_ty) => {
                        let (high, low) = get_float_range(fl_ty);
                        (
                            counter + 1,
                            vec![(exp.ty.clone(), TemporaryTypeHelp::Type)],
                            format!("\n{}temp_{} := to_sfixed({}, {}, {});", OFFSET, counter, val, high, low),
                            val.to_string(),
                        )
                    }
                    _ => panic!("Should not happen: Bug in Lowering!"),
                },
                Constant::Str(_) => unimplemented!("Type str not yet implemented"),
            },
            StreamAccess { target, parameters: _, access_kind } => {
                match access_kind {
                    StreamAccessKind::Offset(offset) => {
                        let target_str = self.ir.get_name_for_stream_ref(*target);
                        let offset = get_offset_as_string(&StreamAccessKind::Offset(*offset)).unwrap();
                        let annotation = format!("{}.offset(by: {})", target_str, offset);
                        (
                            counter + 1,
                            vec![(self.ir.get_ty_for_stream_ref(*target).clone(), TemporaryTypeHelp::Type)],
                            format!(
                                "\n{}--* temp_{} := {}\n{}temp_{} := {}_{};",
                                OFFSET, counter, annotation, OFFSET, counter, target_str, offset
                            ),
                            annotation,
                        )
                    },
                    StreamAccessKind::SlidingWindow(window_ref) => {
                        let window = self.ir.sliding_window(*window_ref);
                        let in_stream = self.ir.get_name_for_stream_ref(window.target);
                        let sw_ty = get_str_for_sw_op(window.op);
                        let annotation =
                            format!("{}.aggregate(over: {}s, using: {})", in_stream, window.duration.as_secs_f64(), sw_ty);
                        (
                            counter + 1,
                            vec![(window.ty.clone(), TemporaryTypeHelp::Type)],
                            format!(
                                "\n{}--* temp_{} := {} \n{}temp_{} := {}_{}_{}_sw;",
                                OFFSET,
                                counter,
                                annotation,
                                OFFSET,
                                counter,
                                in_stream,
                                sw_ty,
                                window_ref.idx()
                            ),
                            annotation,
                        )
                    },
                    StreamAccessKind::Sync => {
                        let name = self.ir.get_name_for_stream_ref(*target);
                        (
                            counter + 1,
                            vec![(self.ir.get_ty_for_stream_ref(*target).clone(), TemporaryTypeHelp::Type)],
                            format!(
                                "\n{}--* temp_{} := {} \n{}temp_{} := {}_0;",
                                OFFSET, counter, name, OFFSET, counter, name
                            ),
                            name.to_string(),
                        )
                    },
                    StreamAccessKind::Hold => {
                        let name = self.ir.get_name_for_stream_ref(*target);
                        let annotation = format!("{}.hold()", name);
                        (
                            counter + 1,
                            vec![(self.ir.get_ty_for_stream_ref(*target).clone(), TemporaryTypeHelp::Type)],
                            format!(
                                "\n{}--* temp_{} := {} \n{}temp_{} := {}_0;",
                                OFFSET, counter, annotation, OFFSET, counter, name
                            ),
                            annotation,
                        )
                    },
                    StreamAccessKind::DiscreteWindow(_) | StreamAccessKind::InstanceAggregation(_) | StreamAccessKind::Get | StreamAccessKind::Fresh => unimplemented!()
                }

            }
            ArithLog(arith_op, operands) => {
                use rtlola_frontend::mir::ArithLogOp::*;
                let op = get_str_for_arith_op(*arith_op);
                let (lhs_counter_ret, mut lhs_types, lhs_res, lhs_as_string) =
                    self.generate_vhdl_expression(&operands[0], counter);

                match arith_op {
                    Not | Neg => {
                        lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                        let annotation = format!("({} {})", op, lhs_as_string);
                        (
                            lhs_counter_ret + 1,
                            lhs_types,
                            format!(
                                "{}\n{}--* temp_{} := {} \n{}temp_{} := {} temp_{};",
                                lhs_res,
                                OFFSET,
                                lhs_counter_ret,
                                annotation,
                                OFFSET,
                                lhs_counter_ret,
                                op,
                                lhs_counter_ret - 1
                            ),
                            annotation,
                        )
                    }
                    Add | Sub => {
                        let (rhs_counter_ret, rhs_types, rhs_res, rhs_as_string) =
                            self.generate_vhdl_expression(&operands[1], lhs_counter_ret);
                        lhs_types.extend(rhs_types);
                        let float_ty = is_float_type(&exp.ty);
                        match float_ty {
                            Some(size_ty) => {
                                let (high, low) = get_float_range(size_ty);
                                lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::FloatBounds(high + 1, low)));
                                lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                OutputStreamVHDL::generate_expression_with_cast(
                                    lhs_counter_ret,
                                    rhs_counter_ret,
                                    lhs_types,
                                    op,
                                    lhs_res,
                                    rhs_res,
                                    high,
                                    low,
                                    lhs_as_string,
                                    rhs_as_string,
                                )
                            }
                            None => {
                                lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                let annotation = format!("({} {} {})", lhs_as_string, op, rhs_as_string);
                                (
                                    rhs_counter_ret + 1,
                                    lhs_types,
                                    format!(
                                        "{}{}\n{}--* temp_{} := {} \n{}temp_{} := temp_{} {} temp_{};",
                                        lhs_res,
                                        rhs_res,
                                        OFFSET,
                                        rhs_counter_ret,
                                        annotation,
                                        OFFSET,
                                        rhs_counter_ret,
                                        lhs_counter_ret - 1,
                                        op,
                                        rhs_counter_ret - 1
                                    ),
                                    annotation,
                                )
                            }
                        }
                    }
                    Div => {
                        let (rhs_counter_ret, rhs_types, rhs_res, rhs_as_string) =
                            self.generate_vhdl_expression(&operands[1], lhs_counter_ret);
                        lhs_types.extend(rhs_types);
                        let float_ty = is_float_type(&exp.ty);
                        match float_ty {
                            Some(fl_ty) => {
                                let (high, low) = get_float_range(fl_ty);
                                lhs_types
                                    .push((exp.ty.clone(), TemporaryTypeHelp::FloatBounds(high - low + 1, low - high)));
                                lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                OutputStreamVHDL::generate_expression_with_cast(
                                    lhs_counter_ret,
                                    rhs_counter_ret,
                                    lhs_types,
                                    op,
                                    lhs_res,
                                    rhs_res,
                                    high,
                                    low,
                                    lhs_as_string,
                                    rhs_as_string,
                                )
                            }
                            None => {
                                lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                let annotation = format!("({} {} {})", lhs_as_string, op, rhs_as_string);
                                (
                                    rhs_counter_ret + 1,
                                    lhs_types,
                                    format!(
                                        "{}{}\n{}--* temp_{} := {} \n{}temp_{} := temp_{} {} temp_{};",
                                        lhs_res,
                                        rhs_res,
                                        OFFSET,
                                        rhs_counter_ret,
                                        annotation,
                                        OFFSET,
                                        rhs_counter_ret,
                                        lhs_counter_ret - 1,
                                        op,
                                        rhs_counter_ret - 1
                                    ),
                                    annotation,
                                )
                            }
                        }
                    }
                    Rem | Pow | And | Or => {
                        let (rhs_counter_ret, rhs_types, rhs_res, rhs_as_string) =
                            self.generate_vhdl_expression(&operands[1], lhs_counter_ret);
                        lhs_types.extend(rhs_types);
                        lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                        let annotation = format!("({} {} {})", lhs_as_string, op, rhs_as_string);
                        (
                            rhs_counter_ret + 1,
                            lhs_types,
                            format!(
                                "{}{}\n{}--* temp_{} := {} \n{}temp_{} := temp_{} {} temp_{};",
                                lhs_res,
                                rhs_res,
                                OFFSET,
                                rhs_counter_ret,
                                annotation,
                                OFFSET,
                                rhs_counter_ret,
                                lhs_counter_ret - 1,
                                op,
                                rhs_counter_ret - 1
                            ),
                            annotation,
                        )
                    }
                    Mul => {
                        let (rhs_counter_ret, rhs_types, rhs_res, rhs_as_string) =
                            self.generate_vhdl_expression(&operands[1], lhs_counter_ret);
                        lhs_types.extend(rhs_types);
                        let float_ty = is_float_type(&exp.ty);
                        match float_ty {
                            Some(size_ty) => {
                                let (high, low) = get_float_range(size_ty);
                                lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::FloatBounds(2 * high + 1, 2 * low)));
                                lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                OutputStreamVHDL::generate_expression_with_cast(
                                    lhs_counter_ret,
                                    rhs_counter_ret,
                                    lhs_types,
                                    op,
                                    lhs_res,
                                    rhs_res,
                                    high,
                                    low,
                                    lhs_as_string,
                                    rhs_as_string,
                                )
                            }
                            None => {
                                lhs_types.push((get_larger_ty(&exp.ty), TemporaryTypeHelp::Type));
                                lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                let annotation = format!("({} {} {})", lhs_as_string, op, rhs_as_string);
                                (
                                    rhs_counter_ret + 2,
                                    lhs_types,
                                    format!(
                                        "{}{}\n{}--* temp_{} := {} \n{}temp_{} := temp_{} {} temp_{};\n{}temp_{} := temp_{}(temp_{}'length-1 downto 0);",
                                        lhs_res, rhs_res, OFFSET, rhs_counter_ret + 1, annotation, OFFSET, rhs_counter_ret, lhs_counter_ret -1 , op, rhs_counter_ret -1, OFFSET, rhs_counter_ret + 1, rhs_counter_ret, rhs_counter_ret + 1
                                    ),
                                    annotation,
                                )
                            }
                        }
                    }
                    Eq | Ne | Le | Lt | Ge | Gt => {
                        // TODO: Is this necessary? Cf. https://www.cs.sfu.ca/~ggbaker/reference/std_logic/arith/comp.html
                        let (rhs_counter_ret, rhs_types, rhs_res, rhs_as_string) =
                            self.generate_vhdl_expression(&operands[1], lhs_counter_ret);
                        lhs_types.extend(rhs_types);
                        lhs_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                        let annotation = format!("({} {} {})", lhs_as_string, op, rhs_as_string);
                        (
                            rhs_counter_ret + 1,
                            lhs_types,
                            format!(
                                "{}{}\n{}--* temp_{} := {} \n{}temp_{} := to_std_logic(temp_{} {} temp_{});",
                                lhs_res,
                                rhs_res,
                                OFFSET,
                                rhs_counter_ret,
                                annotation,
                                OFFSET,
                                rhs_counter_ret,
                                lhs_counter_ret - 1,
                                op,
                                rhs_counter_ret - 1
                            ),
                            annotation,
                        )
                    }
                    _ => unimplemented!(),
                }
            }

            Convert { expr } => {
                let from = &expr.ty;
                let to = &exp.ty;
                let (arg_counter, mut temps_ty, arg_res, arg_as_string) = self.generate_vhdl_expression(expr, counter);
                let (num_temps, arg) = match OutputStreamVHDL::get_convert_ty(from, to) {
                    ConvertType::ContOptional => return (arg_counter, temps_ty, arg_res, arg_as_string),
                    ConvertType::IntToInt(from_size, to_size) => {
                        let from_ty_size = get_value_for_IntTy(from_size);
                        let to_ty_size = get_value_for_IntTy(to_size);
                        if from_size == to_size {
                            return (arg_counter, temps_ty, arg_res, arg_as_string);
                        } else {
                            temps_ty.push((to.clone(), TemporaryTypeHelp::InitialType));
                            temps_ty.push((to.clone(), TemporaryTypeHelp::Type));
                            (
                                2,
                                if from_ty_size > to_ty_size {
                                    OutputStreamVHDL::generate_higher_cast_statement(arg_counter, to_ty_size, to)
                                } else {
                                    OutputStreamVHDL::generate_lower_cast_statement(arg_counter, from_ty_size, to)
                                },
                            )
                        }
                    }
                    ConvertType::UIntToUInt(from_size, to_size) => {
                        let from_ty_size = get_value_for_UIntTy(from_size);
                        let to_ty_size = get_value_for_UIntTy(to_size);
                        if from_size == to_size {
                            return (arg_counter, temps_ty, arg_res, arg_as_string);
                        } else {
                            temps_ty.push((to.clone(), TemporaryTypeHelp::InitialType));
                            temps_ty.push((to.clone(), TemporaryTypeHelp::Type));
                            (
                                2,
                                if from_ty_size > to_ty_size {
                                    OutputStreamVHDL::generate_higher_cast_statement(arg_counter, to_ty_size, to)
                                } else {
                                    OutputStreamVHDL::generate_lower_cast_statement(arg_counter, from_ty_size, to)
                                },
                            )
                        }
                    }
                    ConvertType::IntToUInt(from_size, to_size) => {
                        let from_ty_size = get_value_for_IntTy(from_size);
                        let to_ty_size = get_value_for_UIntTy(to_size);
                        temps_ty.push((to.clone(), TemporaryTypeHelp::InitialType));
                        temps_ty.push((to.clone(), TemporaryTypeHelp::Type));
                        (
                            2,
                            match from_ty_size.cmp(&to_ty_size) {
                                std::cmp::Ordering::Equal =>
                                format!(
                                    "temp_{} := std_logic_vector(temp_{});\n{}temp_{} := unsigned(temp_{});",
                                    arg_counter,
                                    arg_counter - 1,
                                    OFFSET,
                                    arg_counter + 1,
                                    arg_counter
                                ),
                            std::cmp::Ordering::Greater =>
                                OutputStreamVHDL::generate_higher_cast_statement(arg_counter, to_ty_size, to),
                            std::cmp::Ordering::Less =>
                                OutputStreamVHDL::generate_lower_cast_statement(arg_counter, from_ty_size, to)
                            },
                        )
                    }
                    ConvertType::UIntToInt(from_size, to_size) => {
                        let from_ty_size = get_value_for_UIntTy(from_size);
                        let to_ty_size = get_value_for_IntTy(to_size);
                        temps_ty.push((to.clone(), TemporaryTypeHelp::InitialType));
                        temps_ty.push((to.clone(), TemporaryTypeHelp::Type));
                        (
                            2,
                            match from_ty_size.cmp(&to_ty_size) {
                                std::cmp::Ordering::Equal => format!(
                                    "temp_{} := std_logic_vector(temp_{});\n{}temp_{} := signed(temp_{});",
                                    arg_counter,
                                    arg_counter - 1,
                                    OFFSET,
                                    arg_counter + 1,
                                    arg_counter
                                ),
                                std::cmp::Ordering::Greater => OutputStreamVHDL::generate_higher_cast_statement(arg_counter, to_ty_size, to),
                                std::cmp::Ordering::Less =>OutputStreamVHDL::generate_lower_cast_statement(arg_counter, from_ty_size, to)
                            }
                        )
                    }
                    ConvertType::FloatToInt(_fl_ty, _int_ty) => {
                        temps_ty.push((to.clone(), TemporaryTypeHelp::Type));
                        (
                            1,
                            format!(
                                "temp_{} := to_signed(temp_{}, temp_{}'length);",
                                arg_counter,
                                arg_counter - 1,
                                arg_counter
                            ),
                        )
                    }
                    ConvertType::IntToFloat(_int_ty, fl_ty) => {
                        temps_ty.push((to.clone(), TemporaryTypeHelp::Type));
                        let (range_high, range_low) = get_float_range(fl_ty);
                        (
                            1,
                            format!(
                                "temp_{} := to_sfixed(temp_{}, {}, {});",
                                arg_counter,
                                arg_counter - 1,
                                range_high,
                                range_low
                            ),
                        )
                    }
                    ConvertType::FloatToUInt(_fl_ty, _uint_ty) => {
                        temps_ty.push((to.clone(), TemporaryTypeHelp::Type));
                        (
                            1,
                            format!(
                                "temp_{} := unsigned(to_signed(temp_{}, temp_{}'length));",
                                arg_counter,
                                arg_counter - 1,
                                arg_counter
                            ),
                        )
                    }
                    ConvertType::UIntToFloat(_uint_ty, fl_ty) => {
                        temps_ty.push((to.clone(), TemporaryTypeHelp::Type));
                        let (range_high, range_low) = get_float_range(fl_ty);
                        (
                            1,
                            format!(
                                "temp_{} := to_sfixed(signed(temp_{}), {}, {});",
                                arg_counter,
                                arg_counter - 1,
                                range_high,
                                range_low
                            ),
                        )
                    }
                    ConvertType::FloatToFloat(_from_ty, to_ty) => {
                        temps_ty.push((to.clone(), TemporaryTypeHelp::Type));
                        let (range_high, range_low) = get_float_range(to_ty);
                        (
                            1,
                            format!(
                                "temp_{} := resize(temp_{}, {}, {});",
                                arg_counter,
                                arg_counter - 1,
                                range_high,
                                range_low
                            ),
                        )
                    }
                };
                let annotation = format!("cast({})", arg_as_string);
                (arg_counter + num_temps, temps_ty, format!("{}\n{}{}", arg_res, OFFSET, arg), annotation)
            }

            Ite { condition, consequence, alternative } => {
                let (cond_ret_counter, mut cond_temps_ty, cond_res, cond_as_string) =
                    self.generate_vhdl_expression(condition, counter);
                let (cons_ret_counter, cons_temps_ty, cons_res, cons_as_string) =
                    self.generate_vhdl_expression(consequence, cond_ret_counter);
                let (alt_ret_counter, alt_temps_ty, alt_res, alt_as_string) =
                    self.generate_vhdl_expression(alternative, cons_ret_counter);
                cond_temps_ty.extend(cons_temps_ty);
                cond_temps_ty.extend(alt_temps_ty);
                cond_temps_ty.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                let annotation = format!("if {} then {} else {}", cond_as_string, cons_as_string, alt_as_string);
                (
                    alt_ret_counter + 1,
                    cond_temps_ty,
                    format!(
                        "{}\n{}--* temp_{} := {} \n{}if temp_{} = '1' then{}\n{}temp_{} := temp_{};\n{}else{}\n{}temp_{} := temp_{};\n{}end if;",
                        cond_res,
                        OFFSET,
                        alt_ret_counter,
                        annotation,
                        OFFSET,
                        cond_ret_counter - 1,
                        cons_res,
                        OFFSET,
                        alt_ret_counter,
                        cons_ret_counter - 1,
                        OFFSET,
                        alt_res,
                        OFFSET,
                        alt_ret_counter,
                        alt_ret_counter - 1,
                        OFFSET
                    ),
                    annotation,
                )
            }
            Function(name, args) => {
                match name.as_str() {
                    "abs" => {
                        let (arg_counter_ret, mut arg_types, arg_res, arg_as_string) =
                            self.generate_vhdl_expression(&args[0], counter);
                        match get_atomic_type(&exp.ty) {
                            Type::Int(_) | Type::UInt(_) => {
                                arg_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                let annotation = format!("abs({})", arg_as_string);
                                (
                                    arg_counter_ret + 1,
                                    arg_types,
                                    format!(
                                        "{}\n{}--* temp_{} := {} \n{}temp_{} := abs(temp_{});",
                                        arg_res,
                                        OFFSET,
                                        arg_counter_ret,
                                        annotation,
                                        OFFSET,
                                        arg_counter_ret,
                                        arg_counter_ret - 1
                                    ),
                                    annotation,
                                )
                            }
                            Type::Float(fl_ty) => {
                                let (range_high, range_low) = get_float_range(fl_ty);
                                arg_types.push((exp.ty.clone(), TemporaryTypeHelp::FloatBounds(range_high + 1, range_low)));
                                arg_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                let annotation = format!("abs({})", arg_as_string);
                                (
                                    arg_counter_ret + 2,
                                    arg_types,
                                    format!(
                                        "{}\n{}--* temp_{} := {} \n{}temp_{} := abs(temp_{});\n{}temp_{}({} downto {}) := temp_{}({} downto {});",
                                        arg_res,
                                        OFFSET,
                                        arg_counter_ret + 1,
                                        annotation,
                                        OFFSET,
                                        arg_counter_ret,
                                        arg_counter_ret - 1,
                                        OFFSET,
                                        arg_counter_ret + 1,
                                        range_high,
                                        range_low,
                                        arg_counter_ret,
                                        range_high,
                                        range_low
                                    ),
                                    annotation,
                                )
                            }
                            _ => unimplemented!(),
                        }
                    }
                    "sqrt" => {
                        let (arg_counter_ret, mut arg_types, arg_res, arg_as_string) =
                            self.generate_vhdl_expression(&args[0], counter);
                        let (argument_ty, _) = &arg_types.last().expect("should not happen");
                        //arg_types.push((argument_ty.clone(), false));
                        //arg_types.push((Type::UInt(get_UTypeSize_for_ty(argument_ty.clone())),true));
                        let annotation = format!("sqrt({})", arg_as_string);
                        match get_atomic_type(argument_ty) {
                            Type::Int(IntTy::Int32) | Type::UInt(UIntTy::UInt32) => {
                                arg_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                (
                                    arg_counter_ret + 1,
                                    arg_types,
                                    format!(
                                        "{}\n{}--* temp_{} := {} \n{}temp_{}(15 downto 0) := my_sqrt_32(temp_{});",
                                        arg_res,
                                        OFFSET,
                                        arg_counter_ret,
                                        annotation,
                                        OFFSET,
                                        arg_counter_ret,
                                        arg_counter_ret - 1,
                                    ),
                                    annotation,
                                )
                            }
                            Type::Float(fl_ty) => {
                                arg_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                let size = match fl_ty {
                                    FloatTy::Float32 => 32,
                                    FloatTy::Float64 => {
                                        println!(
                                            "Warning: sqrt is only implemented for Float32 and is therefore casted!",
                                        );
                                        64
                                    }
                                };
                                (
                                    arg_counter_ret + 1,
                                    arg_types,
                                    format!(
                                        "{}\n{}--* temp_{} := {} \n{}temp_{} := my_sqrt_fixed_{}(temp_{});",
                                        arg_res,
                                        OFFSET,
                                        arg_counter_ret,
                                        annotation,
                                        OFFSET,
                                        arg_counter_ret,
                                        size,
                                        arg_counter_ret - 1,
                                    ),
                                    annotation,
                                )
                            }
                            _ => {
                                println!(
                                    "Waring: sqrt function is only implemented for 32-bit Integer and 32-bit float in the FPGA compilation"
                                );
                                arg_types.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                                unimplemented!("Type {} not implemented", exp.ty)
                            }
                        }
                    }
                    _ => unimplemented!("Function not implemented, yet."),
                }
                // For trigonometric functions we should use a rather coarse ROM LUT, such as:

                //   type mem_array is array(0 to (2**N_addr)-1) of  int18;   -- 4k x 18
                //
                //  -- function computes contents of cosine lookup ROM
                //  function init_rom return mem_array is
                //    constant N : integer := 2**N_addr;
                //    constant N1 : real := real(N);
                //    variable w, k1 : real;
                //    variable memx : mem_array;
                //  begin
                //    for k in 0 to N-1 loop
                //      k1 := (real(k)+0.5)/N1;          -- offset of 1/2 necessary to use symmetry
                //      w := cos(math_pi_over_2 * k1);   -- first quadrant of cosine wave
                //      memx(k) := int18(round(131071.0*w));  -- scale to 18-bit signed integer
                //    end loop;
                //    return memx;
                //  end function init_rom;
                //
                //
                //
                //  constant rom : mem_array := init_rom;
                // (https://forums.xilinx.com/t5/General-Technical-Discussion/Calculating-Cosine-and-Sine-Functions-In-VHDL-Using-Look-Up/td-p/195612)

                // And access the table in the function call exploiting identities for different
                // quadrants and sin/cos.
            }
            Default { expr, default } => {
                //TODO ask if this check is needed
                let name_valid = match expr.kind {
                    StreamAccess { target, parameters:_, access_kind } => {
                        match access_kind {
                            StreamAccessKind::Offset(offset) => {
                                let stream = self.ir.get_name_for_stream_ref(target);
                                let offset = get_offset_as_string(&StreamAccessKind::Offset(offset)).unwrap();
                                format!("{}_data_valid_{}", stream, offset)
                            }
                            StreamAccessKind::Hold | StreamAccessKind::Sync => format!("{}_data_valid_0", self.ir.get_name_for_stream_ref(target)),
                            StreamAccessKind::SlidingWindow(wr) => {let window = self.ir.sliding_window(wr);
                                let in_stream = self.ir.get_name_for_stream_ref(window.target);
                                let sw_ty = get_str_for_sw_op(window.op);
                                format!("{}_{}_{}_sw_data_valid", in_stream, sw_ty, wr.idx())}
                                _=>unimplemented!("The case {:?} of the default operator is not yet implemented", expr),
                        }
                    }

                    _ => unimplemented!("The case {:?} of the default operator is not yet implemented", expr),
                };
                let (access_counter_ret, mut access_ty, access_res, access_as_string) =
                    self.generate_vhdl_expression(expr, counter);
                let (default_counter_ret, default_ty, default_res, default_as_string) =
                    self.generate_vhdl_expression(default, access_counter_ret);
                access_ty.extend(default_ty);
                access_ty.push((exp.ty.clone(), TemporaryTypeHelp::Type));
                let annotation = format!("{}.defaults(to: {})", access_as_string, default_as_string);
                (
                    default_counter_ret + 1,
                    access_ty,
                    format!(
                        "{}{}\n{}--* temp_{} := {} \n{}temp_{} := sel(temp_{}, temp_{}, {});",
                        access_res,
                        default_res,
                        OFFSET,
                        default_counter_ret,
                        annotation,
                        OFFSET,
                        default_counter_ret,
                        access_counter_ret - 1,
                        default_counter_ret - 1,
                        name_valid
                    ),
                    annotation,
                )
            }
            ParameterAccess(_, _) | Tuple(_) | TupleAccess(_, _) => unimplemented!("Tuples not implemented, yet"),
        }
    }

    fn generate_vhdl_temporaries_declarations(&self, temps: Vec<(Type, TemporaryTypeHelp)>) -> String {
        let mut counter = 0;
        let temp_variables: Vec<String> = temps
            .iter()
            .map(|(cur, not_init_ty)| {
                let res = format!(
                    "\n\t\tvariable temp_{}: {} := {};",
                    counter,
                    match not_init_ty {
                        TemporaryTypeHelp::Type => get_vhdl_type(cur),
                        TemporaryTypeHelp::InitialType => get_vhdl_initial_type(cur),
                        TemporaryTypeHelp::FloatBounds(high, low) => format!("sfixed({} downto {})", high, low),
                    },
                    //if *not_init_ty { get_vhdl_type(cur) } else { get_vhdl_initial_type(cur) },
                    generate_vhdl_type_default_initialisation(cur)
                );
                counter += 1;
                res
            })
            .collect();
        temp_variables.concat()
    }

    pub(crate) fn get_convert_ty(from: &Type, to: &Type) -> ConvertType {
        match from {
            Type::Option(_from_ty) => ConvertType::ContOptional,
            Type::Int(from_ty) => match to {
                Type::Option(_to_ty) => ConvertType::ContOptional,
                Type::Int(to_ty) => ConvertType::IntToInt(*from_ty, *to_ty),
                Type::UInt(to_ty) => ConvertType::IntToUInt(*from_ty, *to_ty),
                Type::Float(to_ty) => ConvertType::IntToFloat(*from_ty, *to_ty),
                _ => unimplemented!(),
            },
            Type::UInt(from_ty) => match to {
                Type::Int(to_ty) => ConvertType::UIntToInt(*from_ty, *to_ty),
                Type::UInt(to_ty) => ConvertType::UIntToUInt(*from_ty, *to_ty),
                Type::Float(to_ty) => ConvertType::UIntToFloat(*from_ty, *to_ty),
                _ => unimplemented!(),
            },
            Type::Float(from_ty) => match to {
                Type::Int(to_ty) => ConvertType::FloatToInt(*from_ty, *to_ty),
                Type::UInt(to_ty) => ConvertType::FloatToUInt(*from_ty, *to_ty),
                Type::Float(to_ty) => ConvertType::FloatToFloat(*from_ty, *to_ty),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    pub(crate) fn generate_higher_cast_statement(counter: u16, size: u16, to_ty: &Type) -> String {
        match to_ty {
            Type::Int(_) => format!(
                "temp_{} := std_logic_vector(temp_{}({} downto 0));\n{}temp_{} := signed(temp_{});",
                counter,
                counter - 1,
                size,
                OFFSET,
                counter + 1,
                counter
            ),
            Type::UInt(_) => format!(
                "temp_{} := std_logic_vector(temp_{}({} downto 0));\n{}temp_{} := unsigned(temp_{});",
                counter,
                counter - 1,
                size,
                OFFSET,
                counter + 1,
                counter
            ),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn generate_lower_cast_statement(counter: u16, size: u16, to_ty: &Type) -> String {
        match to_ty {
            Type::Int(_) => format!(
                "temp_{}({} downto 0) := std_logic_vector(temp_{});\n{}temp_{} := signed(temp_{});",
                counter,
                size,
                counter - 1,
                OFFSET,
                counter + 1,
                counter
            ),
            Type::UInt(_) => format!(
                "temp_{}({} downto 0) := std_logic_vector(temp_{});\n{}temp_{} := unsigned(temp_{});",
                counter,
                size,
                counter - 1,
                OFFSET,
                counter + 1,
                counter
            ),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn generate_dependencies_in_expr(expr: &Expression) -> Vec<StreamReference> {
        match &expr.kind {
            ExpressionKind::LoadConstant(_) => Vec::new(),
            ExpressionKind::ArithLog(_op, args) => OutputStreamVHDL::generate_dependencies_in_expr_over_args(args),
            ExpressionKind::StreamAccess { target, parameters:_, access_kind } => match access_kind {
                StreamAccessKind::Hold | StreamAccessKind::Sync | StreamAccessKind::Offset(_) => vec![*target],

                StreamAccessKind::SlidingWindow(_) => Vec::new(),
                StreamAccessKind::InstanceAggregation(_)
                | StreamAccessKind::DiscreteWindow(_)
                | StreamAccessKind::Get
                | StreamAccessKind::Fresh => unimplemented!(),
            },
            ExpressionKind::Ite { condition, consequence, alternative } => {
                let args = vec![*condition.clone(), *consequence.clone(), *alternative.clone()];
                OutputStreamVHDL::generate_dependencies_in_expr_over_args(&args)
            }
            ExpressionKind::Function(_name, args) => OutputStreamVHDL::generate_dependencies_in_expr_over_args(args),
            ExpressionKind::Convert { expr, .. } => OutputStreamVHDL::generate_dependencies_in_expr(expr),
            ExpressionKind::Default { expr, default } => {
                let args = vec![*expr.clone(), *default.clone()];
                OutputStreamVHDL::generate_dependencies_in_expr_over_args(&args)
            }
            _ => unimplemented!(),
        }
    }

    fn generate_dependencies_in_expr_over_args(args: &[Expression]) -> Vec<StreamReference> {
        let mut stream_deps = Vec::new();
        for arg in args {
            let stream_deps_in_arg = OutputStreamVHDL::generate_dependencies_in_expr(arg);
            stream_deps_in_arg.iter().for_each(|cur_stream_dep| {
                if !stream_deps.contains(cur_stream_dep) {
                    stream_deps.push(*cur_stream_dep);
                }
            });
        }
        stream_deps
    }
}

pub(crate) enum TemporaryTypeHelp {
    InitialType,
    Type,
    FloatBounds(i16, i16),
}

pub(crate) enum ConvertType {
    IntToInt(IntTy, IntTy),
    IntToUInt(IntTy, UIntTy),
    IntToFloat(IntTy, FloatTy),
    UIntToUInt(UIntTy, UIntTy),
    UIntToInt(UIntTy, IntTy),
    UIntToFloat(UIntTy, FloatTy),
    FloatToInt(FloatTy, IntTy),
    FloatToUInt(FloatTy, UIntTy),
    FloatToFloat(FloatTy, FloatTy),
    ContOptional,
}

//--------------------Entity Header-----------------------------------------------------------------
impl OutputStreamVHDL<'_> {
    pub(crate) fn generate_vhdl_dependencies(&self, declaration: bool) -> String {
        let dependencies_streams: Vec<String> = self.output.accesses
            .iter()
            .map(|(stream, accesses)| {
                let name = self.ir.get_name_for_stream_ref(*stream);
                let ty = get_vhdl_type(self.ir.get_ty_for_stream_ref(*stream));
                let mut considered_offsets = Vec::new();
                let dep_strings: Vec<String> = accesses
                    .iter()
                    .filter_map(|(_, kind)| {
                        if considered_offsets.iter().any(|cur_off| *cur_off==*kind) {
                            Some("".to_string())
                        } else {
                            considered_offsets.push(*kind);
                            get_offset_as_string(kind).map(|offset| 
                            if declaration {
                                format!(
                                    "\n\t\t\t{}_{} : in {};\n\t\t\t{}_data_valid_{} : in std_logic;",
                                    name, offset, ty, name, offset
                                )
                            } else {
                                let offset_as_number = get_offset_as_number(kind).unwrap();
                                format!(
                                    "\n\t\t\t{}_{} => {}_entity_data_{},\n\t\t\t{}_data_valid_{} => {}_entity_data_valid_{},",
                                    name, offset, name, offset_as_number, name, offset, name, offset_as_number
                                )
                            })
                        }

                    })
                    .collect();
                dep_strings.concat()
            })
            .collect();
        let dependencies_windows: Vec<String> = self
            .ir
            .get_used_windows_in_expr(&self.output.eval.clauses[0].expression)
            .iter()
            .map(|cur| {
                let window = self.ir.sliding_window(*cur);
                let in_stream = self.ir.get_name_for_stream_ref(window.target);
                let sw_ty = get_str_for_sw_op(window.op);
                let ty = get_vhdl_type(&window.ty);
                if declaration {
                    format!(
                        "\n\t\t\t{}_{}_{}_sw : in {};\n\t\t\t{}_{}_{}_sw_data_valid : in std_logic;",
                        in_stream, sw_ty, cur.idx(),ty, in_stream, sw_ty, cur.idx()
                    )
                } else {
                    format!(
                        "\n\t\t\t{}_{}_{}_sw => {}_{}_{}_entity_data,\n\t\t\t{}_{}_{}_sw_data_valid => {}_{}_{}_entity_data_valid,",
                        in_stream, sw_ty, cur.idx(),in_stream, sw_ty, cur.idx(),in_stream, sw_ty, cur.idx(),in_stream, sw_ty,cur.idx(),
                    )
                }
            })
            .collect();
        let mut dependencies = dependencies_streams;
        dependencies.extend(dependencies_windows);
        dependencies.concat()
    }

    pub(crate) fn generate_output_dependencies_annotations(&self) -> String {
        let dependencies_streams: Vec<String> = self
            .output
            .accesses
            .iter()
            .map(|(stream, accesses)| {
                let name = self.ir.get_name_for_stream_ref(*stream);
                let stream_type = self.ir.get_ty_for_stream_ref(*stream);
                let mut deps: String = String::new();
                let mut first = true;
                accesses.iter().for_each(|(_, kind)| {
                    let off_as_string = match kind {
                        StreamAccessKind::Sync => "0".to_string(),
                        StreamAccessKind::SlidingWindow(window_reference) => format!("window {window_reference}"),
                        StreamAccessKind::Hold => "0".to_string(),
                        StreamAccessKind::Offset(offset) => match offset {
                            Offset::Past(off) => {
                                if *off != 0 {
                                    format!("-{}", off)
                                } else {
                                    off.to_string()
                                }
                            }
                            Offset::Future(off) => off.to_string(),
                        },
                        StreamAccessKind::Get
                        | StreamAccessKind::Fresh
                        | StreamAccessKind::InstanceAggregation(_)
                        | StreamAccessKind::DiscreteWindow(_) => {
                            unimplemented!()
                        }
                    };
                    if first {
                        first = false;
                        deps = off_as_string;
                    } else {
                        deps = format!("{}, {}", deps, off_as_string);
                    }
                });
                format!("--* - {} of Type {}: {}\n", name, stream_type, deps)
            })
            .collect();
        let dependencies_windows: Vec<String> = self
            .ir
            .get_used_windows_in_expr(&self.output.eval.clauses[0].expression)
            .iter()
            .map(|cur| {
                let window = self.ir.sliding_window(*cur);
                let in_stream = self.ir.get_name_for_stream_ref(window.target);
                let sw_ty = get_str_for_sw_op(window.op);
                format!(
                    "--* - {}.aggregate(over: {} s, using: {}) of type {}\n",
                    in_stream,
                    window.duration.as_secs_f64(),
                    sw_ty,
                    window.ty
                )
            })
            .collect();
        let dependencies_streams = if dependencies_streams.is_empty() {
            String::new()
        } else {
            format!("--* Stream Lookups\n{}", dependencies_streams.concat(),)
        };
        let dependencies_windows = if dependencies_windows.is_empty() {
            String::new()
        } else {
            format!("--* Window Lookups:\n{}", dependencies_windows.concat(),)
        };

        format!("{}{}", dependencies_streams, dependencies_windows)
    }
}

//--------------------Helper Functions--------------------------------------------------------------

fn get_str_for_arith_op(op: ArithLogOp) -> String {
    use rtlola_frontend::mir::ArithLogOp::*;
    match op {
        Add => "+",
        Sub => "-",
        Mul => "*",
        Div => "/",
        Rem => "rem",
        Pow => unimplemented!(""),
        Eq => "=",
        Lt => "<",
        Le => "<=",
        Ne => "/=",
        Ge => ">=",
        Gt => ">",
        Not => "not",
        Neg => "-",
        And => "and",
        Or => "or",
        _ => unimplemented!(),
    }
    .to_string()
}

fn get_offset_as_string(offset: &StreamAccessKind) -> Option<String> {
    match offset {
        StreamAccessKind::Sync | StreamAccessKind::Hold => Some("0".to_string()),
        StreamAccessKind::SlidingWindow(_) => None,
        StreamAccessKind::Offset(offset) => Some(match offset {
            Offset::Past(0) => "0".to_string(),
            Offset::Past(of) => format!("neg{}", of),
            _ => unimplemented!(),
        }),
        StreamAccessKind::Get
        | StreamAccessKind::Fresh
        | StreamAccessKind::InstanceAggregation(_)
        | StreamAccessKind::DiscreteWindow(_) => {
            unimplemented!()
        }
    }
}

fn get_offset_as_number(offset: &StreamAccessKind) -> Option<String> {
    match offset {
        StreamAccessKind::Sync | StreamAccessKind::Hold => Some("0".to_string()),
        StreamAccessKind::SlidingWindow(_) => None,
        StreamAccessKind::Offset(offset) => Some(match offset {
            Offset::Past(0) => "0".to_string(),
            Offset::Past(of) => format!("{}", of),
            _ => unimplemented!(),
        }),
        StreamAccessKind::Get
        | StreamAccessKind::Fresh
        | StreamAccessKind::InstanceAggregation(_)
        | StreamAccessKind::DiscreteWindow(_) => {
            unimplemented!()
        }
    }
}

pub(crate) fn get_str_for_sw_op(op: WindowOperation) -> String {
    match op {
        WindowOperation::Sum => "sum".to_string(),
        WindowOperation::Product => "product".to_string(),
        WindowOperation::Average => "avg".to_string(),
        WindowOperation::Count => "count".to_string(),
        WindowOperation::Integral => "integral".to_string(),
        WindowOperation::Min => "min".to_string(),
        WindowOperation::Max => "max".to_string(),
        _ => unimplemented!(),
    }
}

fn get_atomic_type(ty: &Type) -> Type {
    match ty {
        Type::Bool | Type::Float(_) | Type::UInt(_) | Type::Int(_) | Type::String => ty.clone(),
        Type::Function { args: _, ret } => get_atomic_type(ret),
        Type::Option(op_ty) => get_atomic_type(op_ty),
        _ => unimplemented!(),
    }
}
