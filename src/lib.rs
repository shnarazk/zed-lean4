use zed_extension_api as zed;

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
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: get_path_to_language_server_executable()?,
            args: get_args_for_language_server()?,
            env: get_env_for_language_server()?,
        })
    }
}

fn get_path_to_language_server_executable() -> zed::Result<String> {
    Ok("".to_string())
}

fn get_args_for_language_server() -> zed::Result<Vec<String>> {
    Err("not yet".to_string())
}

fn get_env_for_language_server() -> zed::Result<Vec<(String, String)>> {
    Err("not yet".to_string())
}
