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
    fn statistics_error_has_a_non_empty_message() {
        let error = StatisticsError {
            message: "Dashboard statistics are temporarily unavailable.".to_string(),
        };

        assert!(!error.message.is_empty());
    }
}
