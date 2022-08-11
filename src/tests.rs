
#[cfg(test)] 
mod tests {
    use std::f32::consts::PI;

    use glmath::glmath::*;

    #[test]
    fn test_vec2() {
        let v1 = Vec2f::new(10.0, 0.0);
        let v2 = Vec2f::new(1.0, 1.0).get_normalized();
        let zero = Vec2f::ZERO;

        assert_eq!(v1.yx().yx(), v1);
        assert_eq!(v1.length(), 10.0);
        assert_eq!(v1.get_normalized().length(), 1.0);
        assert_ne!(v1, v2 * 10.0);

        assert_eq!(zero.length(), 0.0);
        // This should not throw even though without the comparions there's a divide by zero.
        assert_eq!(zero, zero.get_normalized());

        // The angle between any two axes should be 90 degrees.
        assert_eq!(Vec2f::X.angle_between(&Vec2f::Y), (2.0 * PI) / 4.0);

        // Dot product.
        let a = Vec2f::new(1.0, 1.0);
        let b = Vec2f::new(1.0, 1.0);
        assert_eq!(a * b, 2.0);
    }

    #[test]
    fn test_vec3() {
        let v1 = Vec3f::new(10.0, 0.0, 0.0);
        let v2 = Vec3f::new(1.0, 1.0, 0.0).get_normalized();
        let zero = Vec3f::ZERO;

        assert_eq!(v1.yx().yx(), v1.xy());
        assert_eq!(v1.length(), 10.0);
        assert_eq!(v1.get_normalized().length(), 1.0);
        assert_ne!(v1, v2 * 10.0);

        assert_eq!(zero.length(), 0.0);
        // This should not throw even though without the comparions there's a divide by zero.
        assert_eq!(zero, zero.get_normalized());

        // The angle between any two axes should be 90 degrees.
        assert_eq!(Vec3f::X.angle_between(&Vec3f::Y), (2.0 * PI) / 4.0);

        // Cross product.
        let cross = v1 % Vec3f::new(1.0, 1.0, 0.0);
        assert_eq!(Vec3f::new(0.0, 0.0, 10.0), cross);

        let cross = Vec3f::new(32.0, 1.4, 30.0) %
                               Vec3f::new(10.0, 10.1, 22.0);
        assert_eq!(Vec3f::new(-272.2, -404.0, 309.2), cross);

        assert_eq!(Vec3f::ZERO % Vec3f::ZERO, Vec3f::ZERO);
    }

    #[test]
    fn test_vec4() {
        let v1 = Vec4f::new(10.0, 0.0, 0.0, 0.0);
        let v2 = Vec4f::new(1.0, 1.0, 0.0, 0.0).get_normalized();
        let zero = Vec4f::ZERO;

        assert_eq!(v1.yx().yx(), v1.xy());
        assert_eq!(v1.length(), 10.0);
        assert_eq!(v1.get_normalized().length(), 1.0);
        assert_ne!(v1, v2 * 10.0);

        assert_eq!(zero.length(), 0.0);
        // This should not throw even though without the comparions there's a divide by zero.
        assert_eq!(zero, zero.get_normalized());

        // The angle between any two axes should be 90 degrees.
        assert_eq!(Vec2f::X.angle_between(&Vec2f::Y), (2.0 * PI) / 4.0);
    }

    #[test]
    fn test_quat() {
        // Do nothing yet, most of the functionality isn't in.
    }

    #[test]
    fn test_mat22() {
        let a = Mat22f::new();
        let b = Mat22f::new();

        // Idt * Idt = Idt.
        assert_eq!(a, b);
        assert_eq!(a, Mat22f::IDENTITY);
        assert_eq!(a * b, Mat22f::IDENTITY);

        // Transforming a vector by identity results in itself.
        assert_eq!(Mat22f::IDENTITY * Vec2f::new(1.0, 1.0), 
            Vec2f::new(1.0, 1.0));

        // Scale a vector two times on the x and half on the y.
        let scale_mat = Mat22f::from_axes(
            Vec2f::new(2.0, 0.0), Vec2f::new(0.0, 0.5));
        let scaled_vec = scale_mat * Vec2f::new(1.0, 1.0);
        assert_eq!(scaled_vec, Vec2f::new(2.0, 0.5));

        // Scale another matrix by the same value.
        // Idt * scale matrix = scale matrix.
        assert_eq!(Mat22f::IDENTITY * scale_mat, scale_mat);
    }

    #[test]
    fn test_mat33() { 
        let a = Mat33f::new();
        let b = Mat33f::new();

        // Idt * Idt = Idt.
        assert_eq!(a, b);
        assert_eq!(a, Mat33f::IDENTITY);
        assert_eq!(a * b, Mat33f::IDENTITY);

        // Transforming a vector by identity results in itself.
        assert_eq!(Mat33f::IDENTITY * Vec3f::new(1.0, 1.0, 1.0), 
            Vec3f::new(1.0, 1.0, 1.0));

        // Scale a vector two times on the x and half on the y.
        let scale_mat = Mat33f::from_axes(
            Vec3f::new(2.0, 0.0, 0.0), 
            Vec3f::new(0.0, 0.5, 0.0),
            Vec3f::new(0.0, 0.0, 1.0)
        );

        let scaled_vec = scale_mat * Vec3f::new(1.0, 1.0, 1.0);
        assert_eq!(scaled_vec, Vec3f::new(2.0, 0.5, 1.0));

        // Scale another matrix by the same value.
        // Idt * scale matrix = scale matrix.
        assert_eq!(Mat33f::IDENTITY * scale_mat, scale_mat);
    }

    #[test]
    fn test_mat44() {
        let a = Mat44f::new();
        let b = Mat44f::new();

        // Idt * Idt = Idt.
        assert_eq!(a, b);
        assert_eq!(a, Mat44f::IDENTITY);
        assert_eq!(a * b, Mat44f::IDENTITY);

        // Transforming a vector by identity results in itself.
        assert_eq!(Mat44f::IDENTITY * Vec4f::new(1.0, 1.0, 1.0, 1.0), 
            Vec4f::new(1.0, 1.0, 1.0, 1.0));

        // Scale a vector two times on the x and half on the y.
        let scale_mat = Mat44f::IDENTITY.get_scaled(Vec3f::new(2.0, 0.5, 1.0));

        let scaled_vec = scale_mat * Vec4f::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(scaled_vec, Vec4f::new(2.0, 0.5, 1.0, 1.0));

        // Scale another matrix by the same value.
        // Idt * scale matrix = scale matrix.
        assert_eq!(Mat44f::IDENTITY * scale_mat, scale_mat);
    }
}