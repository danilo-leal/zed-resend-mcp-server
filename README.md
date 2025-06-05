# Resend MCP Server Extension for Zed

This extension integrates the [resend-mcp](https://www.npmjs.com/package/resend-mcp) npm package as a MCP server extension for [Zed](https://zed.dev).

## Configuration

### Minimal Configuration (API Key Only)

Add your Resend API key to your Zed settings (`~/.config/zed/settings.json`):

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

With this minimal setup, you'll be prompted to provide a sender email address each time you send an email.

### Recommended Configuration (With Sender Email)

For a smoother experience, also configure your default sender email:

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

## Getting Help
- [Resend MCP Documentation](https://resend.com/docs/knowledge-base/mcp-server)
- [Zed MCP documentation](https://zed.dev/docs/ai/mcp)
