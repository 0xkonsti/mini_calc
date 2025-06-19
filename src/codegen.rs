use std::path::Path;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    targets::{FileType, InitializationConfig, Target, TargetMachine},
    values::{FloatValue, FunctionValue},
};

use crate::parser::{Expr, Op};

pub fn gen_ir<'a>(expr: &'a Expr, builder: &'a Builder, context: &'a Context, module: &'a Module) -> FloatValue<'a> {
    match expr {
        Expr::Number(f) => context.f64_type().const_float(*f).into(),
        Expr::BinaryOp(lhs, op, rhs) => {
            let lhs_val = gen_ir(lhs, builder, context, module);
            let rhs_val = gen_ir(rhs, builder, context, module);

            match op {
                Op::Add => builder.build_float_add(lhs_val, rhs_val, "add").unwrap(),
                Op::Sub => builder.build_float_sub(lhs_val, rhs_val, "sub").unwrap(),
                Op::Mul => builder.build_float_mul(lhs_val, rhs_val, "mul").unwrap(),
                Op::Div => builder.build_float_div(lhs_val, rhs_val, "div").unwrap(),
            }
        }
    }
}

pub fn declare_printf<'ctx>(module: &Module<'ctx>, context: &'ctx Context) -> FunctionValue<'ctx> {
    let i8ptr = context.i8_type().ptr_type(0.into());
    let printf_type = context.i32_type().fn_type(&[i8ptr.into()], true);
    module.add_function("printf", printf_type, None)
}

pub fn write_obj(module: &Module, output_path: &Path) {
    Target::initialize_all(&InitializationConfig::default());

    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).expect("Failed to get target from triple");
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            inkwell::OptimizationLevel::Default,
            inkwell::targets::RelocMode::Default,
            inkwell::targets::CodeModel::Default,
        )
        .expect("Failed to create target machine");

    module.set_triple(&target_triple);
    target_machine.write_to_file(module, FileType::Object, output_path).expect("Failed to write module to file");
}
