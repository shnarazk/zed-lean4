use zed_extension_api::{self as zed};

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
    lang_id: &zed::LanguageServerId,
    worktree: &zed::Worktree,
) -> zed::Result<String> {
    let lsp_name = lang_id.as_ref();
    if let Ok(lsp_settings) = zed::settings::LspSettings::for_worktree(lsp_name, worktree) {
        dbg!("LEAN4: found by LspSettings::for_worktree");
        if let Some(bin_settings) = lsp_settings.binary.as_ref() {
            if let Some(path) = bin_settings.path.clone() {
                return Ok(path);
            }
        }
    }
    if let Some(lsp_path) = worktree.which(lsp_name) {
        return Ok(lsp_path);
    }
    // Err("LEAN4: not found".to_string())
    Ok(
        "/nix/store/g9vd9n3icplchrxly35vsp7cfmkf7n14-elan-3.1.1-unstable-2024-08-02/bin/lean"
            .to_string(),
    )
}

fn get_args_for_language_server() -> zed::Result<Vec<String>> {
    Ok(vec!["--server".to_string()])
    // Ok(vec!["--server".to_string(), "--memory=1024".to_string()])
}

fn get_env_for_language_server() -> zed::Result<Vec<(String, String)>> {
    Ok(vec![])
}

zed::register_extension!(Lean4Extension);
