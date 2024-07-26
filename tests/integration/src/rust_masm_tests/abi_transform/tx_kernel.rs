use std::fmt::Write;

use expect_test::expect_file;
use miden_assembly::LibraryPath;
use miden_core::{Felt, FieldElement};
use miden_processor::ExecutionError;

use crate::{exec_vm::execute_vm_tracing, execute_emulator, CompilerTest, MidenExecutor};

#[allow(unused)]
fn setup_log() {
    use log::LevelFilter;
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

fn test_get_inputs(test_name: &str, expected_inputs: Vec<Felt>) {
    assert!(expected_inputs.len() == 4, "for now only word-sized inputs are supported");
    let main_fn = "() -> Vec<Felt> { get_inputs() }";
    let artifact_name = format!("abi_transform_tx_kernel_get_inputs_{}", test_name);
    let mut test = CompilerTest::rust_fn_body_with_sdk(&artifact_name, main_fn, true, &[]);
    let masm = format!(
        "
export.get_inputs
    push.{expect1}.{expect2}.{expect3}.{expect4}
    # copy pointer to top of the stack
    dup.4
    mem_storew
    # push the inputs len on the stack
    push.4
end
",
        expect1 = expected_inputs.first().map(|i| i.as_int()).unwrap_or(0),
        expect2 = expected_inputs.get(1).map(|i| i.as_int()).unwrap_or(0),
        expect3 = expected_inputs.get(2).map(|i| i.as_int()).unwrap_or(0),
        expect4 = expected_inputs.get(3).map(|i| i.as_int()).unwrap_or(0),
    );
    test.link_masm_modules = vec![(LibraryPath::new("miden::note").unwrap(), masm)];

    // Test expected compilation artifacts
    test.expect_wasm(expect_file![format!("../../../expected/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../../expected/{artifact_name}.hir")]);
    test.expect_masm(expect_file![format!("../../../expected/{artifact_name}.masm")]);

    let vm_program = test.vm_masm_program();

    let exec = MidenExecutor::new(vec![]);
    let trace = exec.execute(&vm_program);
    let vm_out = trace.into_outputs();
    dbg!(&vm_out);

    // let ir_program = test.ir_masm_program();
    // let emul_out = execute_emulator(ir_program.clone(), &[]);
}

#[test]
fn test_get_inputs_4() {
    test_get_inputs("4", vec![u32::MAX.into(), Felt::ONE, Felt::ZERO, u32::MAX.into()]);
}
