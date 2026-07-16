pub mod cachegrind;
pub mod callgrind;
pub mod dhat;
pub mod drd;
pub mod helgrind;
pub mod memcheck;
pub mod valgrind;

pub(crate) mod constants;

macro_rules! client_request {
    (^ $default:expr, $request:path, $($arg:expr),*) =>  {{
        #[cfg(feature = "valgrind")]
        {
            $crate::requests::assert_defined!($request);
            unsafe {
                $crate::bindings::valgrind_client_request_expr(
                    $default as usize,
                    $request as usize,
                    $($arg as usize),*
                )
            }
        }

        #[cfg(not(feature = "valgrind"))]
        $default
    }};
    ($request:path, $default:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        client_request!(^ $default, $request, $arg1, $arg2, $arg3, $arg4, $arg5)
    };
    ($request:path, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        client_request!($request, 0, $arg1, $arg2, $arg3, $arg4, $arg5)
    };
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
// These assertions are expected to be optimized out by the compiler for supported requests.
macro_rules! assert_defined {
    ($request:path) => {{
        const REQUIREMENT: u32 = $request.required_version();

        // check Valgrind headers indeed found
        assert!(
            crate::VALGRIND_AVAILABLE,
            "\n`bindgen(libclang)` failed to locate `<valgrind/valgrind.h>`.\n\
            \tThis typically means Valgrind headers ain't found on the standard include paths:\n\
            \t\t<sysroot>/usr/include\n\
            \t\t<sysroot>/usr/local/include\n\
            \t\t...\n\
            \t\t\tnor via 'pkg-config' probe.\n\
            \tYou might try 'VALGRIND_INCLUDE=<path to valgrind/include>' as a quick workaround.\n\
            \tBuild configuration doc: https://docs.rs/crabgrind#build-configuration"
        );

        // check request requirement matches local Valgrind version
        assert!(
            $request as u32 > 0x1000,
            "\n'{request}' is not supported by your Valgrind version. \n\
                \t You are compiling against <valgrind/valgrind.h> {major}.{minor}. \n\
                \t This request requires Valgrind {req_major}.{req_minor} or higher.\n",
            request = &stringify!($request)[7..], // CR::CG_*
            major = crate::VALGRIND_VERSION.0,
            minor = crate::VALGRIND_VERSION.1,
            req_major = REQUIREMENT / 100,
            req_minor = REQUIREMENT % 100,
        );
    }};
}

pub(crate) use assert_defined;
pub(crate) use client_request;

/// Behavior for a logically scoped requests.
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
