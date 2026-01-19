pub mod cachegrind;
pub mod callgrind;
pub mod dhat;
pub mod drd;
pub mod helgrind;
pub mod memcheck;
pub mod valgrind;

macro_rules! client_request {
    (^ $($arg:expr),*) =>  {{
        #[cfg(not(feature = "opt-out"))]
        unsafe { $crate::bindings::valgrind_client_request_expr($($arg as usize),*) }

        #[cfg(feature = "opt-out")]
        ($($arg as usize),*).0
    }};
    ($request:path, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        $crate::requests::assert_defined!($request);

        client_request!(^ 0, $request, $arg1, $arg2, $arg3, $arg4, $arg5)
    }};
    ($request:path, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        client_request!($request, $arg1, $arg2, $arg3, $arg4, 0)
    };
    ($request:path, $arg1:expr, $arg2:expr, $arg3:expr) => {
        client_request!($request, $arg1, $arg2, $arg3, 0, 0)
    };
    ($request:path, $arg1:expr, $arg2:expr) => {
        client_request!($request, $arg1, $arg2, 0, 0, 0)
    };
    ($request:path, $arg1:expr) => {
        client_request!($request, $arg1, 0, 0, 0, 0)
    };
    ($request:path) => {
        client_request!($request, 0, 0, 0, 0, 0)
    };
}

// Asserts that a client request is supported by the Valgrind version this crate was compiled against.
// This assertion is expected to be optimized out by the compiler for supported requests.
macro_rules! assert_defined {
    ($request:path) => {
        #[cfg(not(feature = "mismatch-ignore"))]
        {
            const REQUIREMENT: u32 = $request.required_version();
            const SYS_VERSION: u32 = crate::VALGRIND_VERSION.0 * 100 + crate::VALGRIND_VERSION.1;

            assert!(
                REQUIREMENT <= SYS_VERSION,
                "\n'{request}' is not supported by your Valgrind version. \n\
                \t You are compiling against <valgrind/valgrind.h> {major}.{minor}. \n\
                \t This request requires Valgrind {req_major}.{req_minor} or higher.\n",
                request = &stringify!($request)[7..], // CR::CG_*
                major = crate::VALGRIND_VERSION.0,
                minor = crate::VALGRIND_VERSION.1,
                req_major = REQUIREMENT / 100,
                req_minor = REQUIREMENT % 100,
            );
        }
    };
}

pub(crate) use {assert_defined, client_request};

#[macro_export]
macro_rules! println{
    ($($arg:tt)+) => {{
        let msg = format!("{}\n\0", format_args!($($arg)+));

        let msg = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(msg.as_bytes()) };
        $crate::__print(msg);
    }}
}

#[macro_export]
macro_rules! print_stacktrace{
    ($($arg:tt)+) => {{
        let msg = format!("{}\0", format_args!($($arg)+));

        let msg = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(msg.as_bytes()) };
        $crate::__print_stacktrace(msg);
    }}
}

/// Behavior for a scoped requests.
pub trait Scope: sealed::Sealed {
    type Inner: Copy;

    fn enter(arg: Self::Inner);
    fn exit(arg: Self::Inner);
}

#[doc = include_str!("../../doc/ScopeGuard.md")]
#[clippy::has_significant_drop]
#[derive(Debug)]
#[must_use = "The guard activates immediately upon creation. Dropping it instantly reverts the operation."]
pub struct ScopeGuard<S: Scope> {
    inner: S::Inner,
    _marker: core::marker::PhantomData<S>,
}

impl<S: Scope> ScopeGuard<S> {
    #[inline(always)]
    fn new(inner: S::Inner) -> Self {
        S::enter(inner);

        Self { inner, _marker: core::marker::PhantomData }
    }
}

impl<S: Scope> Drop for ScopeGuard<S> {
    #[inline(always)]
    fn drop(&mut self) {
        S::exit(self.inner);
    }
}

mod sealed {
    pub trait Sealed {}
}
