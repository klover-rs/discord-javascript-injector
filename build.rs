fn main() {

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search=native=cpp");

        println!("cargo:rustc-link-lib=static=windows_process");
    
        cc::Build::new()
            .cpp(true)
            .file("src/windows/process.cpp")
            .compile("windows_process");
    }
   
}