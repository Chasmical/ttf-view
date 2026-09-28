use crate::types::F2DOT14;

/// A simple [`F2DOT14`] 2×2 affine transformation
/// <span class="hidden">`[xx, yx; xy, yy]`</span>
/// <math><mo>[</mo><mtable>
///   <mtr><mtd><mi>xx</mi></mtd><mtd><mi>yx</mi></mtd></mtr>
///   <mtr><mtd><mi>xy</mi></mtd><mtd><mi>yy</mi></mtd></mtr>
/// </mtable><mo>]</mo></math>.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Affine2x2 {
    /// X-component of transformed X-basis vector.
    pub xx: F2DOT14,
    /// Y-component of transformed X-basis vector (aka. `scale01` in `glyf`).
    pub yx: F2DOT14, // aka. scale01
    /// X-component of transformed Y-basis vector (aka. `scale10` in `glyf`).
    pub xy: F2DOT14, // aka. scale10
    /// Y-component of transformed Y-basis vector.
    pub yy: F2DOT14,
}

const impl Default for Affine2x2 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Affine2x2 {
    /// The identity transform
    /// <span class="hidden">`[1, 0; 0, 1]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
    /// </mtable><mo>]</mo></math>.
    ///
    /// ```
    /// # use ttf_view::types::Affine2x2;
    /// assert_eq!(Affine2x2::IDENTITY.to_tuple_f32(), (1.0, 0.0, 0.0, 1.0));
    /// ```
    pub const IDENTITY: Self = Self::scale(F2DOT14::ONE);

    /// Creates an [`Affine2x2`] with
    /// <span class="hidden">`[xx, yx; xy, yy]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>xx</mi></mtd><mtd><mi>yx</mi></mtd></mtr>
    ///   <mtr><mtd><mi>xy</mi></mtd><mtd><mi>yy</mi></mtd></mtr>
    /// </mtable><mo>]</mo></math>.
    pub const fn new(xx: F2DOT14, yx: F2DOT14, xy: F2DOT14, yy: F2DOT14) -> Self {
        Self { xx, yx, xy, yy }
    }

    /// Creates an [`Affine2x2`] with
    /// <span class="hidden">`[scale, 0; 0, scale]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>scale</mi></mtd><mtd><mn>0</mn></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mi>scale</mi></mtd></mtr>
    /// </mtable><mo>]</mo></math>.
    pub const fn scale(scale: F2DOT14) -> Self {
        Self::new(scale, F2DOT14::ZERO, F2DOT14::ZERO, scale)
    }
    /// Creates an [`Affine2x2`] with
    /// <span class="hidden">`[x, 0; 0, y]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>x</mi></mtd><mtd><mn>0</mn></mtd></mtr>
    ///   <mtr><mtd><mn>0</mn></mtd><mtd><mi>y</mi></mtd></mtr>
    /// </mtable><mo>]</mo></math>.
    pub const fn scale_xy(x: F2DOT14, y: F2DOT14) -> Self {
        Self::new(x, F2DOT14::ZERO, F2DOT14::ZERO, y)
    }

    /// Creates an [`Affine2x2`] with
    /// <span class="hidden">`[cos(θ),-sin(θ);sin(θ),cos(θ)]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>cos(θ)</mi></mtd><mtd><mi>-sin(θ)</mi></mtd></mtr>
    ///   <mtr><mtd><mi>sin(θ)</mi></mtd><mtd><mi>cos(θ)</mi></mtd></mtr>
    /// </mtable><mo>]</mo></math>,
    /// rotating the image counter-clockwise by specified angle.
    pub fn rotation(radians: f32) -> Self {
        let (sin, cos) = radians.sin_cos();
        Self::new(
            F2DOT14::new(cos).unwrap(),
            F2DOT14::new(-sin).unwrap(),
            F2DOT14::new(sin).unwrap(),
            F2DOT14::new(cos).unwrap(),
        )
    }
    /// Creates an [`Affine2x2`] with
    /// <span class="hidden">`[cos(θ),-sin(θ); sin(θ),cos(θ)]`</span>
    /// <math><mo>[</mo><mtable>
    ///   <mtr><mtd><mi>cos(θ)</mi></mtd><mtd><mi>-sin(θ)</mi></mtd></mtr>
    ///   <mtr><mtd><mi>sin(θ)</mi></mtd><mtd><mi>cos(θ)</mi></mtd></mtr>
    /// </mtable><mo>]</mo></math>,
    /// rotating the image counter-clockwise by specified angle.
    pub fn rotation_degrees(degrees: f32) -> Self {
        Self::rotation(degrees.to_radians())
    }

    /// Applies this [`Affine2x2`] transformation to the point
    /// <span class="hidden">`[x, y]`</span>
    /// <math><mo>\[</mo><mi>x</mi><mo> </mo><mi>y</mi><mo>\]</mo></math>:
    ///
    /// <code class="hidden">\[x,y\]×\[xx,yx; xy,yy\]=\[xx\*x+xy\*y, yx\*x+yy\*y\]</code>
    /// <math>
    ///   <mrow><mo>\[</mo><mi>x</mi><mo> </mo><mi>y</mi><mo>\]</mo></mrow>
    ///   <mo>×</mo>
    ///   <mo>\[</mo><mtable>
    ///     <mtr><mtd><mi>xx</mi></mtd><mtd><mi>yx</mi></mtd></mtr>
    ///     <mtr><mtd><mi>xy</mi></mtd><mtd><mi>yy</mi></mtd></mtr>
    ///   </mtable><mo>\]</mo>
    ///   <mo>=</mo>
    ///   <mrow><mo>\[</mo>
    ///     <mi>xx</mi><mo>\*</mo><mi>x</mi><mo>+</mo><mi>xy</mi><mo>\*</mo><mi>y</mi>
    ///   <mo>, </mo>
    ///     <mi>yx</mi><mo>\*</mo><mi>x</mi><mo>+</mo><mi>yy</mi><mo>\*</mo><mi>y</mi>
    ///   <mo>\]</mo></mrow>
    /// </math>
    ///
    /// ```
    /// use ttf_view::types::Affine2x2;
    ///
    /// assert_eq!(Affine2x2::IDENTITY.transform_f32(10.0, -10.0), (10.0, -10.0));
    /// assert_eq!(Affine2x2::IDENTITY.transform_f32(-2583.2, 1842.2), (-2583.2, 1842.2));
    ///
    /// let rot90 = Affine2x2::rotation_degrees(90.0);
    /// assert_eq!(rot90.transform_f32(23.5, 9.8), (9.8, -23.5));
    /// ```
    pub const fn transform_f32(&self, x: f32, y: f32) -> (f32, f32) {
        (self.xx * x + self.xy * y, self.yx * x + self.yy * y)
    }

    /// Applies this [`Affine2x2`] transformation to the point
    /// <span class="hidden">`[x, y]`</span>
    /// <math><mo>\[</mo><mi>x</mi><mo> </mo><mi>y</mi><mo>\]</mo></math>.
    ///
    /// ```
    /// use ttf_view::types::Affine2x2;
    ///
    /// assert_eq!(Affine2x2::IDENTITY.transform_i16(100, -100), (100, -100));
    /// assert_eq!(Affine2x2::IDENTITY.transform_i16(-25832, 18422), (-25832, 18422));
    ///
    /// let rot90 = Affine2x2::rotation_degrees(90.0);
    /// assert_eq!(rot90.transform_i16(235, 98), (98, -235));
    /// ```
    pub const fn transform_i16(&self, x: i16, y: i16) -> (i16, i16) {
        let (x, y) = (x as i32, y as i32);

        // Multiply the coords by F2DOT14's numerators, and then divide them by the denominator.
        //
        // Both x, y, and numerators can be in range -32768..=32767.
        // The absolute maximum that can be reached is 2147483648 (= -32768*-32768+-32768*-32768).
        // The absolute minimum that can be reached is -2147418112 (= -32768*32767+-32768*32767).
        // The maximum is 1 above i32's max, so we need to use saturating_add here.

        let x_num = (self.xx.frac_num() as i32 * x).saturating_add(self.xy.frac_num() as i32 * y);
        let y_num = (self.yx.frac_num() as i32 * x).saturating_add(self.yy.frac_num() as i32 * y);

        // After dividing by DENOM, the range of values would be -131072..=131071,
        // which is outside i16's range, so we need to use saturating_cast here.
        const DENOM: i32 = F2DOT14::DENOM as i32;
        ((x_num / DENOM).saturating_cast(), (y_num / DENOM).saturating_cast())
    }

    /// Multiplies this [`Affine2x2`] by another, returning `None` if overflow occurs.
    pub const fn checked_mul(&self, other: Self) -> Option<Self> {
        const fn mul(a: F2DOT14, b: F2DOT14, c: F2DOT14, d: F2DOT14) -> Option<F2DOT14> {
            let u = a.frac_num() as i32 * b.frac_num() as i32;
            let v = c.frac_num() as i32 * d.frac_num() as i32;
            Some(F2DOT14::from_frac_num(u.checked_add(v)?.try_into().ok()?))
        }
        Some(Self::new(
            mul(self.xx, other.xx, self.yx, other.xy)?,
            mul(self.xx, other.yx, self.yx, other.yy)?,
            mul(self.xy, other.xx, self.yy, other.xy)?,
            mul(self.xy, other.yx, self.yy, other.yy)?,
        ))
    }
    /// Multiplies this [`Affine2x2`] by another, saturating at [`F2DOT14`]'s bounds.
    pub const fn saturating_mul(&self, other: Self) -> Self {
        const fn mul(a: F2DOT14, b: F2DOT14, c: F2DOT14, d: F2DOT14) -> F2DOT14 {
            let u = a.frac_num() as i32 * b.frac_num() as i32;
            let v = c.frac_num() as i32 * d.frac_num() as i32;
            F2DOT14::from_frac_num(u.saturating_add(v).saturating_cast())
        }
        Self::new(
            mul(self.xx, other.xx, self.yx, other.xy),
            mul(self.xx, other.yx, self.yx, other.yy),
            mul(self.xy, other.xx, self.yy, other.xy),
            mul(self.xy, other.yx, self.yy, other.yy),
        )
    }

    /// Creates an [`Affine2x2`] from big-endian bytes.
    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        let (&[xx, yx, xy, yy], []) = bytes.as_chunks::<2>() else { panic!() };
        Self::new(
            F2DOT14::from_be_bytes(xx),
            F2DOT14::from_be_bytes(yx),
            F2DOT14::from_be_bytes(xy),
            F2DOT14::from_be_bytes(yy),
        )
    }
    /// Returns this [`Affine2x2`] as big-endian bytes.
    pub const fn to_be_bytes(self) -> [u8; 8] {
        let buf = [
            self.xx.to_be_bytes(),
            self.yx.to_be_bytes(),
            self.xy.to_be_bytes(),
            self.yy.to_be_bytes(),
        ];
        *buf.as_flattened().as_array().unwrap()
    }

    /// Returns this [`Affine2x2`]'s `(xx, yx, xy, yy)` as a tuple.
    pub const fn to_tuple(&self) -> (F2DOT14, F2DOT14, F2DOT14, F2DOT14) {
        (self.xx, self.yx, self.xy, self.yy)
    }
    /// Returns this [`Affine2x2`]'s `(xx, yx, xy, yy)` as a tuple of [`f32`]s.
    pub const fn to_tuple_f32(&self) -> (f32, f32, f32, f32) {
        (self.xx.get(), self.yx.get(), self.xy.get(), self.yy.get())
    }
}

/// Multiplies this [`Affine2x2`] by another, saturating at [`F2DOT14`]'s bounds.
const impl std::ops::Mul for Affine2x2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl std::fmt::Debug for Affine2x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Self { xx, yx, xy, yy } = *self;
        write!(f, "[{:?}, {:?}; {:?}, {:?}]", xx, yx, xy, yy)
    }
}
