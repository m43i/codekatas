fn xo(string: &'static str) -> bool {
    let mut x_count = 0;
    let mut o_count = 0;
    for c in string.chars() {
        match c {
            'x' | 'X' => x_count += 1,
            'o' | 'O' => o_count += 1,
            _ => (),
        }
    }
    x_count == o_count
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
