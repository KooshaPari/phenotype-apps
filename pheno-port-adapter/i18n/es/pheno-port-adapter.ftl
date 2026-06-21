# Fluent i18n catalog for `pheno-port-adapter` — Spanish (es).
# Generated from the English baseline at `en/pheno-port-adapter.ftl`.

port-error-not-implemented = El puerto { $port } no tiene un adaptador registrado.
port-error-adapter-missing = No se pudo resolver el adaptador { $adapter } para el puerto { $port }.
port-error-incompatible-version = El adaptador { $adapter } v{ $have } es incompatible con el puerto v{ $want }.
port-error-timeout = El puerto { $port } agotó el tiempo de espera después de { $ms } ms.
port-error-connection-refused = Conexión rechazada por { $endpoint } (puerto { $port }).
adapter-error-init-failed = El adaptador { $adapter } no pudo inicializarse: { $reason }.
port-info-registered = Adaptador { $adapter } registrado para el puerto { $port }.
port-info-resolved = Puerto { $port } → { $adapter } resuelto en { $us } µs.
port-warn-deprecated-port = El puerto { $port } está obsoleto; migre a { $replacement } antes de { $deadline }.
port-help = Uso: pheno-port-adapter list|resolve|register [--json] [PUERTO]
