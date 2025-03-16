//! Angle module providing flexible angle representation and manipulation.
//!
//! This module provides the `Angle` struct and related functionality for working
//! with angles in a type-safe and convenient way. Angles can be created from
//! degrees or radians and converted between units seamlessly.
//!
//! # Usage
//!
//! ```rust
//! # use sfml::system::Angle;
//!
//! // Create angles
//! let a1 = Angle::degrees(90.0);
//! let a2 = Angle::radians(std::f32::consts::PI / 2.0);
//!
//! // Convert between units
//! let deg_value = a1.as_degrees(); // 90.0
//! let rad_value = a2.as_radians(); // ~1.5708
//!
//! // Mathematical operations
//! let sum = a1 + a2;
//! let doubled = a1 * 2.0;
//!
//! // Angle wrapping
//! let wrapped = Angle::degrees(450.0).wrap_unsigned(); // 90 degrees
//! let signed = Angle::degrees(-270.0).wrap_signed();   // 90 degrees
//! ```

use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::f32::consts::{PI, TAU};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/// Internal utility functions
mod utils {
    /// Computes positive remainder for angle wrapping
    /// This ensures the result is always positive, unlike Rust's % operator
    #[inline]
    pub const fn positive_remainder(a: f32, b: f32) -> f32 {
        let result = a % b;
        if result >= 0.0 { result } else { result + b }
    }
}

/// Represents an angle value in a flexible way.
///
/// `Angle` encapsulates an angle value that can be defined as either degrees
/// or radians, and allows reading the value in either unit. This provides
/// flexibility without imposing a fixed representation on the user.
///
/// Internally, angles are stored as radians for precision and performance.
/// All mathematical operations preserve the angle semantics.
///
/// # Examples
///
/// ```rust
/// # use sfml::system::Angle;
///
/// let a1 = Angle::degrees(90.0);
/// let radians_val = a1.as_radians(); // π/2
///
/// let a2 = Angle::radians(std::f32::consts::PI);
/// let degrees_val = a2.as_degrees(); // 180.0
///
/// // Mathematical operations
/// let sum = a1 + a2; // 270 degrees total
/// let scaled = a1 * 2.0; // 180 degrees
/// ```
#[derive(Clone, Copy, Default)]
pub struct Angle {
    /// Angle value stored internally as radians
    radians: f32,
}

impl Angle {
    /// Creates an angle from a number of degrees.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let right_angle = Angle::degrees(90.0);
    /// let straight_angle = Angle::degrees(180.0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn degrees(angle: f32) -> Angle {
        Angle::new_radians(angle * (PI / 180.0))
    }

    /// Creates an angle from a number of radians.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    /// # use std::f32::consts::PI;
    ///
    /// let right_angle = Angle::radians(PI / 2.0);
    /// let straight_angle = Angle::radians(PI);
    /// ```
    #[inline]
    #[must_use]
    pub const fn radians(angle: f32) -> Angle {
        Angle::new_radians(angle)
    }

    /// Creates an angle from radians (internal constructor)
    #[inline]
    const fn new_radians(radians: f32) -> Self {
        Self { radians }
    }

    /// Returns the angle value as degrees.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    /// # use std::f32::consts::PI;
    ///
    /// let angle = Angle::radians(PI);
    /// assert_eq!(angle.as_degrees(), 180.0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_degrees(self) -> f32 {
        self.radians * (180.0 / PI)
    }

    /// Returns the angle value as radians.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    /// use std::f32::consts::PI;
    ///
    /// let angle = Angle::degrees(180.0);
    /// assert!((angle.as_radians() - PI).abs() < f32::EPSILON);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_radians(self) -> f32 {
        self.radians
    }

    /// Wraps the angle to the range [-180°, 180°) (signed representation).
    ///
    /// This is similar to a modulo operation that constrains the angle to the
    /// range [-π, π) in radians. The resulting angle represents an equivalent
    /// rotation to the original angle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let angle = Angle::degrees(450.0);
    /// let wrapped = angle.wrap_signed();
    /// assert!((wrapped.as_degrees() - 90.).abs() < 0.0001);
    ///
    /// let angle = Angle::degrees(-270.0);
    /// let wrapped = angle.wrap_signed();
    /// assert!((wrapped.as_degrees() - 90.).abs() < 0.0001);
    /// ```
    #[inline]
    #[must_use]
    pub const fn wrap_signed(self) -> Self {
        let wrapped = utils::positive_remainder(self.radians + PI, TAU) - PI;
        Self::new_radians(wrapped)
    }

    /// Wraps the angle to the range [0°, 360°) (unsigned representation).
    ///
    /// This is similar to a modulo operation that constrains the angle to the
    /// range [0, 2π) in radians. The resulting angle represents an equivalent
    /// rotation to the original angle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let angle = Angle::degrees(450.0);
    /// let wrapped = angle.wrap_unsigned();
    /// assert!((wrapped.as_degrees() - 90.).abs() < 0.0001);
    ///
    /// let angle = Angle::degrees(-90.0);
    /// let wrapped = angle.wrap_unsigned();
    /// assert!((wrapped.as_degrees() - 270.).abs() < 0.0001);
    /// ```
    #[inline]
    #[must_use]
    pub const fn wrap_unsigned(self) -> Self {
        let wrapped = utils::positive_remainder(self.radians, TAU);
        Self::new_radians(wrapped)
    }
}

// ============================================================================
// Comparison operators
// ============================================================================

impl PartialEq for Angle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.radians == other.radians
    }
}

impl PartialOrd for Angle {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.radians.partial_cmp(&other.radians)
    }
}

// ============================================================================
// Arithmetic operators
// ============================================================================

impl Neg for Angle {
    type Output = Angle;

    /// Negates an angle, representing rotation in the opposite direction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let angle = Angle::degrees(90.0);
    /// let negated = -angle;
    /// assert_eq!(negated.as_degrees(), -90.0);
    /// ```
    #[inline]
    fn neg(self) -> Self::Output {
        Angle::new_radians(-self.radians)
    }
}

impl Add for Angle {
    type Output = Angle;

    /// Adds two angles together.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let a1 = Angle::degrees(45.0);
    /// let a2 = Angle::degrees(45.0);
    /// let sum = a1 + a2;
    /// assert_eq!(sum.as_degrees(), 90.0);
    /// ```
    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Angle::new_radians(self.radians + other.radians)
    }
}

impl AddAssign for Angle {
    /// Adds another angle to this angle in place.
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Angle {
    type Output = Angle;

    /// Subtracts one angle from another.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let a1 = Angle::degrees(90.0);
    /// let a2 = Angle::degrees(45.0);
    /// let diff = a1 - a2;
    /// assert_eq!(diff.as_degrees(), 45.0);
    /// ```
    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Angle::new_radians(self.radians - other.radians)
    }
}

impl SubAssign for Angle {
    /// Subtracts another angle from this angle in place.
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<f32> for Angle {
    type Output = Angle;

    /// Multiplies an angle by a scalar value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let angle = Angle::degrees(45.0);
    /// let doubled = angle * 2.0;
    /// assert_eq!(doubled.as_degrees(), 90.0);
    /// ```
    #[inline]
    fn mul(self, scalar: f32) -> Self::Output {
        Angle::new_radians(self.radians * scalar)
    }
}

impl Mul<Angle> for f32 {
    type Output = Angle;

    /// Multiplies an angle by a scalar value (commutative).
    #[inline]
    fn mul(self, angle: Angle) -> Self::Output {
        angle * self
    }
}

impl MulAssign<f32> for Angle {
    /// Multiplies this angle by a scalar in place.
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        *self = *self * scalar;
    }
}

impl Div<f32> for Angle {
    type Output = Angle;

    /// Divides an angle by a scalar value.
    ///
    /// # Panics
    ///
    /// Panics if the divisor is zero in debug mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let angle = Angle::degrees(90.0);
    /// let half = angle / 2.0;
    /// assert_eq!(half.as_degrees(), 45.0);
    /// ```
    #[inline]
    fn div(self, scalar: f32) -> Self::Output {
        debug_assert!(scalar != 0.0, "Angle division by zero");
        Angle::new_radians(self.radians / scalar)
    }
}

impl DivAssign<f32> for Angle {
    /// Divides this angle by a scalar in place.
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        *self = *self / scalar;
    }
}

impl Div for Angle {
    type Output = f32;

    /// Divides one angle by another, returning the ratio.
    ///
    /// # Panics
    ///
    /// Panics if the divisor angle is zero in debug mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let a1 = Angle::degrees(90.0);
    /// let a2 = Angle::degrees(45.0);
    /// let ratio = a1 / a2;
    /// assert_eq!(ratio, 2.0);
    /// ```
    #[inline]
    fn div(self, other: Self) -> Self::Output {
        debug_assert!(other.radians != 0.0, "Angle division by zero angle");
        self.radians / other.radians
    }
}

impl Rem for Angle {
    type Output = Angle;

    /// Computes the remainder of angle division.
    ///
    /// This operation ensures the result is always positive, unlike Rust's
    /// standard `%` operator. The right-hand angle must be greater than zero.
    ///
    /// # Panics
    ///
    /// Panics if the right-hand angle is zero in debug mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let a1 = Angle::degrees(270.0);
    /// let a2 = Angle::degrees(360.0);
    /// let rem = a1 % a2;
    /// assert_eq!(rem.as_degrees(), 270.0);
    ///
    /// let a3 = Angle::degrees(-90.0);
    /// let a4 = Angle::degrees(360.0);
    /// let rem2 = a3 % a4;
    /// assert_eq!(rem2.as_degrees(), 270.0); // Always positive
    /// ```
    #[inline]
    fn rem(self, other: Self) -> Self::Output {
        debug_assert!(other.radians > 0.0, "Angle modulus by non-positive angle");
        Angle::new_radians(utils::positive_remainder(self.radians, other.radians))
    }
}

impl RemAssign for Angle {
    /// Computes the remainder in place.
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}

// ============================================================================
// Display implementations
// ============================================================================

impl Debug for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Angle")
            .field("degrees", &self.as_degrees())
            .field("radians", &self.radians)
            .finish()
    }
}

impl Display for Angle {
    /// Displays the angle in degrees with the degree symbol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sfml::system::Angle;
    ///
    /// let angle = Angle::degrees(45.0);
    /// assert_eq!(format!("{}", angle), "45°");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}°", self.as_degrees())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn assert_angle_eq(actual: Angle, expected_degrees: f32) {
        let diff = (actual.as_degrees() - expected_degrees).abs();
        assert!(
            diff < 0.0001,
            "Expected {} degrees, got {} degrees (diff: {})",
            expected_degrees,
            actual.as_degrees(),
            diff
        );
    }

    #[test]
    fn test_angle_creation() {
        let deg_angle = Angle::degrees(90.0);
        let rad_angle = Angle::radians(PI / 2.0);

        assert_angle_eq(deg_angle, 90.0);
        assert_angle_eq(rad_angle, 90.0);
    }

    #[test]
    fn test_angle_conversion() {
        let angle = Angle::degrees(180.0);
        assert!((angle.as_radians() - PI).abs() < f32::EPSILON);
        assert_eq!(angle.as_degrees(), 180.0);
    }

    #[test]
    fn test_angle_wrapping() {
        // Test wrap_unsigned
        assert_angle_eq(Angle::degrees(450.0).wrap_unsigned(), 90.0);
        assert_angle_eq(Angle::degrees(-90.0).wrap_unsigned(), 270.0);
        assert_angle_eq(Angle::degrees(720.0).wrap_unsigned(), 0.0);

        // Test wrap_signed
        assert_angle_eq(Angle::degrees(450.0).wrap_signed(), 90.0);
        assert_angle_eq(Angle::degrees(-270.0).wrap_signed(), 90.0);
        assert_angle_eq(Angle::degrees(270.0).wrap_signed(), -90.0);
    }

    #[test]
    fn test_angle_arithmetic() {
        let a1 = Angle::degrees(45.0);
        let a2 = Angle::degrees(45.0);

        assert_angle_eq(a1 + a2, 90.0);
        assert_angle_eq(a1 - a2, 0.0);
        assert_angle_eq(a1 * 2.0, 90.0);
        assert_angle_eq(a1 / 2.0, 22.5);

        let ratio = Angle::degrees(90.0) / Angle::degrees(45.0);
        assert!((ratio - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_angle_remainder() {
        let a1 = Angle::degrees(270.0);
        let a2 = Angle::degrees(360.0);
        assert_angle_eq(a1 % a2, 270.0);

        // Test positive remainder behavior
        let a3 = Angle::degrees(-90.0);
        let a4 = Angle::degrees(360.0);
        assert_angle_eq(a3 % a4, 270.0);
    }

    #[test]
    fn test_angle_comparison() {
        let a1 = Angle::degrees(45.0);
        let a2 = Angle::degrees(45.0);
        let a3 = Angle::degrees(90.0);

        assert_eq!(a1, a2);
        assert!(a1 < a3);
        assert!(a3 > a1);
    }

    #[test]
    fn test_angle_negation() {
        let angle = Angle::degrees(90.0);
        let negated = -angle;
        assert_angle_eq(negated, -90.0);
    }

    #[test]
    fn test_assignment_operators() {
        let mut angle = Angle::degrees(45.0);

        angle += Angle::degrees(45.0);
        assert_angle_eq(angle, 90.0);

        angle -= Angle::degrees(30.0);
        assert_angle_eq(angle, 60.0);

        angle *= 2.0;
        assert_angle_eq(angle, 120.0);

        angle /= 3.0;
        assert_angle_eq(angle, 40.0);

        angle %= Angle::degrees(30.0);
        assert_angle_eq(angle, 10.0);
    }

    #[test]
    fn test_zero_constant() {
        assert_eq!(Angle::default().as_degrees(), 0.0);
        assert_eq!(Angle::default().as_radians(), 0.0);
    }

    #[test]
    fn test_display_formatting() {
        let angle = Angle::degrees(45.0);
        assert_eq!(format!("{}", angle), "45°");

        let debug_str = format!("{:?}", angle);
        assert!(debug_str.contains("degrees"));
        assert!(debug_str.contains("radians"));
    }
}
