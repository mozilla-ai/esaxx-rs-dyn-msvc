#[cfg(feature = "cpp")]
#[cfg(not(target_os = "macos"))]
fn main() {
    let mut build = cc::Build::new();
    build
        .cpp(true)
        // C++11 is the minimum required standard for this codebase, but
        // MSVC only supports the /std:c++14 flag, which enables C++14 features but also allows C++11 code to compile.
        // Therefore, we use /std:c++14 for MSVC and -std=c++11 for other compilers.
        // .flag("-std=c++11")
        .file("src/esaxx.cpp")
        .include("src");
    // Look for a custom environment variable named ESAXX_DYNAMIC_LINK
    if std::env::var("ESAXX_DYNAMIC_LINK").unwrap_or_default() == "1" {
        build.static_crt(false);
    } else {
        build.static_crt(true);
    }
    build.compile("esaxx");
}

#[cfg(feature = "cpp")]
#[cfg(target_os = "macos")]
fn main() {
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag("-std=c++11")
        .flag("-stdlib=libc++")
        .file("src/esaxx.cpp")
        .include("src");
    // Look for a custom environment variable named ESAXX_DYNAMIC_LINK
    if std::env::var("ESAXX_DYNAMIC_LINK").unwrap_or_default() == "1" {
        build.static_crt(false);
    } else {
        build.static_crt(true);
    }
    build.compile("esaxx");
}

#[cfg(not(feature = "cpp"))]
fn main() {}
