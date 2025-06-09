use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;
use zed_extension_api::{
    self as zed, serde_json, settings::ContextServerSettings, Command, ContextServerConfiguration,
    ContextServerId, Project, Result,
};

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
        let settings = ContextServerSettings::for_project("mcp-server-resend", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `resend_api_key` setting".into());
        };

        let settings: ResendContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        // Check if resend-mcp is installed, if not install it
        let node_modules_dir = std::env::current_dir().unwrap().join("node_modules");
        let resend_mcp_dir = node_modules_dir.join("resend-mcp");
        let package_binary = resend_mcp_dir.join("dist").join("index.js");

        if !package_binary.exists() {
            // Create package.json if it doesn't exist
            let package_json_path = std::env::current_dir().unwrap().join("package.json");
            if !package_json_path.exists() {
                let package_json = r#"{
  "name": "mcp-resend-server-wrapper",
  "version": "1.0.0",
  "type": "module",
  "dependencies": {
    "resend-mcp": "^0.1.2"
  }
}"#;
                fs::write(&package_json_path, package_json).map_err(|e| e.to_string())?;
            }

            zed::npm_install_package("resend-mcp", "^0.1.2")?;

            // Verify that the binary exists after installation
            if !package_binary.exists() {
                return Err(format!(
                    "Failed to install resend-mcp package or binary not found at {}",
                    package_binary.display()
                ));
            }
        }

        // Prepare environment variables for the resend-mcp server
        let mut env = vec![("RESEND_API_KEY".to_string(), settings.resend_api_key)];

        if let Some(sender) = settings.sender_email_address {
            if !sender.is_empty() {
                env.push(("SENDER_EMAIL_ADDRESS".to_string(), sender));
            }
        }

        if let Some(reply_to) = settings.reply_to_email_addresses {
            if !reply_to.is_empty() {
                env.push(("REPLY_TO_EMAIL_ADDRESSES".to_string(), reply_to));
            }
        }

        let args = vec![package_binary.to_string_lossy().to_string()];

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
