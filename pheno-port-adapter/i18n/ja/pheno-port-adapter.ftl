# Fluent i18n catalog for `pheno-port-adapter` — Japanese (ja).
# Generated from the English baseline at `en/pheno-port-adapter.ftl`.

port-error-not-implemented = ポート { $port } には登録済みのアダプタがありません。
port-error-adapter-missing = ポート { $port } のアダプタ { $adapter } を解決できませんでした。
port-error-incompatible-version = アダプタ { $adapter } v{ $have } はポート v{ $want } と互換性がありません。
port-error-timeout = ポート { $port } が { $ms } ms でタイムアウトしました。
port-error-connection-refused = { $endpoint } (ポート { $port }) に接続を拒否されました。
adapter-error-init-failed = アダプタ { $adapter } の初期化に失敗しました: { $reason }。
port-info-registered = ポート { $port } にアダプタ { $adapter } を登録しました。
port-info-resolved = ポート { $port } → { $adapter } を { $us } µs で解決しました。
port-warn-deprecated-port = ポート { $port } は非推奨です。{ $deadline } までに { $replacement } に移行してください。
port-help = 使用法: pheno-port-adapter list|resolve|register [--json] [ポート]
