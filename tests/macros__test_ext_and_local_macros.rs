
// Test local macros ===========================================================
use macrobat::macros;

// Macros available to this file
macros! {
    /// doc comment
    macro add($x:expr, $y:expr) {
        $x + $y
    }

    macro collatz_steps($x:expr) {
        let mut n = $x;
        let mut steps = 0u64;
        while n > 1 {
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            steps += 1;
        }
        steps
    } @export @wrap

}

#[test]
fn local_export() {
    assert_eq!(3, add!(1, 2));
}

#[test]
fn local_export_wrap() {
    assert_eq!(111, collatz_steps!(27));
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
    assert_eq!(111, ext_collatz_steps!(27));
}

// should fail
// #[test]
// fn external_noexport() {
//     let _ = ext_noexport!();
// }
