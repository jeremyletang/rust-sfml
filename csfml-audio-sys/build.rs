#[path="../build_sys_common.rs"] mod build_sys_common;

fn main() {
    build_sys_common::link_csfml("audio");
}
