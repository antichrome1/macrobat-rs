use macrobat::macros;

macros! {

    macro ext_add($x:expr, $y:expr) {
        $x + $y
    } @export


    macro ext_collatz_steps($x:expr) {
        let mut n = $x;
        let mut steps = 0u64;
        while n > 1 {
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            steps += 1;
        }
        steps
    } @export @wrap

    // macro ext_noexport($x:expr, $y:expr) {

    // }

}
