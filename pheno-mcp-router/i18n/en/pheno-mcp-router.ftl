# Fluent i18n catalog for `pheno-mcp-router`.
# Format: Fluent (.ftl) — Mozilla Project Fluent.
# Reference: https://projectfluent.org/
# MCP router: provider selection, cost/budget enforcement, tool routing.

mcp-error-route-not-found = No provider matches the routing policy for request { $request_id }.
mcp-error-provider-unavailable = Provider { $provider } is unavailable: { $reason }.
mcp-error-budget-exceeded = Budget exceeded: spent { $spent }, limit { $limit } ({ $scope }).
mcp-error-rate-limited = Rate limit hit for provider { $provider }: retry after { $retry_ms } ms.
mcp-error-context-overflow = Context size { $tokens } exceeds model limit { $limit } for { $provider }.
mcp-error-tool-not-found = Tool { $tool } is not registered on any active provider.
mcp-error-permission-denied = Permission denied for tool { $tool } under identity { $identity }.
mcp-info-routing-decision = Routed { $request_id } → { $provider } ({ $reason }).
mcp-info-cost-report = Cost so far: { $cost_usd } USD across { $requests } requests.
mcp-help = Usage: pheno-mcp-router route|status|cost [--json] [--provider <name>] [REQUEST]
