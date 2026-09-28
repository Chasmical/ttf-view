use crate::types::Fixed;

/// A simple [`Fixed`] 2×3 affine transformation
/// <span class="hidden">`[xx, yx; xy, yy; dx, dy]`</span>
/// <math><mo>[</mo><mtable>
///   <mtr><mtd><mi>xx</mi></mtd><mtd><mi>yx</mi></mtd></mtr>
///   <mtr><mtd><mi>xy</mi></mtd><mtd><mi>yy</mi></mtd></mtr>
///   <mtr><mtd><mi>dx</mi></mtd><mtd><mi>dy</mi></mtd></mtr>
/// </mtable><mo>]</mo></math>.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Affine2x3 {
    /// X-component of transformed X-basis vector.
    pub xx: Fixed,
    /// Y-component of transformed X-basis vector.
    pub yx: Fixed,
    /// X-component of transformed Y-basis vector.
    pub xy: Fixed,
    /// Y-component of transformed Y-basis vector.
    pub yy: Fixed,
    /// Translation in X direction.
    pub dx: Fixed,
    /// Translation in Y direction.
    pub dy: Fixed,
}

const impl Default for Affine2x3 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Affine2x3 {
    /// The identity transform
    /// <span class="hidden">`[1, 0; 0, 1; 0, 0]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
    /// </mtable><mo>]</mo></math>.
    ///
    /// ```
    /// # use ttf_view::types::Affine2x3;
    /// assert_eq!(Affine2x3::IDENTITY.to_tuple_f64(), (1.0, 0.0, 0.0, 1.0, 0.0, 0.0));
    /// ```
    pub const IDENTITY: Self = Self::scale(Fixed::ONE);

    /// Creates an [`Affine2x3`] with
    /// <span class="hidden">`[xx, yx; xy, yy; dx, dy]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>xx</mi></mtd><mtd><mi>yx</mi></mtd></mtr>
    ///   <mtr><mtd><mi>xy</mi></mtd><mtd><mi>yy</mi></mtd></mtr>
    ///   <mtr><mtd><mi>dx</mi></mtd><mtd><mi>dy</mi></mtd></mtr>
    /// </mtable><mo>]</mo></math>.
    pub const fn new(xx: Fixed, yx: Fixed, xy: Fixed, yy: Fixed, dx: Fixed, dy: Fixed) -> Self {
        Self { xx, yx, xy, yy, dx, dy }
    }
    /// Creates an [`Affine2x3`] with
    /// <span class="hidden">`[xx, yx; xy, yy; 0, 0]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>xx</mi></mtd><mtd><mi>yx</mi></mtd></mtr>
    ///   <mtr><mtd><mi>xy</mi></mtd><mtd><mi>yy</mi></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
    /// </mtable><mo>]</mo></math>.
    pub const fn transform(xx: Fixed, yx: Fixed, xy: Fixed, yy: Fixed) -> Self {
        Self::new(xx, yx, xy, yy, Fixed::ZERO, Fixed::ZERO)
    }
    /// Creates an [`Affine2x3`] with
    /// <span class="hidden">`[1, 0; 0, 1; dx, dy]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
    ///   <mtr><mtd><mi>dx</mi></mtd><mtd><mi>dy</mi></mtd></mtr>
    /// </mtable><mo>]</mo></math>.
    pub const fn translate(dx: Fixed, dy: Fixed) -> Self {
        Self::new(Fixed::ONE, Fixed::ZERO, Fixed::ZERO, Fixed::ONE, dx, dy)
    }

    /// Creates an [`Affine2x3`] with
    /// <span class="hidden">`[scale, 0; 0, scale; 0, 0]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>scale</mi></mtd><mtd><mn>0</mn></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mi>scale</mi></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
    /// </mtable><mo>]</mo></math>.
    pub const fn scale(scale: Fixed) -> Self {
        Self::transform(scale, Fixed::ZERO, Fixed::ZERO, scale)
    }
    /// Creates an [`Affine2x3`] with
    /// <span class="hidden">`[x, 0; 0, y; 0, 0]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>x</mi></mtd><mtd><mn>0</mn></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mi>y</mi></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
    /// </mtable><mo>]</mo></math>.
    pub const fn scale_xy(x: Fixed, y: Fixed) -> Self {
        Self::transform(x, Fixed::ZERO, Fixed::ZERO, y)
    }

    /// Creates an [`Affine2x3`] with
    /// <span class="hidden">`[cos(θ),-sin(θ);sin(θ),cos(θ); 0, 0]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>cos(θ)</mi></mtd><mtd><mi>-sin(θ)</mi></mtd></mtr>
    ///   <mtr><mtd><mi>sin(θ)</mi></mtd><mtd><mi>cos(θ)</mi></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
    /// </mtable><mo>]</mo></math>,
    /// rotating the image counter-clockwise by specified angle.
    pub fn rotation(radians: f64) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self::transform(
            Fixed::new(cos).unwrap(),
            Fixed::new(-sin).unwrap(),
            Fixed::new(sin).unwrap(),
            Fixed::new(cos).unwrap(),
        )
    }
    /// Creates an [`Affine2x3`] with
    /// <span class="hidden">`[cos(θ),-sin(θ);sin(θ),cos(θ); 0, 0]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>cos(θ)</mi></mtd><mtd><mi>-sin(θ)</mi></mtd></mtr>
    ///   <mtr><mtd><mi>sin(θ)</mi></mtd><mtd><mi>cos(θ)</mi></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
    /// </mtable><mo>]</mo></math>,
    /// rotating the image counter-clockwise by specified angle.
    pub fn rotation_degrees(degrees: f64) -> Self {
        Self::rotation(degrees.to_radians())
    }

    // TODO: docs
    pub const fn transform_f64(&self, x: f64, y: f64) -> (f64, f64) {
        (self.xx * x + self.xy * y + self.dx.get(), self.yx * x + self.yy * y + self.dy.get())
    }

    pub const fn transform_i32(&self, x: i32, y: i32) -> (i32, i32) {
        let (x, y) = (x as i64, y as i64);

        /// See implementation notes in [`super::Affine2x2::transform_i16`].
        struct __;

        let x_num = (self.xx.frac_num() as i64 * x)
            .saturating_add(self.xy.frac_num() as i64 * y)
            .saturating_add(self.dx.frac_num() as i64);
        let y_num = (self.yx.frac_num() as i64 * x)
            .saturating_add(self.yy.frac_num() as i64 * y)
            .saturating_add(self.dy.frac_num() as i64);

        const DENOM: i64 = Fixed::DENOM as i64;
        ((x_num / DENOM).saturating_cast(), (y_num / DENOM).saturating_cast())
    }

    /// Creates an [`Affine2x3`] from big-endian bytes.
    pub const fn from_be_bytes(bytes: [u8; 24]) -> Self {
        let (&[xx, yx, xy, yy, dx, dy], []) = bytes.as_chunks::<4>() else { panic!() };
        Self::new(
            Fixed::from_be_bytes(xx),
            Fixed::from_be_bytes(yx),
            Fixed::from_be_bytes(xy),
            Fixed::from_be_bytes(yy),
            Fixed::from_be_bytes(dx),
            Fixed::from_be_bytes(dy),
        )
    }
    /// Returns this [`Affine2x3`] as big-endian bytes.
    pub const fn to_be_bytes(self) -> [u8; 24] {
        let buf = [
            self.xx.to_be_bytes(),
            self.yx.to_be_bytes(),
            self.xy.to_be_bytes(),
            self.yy.to_be_bytes(),
            self.dx.to_be_bytes(),
            self.dy.to_be_bytes(),
        ];
        *buf.as_flattened().as_array().unwrap()
    }

    /// Returns this [`Affine2x3`]'s `(xx, yx, xy, yy, dx, dy)` as a tuple.
    pub const fn to_tuple(&self) -> (Fixed, Fixed, Fixed, Fixed, Fixed, Fixed) {
        (self.xx, self.yx, self.xy, self.yy, self.dx, self.dy)
    }
    /// Returns this [`Affine2x3`]'s `(xx, yx, xy, yy, dx, dy)` as a tuple of [`f64`]s.
    pub const fn to_tuple_f64(&self) -> (f64, f64, f64, f64, f64, f64) {
        (self.xx.get(), self.yx.get(), self.xy.get(), self.yy.get(), self.dx.get(), self.dy.get())
    }
}

// TODO: impl std::ops::Mul for Affine2x3

impl std::fmt::Debug for Affine2x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { xx, yx, xy, yy, dx, dy } = *self;
        write!(f, "[{:?}, {:?}; {:?}, {:?}; {:?}, {:?}]", xx, yx, xy, yy, dx, dy)
    }
}
