#![crate_name="doc"]

pub enum FuncType {
    Idem,
    Sum,
    Prod,
    Pow,
    ExpA,
    LogA,
    ExpE,
    LogE,
    TrigSin,
    TrigCos,
    TrigTan,
}

/// Represents a function f that can be evaluated at x value
pub trait FuncEval {
    /// Evaluates a function f at x
    fn eval(&self, x: f64) -> f64;
}

pub struct Func {
    f_type: FuncType,
}

impl FuncEval for Func {
    fn eval(&self, x: f64) -> f64 {
        0f64
    }
}

/// Represents an f identity function, 
/// given x a real number then f(x) = x
pub struct FuncIdem {
    f_type: FuncType,
}

impl FuncIdem {
    fn new() -> Self {
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

struct FuncSum<T>
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

struct FuncProd<T>
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

struct FuncPow<T>
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
        self.f.eval(x)
    }
}

struct FuncExpA<T>
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
        f64::powf(self.a, x)
    }
}
