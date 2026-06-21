# Fluent i18n catalog for `pheno-mcp-router` — Spanish (es).
# Generated from the English baseline at `en/pheno-mcp-router.ftl`.

mcp-error-route-not-found = Ningún proveedor coincide con la política de enrutamiento para la solicitud { $request_id }.
mcp-error-provider-unavailable = El proveedor { $provider } no está disponible: { $reason }.
mcp-error-budget-exceeded = Presupuesto excedido: gastado { $spent }, límite { $limit } ({ $scope }).
mcp-error-rate-limited = Límite de velocidad alcanzado para el proveedor { $provider }: reintentar después de { $retry_ms } ms.
mcp-error-context-overflow = El tamaño del contexto { $tokens } supera el límite del modelo { $limit } para { $provider }.
mcp-error-tool-not-found = La herramienta { $tool } no está registrada en ningún proveedor activo.
mcp-error-permission-denied = Permiso denegado para la herramienta { $tool } bajo la identidad { $identity }.
mcp-info-routing-decision = Enrutado { $request_id } → { $provider } ({ $reason }).
mcp-info-cost-report = Costo hasta ahora: { $cost_usd } USD en { $requests } solicitudes.
mcp-help = Uso: pheno-mcp-router route|status|cost [--json] [--provider <nombre>] [SOLICITUD]
