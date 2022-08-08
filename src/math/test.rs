use crate::mat3_zero;
use crate::mat3_identity;
use crate::mat4_zero;
use crate::mat4_identity;

use super::preclude::*;


#[test]
fn test_mat3_macro() {
    let a = mat3_identity!();
    assert_eq!(a, Mat3 { elem: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]] });
    let a = mat3_zero!();
    assert_eq!(a, Mat3 { elem: [[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]] });
}

#[test]
fn test_mat4_macro() {
    let a = mat4_identity!();
    assert_eq!(a, Mat4 { elem: [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]]});
    let a = mat4_zero!();
    assert_eq!(a, Mat4 { elem: [[0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.]]});
}

#[test]
fn test_mat3_impl() {
    let a = Mat3 { elem: [
        [1., 2., 3.],
        [4., 5., 6.],
        [7., 8., 9.],
    ] };

    let b = Mat3 { elem: [
        [9., 8., 7.],
        [6., 5., 4.],
        [3., 2., 1.],
    ] };

    // impl ops::Mul for Mat3
    assert_eq!(a * b, Mat3 { elem: [
        [30., 24., 18.],
        [84., 69., 54.],
        [138., 114., 90.],
    ]});

    // fn transpose(self) -> Mat3
    let c = b.transpose();
    assert_eq!(c, Mat3 { elem: [
        [9., 6., 3.],
        [8., 5., 2.],
        [7., 4., 1.],
    ] });
    assert_eq!(b, Mat3 { elem: [
        [9., 8., 7.],
        [6., 5., 4.],
        [3., 2., 1.],
    ] });

    // impl ops::Mul<Vec4> for Mat3
    let v = Vec3::from(&[1., 0., 1.]);
    let m = Mat3 { elem:[
        [1., 1., 1.],
        [2., 2., 2.],
        [3., 3., 3.],
    ] };
    assert_eq!(m * v, Vec3::from(&[2., 4., 6.]));
    
    // impl ops::Index<usize> for Mat3
    let m1 = m[0];
    assert_eq!(m1, [1., 1., 1.]);
    let m11 = m[0][0];
    assert_eq!(m11, 1.);

    // impl ops::IndexMut<usize> for Mat3
    let mut m22 = m[1][1];
    m22 = 0.;
    assert_eq!(m22, 0.);
    assert_eq!(m[1][1], 2.);
}

#[test]
fn test_mat4_impl() {
    let a = Mat4 { elem: [
        [1., 2., 3., 1.],
        [4., 5., 6., 0.],
        [7., 8., 9., 1.],
        [1., 0., 1., 0.],
    ] };

    let b = Mat4 { elem: [
        [9., 8., 7., 0.],
        [6., 5., 4., 1.],
        [3., 2., 1., 0.],
        [0., 1., 0., 1.],
    ] };

    // impl ops::Mul for Mat4
    assert_eq!(a * b, Mat4 { elem: [
        [30., 25., 18., 3.],
        [84., 69., 54., 5.],
        [138., 115., 90., 9.],
        [12., 10., 8., 0.],
    ]});

    // fn transpose(self) -> Mat4
    let c = b.transpose();
    assert_eq!(c, Mat4 { elem: [
        [9., 6., 3., 0.],
        [8., 5., 2., 1.],
        [7., 4., 1., 0.],
        [0., 1., 0., 1.],
    ] });
    assert_eq!(b, Mat4 { elem: [
        [9., 8., 7., 0.],
        [6., 5., 4., 1.],
        [3., 2., 1., 0.],
        [0., 1., 0., 1.],
    ] });

    // impl ops::Mul<Vec4> for Mat4
    let v = Vec4::from(&[1., 0., 1., 0.]);
    let m = Mat4 { elem:[
        [1., 1., 1., 1.],
        [2., 2., 2., 2.],
        [3., 3., 3., 3.],
        [4., 4., 4., 4.],
    ] };
    assert_eq!(m * v, Vec4::from(&[2., 4., 6., 8.]));

    // Note: There is a precision problem for floating point number.
    // Calculate the inverse of the matrix.
    // impl ops::Not for Mat4
    // assert_eq!(!m, mat4_zero!());
    // let m = Mat4 { elem: [
    //     [2., 1., 1., 1.],
    //     [2., 3., 1., 1.],
    //     [2., 2., 5., 1.],
    //     [2., 2., 2., 7.],
    // ] };
    //
    // assert_eq!(!m, Mat4{ elem: [
    //     [27./32., -5./32., -5./48., -1./12.],
    //     [-1./2., 1./2., 0., 0.],
    //     [-1./8., -1./8., 1./4., 0.],
    //     [-1./16., -1./16., -1./24., 1./6.],
    // ] });

    // impl ops::Index for Mat4
    let m1 = m[0];
    assert_eq!(m1, [1., 1., 1., 1.]);
    let m11 = m[0][0];
    assert_eq!(m11, 1.);
}
