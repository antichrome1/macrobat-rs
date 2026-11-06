
// Test local macros ===========================================================
use macrobat::macros;

// Macros available to this file
macros! {
    /// doc comment
    macro add($x:expr, $y:expr) {
        $x + $y
    }

    macro echo_wrap($x:expr) {
        let _gg = 11;

        $x
    } @wrap
}

#[test]
fn local_export() {
    assert_eq!(3, add!(1, 2));
}

#[test]
fn local_export_wrap() {
    assert_eq!(1, echo_wrap!(1));
}


// Test external module use case ===============================================
// Import macros from a module
#[path = "macros__define_ext_macros.rs"]
pub mod my_macros;

#[test]
fn external_export() {
    assert_eq!(3, ext_add!(1, 2));
}

#[test]
fn external_export_wrap() {
    assert_eq!(1, ext_echo_wrap!(1));
}

// should fail
// #[test]
// fn external_noexport() {
//     let _ = ext_noexport!();
// }
