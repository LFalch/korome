extern crate korome;

use korome::Vector2;

#[test]
fn feminism(){
    let v0 = Vector2::from((3., 6.));
    let v1 = Vector2::from([3., 6.]);
    let something_different = Vector2(455., 1.2);
    let nan = Vector2(4., 0./0.);

    assert_eq!(v0, v1);
    assert_eq!(v1, v0);
    assert_eq!(something_different, something_different);
    assert!(nan != nan);
    assert!(nan != v0);
    assert!(nan != v1);
    assert!(v0 != something_different);
    assert!(something_different != v0);
    assert!(v1 != something_different);
    assert!(something_different != v1);
}

#[test]
fn numberwang(){
    let v0 = Vector2(5., 3.);
    let v1 = Vector2(4., 4.);

    assert_eq!(v0-v1, Vector2(1., -1.));
    assert_eq!(v0+v1, Vector2(9., 7.));
    assert_eq!(v0*4., Vector2(20., 12.));
    assert_eq!(v0/2., Vector2(2.5, 1.5));
    assert_eq!(v1*1., v1);
    assert_eq!(v1/2., Vector2(2., 2.));
    assert_eq!(-v0, Vector2(-5., -3.));
    assert_eq!(-v1, Vector2(-4., -4.));
}

#[test]
fn following_directions(){
    let v = Vector2(3., 4.);

    assert_eq!(v.length(), 5.);
    assert_eq!(v.normalise().length(), 1.);
    assert_eq!(v.direction(), 0.9272952180016122);
    assert!((v.normalise()-Vector2::unit_vector(0.9272952180016122)).length() < std::f64::EPSILON);
    assert_eq!(Vector2(1., 1.).direction(), std::f64::consts::FRAC_PI_4);
    assert_eq!(Vector2(2., -4.).distance_to(Vector2(4., -4.)), 2.);
    assert_eq!(Vector2(2., -4.).direction_to(Vector2(4., -4.)), 0.);
    assert_eq!(Vector2(2., 2.).direction_to(Vector2(4., 4.)), std::f64::consts::FRAC_PI_4);
}
