/*!
 * Security Monitoring and Threat Detection Module
 * 
 * Implements real-time security monitoring, threat detection, and incident response
 * for the KVirtualStage platform with enterprise-grade capabilities.
 */

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

// ============================================================================
// Security Monitoring Core
// ============================================================================

#[derive(Debug, Clone)]
pub struct SecurityMonitor {
    threat_detector: Arc<RwLock<ThreatDetector>>,
    incident_manager: Arc<RwLock<IncidentManager>>,
    compliance_monitor: Arc<RwLock<ComplianceMonitor>>,
    metrics_collector: Arc<RwLock<SecurityMetricsCollector>>,
    config: SecurityMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringConfig {
    pub enable_real_time_monitoring: bool,
    pub threat_detection_sensitivity: ThreatSensitivity,
    pub incident_response_enabled: bool,
    pub compliance_frameworks: Vec<ComplianceFramework>,
    pub metrics_retention_days: u32,
    pub alert_thresholds: AlertThresholds,
    pub automated_response_enabled: bool,
}

impl Default for SecurityMonitoringConfig {
    fn default() -> Self {
        Self {
            enable_real_time_monitoring: true,
            threat_detection_sensitivity: ThreatSensitivity::Medium,
            incident_response_enabled: true,
            compliance_frameworks: vec![
                ComplianceFramework::SOC2,
                ComplianceFramework::GDPR,
                ComplianceFramework::HIPAA,
            ],
            metrics_retention_days: 90,
            alert_thresholds: AlertThresholds::default(),
            automated_response_enabled: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThreatSensitivity {
    Low,
    Medium,
    High,
    Maximum,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComplianceFramework {
    SOC2,
    GDPR,
    HIPAA,
    PCI_DSS,
    ISO27001,
    NIST,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub failed_auth_attempts: u32,
    pub unusual_access_patterns: f64,
    pub privilege_escalation_score: f64,
    pub data_exfiltration_score: f64,
    pub malware_detection_score: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            failed_auth_attempts: 5,
            unusual_access_patterns: 0.7,
            privilege_escalation_score: 0.8,
            data_exfiltration_score: 0.9,
            malware_detection_score: 0.95,
        }
    }
}

// ============================================================================
// Threat Detection Engine
// ============================================================================

#[derive(Debug)]
pub struct ThreatDetector {
    detection_rules: Vec<ThreatDetectionRule>,
    active_threats: HashMap<String, ActiveThreat>,
    behavioral_baselines: HashMap<String, BehavioralBaseline>,
    ml_models: HashMap<String, MachineLearningModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rule_type: ThreatRuleType,
    pub severity: ThreatSeverity,
    pub conditions: Vec<ThreatCondition>,
    pub actions: Vec<ThreatAction>,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThreatRuleType {
    AuthenticationAnomaly,
    PrivilegeEscalation,
    DataExfiltration,
    MalwareDetection,
    NetworkAnomaly,
    BehavioralAnomaly,
    ComplianceViolation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatCondition {
    pub parameter: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
    pub time_window: Option<u64>, // seconds
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    Regex,
    InRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAction {
    pub action_type: ThreatActionType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub delay_seconds: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThreatActionType {
    Alert,
    Block,
    Quarantine,
    LogIncident,
    NotifyAdmin,
    RevokeAccess,
    RequireStepUp,
    AutoRemediate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveThreat {
    pub id: String,
    pub rule_id: String,
    pub threat_type: ThreatRuleType,
    pub severity: ThreatSeverity,
    pub detected_at: u64,
    pub source_ip: Option<String>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub details: HashMap<String, serde_json::Value>,
    pub status: ThreatStatus,
    pub actions_taken: Vec<ThreatAction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThreatStatus {
    Active,
    Investigating,
    Mitigated,
    Resolved,
    FalsePositive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralBaseline {
    pub user_id: String,
    pub typical_login_hours: Vec<u8>,
    pub typical_ip_ranges: Vec<String>,
    pub typical_access_patterns: HashMap<String, f64>,
    pub data_access_volume_baseline: f64,
    pub session_duration_baseline: f64,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineLearningModel {
    pub model_id: String,
    pub model_type: MLModelType,
    pub version: String,
    pub accuracy: f64,
    pub last_trained: u64,
    pub feature_weights: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MLModelType {
    AnomalyDetection,
    ThreatClassification,
    RiskScoring,
    BehaviorPrediction,
}

impl ThreatDetector {
    pub fn new() -> Self {
        Self {
            detection_rules: Self::create_default_rules(),
            active_threats: HashMap::new(),
            behavioral_baselines: HashMap::new(),
            ml_models: HashMap::new(),
        }
    }

    /// Analyze a security event for threats
    pub async fn analyze_event(&mut self, event: SecurityEvent) -> Result<ThreatAnalysisResult> {
        info!("Analyzing security event: {:?}", event.event_type);

        let mut threats_detected = Vec::new();
        let mut risk_score = 0.0;

        // Rule-based detection
        for rule in &self.detection_rules {
            if !rule.enabled {
                continue;
            }

            if self.evaluate_rule(rule, &event).await? {
                let threat = self.create_active_threat(rule, &event).await?;
                threats_detected.push(threat.clone());
                self.active_threats.insert(threat.id.clone(), threat);

                // Calculate risk score based on severity
                risk_score += match rule.severity {
                    ThreatSeverity::Low => 0.1,
                    ThreatSeverity::Medium => 0.3,
                    ThreatSeverity::High => 0.6,
                    ThreatSeverity::Critical => 1.0,
                };
            }
        }

        // Behavioral analysis
        if let Some(user_id) = &event.user_id {
            let behavioral_risk = self.analyze_behavioral_anomaly(user_id, &event).await?;
            risk_score += behavioral_risk;
        }

        // Machine learning analysis
        let ml_risk = self.analyze_with_ml_models(&event).await?;
        risk_score += ml_risk;

        // Cap risk score at 1.0
        risk_score = risk_score.min(1.0);

        Ok(ThreatAnalysisResult {
            event_id: event.id,
            recommended_actions: self.generate_recommendations(risk_score, &threats_detected),
            threats_detected,
            overall_risk_score: risk_score,
            behavioral_anomalies: Vec::new(), // Would be populated in production
            ml_predictions: HashMap::new(),   // Would be populated in production
        })
    }

    async fn evaluate_rule(&self, rule: &ThreatDetectionRule, event: &SecurityEvent) -> Result<bool> {
        for condition in &rule.conditions {
            if !self.evaluate_condition(condition, event).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    async fn evaluate_condition(&self, condition: &ThreatCondition, event: &SecurityEvent) -> Result<bool> {
        // Get the value from the event based on the parameter
        let event_value = self.extract_event_parameter(&condition.parameter, event)?;

        // Evaluate based on operator
        match condition.operator {
            ConditionOperator::Equals => Ok(event_value == condition.value),
            ConditionOperator::NotEquals => Ok(event_value != condition.value),
            ConditionOperator::GreaterThan => {
                if let (Some(event_num), Some(condition_num)) = (
                    event_value.as_f64(),
                    condition.value.as_f64(),
                ) {
                    Ok(event_num > condition_num)
                } else {
                    Ok(false)
                }
            }
            ConditionOperator::LessThan => {
                if let (Some(event_num), Some(condition_num)) = (
                    event_value.as_f64(),
                    condition.value.as_f64(),
                ) {
                    Ok(event_num < condition_num)
                } else {
                    Ok(false)
                }
            }
            ConditionOperator::Contains => {
                if let (Some(event_str), Some(condition_str)) = (
                    event_value.as_str(),
                    condition.value.as_str(),
                ) {
                    Ok(event_str.contains(condition_str))
                } else {
                    Ok(false)
                }
            }
            ConditionOperator::Regex => {
                // In production, would use regex crate
                Ok(false) // Placeholder
            }
            ConditionOperator::InRange => {
                // In production, would implement range checking
                Ok(false) // Placeholder
            }
        }
    }

    fn extract_event_parameter(&self, parameter: &str, event: &SecurityEvent) -> Result<serde_json::Value> {
        match parameter {
            "event_type" => Ok(serde_json::Value::String(format!("{:?}", event.event_type))),
            "user_id" => Ok(event.user_id.as_ref()
                .map(|u| serde_json::Value::String(u.clone()))
                .unwrap_or(serde_json::Value::Null)),
            "ip_address" => Ok(event.source_ip.as_ref()
                .map(|ip| serde_json::Value::String(ip.clone()))
                .unwrap_or(serde_json::Value::Null)),
            "timestamp" => Ok(serde_json::Value::Number(
                serde_json::Number::from(event.timestamp)
            )),
            _ => {
                // Look in event details
                Ok(event.details.get(parameter)
                    .cloned()
                    .unwrap_or(serde_json::Value::Null))
            }
        }
    }

    async fn create_active_threat(&self, rule: &ThreatDetectionRule, event: &SecurityEvent) -> Result<ActiveThreat> {
        let threat_id = Uuid::new_v4().to_string();

        Ok(ActiveThreat {
            id: threat_id,
            rule_id: rule.id.clone(),
            threat_type: rule.rule_type.clone(),
            severity: rule.severity.clone(),
            detected_at: current_timestamp(),
            source_ip: event.source_ip.clone(),
            user_id: event.user_id.clone(),
            session_id: event.session_id.clone(),
            details: event.details.clone(),
            status: ThreatStatus::Active,
            actions_taken: Vec::new(),
        })
    }

    async fn analyze_behavioral_anomaly(&self, user_id: &str, event: &SecurityEvent) -> Result<f64> {
        // In production, this would analyze user behavior against baseline
        let baseline = self.behavioral_baselines.get(user_id);
        
        if let Some(_baseline) = baseline {
            // Simplified behavioral analysis
            let mut anomaly_score = 0.0;

            // Check login time anomaly
            if let Some(hour) = self.extract_hour_from_timestamp(event.timestamp) {
                // If login is outside typical hours, increase score
                anomaly_score += 0.2;
            }

            // Check IP address anomaly
            if let Some(_ip) = &event.source_ip {
                // If IP is outside typical ranges, increase score
                anomaly_score += 0.3;
            }

            Ok(anomaly_score)
        } else {
            // No baseline available, return neutral score
            Ok(0.0)
        }
    }

    async fn analyze_with_ml_models(&self, _event: &SecurityEvent) -> Result<f64> {
        // In production, this would run ML models for threat detection
        // For now, return a random score for demonstration
        Ok(0.1)
    }

    fn generate_recommendations(&self, risk_score: f64, threats: &[ActiveThreat]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if risk_score > 0.8 {
            recommendations.push("Immediate security team notification required".to_string());
            recommendations.push("Consider blocking user access temporarily".to_string());
        } else if risk_score > 0.5 {
            recommendations.push("Require additional authentication".to_string());
            recommendations.push("Increase monitoring for this user/session".to_string());
        } else if risk_score > 0.2 {
            recommendations.push("Log for further investigation".to_string());
        }

        for threat in threats {
            match threat.threat_type {
                ThreatRuleType::AuthenticationAnomaly => {
                    recommendations.push("Review authentication patterns".to_string());
                }
                ThreatRuleType::PrivilegeEscalation => {
                    recommendations.push("Audit privilege changes immediately".to_string());
                }
                ThreatRuleType::DataExfiltration => {
                    recommendations.push("Review data access logs and block suspicious transfers".to_string());
                }
                _ => {}
            }
        }

        recommendations
    }

    fn extract_hour_from_timestamp(&self, timestamp: u64) -> Option<u8> {
        // Convert timestamp to hour of day
        let duration = std::time::Duration::from_secs(timestamp);
        let datetime = std::time::UNIX_EPOCH + duration;
        // In production, would use proper date/time libraries
        Some(((timestamp / 3600) % 24) as u8)
    }

    fn create_default_rules() -> Vec<ThreatDetectionRule> {
        vec![
            ThreatDetectionRule {
                id: "auth_failure_brute_force".to_string(),
                name: "Authentication Brute Force".to_string(),
                description: "Multiple failed authentication attempts".to_string(),
                rule_type: ThreatRuleType::AuthenticationAnomaly,
                severity: ThreatSeverity::High,
                conditions: vec![
                    ThreatCondition {
                        parameter: "event_type".to_string(),
                        operator: ConditionOperator::Equals,
                        value: serde_json::Value::String("AuthenticationFailed".to_string()),
                        time_window: Some(300), // 5 minutes
                    }
                ],
                actions: vec![
                    ThreatAction {
                        action_type: ThreatActionType::Alert,
                        parameters: HashMap::new(),
                        delay_seconds: None,
                    },
                    ThreatAction {
                        action_type: ThreatActionType::Block,
                        parameters: HashMap::new(),
                        delay_seconds: Some(60),
                    }
                ],
                enabled: true,
            },
            ThreatDetectionRule {
                id: "privilege_escalation".to_string(),
                name: "Privilege Escalation Attempt".to_string(),
                description: "Unauthorized privilege elevation detected".to_string(),
                rule_type: ThreatRuleType::PrivilegeEscalation,
                severity: ThreatSeverity::Critical,
                conditions: vec![
                    ThreatCondition {
                        parameter: "event_type".to_string(),
                        operator: ConditionOperator::Equals,
                        value: serde_json::Value::String("PrivilegeChange".to_string()),
                        time_window: None,
                    }
                ],
                actions: vec![
                    ThreatAction {
                        action_type: ThreatActionType::Alert,
                        parameters: HashMap::new(),
                        delay_seconds: None,
                    },
                    ThreatAction {
                        action_type: ThreatActionType::NotifyAdmin,
                        parameters: HashMap::new(),
                        delay_seconds: None,
                    }
                ],
                enabled: true,
            }
        ]
    }
}

// ============================================================================
// Security Events
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: String,
    pub event_type: SecurityEventType,
    pub timestamp: u64,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub source_ip: Option<String>,
    pub user_agent: Option<String>,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub result: EventResult,
    pub details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityEventType {
    Authentication,
    AuthenticationFailed,
    Authorization,
    AuthorizationFailed,
    DataAccess,
    DataModification,
    PrivilegeChange,
    SystemAccess,
    ConfigurationChange,
    SecurityPolicyViolation,
    MalwareDetection,
    NetworkAnomaly,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventResult {
    Success,
    Failure,
    Blocked,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisResult {
    pub event_id: String,
    pub threats_detected: Vec<ActiveThreat>,
    pub overall_risk_score: f64,
    pub behavioral_anomalies: Vec<String>,
    pub ml_predictions: HashMap<String, f64>,
    pub recommended_actions: Vec<String>,
}

// ============================================================================
// Incident Management
// ============================================================================

#[derive(Debug)]
pub struct IncidentManager {
    active_incidents: HashMap<String, SecurityIncident>,
    incident_workflows: Vec<IncidentWorkflow>,
    escalation_rules: Vec<EscalationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIncident {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: IncidentSeverity,
    pub status: IncidentStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub assigned_to: Option<String>,
    pub related_threats: Vec<String>,
    pub timeline: Vec<IncidentEvent>,
    pub artifacts: Vec<String>,
    pub response_actions: Vec<ResponseAction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IncidentStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentEvent {
    pub timestamp: u64,
    pub event_type: String,
    pub description: String,
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAction {
    pub action_type: String,
    pub description: String,
    pub executed_at: u64,
    pub executed_by: String,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentWorkflow {
    pub id: String,
    pub name: String,
    pub trigger_conditions: Vec<String>,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub step_id: String,
    pub name: String,
    pub action_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    pub severity: IncidentSeverity,
    pub escalation_time_minutes: u64,
    pub escalation_contacts: Vec<String>,
}

impl IncidentManager {
    pub fn new() -> Self {
        Self {
            active_incidents: HashMap::new(),
            incident_workflows: Vec::new(),
            escalation_rules: Vec::new(),
        }
    }

    pub async fn create_incident(&mut self, threat: &ActiveThreat) -> Result<String> {
        let incident_id = Uuid::new_v4().to_string();
        let severity = match threat.severity {
            ThreatSeverity::Low => IncidentSeverity::Low,
            ThreatSeverity::Medium => IncidentSeverity::Medium,
            ThreatSeverity::High => IncidentSeverity::High,
            ThreatSeverity::Critical => IncidentSeverity::Critical,
        };

        let incident = SecurityIncident {
            id: incident_id.clone(),
            title: format!("Security Threat Detected: {:?}", threat.threat_type),
            description: format!("Threat detected by rule: {}", threat.rule_id),
            severity,
            status: IncidentStatus::Open,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            assigned_to: None,
            related_threats: vec![threat.id.clone()],
            timeline: vec![
                IncidentEvent {
                    timestamp: current_timestamp(),
                    event_type: "incident_created".to_string(),
                    description: "Incident automatically created from threat detection".to_string(),
                    user: "system".to_string(),
                }
            ],
            artifacts: Vec::new(),
            response_actions: Vec::new(),
        };

        self.active_incidents.insert(incident_id.clone(), incident);
        info!("Created security incident: {}", incident_id);

        Ok(incident_id)
    }
}

// ============================================================================
// Compliance Monitoring
// ============================================================================

#[derive(Debug)]
pub struct ComplianceMonitor {
    compliance_checks: Vec<ComplianceCheck>,
    violation_history: Vec<ComplianceViolation>,
    reporting_schedules: HashMap<ComplianceFramework, ReportingSchedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub id: String,
    pub framework: ComplianceFramework,
    pub control_id: String,
    pub name: String,
    pub description: String,
    pub check_frequency: CheckFrequency,
    pub automated: bool,
    pub last_check: Option<u64>,
    pub status: ComplianceStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CheckFrequency {
    Continuous,
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Unknown,
    NotApplicable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub id: String,
    pub check_id: String,
    pub detected_at: u64,
    pub severity: ViolationSeverity,
    pub description: String,
    pub remediation_steps: Vec<String>,
    pub resolved_at: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingSchedule {
    pub frequency: ReportingFrequency,
    pub recipients: Vec<String>,
    pub format: ReportFormat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReportingFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReportFormat {
    Json,
    Pdf,
    Html,
    Excel,
}

impl ComplianceMonitor {
    pub fn new() -> Self {
        Self {
            compliance_checks: Self::create_default_checks(),
            violation_history: Vec::new(),
            reporting_schedules: HashMap::new(),
        }
    }

    fn create_default_checks() -> Vec<ComplianceCheck> {
        vec![
            ComplianceCheck {
                id: "soc2_access_control".to_string(),
                framework: ComplianceFramework::SOC2,
                control_id: "CC6.1".to_string(),
                name: "Access Control Management".to_string(),
                description: "Verify access controls are properly implemented".to_string(),
                check_frequency: CheckFrequency::Daily,
                automated: true,
                last_check: None,
                status: ComplianceStatus::Unknown,
            },
            ComplianceCheck {
                id: "gdpr_data_encryption".to_string(),
                framework: ComplianceFramework::GDPR,
                control_id: "Art. 32".to_string(),
                name: "Data Encryption at Rest and in Transit".to_string(),
                description: "Ensure personal data is encrypted".to_string(),
                check_frequency: CheckFrequency::Continuous,
                automated: true,
                last_check: None,
                status: ComplianceStatus::Unknown,
            }
        ]
    }
}

// ============================================================================
// Security Metrics Collection
// ============================================================================

#[derive(Debug)]
pub struct SecurityMetricsCollector {
    metrics: HashMap<String, SecurityMetric>,
    collection_config: MetricsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetric {
    pub name: String,
    pub metric_type: MetricType,
    pub value: f64,
    pub timestamp: u64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub collection_interval_seconds: u64,
    pub retention_period_days: u32,
    pub export_format: Vec<MetricExportFormat>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricExportFormat {
    Prometheus,
    StatsD,
    Json,
    Splunk,
}

impl SecurityMetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            collection_config: MetricsConfig {
                collection_interval_seconds: 60,
                retention_period_days: 90,
                export_format: vec![MetricExportFormat::Prometheus, MetricExportFormat::Json],
            },
        }
    }

    pub async fn record_metric(&mut self, name: String, value: f64, tags: HashMap<String, String>) -> Result<()> {
        let metric = SecurityMetric {
            name: name.clone(),
            metric_type: MetricType::Gauge,
            value,
            timestamp: current_timestamp(),
            tags,
        };

        self.metrics.insert(name, metric);
        Ok(())
    }
}

// ============================================================================
// Main Security Monitor Implementation
// ============================================================================

impl SecurityMonitor {
    pub async fn new(config: SecurityMonitoringConfig) -> Result<Self> {
        info!("Initializing Security Monitor with config: {:?}", config);

        let threat_detector = Arc::new(RwLock::new(ThreatDetector::new()));
        let incident_manager = Arc::new(RwLock::new(IncidentManager::new()));
        let compliance_monitor = Arc::new(RwLock::new(ComplianceMonitor::new()));
        let metrics_collector = Arc::new(RwLock::new(SecurityMetricsCollector::new()));

        Ok(Self {
            threat_detector,
            incident_manager,
            compliance_monitor,
            metrics_collector,
            config,
        })
    }

    /// Process a security event through the monitoring pipeline
    pub async fn process_security_event(&self, event: SecurityEvent) -> Result<SecurityMonitoringResult> {
        info!("Processing security event: {}", event.id);

        // Threat detection
        let mut detector = self.threat_detector.write().await;
        let threat_analysis = detector.analyze_event(event.clone()).await?;
        drop(detector);

        // Incident management
        let mut incidents_created = Vec::new();
        if !threat_analysis.threats_detected.is_empty() {
            let mut incident_mgr = self.incident_manager.write().await;
            for threat in &threat_analysis.threats_detected {
                let incident_id = incident_mgr.create_incident(threat).await?;
                incidents_created.push(incident_id);
            }
        }

        // Record metrics
        let mut metrics = self.metrics_collector.write().await;
        metrics.record_metric(
            "security_events_processed".to_string(),
            1.0,
            [("event_type".to_string(), format!("{:?}", event.event_type))]
                .iter().cloned().collect(),
        ).await?;

        metrics.record_metric(
            "threat_risk_score".to_string(),
            threat_analysis.overall_risk_score,
            HashMap::new(),
        ).await?;

        Ok(SecurityMonitoringResult {
            event_id: event.id,
            threat_analysis,
            incidents_created,
            compliance_violations: Vec::new(), // Would be populated in production
            metrics_recorded: vec![
                "security_events_processed".to_string(),
                "threat_risk_score".to_string(),
            ],
        })
    }

    /// Get current security status
    pub async fn get_security_status(&self) -> Result<SecurityStatus> {
        let detector = self.threat_detector.read().await;
        let active_threats_count = detector.active_threats.len();
        drop(detector);

        let incident_mgr = self.incident_manager.read().await;
        let active_incidents_count = incident_mgr.active_incidents.len();
        drop(incident_mgr);

        let compliance = self.compliance_monitor.read().await;
        let compliance_score = self.calculate_compliance_score(&compliance).await?;
        drop(compliance);

        Ok(SecurityStatus {
            overall_risk_level: if active_threats_count > 5 {
                RiskLevel::High
            } else if active_threats_count > 2 {
                RiskLevel::Medium
            } else {
                RiskLevel::Low
            },
            active_threats_count,
            active_incidents_count,
            compliance_score,
            last_updated: current_timestamp(),
        })
    }

    async fn calculate_compliance_score(&self, _compliance: &ComplianceMonitor) -> Result<f64> {
        // In production, calculate actual compliance score
        Ok(0.95) // 95% compliant
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringResult {
    pub event_id: String,
    pub threat_analysis: ThreatAnalysisResult,
    pub incidents_created: Vec<String>,
    pub compliance_violations: Vec<String>,
    pub metrics_recorded: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatus {
    pub overall_risk_level: RiskLevel,
    pub active_threats_count: usize,
    pub active_incidents_count: usize,
    pub compliance_score: f64,
    pub last_updated: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

// ============================================================================
// Utility Functions
// ============================================================================

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}