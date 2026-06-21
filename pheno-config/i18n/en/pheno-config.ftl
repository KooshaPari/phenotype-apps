# Fluent i18n catalog for `pheno-config`.
# Format: Fluent (.ftl) — Mozilla Project Fluent.
# Reference: https://projectfluent.org/
# This file is the English baseline. Translations live in `es/pheno-config.ftl` and `ja/pheno-config.ftl`.

config-error-missing-key = Configuration key is required but was not provided.
config-error-invalid-value = The provided value is not valid.
config-error-parse-failed = Configuration file could not be parsed.
config-error-type-mismatch = Expected { $expected }, received { $received }.
config-error-cascade-empty = No configuration sources were registered in the cascade.
config-error-file-not-found = Configuration file not found at { $path }.
config-warn-deprecated-key = Configuration key { $key } is deprecated; use { $replacement } instead.
config-info-loaded = Loaded { $count } configuration keys from { $source }.
config-info-cascade-applied = Applied { $layers }-layer cascade (priority { $priority }).
config-help = Usage: pheno-config [--cascade <layers>] [--strict] [--json] PATH
