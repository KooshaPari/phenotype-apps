# Fluent i18n catalog for `pheno-port-adapter`.
# Format: Fluent (.ftl) — Mozilla Project Fluent.
# Reference: https://projectfluent.org/
# Hexagonal architecture: ports (traits) + adapters (impls).

port-error-not-implemented = Port { $port } has no registered adapter.
port-error-adapter-missing = Adapter { $adapter } could not be resolved for port { $port }.
port-error-incompatible-version = Adapter { $adapter } v{ $have } is incompatible with port v{ $want }.
port-error-timeout = Port { $port } timed out after { $ms } ms.
port-error-connection-refused = Connection refused by { $endpoint } (port { $port }).
adapter-error-init-failed = Adapter { $adapter } failed to initialize: { $reason }.
port-info-registered = Registered adapter { $adapter } for port { $port }.
port-info-resolved = Resolved port { $port } → { $adapter } in { $us } µs.
port-warn-deprecated-port = Port { $port } is deprecated; migrate to { $replacement } by { $deadline }.
port-help = Usage: pheno-port-adapter list|resolve|register [--json] [PORT]
