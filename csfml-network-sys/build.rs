#[path="../build_sys_common.rs"] mod build_sys_common;

fn main() {
    build_sys_common::link_sfml("network");
    build_sys_common::link_csfml("network");
}
