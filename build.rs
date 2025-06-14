use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Tell Cargo to rerun if these files change
    println!("cargo:rerun-if-changed=src/nnn.c");
    println!("cargo:rerun-if-changed=src/nnn.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Get configuration from environment variables or features
    let o_debug =
        env::var("O_DEBUG").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "debug");
    let o_norl =
        env::var("O_NORL").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "norl");
    let o_pcre =
        env::var("O_PCRE").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "pcre");
    let o_nolc =
        env::var("O_NOLC").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "nolc");
    let o_nomouse = env::var("O_NOMOUSE").unwrap_or_else(|_| "0".to_string()) == "1"
        || cfg!(feature = "nomouse");
    let o_nobatch = env::var("O_NOBATCH").unwrap_or_else(|_| "0".to_string()) == "1"
        || cfg!(feature = "nobatch");
    let o_nofifo =
        env::var("O_NOFIFO").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "nofifo");
    let o_ctx8 =
        env::var("O_CTX8").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "ctx8");
    let o_icons =
        env::var("O_ICONS").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "icons");
    let o_nerd =
        env::var("O_NERD").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "nerd");
    let o_emoji =
        env::var("O_EMOJI").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "emoji");
    let o_qsort =
        env::var("O_QSORT").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "qsort");
    let o_bench =
        env::var("O_BENCH").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "bench");
    let o_nossn =
        env::var("O_NOSSN").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "nossn");
    let o_noug =
        env::var("O_NOUG").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "noug");
    let o_nox11 =
        env::var("O_NOX11").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "nox11");
    let o_matchfltr = env::var("O_MATCHFLTR").unwrap_or_else(|_| "0".to_string()) == "1"
        || cfg!(feature = "matchfltr");
    let o_nosort =
        env::var("O_NOSORT").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "nosort");
    let o_static =
        env::var("O_STATIC").unwrap_or_else(|_| "0".to_string()) == "1" || cfg!(feature = "static");

    // User patches
    let o_colemak = env::var("O_COLEMAK").unwrap_or_else(|_| "0".to_string()) == "1"
        || cfg!(feature = "colemak");
    let o_gitstatus = env::var("O_GITSTATUS").unwrap_or_else(|_| "0".to_string()) == "1"
        || cfg!(feature = "gitstatus");
    let o_namefirst = env::var("O_NAMEFIRST").unwrap_or_else(|_| "0".to_string()) == "1"
        || cfg!(feature = "namefirst");
    let o_restorepreview = env::var("O_RESTOREPREVIEW").unwrap_or_else(|_| "0".to_string()) == "1"
        || cfg!(feature = "restorepreview");

    // Validate icon options (only one can be set)
    let icon_count = [o_icons, o_nerd, o_emoji]
        .iter()
        .map(|&x| if x { 1 } else { 0 })
        .sum::<i32>();
    if icon_count > 1 {
        panic!("Choose only one system for icons (icons, nerd, or emoji)");
    }

    // Apply patches if needed
    apply_patches(o_namefirst, o_gitstatus, o_restorepreview, o_colemak);

    let mut build = cc::Build::new();

    // Set C standard and basic flags
    build.std("c11");
    build.flag("-Wall");
    build.flag("-Wextra");
    build.flag("-Wshadow");

    // Optimization
    let optimization = env::var("CFLAGS_OPTIMIZATION").unwrap_or_else(|_| "-O3".to_string());
    build.flag(&optimization);

    // Debug configuration
    if o_debug {
        build.define("DEBUG", None);
        build.flag("-g3");
    }

    // No readline support
    if o_norl || o_static {
        build.define("NORL", None);
    } else {
        println!("cargo:rustc-link-lib=readline");
    }

    // PCRE support
    if o_pcre {
        build.define("PCRE", None);
        println!("cargo:rustc-link-lib=pcre");
    }

    // No locale support (with icon compatibility checks)
    if o_nolc && !o_icons && !o_nerd && !o_emoji {
        build.define("NOLC", None);
    } else if o_nolc && (o_icons || o_nerd || o_emoji) {
        println!("cargo:warning=Ignoring O_NOLC since icons are enabled");
    }

    // Other feature flags
    if o_nomouse {
        build.define("NOMOUSE", None);
    }
    if o_nobatch {
        build.define("NOBATCH", None);
    }
    if o_nofifo {
        build.define("NOFIFO", None);
    }
    if o_ctx8 {
        build.define("CTX8", None);
    }
    if o_qsort {
        build.define("TOURBIN_QSORT", None);
    }
    if o_bench {
        build.define("BENCH", None);
    }
    if o_nossn {
        build.define("NOSSN", None);
    }
    if o_noug {
        build.define("NOUG", None);
    }
    if o_nox11 {
        build.define("NOX11", None);
    }
    if o_matchfltr {
        build.define("MATCHFLTR", None);
    }
    if o_nosort {
        build.define("NOSORT", None);
    }

    // Icon support
    let mut icons_include = None;
    if o_icons {
        icons_include = Some("icons-generated-icons-in-term.h");
        build.define("ICONS_IN_TERM", None);
        build.define("ICONS_INCLUDE", Some("\"icons-generated-icons-in-term.h\""));
    } else if o_nerd {
        icons_include = Some("icons-generated-nerd.h");
        build.define("NERD", None);
        build.define("ICONS_INCLUDE", Some("\"icons-generated-nerd.h\""));
    } else if o_emoji {
        icons_include = Some("icons-generated-emoji.h");
        build.define("EMOJI", None);
        build.define("ICONS_INCLUDE", Some("\"icons-generated-emoji.h\""));
    }

    // Generate icons if needed
    if let Some(include_file) = icons_include {
        generate_icons_header(include_file);
    }

    // Configure ncurses
    configure_ncurses(&mut build);

    // macOS compatibility
    configure_macos(&mut build);

    // Static linking
    if o_static {
        println!("cargo:rustc-link-lib=static=gpm");
    }

    // Add pthread
    println!("cargo:rustc-link-lib=pthread");

    // Add source files
    build.file("src/nnn.c");

    // Add macOS compatibility source if needed
    if is_macos_below_1012() {
        build.file("misc/macos-legacy/mach_gettime.c");
        build.define("MACOS_BELOW_1012", None);
    }

    // Compile
    build.compile("nnn");

    // Reverse patches after compilation
    reverse_patches(o_namefirst, o_gitstatus, o_restorepreview, o_colemak);
}

fn configure_ncurses(build: &mut cc::Build) {
    // Try to use pkg-config for ncurses configuration
    if let Ok(lib) = pkg_config::Config::new().probe("ncursesw") {
        for include_path in lib.include_paths {
            build.include(include_path);
        }
        for lib_path in lib.link_paths {
            println!("cargo:rustc-link-search=native={}", lib_path.display());
        }
        for lib in lib.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    } else if let Ok(lib) = pkg_config::Config::new().probe("ncurses") {
        for include_path in lib.include_paths {
            build.include(include_path);
        }
        for lib_path in lib.link_paths {
            println!("cargo:rustc-link-search=native={}", lib_path.display());
        }
        for lib in lib.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    } else {
        // Fallback to linking ncurses directly
        println!("cargo:rustc-link-lib=ncurses");
    }
}

fn configure_macos(build: &mut cc::Build) {
    if cfg!(target_os = "macos") && is_macos_below_1012() {
        build.include("misc/macos-legacy");
    }
}

fn is_macos_below_1012() -> bool {
    if !cfg!(target_os = "macos") {
        return false;
    }

    // Try to get macOS version
    if let Ok(output) = Command::new("sw_vers").arg("-productVersion").output() {
        if let Ok(version_str) = String::from_utf8(output.stdout) {
            let version = version_str.trim();
            // Simple version comparison - this is a basic implementation
            let parts: Vec<&str> = version.split('.').collect();
            if parts.len() >= 2 {
                if let (Ok(major), Ok(minor)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    return major < 10 || (major == 10 && minor < 12);
                }
            }
        }
    }
    false
}

fn generate_icons_header(include_file: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(include_file);

    // Build the icon hash generator
    let mut icon_build = cc::Build::new();
    icon_build.define("ICONS_GENERATE", None);
    icon_build.file("src/icons-hash.c");
    icon_build.compile("icons-hash-gen");

    // Generate the header file
    let output = Command::new(format!("{}/build/icons-hash-gen", out_dir))
        .output()
        .expect("Failed to generate icons header");

    std::fs::write(&dest_path, output.stdout).expect("Failed to write icons header");

    // Tell the compiler where to find the generated header
    println!("cargo:rustc-env=ICONS_INCLUDE_PATH={}", dest_path.display());
}

fn apply_patches(o_namefirst: bool, o_gitstatus: bool, o_restorepreview: bool, o_colemak: bool) {
    let patch_opts = env::var("PATCH_OPTS").unwrap_or_default();

    if o_namefirst {
        apply_patch("patches/namefirst/mainline.diff", &patch_opts);
        if o_gitstatus {
            apply_patch("patches/gitstatus/namefirst.diff", &patch_opts);
        }
    } else if o_gitstatus {
        apply_patch("patches/gitstatus/mainline.diff", &patch_opts);
    }

    if o_restorepreview {
        apply_patch("patches/restorepreview/mainline.diff", &patch_opts);
    }

    if o_colemak {
        apply_patch("patches/colemak/mainline.diff", &patch_opts);
    }
}

fn reverse_patches(o_namefirst: bool, o_gitstatus: bool, o_restorepreview: bool, o_colemak: bool) {
    let patch_opts = env::var("PATCH_OPTS").unwrap_or_default();

    if o_namefirst {
        if o_gitstatus {
            reverse_patch("patches/gitstatus/namefirst.diff", &patch_opts);
        }
        reverse_patch("patches/namefirst/mainline.diff", &patch_opts);
    } else if o_gitstatus {
        reverse_patch("patches/gitstatus/mainline.diff", &patch_opts);
    }

    if o_restorepreview {
        reverse_patch("patches/restorepreview/mainline.diff", &patch_opts);
    }

    if o_colemak {
        reverse_patch("patches/colemak/mainline.diff", &patch_opts);
    }
}

fn apply_patch(patch_file: &str, opts: &str) {
    let mut cmd = Command::new("patch");
    cmd.arg("--forward")
        .arg("--strip=1")
        .arg(format!("--input={}", patch_file));

    if !opts.is_empty() {
        for opt in opts.split_whitespace() {
            cmd.arg(opt);
        }
    }

    let output = cmd.output().expect("Failed to apply patch");
    if !output.status.success() {
        println!(
            "cargo:warning=Failed to apply patch {}: {}",
            patch_file,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn reverse_patch(patch_file: &str, opts: &str) {
    let mut cmd = Command::new("patch");
    cmd.arg("--reverse")
        .arg("--strip=1")
        .arg(format!("--input={}", patch_file));

    if !opts.is_empty() {
        for opt in opts.split_whitespace() {
            cmd.arg(opt);
        }
    }

    let output = cmd.output().expect("Failed to reverse patch");
    if !output.status.success() {
        println!(
            "cargo:warning=Failed to reverse patch {}: {}",
            patch_file,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
