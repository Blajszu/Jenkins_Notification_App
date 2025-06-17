use notify_rust::Notification;
use std::path::Path;

pub fn send_fail_notification(job_names: Vec<&str>, job_status: &str) {
    let summary = "JENKINS BUILD FAILED!".to_string();
    let body = if job_names.len() == 1 {
        format!("Job {} ended with status: {}", job_names[0], job_status)
    } else {
        let joined_jobs = job_names.join(", ");
        format!("Jobs {} ended with status: {}", joined_jobs, job_status)
    };

    let abs_path = Path::new("assets\\notification_image.png")
    .canonicalize()
    .expect("Plik nie istnieje")
    .to_str()
    .unwrap()
    .replace("\\\\?\\", "");

    Notification::new()
    .summary(&summary)
    .body(&body)
    .image_path(&abs_path)
    .show()
    .unwrap();
}