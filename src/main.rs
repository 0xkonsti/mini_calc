use std::path::Path;

use inkwell::context::Context;

mod codegen;
mod lexer;
mod link;
mod parser;

fn main() {
    let input = "3 + 5 * (10 - 2) / 4.0";
    //let input = "3 + 5";

    println!("Input: {}", input);

    let tokens = lexer::lex(input);

    println!("Tokens:");
    for token in tokens.clone() {
        println!("    {:?}", token);
    }

    let ast = parser::parse(&mut tokens.clone());

    println!("Parsed Expression: {:#?}", ast);

    let context = Context::create();
    let module = context.create_module("mini_calc");
    let builder = context.create_builder();

    let fn_type = context.i32_type().fn_type(&[], false);
    let main_fn = module.add_function("main", fn_type, None);
    let entry_block = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(entry_block);

    let ir = codegen::gen_ir(&ast, &builder, &context, &module);

    let printf = codegen::declare_printf(&module, &context);
    let fmt_str = builder.build_global_string_ptr("Result: %f\n\0", "fmt").expect("Failed to create format string");

    builder
        .build_call(printf, &[fmt_str.as_pointer_value().into(), ir.into()], "printf_call")
        .expect("Failed to call printf");

    let ret_val = context.i32_type().const_int(0, false);
    builder.build_return(Some(&ret_val)).expect("Failed to build return");

    module.print_to_stderr();

    // Create build directory if it doesn't exist
    if !Path::new("build").exists() {
        std::fs::create_dir("build").expect("Failed to create build directory");
    }

    let obj_path = Path::new("build/calc_output.o");
    codegen::write_obj(&module, obj_path);

    let mut executable_name = "build/calc_output".to_string();
    if cfg!(windows) {
        executable_name.push_str(".exe");
    }

    link::Linker::Clang.link_obj(obj_path.to_str().unwrap(), &executable_name);
}
