fn main() {
    cynic_codegen::register_schema("main")
        .from_sdl_file("schemas/main.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
