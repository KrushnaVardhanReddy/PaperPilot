//! Client configuration rendering. This module never edits client settings.
use clap::ValueEnum;
use serde_json::json;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Client {
    Codex,
    ClaudeCode,
    KimiCode,
    Pi,
    ClaudeDesktop,
    Cursor,
}

pub fn render(client: Client, command: &str) -> Result<String, serde_json::Error> {
    match client {
        Client::Codex => Ok(format!(
            "[mcp_servers.paperpilot]\ncommand = {}\nargs = []\n",
            serde_json::to_string(command)?
        )),
        _ => serde_json::to_string_pretty(&json!({
            "mcpServers": {
                "paperpilot": { "command": command, "args": [] }
            }
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_clients_preserve_executable_paths() {
        for client in [
            Client::ClaudeCode,
            Client::KimiCode,
            Client::Pi,
            Client::ClaudeDesktop,
            Client::Cursor,
        ] {
            let command = "C:\\Program Files\\PaperPilot\\paperpilot-mcp.exe";
            let config: serde_json::Value =
                serde_json::from_str(&render(client, command).unwrap()).unwrap();
            assert_eq!(config["mcpServers"]["paperpilot"]["command"], command);
            assert_eq!(config["mcpServers"]["paperpilot"]["args"], json!([]));
        }
    }

    #[test]
    fn codex_escapes_windows_paths_and_quotes() {
        assert_eq!(
            render(Client::Codex, "C:\\PDF tools\\a\"b.exe").unwrap(),
            "[mcp_servers.paperpilot]\ncommand = \"C:\\\\PDF tools\\\\a\\\"b.exe\"\nargs = []\n"
        );
    }
}
