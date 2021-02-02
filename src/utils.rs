pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// A function that returns the sign of a floating-point number or zero if it is
/// close to zero (within epsilon). Note that the method `std::f32::signum()` exists,
/// but it doesn't work exactly the same way.
pub fn sign_with_tolerance(value: f32) -> f32 {
    if value > 0.001 {
        1.0
    } else if value < -0.001 {
        -1.0
    } else {
        0.0
    }
}
