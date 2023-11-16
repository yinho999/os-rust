// Dont link the standard library
#![no_std]
// Disable C runtime and Rust runtime initialization
#![no_main]
use core::panic::PanicInfo;


/// Overwriting the operating system entry point with our own (Also replace main() because there is no C runtime on bare metal)
/// `#[no_mangle]` is used to prevent the Rust compiler from changing the name of this function, or it will be given an unique name by compiler
/// `extern "C"` is used to tell the compiler to use the C calling convention for this function
/// `!` means that this function is diverging. This entry point is not called by any function but invoked directly by the OS or bootloader, and it will never return. The entry point should eg. invoke the `[exit]` system call to terminate the process(Shut down computer).
#[no_mangle]
/// this function is the entry point, since the linker looks for a function
/// named `_start` by default
pub extern "C" fn _start() -> ! {
    loop {}
}

/// `#[panic_handler]` is called on panic
/// Returning a `!` means that this function is diverging, which is a special for never type
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/*
    eh_personality is used to handle stack unwinding, which requires some OS specific library - Linux libunwind / Windows structured exception handling. Therefore we need to disable this feature in Cargo.toml.
 */

/*
https://os.phil-opp.com/freestanding-rust-binary/#linker-arguments
 `linking with `cc` failed: exit status: 1` means that the linker cannot find the C runtime library. we can tell the linker do not include C runtime.
  Do this by passing sets of arguments to the linker or building for bare metal target.
  - Passing Arguments To The Linker: Linux
    The linker includes the startup routine of the C runtime by default, which is also called _start. The _start function from C runtime requires some symbols from C standard library libc that we dont include due to no_std attribute. Therefore we need to tell the linker to not include the C startup routine by passing -nostartfiles to the linker.

    `cargo rustc` is equals to `cargo build` but allows to pass options to rustc
    `-C link-arg` passes an argument to the linker
    Combined, this command tells the linker to not include the C startup routine.
    `cargo rustc -- -C link-arg=-nostartfiles`

    Contain all commands in .cargo/config.toml

  - Building For Bare Metal Target
    Download a bare metal target tripe - embedded ARM
    `rustup target add thumbv7em-none-eabihf`
    Build with the target
    `cargo build --target thumbv7em-none-eabihf`

*/