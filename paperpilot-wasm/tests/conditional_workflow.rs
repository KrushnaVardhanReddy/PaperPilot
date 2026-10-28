#[path = "../../paperpilot-pdf/tests/support/workflow_fixture.rs"]
mod fixture;
use fixture::{PageKind::*, pdf};
use paperpilot_wasm::operations::workflow;

#[test]
fn wasm_wrapper_uses_the_shared_conditions_and_operations() {
    let source = pdf(&[Text]);
    let out = workflow(&source, "name='WASM'\n[[stage]]\nid='skip'\nop='rotate'\ncondition={if_scanned=true}\n[[stage]]\nid='bates'\nop='bates'\n[stage.params]\nprefix='CHECK-'").unwrap();
    let d = lopdf::Document::load_mem(&out).unwrap();
    assert_eq!(d.get_pages().len(), 1);
    assert!(d.extract_text(&[1]).unwrap().contains("CHECK-000001"));
}

#[test]
fn wasm_wrapper_rejects_unknown_operations_instead_of_returning_original() {
    assert!(
        workflow(
            &pdf(&[Text]),
            "name='Bad'\n[[stage]]\nid='bad'\nop='unknown'"
        )
        .is_err()
    );
}
