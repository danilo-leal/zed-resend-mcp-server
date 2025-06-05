# Resend MCP Server Extension for Zed

This extension integrates the [resend-mcp](https://www.npmjs.com/package/resend-mcp) npm package as a MCP server extension for [Zed](https://zed.dev).

## Features

- **Automatic Installation**: The extension automatically downloads and manages the `resend-mcp` npm package
- **Simple Configuration**: Only requires your Resend API key (sender email is optional)
- **Rich Email Support**: Send plain text, HTML, and scheduled emails
- **Attachment Support**: Send local files or remote URLs as attachments
- **Multiple Recipients**: Support for CC, BCC, and reply-to addresses

## Prerequisites

1. **Node.js**: Make sure you have Node.js installed (version 18 or later)
2. **Resend Account**: Sign up at [resend.com](https://resend.com) and create an API key
3. **Verified Domain** (optional): To send emails to external recipients, verify your domain in Resend

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
- [Resend Documentation](https://resend.com/docs)
- [Zed MCP documentation](https://zed.dev/docs/ai/mcp)
