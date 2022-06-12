#![crate_name = "doc"]

pub enum FuncType {
    Abstract,
    Const,
    Idem,
    Sum,
    Prod,
    Pow,
    ExpA,
    ExpE,
    LogA,
    LogE,
    TrigSin,
    TrigCos,
    TrigTan,
    TrigAsin,
    TrigAcos,
    TrigAtan,
    Sqrt,
    Qbrt,
    Nthrt,
}

/// Represents a function f that can be evaluated at x value
pub trait FuncEval {
    /// Evaluates a function f at x
    fn eval(&self, x: f64) -> f64;
}

pub struct Func {
    f_type: FuncType,
}

impl Func {
    pub fn new() -> Self {
        Self {f_type: FuncType::Abstract}
    }
}

impl FuncEval for Func {
    fn eval(&self, _: f64) -> f64 {
        0f64
    }
}

/// Represents a constant function
/// Given x a real number C(x) = c
pub struct FuncConst {
    c: f64,
}

impl FuncConst {
    pub fn new(c: f64) -> Self {
        Self { c }
    }
}

impl FuncEval for FuncConst {
    fn eval(&self, x: f64) -> f64 {
        self.c
    }
}

/// Represents an f identity function,
/// given x a real number then f(x) = x
pub struct FuncIdem {
    f_type: FuncType,
}

impl FuncIdem {
    pub fn new() -> Self {
        Self {
            f_type: FuncType::Idem,
        }
    }
}

impl FuncEval for FuncIdem {
    fn eval(&self, x: f64) -> f64 {
        x
    }
}

/// Represents a sum function s of two function f and g
/// Given x a real number s(x) = f(x) + g(x)
pub struct FuncSum<T>
where
    T: FuncEval,
{
    f: T,
    g: T,
}

impl<T> FuncSum<T>
where
    T: FuncEval,
{
    pub fn new(f: T, g: T) -> Self {
        Self { f, g }
    }
}

impl<T> FuncEval for FuncSum<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x) + self.g.eval(x)
    }
}

/// Represents a product function p of two functions f and g
/// Given x a real number then p(x) = f(x) * g(x)
pub struct FuncProd<T>
where
    T: FuncEval,
{
    f: T,
    g: T,
}

impl<T> FuncProd<T>
where
    T: FuncEval,
{
    pub fn new(f: T, g: T) -> Self {
        Self { f, g }
    }
}

impl<T> FuncEval for FuncProd<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x) * self.g.eval(x)
    }
}

/// Represents a power function p of a function f and constant n
/// Given x a real number p(x) = f(x) ^ n
pub struct FuncPow<T>
where
    T: FuncEval,
{
    f: T,
    n: f64,
}

impl<T> FuncPow<T>
where
    T: FuncEval,
{
    pub fn new(f: T, n: f64) -> Self {
        Self { f, n }
    }
}

impl<T> FuncEval for FuncPow<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).powf(self.n)
    }
}

/// Represents a exponetial function exp of a constant a and a function f
/// Given x a real number then exp(x) = a ^ f(x)
pub struct FuncExpA<T>
where
    T: FuncEval,
{
    a: f64,
    f: T,
}

impl<T> FuncExpA<T>
where
    T: FuncEval,
{
    pub fn new(a: f64, f: T) -> Self {
        Self { a, f }
    }
}

impl<T> FuncEval for FuncExpA<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.a.powf(self.f.eval(x))
    }
}

/// Represents an exponential function of Euler constant e and a fuction f
/// Given x a real number exp(x) = e ^ f(x)
pub struct FuncExpE<T>
where
    T: FuncEval,
{
    f: T,
}

impl<T> FuncExpE<T>
where
    T: FuncEval,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> FuncEval for FuncExpE<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        std::f64::consts::E.powf(x)
    }
}

/// Represents log base b function of f function
/// Given x a positive real number then lg_b(x) = log_b(f(x))
pub struct FuncLogA<T>
where
    T: FuncEval,
{
    b: f64,
    f: T,
}

impl<T> FuncLogA<T>
where
    T: FuncEval,
{
    pub fn new(b: f64, f: T) -> Self {
        Self { b, f }
    }
}

impl<T> FuncEval for FuncLogA<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).log(self.b)
    }
}

/// Represents neperian logarithm (natural log, log base e where e is Eulers constant) ln of function f
/// Given x a positive real number ln(x) = log_e(f(x))
pub struct FuncLogE<T>
where
    T: FuncEval,
{
    f: T,
}

impl<T> FuncLogE<T>
where
    T: FuncEval,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> FuncEval for FuncLogE<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).ln()
    }
}

/// Represents a sine function of a function f
/// Given x a real number (representing an angle in radians) Sin(x) = sin(f(x))
pub struct FuncSin<T>
where
    T: FuncEval,
{
    f: T,
}

impl<T> FuncSin<T>
where
    T: FuncEval,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> FuncEval for FuncSin<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).sin()
    }
}

/// Represents a cosine function of a f function
/// Given x a real number (representing an angle in radians) Cos(x) = cos(f(x))
pub struct FuncCos<T>
where
    T: FuncEval,
{
    f: T,
}

impl<T> FuncCos<T>
where
    T: FuncEval,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> FuncEval for FuncCos<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).cos()
    }
}

/// Represents a tangent function of a function f
/// Given x a real number (different from k*pi + pi/2 where k is an integer)
pub struct FuncTan<T>
where
    T: FuncEval,
{
    f: T,
}

impl<T> FuncTan<T>
where
    T: FuncEval,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> FuncEval for FuncTan<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).tan()
    }
}

/// Represents a arc sine function of a f function
/// Given x a real number (-1 <= x <= 1) then Asin(x) = asin(f(x))
pub struct FuncAsin<T>
where
    T: FuncEval,
{
    f: T,
}

impl<T> FuncAsin<T>
where
    T: FuncEval,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> FuncEval for FuncAsin<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).asin()
    }
}

/// Represents a function arc cosine for a function f
/// Given x a real number (-1 <= x <= 1) then Acos(x) = acos(f(x))
pub struct FuncAcos<T>
where
    T: FuncEval,
{
    f: T,
}

impl<T> FuncAcos<T>
where
    T: FuncEval,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

/// Represents a arc tan function of a f function
/// Given x a real number then Atan(x) = atan(f(x))
pub struct FuncAtan<T>
where
    T: FuncEval,
{
    f: T,
}

impl<T> FuncAtan<T>
where
    T: FuncEval,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> FuncEval for FuncAtan<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).atan()
    }
}

/// Represents a square root function of a f function
/// Given x a real number then Sqrt(x) = sqrt(f(x))
pub struct FuncSqrt<T> {
    f: T,
}

impl<T> FuncSqrt<T>
where
    T: FuncEval,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> FuncEval for FuncSqrt<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).sqrt()
    }
}

/// Represents a cubic root of a function f
/// Given x a real number Qbrt(x) = qbrt(f(x))
pub struct FuncQbrt<T>
where
    T: FuncEval,
{
    f: T,
}

impl<T> FuncQbrt<T>
where
    T: FuncEval,
{
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T> FuncEval for FuncQbrt<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).cbrt()
    }
}

/// Represents a nth root of a function f
/// Given x a real number and n != 0 then Nroot(x) = (f(x))^(1/n)
pub struct FuncNthrt<T>
where
    T: FuncEval,
{
    n: f64,
    f: T,
}

impl<T> FuncNthrt<T>
where
    T: FuncEval,
{
    pub fn new(n: f64, f: T) -> Self {
        assert!(n != 0f64);
        Self { n, f }
    }
}

impl<T> FuncEval for FuncNthrt<T>
where
    T: FuncEval,
{
    fn eval(&self, x: f64) -> f64 {
        self.f.eval(x).powf(1f64 / self.n)
    }
}
