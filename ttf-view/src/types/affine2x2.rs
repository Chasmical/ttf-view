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
    /// # Examples
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
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::Affine2x2;
    ///
    /// assert_eq!(Affine2x2::IDENTITY.map_f32(10.0, -10.0), (10.0, -10.0));
    /// assert_eq!(Affine2x2::IDENTITY.map_f32(-2583.2, 1842.2), (-2583.2, 1842.2));
    ///
    /// let rot90 = Affine2x2::rotation_degrees(90.0);
    /// assert_eq!(rot90.map_f32(23.5, 9.8), (9.8, -23.5));
    /// ```
    pub const fn map(&self, x: F2DOT14, y: F2DOT14) -> (F2DOT14, F2DOT14) {
        let new_x = F2DOT14::saturating_maddp(self.xx, x, self.xy, y, F2DOT14::ZERO);
        let new_y = F2DOT14::saturating_maddp(self.yx, x, self.yy, y, F2DOT14::ZERO);
        (new_x, new_y)
    }

    /// Applies this [`Affine2x2`] transformation to the point
    /// <span class="hidden">`[x, y]`</span>
    /// <math><mo>\[</mo><mi>x</mi><mo> </mo><mi>y</mi><mo>\]</mo></math>.
    pub const fn map_i16(&self, x: i16, y: i16) -> (i16, i16) {
        // Cast X and Y to F2DOT14 and back. The results will be the same, since F2DOT14 is
        // essentially just a wrapper over i16, and there's only scaling and no translation.
        let (x, y) = self.map(F2DOT14::from_frac_num(x), F2DOT14::from_frac_num(y));
        (x.frac_num(), y.frac_num())
    }
    /// Applies this [`Affine2x2`] transformation to the point
    /// <span class="hidden">`[x, y]`</span>
    /// <math><mo>\[</mo><mi>x</mi><mo> </mo><mi>y</mi><mo>\]</mo></math>.
    pub const fn map_f32(&self, x: f32, y: f32) -> (f32, f32) {
        (self.xx * x + self.xy * y, self.yx * x + self.yy * y)
    }

    // self  × other
    // [A B] × [a b] = [Aa+Bc Ab+Bd] = [xx*xx+yx*xy xx*yx+yx*yy]
    // [C D]   [c d]   [Ca+Dc Cb+Dd]   [xy*xx+yy*xy xy*yx+yy*yy]

    /// Multiplies this [`Affine2x2`] by another, wrapping and truncating at [`F2DOT14`]'s bounds.
    pub const fn wrapping_mul(&self, rhs: Self) -> Self {
        Self::new(
            F2DOT14::wrapping_maddp(self.xx, rhs.xx, self.yx, rhs.xy, F2DOT14::ZERO),
            F2DOT14::wrapping_maddp(self.xx, rhs.yx, self.yx, rhs.yy, F2DOT14::ZERO),
            F2DOT14::wrapping_maddp(self.xy, rhs.xx, self.yy, rhs.xy, F2DOT14::ZERO),
            F2DOT14::wrapping_maddp(self.xy, rhs.yx, self.yy, rhs.yy, F2DOT14::ZERO),
        )
    }
    /// Multiplies this [`Affine2x2`] by another, saturating at [`F2DOT14`]'s bounds.
    pub const fn saturating_mul(&self, rhs: Self) -> Self {
        Self::new(
            F2DOT14::saturating_maddp(self.xx, rhs.xx, self.yx, rhs.xy, F2DOT14::ZERO),
            F2DOT14::saturating_maddp(self.xx, rhs.yx, self.yx, rhs.yy, F2DOT14::ZERO),
            F2DOT14::saturating_maddp(self.xy, rhs.xx, self.yy, rhs.xy, F2DOT14::ZERO),
            F2DOT14::saturating_maddp(self.xy, rhs.yx, self.yy, rhs.yy, F2DOT14::ZERO),
        )
    }
    /// Multiplies this [`Affine2x2`] by another, returning `None` if overflow occurs.
    pub const fn checked_mul(&self, rhs: Self) -> Option<Self> {
        Some(Self::new(
            F2DOT14::checked_maddp(self.xx, rhs.xx, self.yx, rhs.xy, F2DOT14::ZERO)?,
            F2DOT14::checked_maddp(self.xx, rhs.yx, self.yx, rhs.yy, F2DOT14::ZERO)?,
            F2DOT14::checked_maddp(self.xy, rhs.xx, self.yy, rhs.xy, F2DOT14::ZERO)?,
            F2DOT14::checked_maddp(self.xy, rhs.yx, self.yy, rhs.yy, F2DOT14::ZERO)?,
        ))
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

    /// Creates an [`Affine2x2`] from a `[xx, yx, xy, yy]` array.
    pub const fn from_array([xx, yx, xy, yy]: [F2DOT14; 4]) -> Self {
        Self { xx, yx, xy, yy }
    }
    /// Returns this [`Affine2x2`]'s `[xx, yx, xy, yy]` as an array.
    pub const fn to_array(self) -> [F2DOT14; 4] {
        [self.xx, self.yx, self.xy, self.yy]
    }
}

/// Performs multiplication `*` (panics on overflow in debug configuration).
const impl std::ops::Mul for Affine2x2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        return self.checked_mul(rhs).expect("attempt to multiply with overflow");
        #[cfg(not(debug_assertions))]
        return self.wrapping_mul(rhs);
    }
}

impl std::fmt::Debug for Affine2x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Self { xx, yx, xy, yy } = *self;
        write!(f, "[{:?}, {:?}; {:?}, {:?}]", xx, yx, xy, yy)
    }
}
