# Fluent i18n catalog for `pheno-config` — Spanish (es).
# Generated from the English baseline at `en/pheno-config.ftl`.

config-error-missing-key = La clave de configuración es obligatoria pero no se proporcionó.
config-error-invalid-value = El valor proporcionado no es válido.
config-error-parse-failed = No se pudo analizar el archivo de configuración.
config-error-type-mismatch = Se esperaba { $expected }, se recibió { $received }.
config-error-cascade-empty = No se registraron orígenes de configuración en la cascada.
config-error-file-not-found = Archivo de configuración no encontrado en { $path }.
config-warn-deprecated-key = La clave de configuración { $key } está obsoleta; use { $replacement } en su lugar.
config-info-loaded = Se cargaron { $count } claves de configuración desde { $source }.
config-info-cascade-applied = Se aplicó la cascada de { $layers } capas (prioridad { $priority }).
config-help = Uso: pheno-config [--cascade <capas>] [--strict] [--json] RUTA
