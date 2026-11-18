use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DashboardStatistics {
    pub total_registered_users: i64,
    pub active_study_date_requests: i64,
    pub pending_reports: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct StatisticsError {
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct HealthComponent {
    pub healthy: bool,
    pub checked_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct HealthStatus {
    pub server: HealthComponent,
    pub database: HealthComponent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dashboard_statistics_contains_all_metrics() {
        let statistics = DashboardStatistics {
            total_registered_users: 12,
            active_study_date_requests: 3,
            pending_reports: 2,
        };

        assert_eq!(statistics.total_registered_users, 12);
        assert_eq!(statistics.active_study_date_requests, 3);
        assert_eq!(statistics.pending_reports, 2);
    }

    #[test]
    fn us14_t03_statistics_error_has_a_recovery_message() {
        let error = StatisticsError {
            message: "Dashboard statistics are temporarily unavailable.".to_string(),
        };

        assert_eq!(
            error.message,
            "Dashboard statistics are temporarily unavailable."
        );
    }

    #[test]
    fn us13_t02_health_status_reports_database_failure_separately() {
        let checked_at = "2026-10-05T15:00:00Z".to_string();
        let status = HealthStatus {
            server: HealthComponent {
                healthy: true,
                checked_at: checked_at.clone(),
            },
            database: HealthComponent {
                healthy: false,
                checked_at,
            },
        };

        assert!(status.server.healthy);
        assert!(!status.database.healthy);
    }

    #[test]
    fn us14_t04_zero_statistics_are_valid_values() {
        let statistics = DashboardStatistics {
            total_registered_users: 0,
            active_study_date_requests: 0,
            pending_reports: 0,
        };

        assert_eq!(
            serde_json::to_value(statistics).expect("statistics should serialize"),
            serde_json::json!({
                "total_registered_users": 0,
                "active_study_date_requests": 0,
                "pending_reports": 0
            })
        );
    }
}
