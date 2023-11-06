fn main() {
    pkg_config::Config::new().atleast_version("1.2");
    // .probe("z")
    // .unwrap();
    let src = ["Unishox2/unishox2.c"];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("Unishox2")
        .flag("-Wno-unused-parameter")
        .opt_level(3);
    build.compile("unishox2");
}
