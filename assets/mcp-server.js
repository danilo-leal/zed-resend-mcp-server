#!/usr/bin/env node

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { Resend } from "resend";

const resend = new Resend(process.env.RESEND_API_KEY);
const defaultFrom = process.env.RESEND_DEFAULT_FROM || "";
const defaultReplyTo = process.env.RESEND_DEFAULT_REPLY_TO || "";

const server = new Server(
  {
    name: "resend-mcp-server",
    version: "1.0.0",
  },
  {
    capabilities: {
      tools: {},
    },
  },
);

server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools: [
      {
        name: "resend_send_email",
        description: "Send an email using the Resend API.",
        inputSchema: {
          type: "object",
          properties: {
            to: {
              type: "array",
              items: { type: "string" },
              description: "Email recipient(s)",
            },
            from: {
              type: "string",
              description: "Sender email address (must be verified in Resend)",
              default: defaultFrom,
            },
            subject: {
              type: "string",
              description: "Email subject line",
            },
            text: {
              type: "string",
              description: "Plain text content of the email",
            },
            html: {
              type: "string",
              description: "HTML content of the email (optional)",
            },
            cc: {
              type: "array",
              items: { type: "string" },
              description: "CC recipients (optional)",
            },
            bcc: {
              type: "array",
              items: { type: "string" },
              description: "BCC recipients (optional)",
            },
            replyTo: {
              type: "array",
              items: { type: "string" },
              description: "Reply-to email addresses (optional)",
              default: defaultReplyTo ? [defaultReplyTo] : [],
            },
            scheduledAt: {
              type: "string",
              description:
                "ISO 8601 date string to schedule the email (optional)",
            },
          },
          required: ["to", "from", "subject"],
          oneOf: [
            { required: ["text"] },
            { required: ["html"] },
            { required: ["text", "html"] },
          ],
        },
      },
    ],
  };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  if (request.params.name === "resend_send_email") {
    try {
      const args = request.params.arguments;

      // Validate required fields
      if (!args.to || !args.from || !args.subject) {
        throw new Error(
          "Missing required fields: to, from, and subject are required",
        );
      }

      if (!args.text && !args.html) {
        throw new Error("Either text or html content is required");
      }

      // Prepare email data
      const emailData = {
        from: args.from,
        to: args.to,
        subject: args.subject,
        text: args.text,
        html: args.html,
        cc: args.cc,
        bcc: args.bcc,
        reply_to: args.replyTo,
        scheduled_at: args.scheduledAt,
      };

      // Remove undefined values
      Object.keys(emailData).forEach((key) => {
        if (emailData[key] === undefined) {
          delete emailData[key];
        }
      });

      // Send email using Resend API
      const data = await resend.emails.send(emailData);

      return {
        content: [
          {
            type: "text",
            text: `Email sent successfully! ID: ${data.id}`,
          },
        ],
      };
    } catch (error) {
      return {
        content: [
          {
            type: "text",
            text: `Failed to send email: ${error.message}`,
          },
        ],
        isError: true,
      };
    }
  }

  throw new Error(`Unknown tool: ${request.params.name}`);
});

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("Resend MCP server running...");
}

main().catch((error) => {
  console.error("Fatal error:", error);
  process.exit(1);
});
