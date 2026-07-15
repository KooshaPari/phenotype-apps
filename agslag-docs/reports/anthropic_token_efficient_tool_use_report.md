# Anthropic Token-Efficient Tool Use - Technical Report

*Generated on 2025-04-07*

---

## Overview

Anthropic's Claude 3.7 Sonnet model introduces **token-efficient tool use** (beta), which reduces the number of tokens consumed during tool calls, leading to **lower costs and reduced latency**.

- Average token savings: **14%**
- Maximum observed savings: **up to 70%**
- Actual savings depend on response shape and size

---

## How It Works

- The model optimizes how tool calls are represented internally, reducing output tokens.
- This also reduces latency, as fewer tokens are generated and processed.
- The feature is **currently in beta** and should be evaluated before production use.

---

## Enabling Token-Efficient Tool Use

- Add the following beta header to your API requests:

```
anthropic-beta: token-efficient-tools-2025-02-19
```

- Use the Claude 3.7 Sonnet model version:

```
"model": "claude-3-7-sonnet-20250219"
```

- If using the SDK, ensure you are on the **beta SDK** with `anthropic.beta.messages`.

- **Note:**  
  This feature **does not work** with `disable_parallel_tool_use`.

---

## Example API Call (Shell)

```bash
curl https://api.anthropic.com/v1/messages \
  -H "content-type: application/json" \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: token-efficient-tools-2025-02-19" \
  -d '{
    "model": "claude-3-7-sonnet-20250219",
    "max_tokens": 1024,
    "tools": [
      {
        "name": "get_weather",
        "description": "Get the current weather in a given location",
        "input_schema": {
          "type": "object",
          "properties": {
            "location": {
              "type": "string",
              "description": "The city and state, e.g. San Francisco, CA"
            }
          },
          "required": ["location"]
        }
      }
    ],
    "messages": [
      {
        "role": "user",
        "content": "Tell me the weather in San Francisco."
      }
    ]
  }' | jq '.usage'
```

- This request will, on average, use **fewer tokens** than a normal request.
- To compare, try the same request **without** the beta header.

---

## Best Practices and Caveats

- **Prompt Caching:**  
  To benefit from prompt caching, **use the beta header consistently**.  
  Mixing requests with and without the header will prevent caching.

- **Prompt Improvement:**  
  Use the **Prompt Improver** in the Anthropic Console to optimize prompts for this feature.

- **Beta Status:**  
  Evaluate responses carefully before production use.

- **Incompatibility:**  
  Does **not** work with `disable_parallel_tool_use`.

---

## Summary of Benefits

- **Reduced token usage** → Lower costs
- **Reduced latency** → Faster responses
- **Simple to enable** via a beta header
- **No changes** needed to tool schemas or prompts

---

*End of report.*
