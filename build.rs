extern crate gcc;
extern crate pkg_config;

fn main() {
    // let weechat = match pkg_config::probe_library("weechat") {
    //     Ok(weechat) => weechat,
    //     Err(error) => panic!(format!(
    //         "Unable to find weechat.pc. Error: [[ {} ]]\n\
    //          ... Ensure that an up-to-date weechat is installed, and if your distro has it, \
    //          the weechat-dev package as well.",
    //         error
    //     )),
    // };
    // let mut config = gcc::Build::new();
    // for path in weechat.include_paths {
    //     config.include(path);
    // }
    // // config.file("src/ffi/weecord.c");
    // config.flag("-Wall");
    // config.flag("-Wextra");
    // config.compile("libweecord.a");
}
