fn main() {
    glib_build_tools::compile_resources(
        &["res"],
        "res/resources.gresource.xml",
        "compiled.gresource"
    );
    println!("cargo:rerun-if-changed=res/icons");
}