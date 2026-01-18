#![allow(dead_code)]
use lazy_static::lazy_static;
use regex::Regex;

const ANSI_CLEAR_LAST_LINE: &'static str = "\x1b[1A\x1b[2K";

lazy_static! {
    // filter everything from the failed test output, but the panic info(stack, line, message)
    static ref STRIP_HARNESS_RE: Regex = regex::Regex::new(r"----\n\s+([\s\S]*)note:").unwrap();
    // DRD address trace
    pub static ref DRD_TRACE_RE: Regex = regex::Regex::new(r"(end|load|store)\s+0x\S* size (\d+) \(thread (\d+) \/ vc \[(.*)\]").unwrap();
    // capture number of errors 'ERROR SUMMARY: (n)'
    pub static ref ERROR_SUMMARY_RE: Regex = regex::Regex::new(r"ERROR SUMMARY: (\d+)").unwrap();
    // capture number of 'definitely lost' bytes from memcheck report
    pub static ref DEFINITELY_LOST_RE: Regex = regex::Regex::new(r"definitely lost: (\d+) bytes").unwrap();
    // capture number of 'definitely lost (+added)' bytes from memcheck report
    pub static ref DEFINITELY_LOST_ADDED_RE: Regex = regex::Regex::new(r"definitely lost: \d+ \(\+(\d*)\) bytes").unwrap();
}

// 'panicks' current process, substituting panic message with the one from child process stdout
pub fn inline_panic(stdout: Vec<u8>, stderr: Vec<u8>) -> ! {
    let stdout = std::str::from_utf8(&stdout).unwrap();

    match STRIP_HARNESS_RE.captures(&stdout) {
        Some(caps) => panic!("{ANSI_CLEAR_LAST_LINE}{}", &caps[1]),
        None => panic!("{stdout}\n{}", std::str::from_utf8(&stderr).unwrap()),
    }
}

pub const TEST_RUNNER: &'static str = "__TEST__RUNNER";

#[macro_export]
// (c) stdext
macro_rules! wrapping_function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        // [mod path]::<fun>::f
        let name = type_name_of(f);
        (&name[..name.len() - 3]).split("::").last().unwrap()
    }};
}

#[macro_export]
macro_rules! valgrind {
    (@ $tool:expr, $($options:expr)?, $output_fn:expr) =>
    {{
        let test_bin = std::env::current_exe().unwrap();
        let test_name = wrapping_function_name!();

        let mut cmd = std::process::Command::new("valgrind");

        cmd.arg(concat!("--tool=", stringify!($tool)));
        $(cmd.args(stringify!($options).split(" "));)?

        let output = cmd
            .args([test_bin, test_name.into()])
            .args(["--no-capture"])
            .env(TEST_RUNNER, "")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
            .unwrap();

        if output.status.success() {
            $output_fn(output);
        } else {
            inline_panic(output.stdout, output.stderr);
        }
    }};
    ($tool:tt $($options:expr)? => $test_fn:expr, $output_fn:expr) => {{
        if std::env::var(TEST_RUNNER).is_ok() {
            $test_fn();
        } else {
            valgrind!(@ $tool, $($options)*, $output_fn);
        }
    }};
    ($tool:tt $($options:expr)? => $test_fn:expr) => {
        valgrind!($tool $($options)* => $test_fn, |_| {});
    };
}

#[macro_export]
macro_rules! cstr {
    ($arg:expr) => {
        unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(concat!($arg, "\0").as_bytes()) }
    };
}

#[macro_export]
macro_rules! as_str {
    ($arg:expr) => {
        unsafe { std::str::from_utf8_unchecked($arg) }
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[inline(never)]
pub unsafe fn oob_read_heap() {
    let v = vec![0u8; 3];
    let p = v.as_ptr().add(v.len());

    std::ptr::read_volatile(p);
}

#[inline(never)]
pub fn leak<const N: usize>() {
    let _ = std::hint::black_box(Box::leak(Box::new([0u8; N])));
}

#[inline(never)]
pub fn mmap<const N: usize>() -> *mut std::ffi::c_void {
    let ptr = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            N,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        )
    };
    assert_ne!(ptr, libc::MAP_FAILED, "mmap failed");
    ptr
}

#[inline(never)]
pub fn contention(pre: fn(*const u16)) -> *const u16 {
    use std::{
        sync::{Arc, Barrier, Mutex},
        thread,
    };
    let counter = Arc::new(Mutex::new(0u16));
    let c1 = Arc::clone(&counter);
    let barrier = Arc::new(Barrier::new(2));
    let b1 = Arc::clone(&barrier);

    let guard = counter.lock().unwrap();
    let addr = (&*guard) as *const u16;
    drop(guard);

    pre(addr);

    let h1 = thread::spawn(move || {
        b1.wait();
        *c1.lock().unwrap() += 1;
    });

    barrier.wait();
    *counter.lock().unwrap() += 1;

    h1.join().unwrap();

    addr
}

#[inline(never)]
pub fn race_unsafe(access: fn(*mut u16)) -> *const u16 {
    use std::{
        sync::{Arc, Barrier},
        thread,
    };

    static mut DATA: u16 = 2;

    let barrier = Arc::new(Barrier::new(2));
    let b1 = Arc::clone(&barrier);

    let h1 = thread::spawn(move || {
        b1.wait();
        access(&raw mut DATA);
    });

    barrier.wait();

    access(&raw mut DATA);

    h1.join().unwrap();

    &raw mut DATA
}

pub fn print_addr<T>(ptr: *const T) {
    println!("||> {ptr:p}||");
}

pub fn parse_addr(stdout: &str) -> &str {
    let var_addr = stdout.split_once("||>").unwrap().1.split_once("||").unwrap().0;
    &var_addr[2..]
}
