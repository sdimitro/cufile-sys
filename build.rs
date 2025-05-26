use std::env;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    // Tell cargo to look for shared libraries in the standard locations
    println!("cargo:rustc-link-lib=cufile");

    // Try to find CUDA installation
    if let Ok(cuda_path) = env::var("CUDA_PATH") {
        println!("cargo:rustc-link-search=native={}/lib64", cuda_path);
    } else if let Ok(cuda_home) = env::var("CUDA_HOME") {
        println!("cargo:rustc-link-search=native={}/lib64", cuda_home);
    } else {
        // Default CUDA installation paths
        println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
        println!("cargo:rustc-link-search=native=/opt/cuda/lib64");
    }

    #[cfg(feature = "generate-bindings")]
    {
        generate_bindings();
    }
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings() {
    use bindgen;
    use std::path::PathBuf;

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("cuFile.*")
        .allowlist_type("CU.*")
        .allowlist_var("CU.*")
        .allowlist_var("CUFILE.*")
        .derive_default(true)
        .derive_debug(true)
        .derive_copy(true)
        .derive_clone(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .derive_hash(true)
        .generate_comments(true);

    // Add CUDA include paths
    if let Ok(cuda_path) = env::var("CUDA_PATH") {
        builder = builder.clang_arg(format!("-I{}/include", cuda_path));
    } else if let Ok(cuda_home) = env::var("CUDA_HOME") {
        builder = builder.clang_arg(format!("-I{}/include", cuda_home));
    } else {
        builder = builder
            .clang_arg("-I/usr/local/cuda/include")
            .clang_arg("-I/opt/cuda/include");
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
