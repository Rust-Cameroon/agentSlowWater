#[derive(Debug)]
pub enum Diagnosis {
    FlakyTest { test_name: String, reason: String },
    LongRuntime { job_name: String, duration: u64 },
    CacheMiss { step: String },
    InfraFailure,
    InefficientJobOrder { recommendation: String },
    ConfigurationViolation { description: String },
}