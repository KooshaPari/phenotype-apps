# MCP Tool Poisoning Attacks: Risks & Mitigation for Our Multi-Agent System

---

## Overview

Recent security research by Invariant Labs has revealed a **critical vulnerability** in the Model Context Protocol (MCP) ecosystem, called **Tool Poisoning Attacks (TPAs)**. These attacks can **compromise our multi-agent orchestration**, leading to **data leaks, hijacked workflows, and unauthorized actions**.

Given our use of **multiple MCP servers** (internal and third-party), this is a **high-priority security concern**.

---

## How Tool Poisoning Attacks Work

- **Malicious MCP servers** embed **hidden instructions** inside tool descriptions.
- Our AI models **see and follow** these hidden instructions, which can:
  - **Exfiltrate sensitive files** (SSH keys, configs, credentials)
  - **Transmit data covertly** via tool parameters
  - **Override or hijack** trusted tool behaviors (shadowing)
  - **Conceal malicious actions** behind benign-looking UI
- **Users and developers only see simplified tool descriptions**, missing the hidden instructions.
- **Rug pulls:** Servers can **change tool descriptions after approval**, injecting malicious payloads later.
- **Shadowing:** Malicious servers can **modify agent behavior** with respect to other trusted servers, hijacking workflows.

---

## Risks to Our Project

- **Credential theft:** SSH keys, API tokens, internal configs
- **Sensitive data leaks:** User data, chat logs, proprietary code
- **Hijacked workflows:** Agents sending data to attackers, overriding trusted tools
- **Invisible attacks:** Hidden from UI and logs
- **Supply chain compromise:** Malicious updates from third-party servers

---

## How We Can Defend Against Tool Poisoning

### 1. **Full Transparency of Tool Descriptions**

- **Display full, raw tool descriptions** during server integration and tool approval.
- **Highlight AI-visible instructions** (e.g., special tags, hidden prompts).
- **Warn developers** if descriptions contain suspicious content.

### 2. **Pin and Verify Tool Versions**

- **Pin tool descriptions** by hash/checksum at approval time.
- **Reject or warn** if a server changes a tool after approval.
- Use **signed manifests** or **content hashes** to verify integrity.

### 3. **Isolate and Sandbox MCP Servers**

- **Limit data sharing** between servers.  
- Prevent malicious servers from **shadowing or hijacking** trusted tools.
- Use **strict dataflow controls** and **server allowlists**.

### 4. **Implement Guardrails and Validation**

- **Scan tool descriptions** for hidden instructions or prompt injections.
- **Restrict file access** and sensitive operations by default.
- Require **explicit user approval** for sensitive actions.

### 5. **Audit and Monitor**

- Log **all tool descriptions** and **tool calls**.
- Detect **anomalous behavior** or unexpected data flows.
- Regularly **review connected servers** and their permissions.

### 6. **Limit Trust in Third-Party Servers**

- Avoid connecting to **untrusted or unknown MCP servers**.
- Prefer **self-hosted or audited servers**.
- Use **manual review** before enabling new tools.

---

## Action Items for Our Team

- **Review all current MCP server connections** for trustworthiness.
- **Implement tool description transparency** in our UI and approval flows.
- **Pin tool versions** and verify integrity before use.
- **Isolate third-party servers** from sensitive internal data.
- **Add security scanning** for tool descriptions.
- **Increase audit logging** of tool calls and data flows.
- **Educate developers** about Tool Poisoning risks.

---

## Conclusion

Tool Poisoning Attacks pose a **serious threat** to our multi-agent system.  
We must **immediately improve our security posture** by increasing transparency, validation, isolation, and monitoring of all MCP integrations.

**Treat every new tool as potentially malicious until proven safe.**

---

_Last updated: 2025-04-09_
