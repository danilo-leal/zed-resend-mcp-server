use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use std::fs;
use zed_extension_api::{
    self as zed, serde_json, settings::ContextServerSettings, Command, ContextServerConfiguration,
    ContextServerId, Project, Result,
};

const SERVER_PATH: &str = "mcp-resend-server/index.js";

#[derive(Debug, Deserialize, JsonSchema)]
struct ResendContextServerSettings {
    resend_api_key: String,
    #[serde(default)]
    sender_email_address: Option<String>,
    #[serde(default)]
    reply_to_email_addresses: Option<String>,
}

struct ResendModelContextExtension;

impl zed::Extension for ResendModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let server_dir = env::current_dir().unwrap().join("mcp-resend-server");
        let server_file = server_dir.join("index.js");

        if !server_file.exists() {
            fs::create_dir_all(&server_dir).map_err(|e| e.to_string())?;
            let server_script = include_str!("../assets/mcp-server.js");
            fs::write(&server_file, server_script).map_err(|e| e.to_string())?;

            // Create package.json
            let package_json = r#"{
  "name": "mcp-resend-server",
  "version": "1.0.0",
  "type": "module",
  "dependencies": {
    "@modelcontextprotocol/sdk": "^1.0.0",
    "resend": "^4.0.0"
  }
}"#;
            fs::write(server_dir.join("package.json"), package_json).map_err(|e| e.to_string())?;

            zed::npm_install_package("@modelcontextprotocol/sdk", "^1.0.0")?;
            zed::npm_install_package("resend", "^4.0.0")?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-resend", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `resend_api_key` setting".into());
        };
        let settings: ResendContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        let mut env = vec![("RESEND_API_KEY".to_string(), settings.resend_api_key)];

        if let Some(sender) = settings.sender_email_address {
            env.push(("RESEND_DEFAULT_FROM".to_string(), sender));
        }

        if let Some(reply_to) = settings.reply_to_email_addresses {
            env.push(("RESEND_DEFAULT_REPLY_TO".to_string(), reply_to));
        }

        let args = vec![env::current_dir()
            .unwrap()
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string()];

        Ok(Command {
            command: zed::node_binary_path()?,
            args,
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();

        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(ResendContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(ResendModelContextExtension);
