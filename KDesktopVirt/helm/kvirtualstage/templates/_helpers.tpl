{{/*
Expand the name of the chart.
*/}}
{{- define "kvirtualstage.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "kvirtualstage.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "kvirtualstage.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "kvirtualstage.labels" -}}
helm.sh/chart: {{ include "kvirtualstage.chart" . }}
{{ include "kvirtualstage.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "kvirtualstage.selectorLabels" -}}
app.kubernetes.io/name: {{ include "kvirtualstage.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/component: application
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "kvirtualstage.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "kvirtualstage.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Redis fullname
*/}}
{{- define "kvirtualstage.redis.fullname" -}}
{{- if .Values.redis.enabled }}
{{- printf "%s-redis" (include "kvirtualstage.fullname" .) }}
{{- else }}
{{- .Values.config.redis.url | replace "redis://" "" | replace ":6379" "" }}
{{- end }}
{{- end }}

{{/*
Redis secret name
*/}}
{{- define "kvirtualstage.redis.secretName" -}}
{{- if .Values.redis.enabled }}
{{- printf "%s-redis" (include "kvirtualstage.fullname" .) }}
{{- else }}
{{- printf "%s-redis-secret" (include "kvirtualstage.fullname" .) }}
{{- end }}
{{- end }}

{{/*
Redis secret password key
*/}}
{{- define "kvirtualstage.redis.secretPasswordKey" -}}
{{- if .Values.redis.enabled }}
redis-password
{{- else }}
password
{{- end }}
{{- end }}

{{/*
Create the Redis URL
*/}}
{{- define "kvirtualstage.redis.url" -}}
{{- if .Values.redis.enabled }}
{{- if .Values.redis.auth.enabled }}
redis://:{{ .Values.redis.auth.password }}@{{ include "kvirtualstage.redis.fullname" . }}:6379
{{- else }}
redis://{{ include "kvirtualstage.redis.fullname" . }}:6379
{{- end }}
{{- else }}
{{- .Values.config.redis.url }}
{{- end }}
{{- end }}

{{/*
Create prometheus rules labels
*/}}
{{- define "kvirtualstage.prometheus.ruleLabels" -}}
{{- if .Values.monitoring.prometheus.rules.labels }}
{{- toYaml .Values.monitoring.prometheus.rules.labels }}
{{- else }}
app: {{ include "kvirtualstage.name" . }}
release: {{ .Release.Name }}
{{- end }}
{{- end }}

{{/*
Create service monitor labels
*/}}
{{- define "kvirtualstage.serviceMonitor.labels" -}}
{{- if .Values.monitoring.prometheus.serviceMonitor.labels }}
{{- toYaml .Values.monitoring.prometheus.serviceMonitor.labels }}
{{- else }}
app: {{ include "kvirtualstage.name" . }}
release: {{ .Release.Name }}
{{- end }}
{{- end }}

{{/*
Create network policy labels
*/}}
{{- define "kvirtualstage.networkPolicy.labels" -}}
app.kubernetes.io/name: {{ include "kvirtualstage.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/component: network-policy
{{- end }}

{{/*
Validate configuration
*/}}
{{- define "kvirtualstage.validateConfig" -}}
{{- if and .Values.kvirtualstage.autoscaling.enabled (not .Values.monitoring.prometheus.enabled) }}
{{- fail "Autoscaling requires Prometheus monitoring to be enabled" }}
{{- end }}
{{- if and .Values.redis.enabled (not .Values.redis.auth.enabled) }}
{{- fail "Redis authentication must be enabled for production deployment" }}
{{- end }}
{{- if and .Values.kvirtualstage.ingress.enabled (not .Values.kvirtualstage.ingress.tls) }}
{{- fail "TLS must be enabled for ingress in production" }}
{{- end }}
{{- end }}

{{/*
Generate TLS certificate
*/}}
{{- define "kvirtualstage.generateCerts" -}}
{{- $ca := genCA "kvirtualstage-ca" 365 }}
{{- $cert := genSignedCert "kvirtualstage" nil (list "kvirtualstage" "kvirtualstage.default" "kvirtualstage.default.svc" "kvirtualstage.default.svc.cluster.local") 365 $ca }}
ca.crt: {{ $ca.Cert | b64enc }}
tls.crt: {{ $cert.Cert | b64enc }}
tls.key: {{ $cert.Key | b64enc }}
{{- end }}

{{/*
Storage class
*/}}
{{- define "kvirtualstage.storageClass" -}}
{{- if .Values.global.storageClass }}
{{- .Values.global.storageClass }}
{{- else }}
{{- .Values.storageClasses.fastSsd.name }}
{{- end }}
{{- end }}

{{/*
Image pull policy
*/}}
{{- define "kvirtualstage.imagePullPolicy" -}}
{{- if .Values.development.enabled }}
Always
{{- else }}
{{- .Values.kvirtualstage.image.pullPolicy }}
{{- end }}
{{- end }}

{{/*
Log level
*/}}
{{- define "kvirtualstage.logLevel" -}}
{{- if .Values.development.debugMode }}
debug
{{- else }}
{{- .Values.config.logging.level }}
{{- end }}
{{- end }}

{{/*
Environment specific annotations
*/}}
{{- define "kvirtualstage.annotations" -}}
{{- if .Values.development.enabled }}
kvirtualstage.dev/environment: "development"
{{- else if .Values.testing.enabled }}
kvirtualstage.dev/environment: "testing"
{{- else }}
kvirtualstage.dev/environment: "production"
{{- end }}
deployment.kubernetes.io/revision: {{ .Release.Revision | quote }}
{{- end }}

{{/*
Resource limits for development
*/}}
{{- define "kvirtualstage.resources" -}}
{{- if .Values.development.enabled }}
requests:
  cpu: 100m
  memory: 256Mi
limits:
  cpu: 500m
  memory: 1Gi
{{- else }}
{{- toYaml .Values.kvirtualstage.resources }}
{{- end }}
{{- end }}

{{/*
Replicas based on environment
*/}}
{{- define "kvirtualstage.replicaCount" -}}
{{- if .Values.development.enabled }}
1
{{- else if .Values.testing.enabled }}
2
{{- else }}
{{- .Values.kvirtualstage.replicaCount }}
{{- end }}
{{- end }}

{{/*
Create enterprise features configuration
*/}}
{{- define "kvirtualstage.enterpriseConfig" -}}
{{- if .Values.enterprise.multiTenant.enabled }}
multiTenant:
  enabled: true
{{- end }}
{{- if .Values.enterprise.sso.enabled }}
sso:
  enabled: true
  provider: {{ .Values.enterprise.sso.provider }}
  clientId: {{ .Values.enterprise.sso.clientId }}
{{- end }}
{{- if .Values.enterprise.audit.enabled }}
audit:
  enabled: true
  retention: {{ .Values.enterprise.audit.retention }}
{{- end }}
{{- end }}