extern crate glutin;

use glutin::HeadlessRendererBuilder;

#[test]
fn test_headless() {
    HeadlessRendererBuilder::new(256, 256).build().unwrap();
}
