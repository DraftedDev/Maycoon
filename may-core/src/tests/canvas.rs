use crate::canvas::Canvas;

#[test]
fn validate_size() {
    const WIDTH: i32 = 4096;
    const HEIGHT: i32 = 2048;

    assert_eq!(
        Canvas::new(WIDTH, HEIGHT).data().len(),
        (WIDTH * HEIGHT) as usize
    )
}
