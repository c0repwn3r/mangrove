fn main() {
    build_data::set_BUILD_HOSTNAME();
    build_data::set_BUILD_TIMESTAMP();
    build_data::set_GIT_BRANCH();
    build_data::set_GIT_COMMIT_SHORT();
    build_data::set_GIT_DIRTY();
    build_data::set_RUST_CHANNEL();
    build_data::set_RUSTC_VERSION();
    build_data::set_SOURCE_TIMESTAMP();
}