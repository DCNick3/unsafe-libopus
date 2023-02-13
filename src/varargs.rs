//! A bit more type-safe variadic function APIs. Not really ABI-compatible with C, just a compat layer.

#[derive(Copy, Clone, Debug)]
pub enum VarArg {
    I32(i32),
    I32Out(*mut i32),
    U32Out(*mut u32),
    CustomModeOut(*mut *const crate::OpusCustomMode),
    OpusDecoderOut(*mut *mut crate::OpusDecoder),
}

pub trait FromVarArg {
    fn from_vararg(arg: VarArg) -> Self;
}

pub trait IntoVarArg {
    fn into_vararg(self) -> VarArg;
}

macro_rules! impl_from_vararg {
    ($t:ty, $v:ident) => {
        impl FromVarArg for $t {
            fn from_vararg(arg: VarArg) -> Self {
                match arg {
                    VarArg::$v(v) => v,
                    _ => panic!("invalid vararg type"),
                }
            }
        }
    };
}
macro_rules! impl_into_vararg {
    ($t:ty, $v:ident) => {
        impl IntoVarArg for $t {
            fn into_vararg(self) -> VarArg {
                VarArg::$v(self)
            }
        }
    };
}
macro_rules! impl_vararg {
    ($t:ty, $v:ident) => {
        impl_from_vararg!($t, $v);
        impl_into_vararg!($t, $v);
    };
}

impl_vararg!(i32, I32);
impl_vararg!(*mut i32, I32Out);
impl_vararg!(*mut u32, U32Out);
impl_vararg!(*mut *const crate::OpusCustomMode, CustomModeOut);
impl_vararg!(*mut *mut crate::OpusDecoder, OpusDecoderOut);

pub struct VarArgs(Vec<VarArg>);

impl VarArgs {
    pub fn new(mut varargs: Vec<VarArg>) -> Self {
        // reverse them, because we pop them off the end, but want to have order "from left to right"
        varargs.reverse();
        Self(varargs)
    }

    pub fn arg<T: FromVarArg>(&mut self) -> T {
        T::from_vararg(
            self.0
                .pop()
                .expect("Attempt to pop from empty varargs (are the function arguments correct?)"),
        )
    }
}

#[macro_export]
macro_rules! varargs {
    ($($arg:expr),*) => {
        VarArgs::new(vec![$(IntoVarArg::into_vararg($arg)),*])
    };
}
