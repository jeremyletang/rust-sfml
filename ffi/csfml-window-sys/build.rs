#[path="../build_common.rs"] mod build_sys_common;

fn main() {
    build_sys_common::link_sfml("window");
    build_sys_common::link_csfml("window");
}
