extern crate gynoo_r;
use gynoo_r::geometry;

#[test]
fn test_gvector() {

    let mut v1 = geometry::GVector{x:1.0, y:2.0, z:3.0};
    let mut v2 = geometry::GVector{x:2.0, y:3.0, z:4.0};
    let mut v3 = v1 + v2;
    let mut v4 = geometry::GVector{x:20.0, y:30.0, z:40.0};
    println!("{:?}", v3);
    println!("{:?}", v1.clone()+v4);
}
