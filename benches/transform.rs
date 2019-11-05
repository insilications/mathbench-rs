#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

/// Homogeneous 4x4 matrix transform of a 3D vector
fn bench_mat4_transform_vector3(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat4 transform vector3");
    group.bench_function("glam", |b| {
        use glam::{Mat4, Vec3};
        bench_binop!(b, op => transform_vector3, ty1 => Mat4, ty2 => Vec3)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{Matrix4, Transform, Vector3};
        bench_binop!(b, op => transform_vector, ty1 => Matrix4<f32>, ty2 => Vector3<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::{Transform3, Vector3};
        bench_binop!(b, op => transform_vector, ty1 => Transform3<f32>, ty2 => Vector3<f32>, param => by_ref)
    });
    group.bench_function("euclid", |b| {
        use euclid::{Transform3D, Vector3D, UnknownUnit};
        bench_binop!(b, op => transform_vector3d, ty1 => Transform3D<f32, UnknownUnit, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
    });
    group.finish();
}

/// Homogeneous 4x4 matrix transform of a 3D point
fn bench_mat4_transform_point3(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat4 transform point3");
    group.bench_function("glam", |b| {
        use glam::{Mat4, Vec3};
        bench_binop!(b, op => transform_point3, ty1 => Mat4, ty2 => Vec3)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{Matrix4, Point3, Transform};
        bench_binop!(b, op => transform_point, ty1 => Matrix4<f32>, ty2 => Point3<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::{Transform3, Point3};
        bench_binop!(b, op => transform_point, ty1 => Transform3<f32>, ty2 => Point3<f32>, param => by_ref)
    });
    group.bench_function("euclid", |b| {
        use euclid::{Transform3D, Point3D, UnknownUnit};
        bench_binop!(b, op => transform_point3d, ty1 => Transform3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
    });
    group.finish();
}

/// 4x3 matrix multiply of a 4D vector
fn bench_mat4_transform_vector4(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("mat4 transform vector4");
    group.bench_function("glam", |b| {
        use glam::{Mat4, Vec4};
        bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Vec4)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{Matrix4, Vector4};
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::{Matrix4, Vector4};
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
    });
    group.finish();
}

/// Homogeneous 3x3 matrix transform of a 2D point
fn bench_mat3_transform_point2(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat3 transform point2");
    group.bench_function("glam", |b| {
        use glam::{Mat3, Vec2};
        bench_binop!(b, op => transform_point2, ty1 => Mat3, ty2 => Vec2)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{Matrix3, Point2, Transform};
        bench_binop!(b, op => transform_point, ty1 => Matrix3<f32>, ty2 => Point2<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::{Transform2, Point2};
        bench_binop!(b, op => transform_point, ty1 => Transform2<f32>, ty2 => Point2<f32>, param => by_ref)
    });
    group.bench_function("euclid", |b| {
        use euclid::{Transform2D, Point2D, UnknownUnit};
        bench_binop!(b, op => transform_point, ty1 => Transform2D<f32, UnknownUnit, UnknownUnit>, ty2 => Point2D<f32, UnknownUnit>)
    });
    group.finish();
}

/// Homogeneous 3x3 matrix transform of a 2D vector
fn bench_mat3_transform_vector2(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat3 transform vector2");
    group.bench_function("glam", |b| {
        use glam::{Mat3, Vec2};
        bench_binop!(b, op => transform_vector2, ty1 => Mat3, ty2 => Vec2)
    });
    // TODO: doesn't compile but might need a macro change to set return type
    // .with_function("cgmath", |b| {
    //     use cgmath::{Matrix3, Transform, Vector2};
    //     bench_binop!(b, op => transform_vector, ty1 => Matrix3<f32>, ty2 => Vector2<f32>)
    // })
    group.bench_function("nalgebra", |b| {
        use nalgebra::{Transform2, Vector2};
        bench_binop!(b, op => transform_vector, ty1 => Transform2<f32>, ty2 => Vector2<f32>, param => by_ref)
    });
    group.bench_function("euclid", |b| {
        use euclid::{Transform2D, Vector2D, UnknownUnit};
        bench_binop!(b, op => transform_vector, ty1 => Transform2D<f32, UnknownUnit, UnknownUnit>, ty2 => Vector2D<f32, UnknownUnit>)
    });
    group.finish();
}

/// 3x3 matrix multiply of a 3D vector
fn bench_mat3_transform_vector3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("mat3 transform vector3");
    group.bench_function("glam", |b| {
        use glam::{Mat3, Vec3};
        bench_binop!(b, op => mul, ty1 => Mat3, ty2 => Vec3)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{Matrix3, Vector3};
        bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::{Matrix3, Vector3};
        bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
    });
    group.finish();
}

/// 2x2 matrix multiply of a 2D vector
fn bench_mat2_transform_vector2(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("mat2 transform vector2");
    group.bench_function("glam", |b| {
        use glam::{Mat2, Vec2};
        bench_binop!(b, op => mul, ty1 => Mat2, ty2 => Vec2)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{Matrix2, Vector2};
        bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::{Matrix2, Vector2};
        bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
    });
    group.finish();
}

/// Quaternion rotation of a 3D vector
fn bench_quat_transform_vector3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("quat transform vector3");
    group.bench_function("glam", |b| {
        use glam::{Quat, Vec3};
        bench_binop!(b, op => mul, ty1 => Quat, ty2 => Vec3)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{Quaternion, Vector3};
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vector3<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::{UnitQuaternion, Vector3};
        bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => Vector3<f32>)
    });
    group.bench_function("euclid", |b| {
        use euclid::{Rotation3D, Point3D, UnknownUnit};
        bench_binop!(b, op => transform_point3d, ty1 => Rotation3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
    });
    group.finish();
}

criterion_group!(
    transform_benches,
    bench_mat2_transform_vector2,
    bench_mat3_transform_point2,
    bench_mat3_transform_vector2,
    bench_mat3_transform_vector3,
    bench_mat4_transform_point3,
    bench_mat4_transform_vector3,
    bench_mat4_transform_vector4,
    bench_quat_transform_vector3,
);
criterion_main!(transform_benches);
