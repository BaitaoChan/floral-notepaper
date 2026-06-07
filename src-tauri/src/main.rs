// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<std::ffi::OsString> = std::env::args_os().collect();
    if args
        .get(1)
        .is_some_and(|flag| flag == std::ffi::OsStr::new("--update-helper"))
    {
        let exit_code = floral_notepaper_lib::updater::helper::run_cli(args.into_iter().skip(2));
        std::process::exit(exit_code.as_i32());
    }

    match floral_notepaper_lib::cli::dispatch(args) {
        floral_notepaper_lib::cli::CliOutcome::Exit(code) => std::process::exit(code),
        floral_notepaper_lib::cli::CliOutcome::RunGui(options) => {
            floral_notepaper_lib::desktop::set_gui_launch_options(options);
            floral_notepaper_lib::run();
        }
    }
}
