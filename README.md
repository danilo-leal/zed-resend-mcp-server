# Resend MCP Server Extension for Zed

This extension provides a MCP server for sending emails via [Resend](https://resend.com) directly from [Zed](https://zed.dev).
It uses the (unofficial) [resend-mcp](https://www.npmjs.com/package/resend-mcp) npm package.

## Configuration

### Minimal Configuration (API Key Only)

Add your Resend API key to your Zed `settings.json`:

```json
{
  "context_servers": {
    "mcp-server-resend": {
      "settings": {
        "resend_api_key": "re_your_actual_api_key_here"
      }
    }
  }
}
```

### Recommended Configuration (With Sender Email)

For a nicer experience, also configure your default sender email:

```json
{
  "context_servers": {
    "mcp-server-resend": {
      "settings": {
        "resend_api_key": "re_your_actual_api_key_here",
        "sender_email_address": "your-verified@yourdomain.com"
      }
    }
  }
}
```

### Complete Configuration (All Options)

You can also configure reply-to addresses:

```json
{
  "context_servers": {
    "mcp-server-resend": {
      "settings": {
        "resend_api_key": "re_your_actual_api_key_here",
        "sender_email_address": "your-verified@yourdomain.com",
        "reply_to_email_addresses": "replies@yourdomain.com,support@yourdomain.com"
      }
    }
  }
}
```

## Features

| Feature | Status |
|---------|--------|
| Send plain text emails | **Supported** |
| Send HTML emails | **Supported** |
| Multiple recipients | **Supported** |
| CC and BCC | **Supported** |
| Reply-to addresses | **Supported** |
| Scheduled sending | **Supported** |
| Custom sender email | **Supported** |
| Environment-based configuration | **Supported** |
| File attachments | **Supported** |
| Remote URL attachments | **Supported** |
| React Email templates | Not Supported |
| Batch sending | Not Supported |
| Email tracking | Not Supported |

## Further Information

- [Resend MCP Docs](https://resend.com/docs/knowledge-base/mcp-server)
- [resend-mcp npm package](https://www.npmjs.com/package/resend-mcp) (Unofficial)
- [Resend MCP Server](https://github.com/resend/mcp-send-email)
- [Zed MCP Docs](https://zed.dev/docs/assistant/model-context-protocol)
