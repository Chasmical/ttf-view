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

    /// Applies this [`Affine2x3`] transformation to the point
    /// <span class="hidden">`[x, y, 1]`</span>
    /// <math><mo>\[</mo><mi>x</mi><mo> </mo><mi>y</mi><mo> </mo><mn>1</mn><mo>\]</mo></math>:
    ///
    /// <code class="hidden">\[x,y,1\]×\[xx,yx; xy,yy; dx,dy\]=\[xx\*x+xy\*y+dx, yx\*x+yy\*y+dy\]</code>
    /// <math>
    ///   <mrow><mo>\[</mo><mi>x</mi><mo> </mo><mi>y</mi><mo> </mo><mn>1</mn><mo>\]</mo></mrow>
    ///   <mo>×</mo>
    ///   <mo>\[</mo><mtable>
    ///     <mtr><mtd><mi>xx</mi></mtd><mtd><mi>yx</mi></mtd></mtr>
    ///     <mtr><mtd><mi>xy</mi></mtd><mtd><mi>yy</mi></mtd></mtr>
    ///     <mtr><mtd><mi>dx</mi></mtd><mtd><mi>dy</mi></mtd></mtr>
    ///   </mtable><mo>\]</mo>
    ///   <mo>=</mo>
    ///   <mrow><mo>\[</mo>
    ///     <mi>xx</mi><mo>\*</mo><mi>x</mi><mo>+</mo><mi>xy</mi><mo>\*</mo><mi>y</mi>
    ///     <mo>+</mo><mi>dx</mi>
    ///   <mo>, </mo>
    ///     <mi>yx</mi><mo>\*</mo><mi>x</mi><mo>+</mo><mi>yy</mi><mo>\*</mo><mi>y</mi>
    ///     <mo>+</mo><mi>dy</mi>
    ///   <mo>\]</mo></mrow>
    /// </math>
    ///
    /// # Examples
    ///
    /// ```
    /// use ttf_view::types::Affine2x3;
    ///
    /// assert_eq!(Affine2x3::IDENTITY.map_f64(10.0, -10.0), (10.0, -10.0));
    /// assert_eq!(Affine2x3::IDENTITY.map_f64(-2583.2, 1842.2), (-2583.2, 1842.2));
    ///
    /// let rot90 = Affine2x3::rotation_degrees(90.0);
    /// assert_eq!(rot90.map_f64(23.5, 9.8), (9.8, -23.5));
    /// ```
    pub const fn map(&self, x: Fixed, y: Fixed) -> (Fixed, Fixed) {
        let new_x = Fixed::saturating_maddp(self.xx, x, self.xy, y, self.dx);
        let new_y = Fixed::saturating_maddp(self.yx, x, self.yy, y, self.dy);
        (new_x, new_y)
    }

    /// Applies this [`Affine2x3`] transformation to the point
    /// <span class="hidden">`[x, y, 1]`</span>
    /// <math><mo>\[</mo><mi>x</mi><mo> </mo><mi>y</mi><mo> </mo><mn>1</mn><mo>\]</mo></math>.
    pub const fn map_f64(&self, x: f64, y: f64) -> (f64, f64) {
        (self.xx * x + self.xy * y + self.dx.get(), self.yx * x + self.yy * y + self.dy.get())
    }

    //  self   ×  other
    // [A B 0]   [a b 0]   [Aa+Bc   Ab+Bd   0]   [xx*xx+yx*xy    xx*yx+yx*yy    0]
    // [C D 0] × [c d 0] = [Ca+Dc   Cb+Dd   0] = [xy*xx+yy*xy    xy*yx+yy*yy    0]
    // [X Y 1]   [x y 1]   [Xa+Yc+x Xb+Yd+y 1]   [dx*xx+dy*xy+dx dx*yx+dy*yy+dy 1]

    /// Multiplies this [`Affine2x3`] by another, wrapping and truncating at [`Fixed`]'s bounds.
    pub const fn wrapping_mul(self, other: Self) -> Self {
        Self::new(
            Fixed::wrapping_maddp(self.xx, other.xx, self.yx, other.xy, Fixed::ZERO),
            Fixed::wrapping_maddp(self.xx, other.yx, self.yx, other.yy, Fixed::ZERO),
            Fixed::wrapping_maddp(self.xy, other.xx, self.yy, other.xy, Fixed::ZERO),
            Fixed::wrapping_maddp(self.xy, other.yx, self.yy, other.yy, Fixed::ZERO),
            Fixed::wrapping_maddp(self.dx, other.xx, self.dy, other.xy, self.dx),
            Fixed::wrapping_maddp(self.dx, other.yx, self.dy, other.yy, self.dy),
        )
    }
    /// Multiplies this [`Affine2x3`] by another, saturating at [`Fixed`]'s bounds.
    pub const fn saturating_mul(self, other: Self) -> Self {
        Self::new(
            Fixed::saturating_maddp(self.xx, other.xx, self.yx, other.xy, Fixed::ZERO),
            Fixed::saturating_maddp(self.xx, other.yx, self.yx, other.yy, Fixed::ZERO),
            Fixed::saturating_maddp(self.xy, other.xx, self.yy, other.xy, Fixed::ZERO),
            Fixed::saturating_maddp(self.xy, other.yx, self.yy, other.yy, Fixed::ZERO),
            Fixed::saturating_maddp(self.dx, other.xx, self.dy, other.xy, self.dx),
            Fixed::saturating_maddp(self.dx, other.yx, self.dy, other.yy, self.dy),
        )
    }
    /// Multiplies this [`Affine2x3`] by another, returning `None` if overflow occurs.
    pub const fn checked_mul(self, other: Self) -> Option<Self> {
        Some(Self::new(
            Fixed::checked_maddp(self.xx, other.xx, self.yx, other.xy, Fixed::ZERO)?,
            Fixed::checked_maddp(self.xx, other.yx, self.yx, other.yy, Fixed::ZERO)?,
            Fixed::checked_maddp(self.xy, other.xx, self.yy, other.xy, Fixed::ZERO)?,
            Fixed::checked_maddp(self.xy, other.yx, self.yy, other.yy, Fixed::ZERO)?,
            Fixed::checked_maddp(self.dx, other.xx, self.dy, other.xy, self.dx)?,
            Fixed::checked_maddp(self.dx, other.yx, self.dy, other.yy, self.dy)?,
        ))
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
    pub const fn to_tuple(self) -> (Fixed, Fixed, Fixed, Fixed, Fixed, Fixed) {
        (self.xx, self.yx, self.xy, self.yy, self.dx, self.dy)
    }
    /// Returns this [`Affine2x3`]'s `(xx, yx, xy, yy, dx, dy)` as a tuple of [`f64`]s.
    pub const fn to_tuple_f64(self) -> (f64, f64, f64, f64, f64, f64) {
        (self.xx.get(), self.yx.get(), self.xy.get(), self.yy.get(), self.dx.get(), self.dy.get())
    }

    /// Creates an [`Affine2x3`] from a `[xx, yx, xy, yy, dx, dy]` array.
    pub const fn from_array([xx, yx, xy, yy, dx, dy]: [Fixed; 6]) -> Self {
        Self { xx, yx, xy, yy, dx, dy }
    }
    /// Returns this [`Affine2x3`]'s `[xx, yx, xy, yy, dx, dy]` as an array.
    pub const fn to_array(self) -> [Fixed; 6] {
        [self.xx, self.yx, self.xy, self.yy, self.dx, self.dy]
    }
}

/// Performs multiplication `*` (panics on overflow in debug configuration).
const impl std::ops::Mul for Affine2x3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        #[cfg(debug_assertions)]
        return self.checked_mul(rhs).expect("attempt to multiply with overflow");
        #[cfg(not(debug_assertions))]
        return self.wrapping_mul(rhs);
    }
}

impl std::fmt::Debug for Affine2x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { xx, yx, xy, yy, dx, dy } = *self;
        write!(f, "[{:?}, {:?}; {:?}, {:?}; {:?}, {:?}]", xx, yx, xy, yy, dx, dy)
    }
}
