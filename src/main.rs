fn main() {
    /* let mut x = true;
    assert!(x, "x is true?");
    x = false;
    assert!(x, "x is true?");
    assert!(x, "x={}",x); */

    let (a,b) = (2,27);
    assert!(a + b == 30, "a = {}, b = {}", a, b);
}
