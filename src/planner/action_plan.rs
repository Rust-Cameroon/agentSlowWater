use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionPlan {
    RetryJob { job_id: String },
    CommentOnPR { message: String },
    SendEmail { subject: String, body: String },
    SendSlack { channel: String, message: String },
}