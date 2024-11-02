use super::*;
use crate::test_helpers::example::VdSynExprExample;
use expect_test::{expect, Expect};
use latex_prelude::mode::LxMode;
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};

fn t(
    input: &str,
    token_annotations: &[((&str, &str), VdTokenAnnotation)],
    space_annotations: &[((&str, &str), VdSpaceAnnotation)],
    expected: &Expect,
) {
    use crate::helpers::show::display_tree::VdSynExprDisplayTreeBuilder;

    let db = &DB::default();
    let example = VdSynExprExample::new(
        input,
        LxMode::Math,
        token_annotations,
        space_annotations,
        db,
    );
    expected.assert_eq(&example.show_display_tree(db));
}

#[test]
fn litera_math_ast_idx_to_vd_syn_expr_idx_works() {
    t(
        "",
        &[],
        &[],
        &expect![[r#"
            "" error
        "#]],
    );
    t(
        "1",
        &[],
        &[],
        &expect![[r#"
            "1" literal
        "#]],
    );
    t(
        "11",
        &[],
        &[],
        &expect![[r#"
            "11" literal
        "#]],
    );
    t(
        "1 1",
        &[],
        &[],
        &expect![[r#"
            "1 1" separated list
            ├─ "1" literal
            └─ "1" literal
        "#]],
    );
}

#[test]
fn basic_arithematics_math_ast_idx_to_vd_syn_expr_idx_works() {
    t(
        "1 + 1",
        &[],
        &[],
        &expect![[r#"
            "1 + 1" binary
            ├─ "1" literal
            └─ "1" literal
        "#]],
    );
    t(
        "1 + 1 = 2",
        &[],
        &[],
        &expect![[r#"
            "1 + 1 = 2" binary
            ├─ "1 + 1" binary
            │ ├─ "1" literal
            │ └─ "1" literal
            └─ "2" literal
        "#]],
    );
    t(
        "1 + 1 = 2",
        &[],
        &[],
        &expect![[r#"
            "1 + 1 = 2" binary
            ├─ "1 + 1" binary
            │ ├─ "1" literal
            │ └─ "1" literal
            └─ "2" literal
        "#]],
    );
}

#[test]
fn xyz_math_ast_idx_to_vd_syn_expr_idx_works() {
    t(
        "x",
        &[],
        &[],
        &expect![[r#"
            "x" letter
        "#]],
    );
    t(
        "y",
        &[],
        &[],
        &expect![[r#"
            "y" letter
        "#]],
    );
    t(
        "z",
        &[],
        &[],
        &expect![[r#"
            "z" letter
        "#]],
    );
    t(
        "xyz",
        &[],
        &[],
        &expect![[r#"
            "z" letter
        "#]],
    );
}

#[test]
fn arithemtics_with_xyz_math_ast_idx_to_vd_syn_expr_idx_works() {
    t(
        "1+x",
        &[],
        &[],
        &expect![[r#"
            "1+x" binary
            ├─ "1" literal
            └─ "x" letter
        "#]],
    );
    t(
        "1+2x",
        &[],
        &[],
        &expect![[r#"
            "1+2x" binary
            ├─ "1" literal
            └─ "2x" separated list
              ├─ "2" literal
              └─ "x" letter
        "#]],
    );
    t(
        "1+x+y+z+xyz",
        &[],
        &[],
        &expect![[r#"
            "1+x+y+z+xyz" binary
            ├─ "1+x+y+z" binary
            │ ├─ "1+x+y" binary
            │ │ ├─ "1+x" binary
            │ │ │ ├─ "1" literal
            │ │ │ └─ "x" letter
            │ │ └─ "y" letter
            │ └─ "z" letter
            └─ "z" letter
        "#]],
    );
}
