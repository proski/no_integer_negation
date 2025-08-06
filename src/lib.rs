#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_middle;

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{Expr, ExprKind, UnOp};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{Ty, TyKind};

dylint_linting::declare_late_lint! {
    /// **What it does:** Disallows the `!` (bitwise NOT/negation) operator on integral types.
    ///
    /// **Why is this bad?** This lint enforces a coding standard that prohibits bitwise
    /// negation on integral types, which might be part of a security policy or coding
    /// convention to avoid certain patterns.
    ///
    /// **Example:**
    /// ```rust
    /// // Bad
    /// let x = 5u32;
    /// let y = !x; // This will trigger the lint
    ///
    /// let z = 10i64;
    /// let w = !z; // This will also trigger the lint
    /// ```
    ///
    /// **Use instead:**
    /// Consider using explicit bitwise operations or alternative approaches
    /// depending on your use case.
    pub NO_INTEGRAL_NEGATION,
    Warn,
    "disallow bitwise NOT operator on integral types"
}

impl<'tcx> LateLintPass<'tcx> for NoIntegralNegation {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if let ExprKind::Unary(UnOp::Not, operand) = expr.kind {
            let operand_ty = cx.typeck_results().expr_ty(operand);

            if is_integral_type(operand_ty) {
                span_lint_and_help(
                    cx,
                    NO_INTEGRAL_NEGATION,
                    expr.span,
                    "bitwise NOT operator `!` is not allowed on integral types",
                    None,
                    "consider using explicit bitwise operations or alternative approaches",
                );
            }
        }
    }
}

/// Check if the given type is an integral type
fn is_integral_type(ty: Ty<'_>) -> bool {
    match ty.kind() {
        TyKind::Int(_) | TyKind::Uint(_) => true,
        // Also handle type aliases and other indirections
        TyKind::Adt(adt_def, _) if adt_def.is_enum() => {
            // Handle cases like std::os::raw::c_int which might be integral types
            false
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {

    use dylint_testing::ui_test;

    #[test]
    fn test_no_integral_negation() {
        ui_test(
            env!("CARGO_PKG_NAME"),
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
        );
    }
}

// Test cases for the lint (would typically go in a separate ui test file)
#[cfg(test)]
mod test_cases {
    #[allow(dead_code)]
    fn test_cases() {
        // These should trigger the lint
        let x: u32 = 5;
        let _y = !x; // Should warn

        let z: i64 = 10;
        let _w = !z; // Should warn

        let a: usize = 100;
        let _b = !a; // Should warn

        let c: i8 = -5;
        let _d = !c; // Should warn

        // These should NOT trigger the lint
        let flag = true;
        let _not_flag = !flag; // OK - boolean

        let opt: Option<i32> = Some(42);
        if !opt.is_some() { // OK - boolean result
             // ...
        }

        // Bitwise operations that are not negation should be OK
        let e: u32 = 0b1010;
        let f: u32 = 0b1100;
        let _g = e & f; // OK - bitwise AND
        let _h = e | f; // OK - bitwise OR
        let _i = e ^ f; // OK - bitwise XOR
    }
}
