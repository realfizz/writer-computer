#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        unsafe {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
    }
    if is_cli_invocation() {
        let argv: Vec<_> = std::env::args_os().collect();
        let cwd = std::env::current_dir().unwrap_or_else(|_| Path::new(".").into());
        return desktop_lib::writer_cli::run(argv, &cwd, &desktop_lib::writer_cli::SystemLauncher);
    }
    desktop_lib::run();
    ExitCode::SUCCESS
}

fn is_cli_invocation() -> bool {
    if std::env::var_os("WRITER_FORCE_GUI").is_some() {
        return false;
    }
    let Some(arg0) = std::env::args_os().next() else {
        return false;
    };
    Path::new(&arg0)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|name| name.eq_ignore_ascii_case("writer"))
        .unwrap_or(false)
}
