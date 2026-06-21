# Fluent i18n catalog for `pheno-config` — Japanese (ja).
# Generated from the English baseline at `en/pheno-config.ftl`.

config-error-missing-key = 設定キーは必須ですが、指定されていません。
config-error-invalid-value = 指定された値が無効です。
config-error-parse-failed = 設定ファイルを解析できませんでした。
config-error-type-mismatch = { $expected } が必要ですが、{ $received } が指定されました。
config-error-cascade-empty = カスケードに設定ソースが登録されていません。
config-error-file-not-found = 設定ファイルが { $path } に見つかりません。
config-warn-deprecated-key = 設定キー { $key } は非推奨です。代わりに { $replacement } を使用してください。
config-info-loaded = { $source } から { $count } 件の設定キーを読み込みました。
config-info-cascade-applied = { $layers } 層のカスケードを適用しました (優先度 { $priority })。
config-help = 使用法: pheno-config [--cascade <レイヤー>] [--strict] [--json] パス
