use zed_extension_api::{self as zed /* Worktree */};

struct Lean4Extension {}

impl zed::Extension for Lean4Extension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: get_path_to_language_server_executable(language_server_id, worktree)?,
            args: get_args_for_language_server()?,
            env: get_env_for_language_server()?,
        })
    }
}

fn get_path_to_language_server_executable(
    _lang_id: &zed::LanguageServerId,
    worktree: &zed::Worktree,
) -> zed::Result<String> {
    if let Some(path) = worktree.which("lean") {
        if std::fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
            return Ok(dbg!(path.clone()));
        }
    }
    Err("no lean".to_string())
}

fn get_args_for_language_server() -> zed::Result<Vec<String>> {
    Ok(vec!["--server".to_string()])
}

fn get_env_for_language_server() -> zed::Result<Vec<(String, String)>> {
    Ok(vec![])
}
