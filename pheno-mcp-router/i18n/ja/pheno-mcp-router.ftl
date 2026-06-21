# Fluent i18n catalog for `pheno-mcp-router` — Japanese (ja).
# Generated from the English baseline at `en/pheno-mcp-router.ftl`.

mcp-error-route-not-found = リクエスト { $request_id } に一致するプロバイダがルーティングポリシーにありません。
mcp-error-provider-unavailable = プロバイダ { $provider } は利用できません: { $reason }。
mcp-error-budget-exceeded = 予算超過: 使用済み { $spent }、上限 { $limit } ({ $scope })。
mcp-error-rate-limited = プロバイダ { $provider } のレート制限に達しました: { $retry_ms } ms 後に再試行してください。
mcp-error-context-overflow = コンテキストサイズ { $tokens } が { $provider } のモデル上限 { $limit } を超えています。
mcp-error-tool-not-found = ツール { $tool } はどのアクティブなプロバイダにも登録されていません。
mcp-error-permission-denied = ID { $identity } の下でツール { $tool } の権限が拒否されました。
mcp-info-routing-decision = { $request_id } → { $provider } にルーティングしました ({ $reason })。
mcp-info-cost-report = これまでのコスト: { $cost_usd } USD ({ $requests } 件のリクエスト)。
mcp-help = 使用法: pheno-mcp-router route|status|cost [--json] [--provider <名前>] [リクエスト]
