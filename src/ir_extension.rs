use crate::vhdl_wrapper::expression_and_statement_serialize::get_str_for_sw_op;
use rtlola_frontend::ir::{
    Expression, ExpressionKind, Offset, OutputReference, OutputStream, RTLolaIR, SlidingWindow, StreamReference, Type,
    WindowReference,
};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct InputDependency {
    pub(crate) offsets: Vec<InputOffsetDependency>,
    pub(crate) windows: Vec<InputWindowDependency>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct InputOffsetDependency {
    pub(crate) stream: StreamReference,
    pub(crate) offsets: Vec<Offset>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct InputWindowDependency {
    pub(crate) stream: StreamReference,
    pub(crate) window: WindowReference,
}

pub(crate) trait ExtendedRTLolaIR {
    fn get_name_for_stream_ref(&self, stream_ref: StreamReference) -> &str;
    fn get_ty_for_stream_ref(&self, stream_ref: StreamReference) -> &Type;
    fn get_ty_for_temporary_in_stream(&self, sr: StreamReference) -> &Type;
    fn is_output_reference(&self, sr: StreamReference) -> bool;
    fn get_output_reference(&self, sr: StreamReference) -> OutputReference;
    fn get_used_windows_in_stream(&self, stream: &OutputStream) -> Vec<WindowReference>;
    fn get_used_windows_in_expr(&self, expr: &Expression) -> Vec<WindowReference>;
    fn get_streams_where_window_is_used(&self, windows: &SlidingWindow) -> Vec<StreamReference>;
    fn get_duration_for_stream(&self, s: StreamReference) -> Option<Duration>;
    fn get_num_buckets(&self, sw: &SlidingWindow) -> u16;
    fn get_input_offset_dependencies_for_stream(&self, sr: StreamReference) -> Vec<InputOffsetDependency>;
    fn get_input_window_dependencies_for_stream(&self, sr: StreamReference) -> Vec<InputWindowDependency>;
    fn get_input_dependencies_for_stream(&self, sr: StreamReference) -> InputDependency;
    fn get_input_dependencies_for_stream_as_annotation(&self, sr: StreamReference) -> String;
    //    fn get_input_window_dependencies_for_stream_as_annotation(&self, win: Vec<WindowReference>) -> String;
}

impl ExtendedRTLolaIR for RTLolaIR {
    fn get_name_for_stream_ref(&self, stream_ref: StreamReference) -> &str {
        match stream_ref {
            StreamReference::InRef(_) => &self.get_in(stream_ref).name,
            StreamReference::OutRef(_) => &self.get_out(stream_ref).name,
        }
        .as_str()
    }

    fn get_ty_for_stream_ref(&self, stream_ref: StreamReference) -> &Type {
        match stream_ref {
            StreamReference::InRef(_) => &self.get_in(stream_ref).ty,
            StreamReference::OutRef(_) => &self.get_out(stream_ref).ty,
        }
    }

    fn get_ty_for_temporary_in_stream(&self, _sr: StreamReference) -> &Type {
        // TODO
        unimplemented!("Adapt to new lowering!")
        //        match sr {
        //            StreamReference::InRef(_) => panic!("method can be called on output streams"),
        //            StreamReference::OutRef(_) => self
        //                .get_out(sr)
        //                .expr
        //                .temporaries
        //                .get(temp.0)
        //                .unwrap_or_else(|| panic!("temporary {} should be contained in Stream {:?}", temp.0, sr)),
        //        }
    }

    fn is_output_reference(&self, sr: StreamReference) -> bool {
        match sr {
            StreamReference::InRef(_) => false,
            StreamReference::OutRef(_) => true,
        }
    }

    fn get_output_reference(&self, sr: StreamReference) -> OutputReference {
        match sr {
            StreamReference::InRef(_) => panic!("Error in Analysis!"),
            StreamReference::OutRef(reference) => reference,
        }
    }

    fn get_used_windows_in_stream(&self, stream: &OutputStream) -> Vec<WindowReference> {
        self.get_used_windows_in_expr(&stream.expr)
    }

    fn get_used_windows_in_expr(&self, expr: &Expression) -> Vec<WindowReference> {
        use ExpressionKind::*;
        match &expr.kind {
            LoadConstant(_) | OffsetLookup { .. } | StreamAccess(_, _) => Vec::new(),
            ArithLog(_, operands, _) => {
                let mut res = Vec::new();
                operands.iter().for_each(|cur| res.extend(self.get_used_windows_in_expr(cur)));
                res
            }
            WindowLookup(window_ref) => vec![*window_ref],
            Ite { condition, consequence, alternative, .. } => {
                let mut res = self.get_used_windows_in_expr(condition);
                res.extend(self.get_used_windows_in_expr(consequence));
                res.extend(self.get_used_windows_in_expr(alternative));
                res
            }
            Tuple(args) => {
                let mut res = Vec::new();
                args.iter().for_each(|cur| res.extend(self.get_used_windows_in_expr(cur)));
                res
            }
            TupleAccess(tuple, _) => self.get_used_windows_in_expr(tuple),
            Function(_, args, _) => {
                let mut res = Vec::new();
                args.iter().for_each(|cur| res.extend(self.get_used_windows_in_expr(cur)));
                res
            }
            Convert { expr, .. } => self.get_used_windows_in_expr(expr),
            Default { expr, default, .. } => {
                let mut res = self.get_used_windows_in_expr(expr);
                res.extend(self.get_used_windows_in_expr(default));
                res
            }
        }
    }

    fn get_streams_where_window_is_used(&self, window: &SlidingWindow) -> Vec<StreamReference> {
        let mut res = Vec::new();
        for s in self.get_time_driven() {
            let used_sws = self.get_used_windows_in_stream(s);
            if used_sws.contains(&window.reference) {
                res.push(s.reference)
            }
        }
        res
    }

    fn get_duration_for_stream(&self, s: StreamReference) -> Option<Duration> {
        for ts in &self.time_driven {
            if ts.reference == s {
                return Some(ts.extend_rate);
            }
        }
        None
    }

    fn get_num_buckets(&self, sw: &SlidingWindow) -> u16 {
        let streams_where_window_is_used = &self.get_streams_where_window_is_used(sw);
        assert_eq!(streams_where_window_is_used.len(), 1, "not implemented, when window is used more than one time");
        let extend_rate = &self.get_duration_for_stream(streams_where_window_is_used[0]).expect("Should not happen");
        (sw.duration.as_nanos() / extend_rate.as_nanos()) as u16
    }

    fn get_input_offset_dependencies_for_stream(&self, sr: StreamReference) -> Vec<InputOffsetDependency> {
        let mut res = Vec::new();
        self.outputs.iter().for_each(|cur_output| {
            cur_output.outgoing_dependencies.iter().for_each(|cur_dep| {
                if cur_dep.stream == sr {
                    res.push(InputOffsetDependency { stream: cur_output.reference, offsets: cur_dep.clone().offsets });
                }
            })
        });
        res
    }

    fn get_input_window_dependencies_for_stream(&self, sr: StreamReference) -> Vec<InputWindowDependency> {
        let mut res = Vec::new();
        self.outputs.iter().for_each(|cur_output| {
            self.get_used_windows_in_expr(&cur_output.expr).iter().for_each(|cur_win| {
                let window = self.get_window(*cur_win);
                if window.target == sr {
                    res.push(InputWindowDependency { stream: cur_output.reference, window: *cur_win });
                }
            })
        });
        res
    }

    fn get_input_dependencies_for_stream(&self, sr: StreamReference) -> InputDependency {
        let offsets = self.get_input_offset_dependencies_for_stream(sr);
        let windows = self.get_input_window_dependencies_for_stream(sr);
        InputDependency { offsets, windows }
    }

    fn get_input_dependencies_for_stream_as_annotation(&self, sr: StreamReference) -> String {
        let input_dependencies = self.get_input_dependencies_for_stream(sr);
        let input_offset_dependencies: Vec<String> = input_dependencies
            .offsets
            .iter()
            .map(|cur_input_dep| {
                let mut first = true;
                let off_as_string: Vec<String> = cur_input_dep
                    .offsets
                    .iter()
                    .map(|cur_offset| {
                        let comma = if first { "" } else { ", " };
                        first = false;
                        match cur_offset {
                            Offset::FutureDiscreteOffset(off) => format!("{}{}", comma, off),
                            Offset::PastDiscreteOffset(off) => {
                                if *off != 0 {
                                    format!("{}-{}", comma, off)
                                } else {
                                    format!("{}{}", comma, off)
                                }
                            }
                            _ => unimplemented!(),
                        }
                    })
                    .collect();
                format!("--* - {}: {}\n", self.get_name_for_stream_ref(cur_input_dep.stream), off_as_string.concat())
            })
            .collect();
        let input_window_dependencies: Vec<String> = input_dependencies
            .windows
            .iter()
            .map(|cur_input_dep| {
                let window = self.get_window(cur_input_dep.window);
                format!(
                    "--* - {}: ({}, {})\n",
                    self.get_name_for_stream_ref(cur_input_dep.stream),
                    window.duration.as_secs_f64(),
                    get_str_for_sw_op(window.op)
                )
            })
            .collect();
        let dependencies_streams = if input_offset_dependencies.is_empty() {
            String::new()
        } else {
            format!("--* Stream Lookups:\n{}", input_offset_dependencies.concat())
        };
        let dependencies_windows = if input_window_dependencies.is_empty() {
            String::new()
        } else {
            format!("--* Window Lookups:\n{}", input_window_dependencies.concat())
        };

        format!("{}{}", dependencies_streams, dependencies_windows)
    }
}
