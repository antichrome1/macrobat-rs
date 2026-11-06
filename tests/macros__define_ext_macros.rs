use macrobat::macros;

macros! {

    macro ext_add($x:expr, $y:expr) {
        $x + $y
    } @export

    macro ext_echo_wrap($x:expr) {
        let _gg = 11;

        $x
    } @export @wrap

    // macro ext_noexport($x:expr, $y:expr) {

    // }

}
