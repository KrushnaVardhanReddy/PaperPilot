use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobProgress {
    pub job_id: String,
    pub percentage: u8,
    pub current_step: u32,
    pub total_steps: u32,
    pub status_message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_progress_creation() {
        let progress = JobProgress {
            job_id: "job-123".to_string(),
            percentage: 50,
            current_step: 1,
            total_steps: 2,
            status_message: "Processing".to_string(),
        };

        assert_eq!(progress.job_id, "job-123");
        assert_eq!(progress.percentage, 50);
        assert_eq!(progress.current_step, 1);
        assert_eq!(progress.total_steps, 2);
        assert_eq!(progress.status_message, "Processing");
    }
}
