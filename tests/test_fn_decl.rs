use syntax::ast;
use syntax::codemap::DUMMY_SP;
use syntax::ptr::P;

use aster::AstBuilder;

#[test]
fn test_no_args_return_isize() {
    let builder = AstBuilder::new();
    let fn_decl = builder.fn_decl().return_().isize();

    assert_eq!(
        fn_decl,
        P(ast::FnDecl {
            inputs: vec![],
            output: ast::FunctionRetTy::Ty(builder.ty().isize()),
            variadic: false,
        })
    );
}

#[test]
fn test_args_return_isize() {
    let builder = AstBuilder::new();
    let fn_decl = builder.fn_decl()
        .arg_id("x").ty().isize()
        .arg_id("y").ty().isize()
        .return_().isize();

    assert_eq!(
        fn_decl,
        P(ast::FnDecl {
            inputs: vec![
                ast::Arg {
                    ty: builder.ty().isize(),
                    pat: builder.pat().id("x"),
                    id: ast::DUMMY_NODE_ID,
                },
                ast::Arg {
                    ty: builder.ty().isize(),
                    pat: builder.pat().id("y"),
                    id: ast::DUMMY_NODE_ID,
                },
            ],
            output: ast::FunctionRetTy::Ty(builder.ty().isize()),
            variadic: false,
        })
    );
}

#[test]
fn test_no_return() {
    let builder = AstBuilder::new();
    let fn_decl = builder.fn_decl().no_return();

    assert_eq!(
        fn_decl,
        P(ast::FnDecl {
            inputs: vec![],
            output: ast::FunctionRetTy::Ty(builder.ty().never()),
            variadic: false,
        })
    );
}

#[test]
fn test_default_return() {
    let builder = AstBuilder::new();
    let fn_decl = builder.fn_decl().default_return();

    assert_eq!(
        fn_decl,
        P(ast::FnDecl {
            inputs: vec![],
            output: ast::FunctionRetTy::Default(DUMMY_SP),
            variadic: false,
        })
    );
}
