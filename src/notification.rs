use notify_rust::Notification;
use std::path::Path;

pub fn send_fail_notification(job_name: &str) {
    let summary = "JENKINS BUILD FAILED!".to_string();
    let body = format!("Failed to build job {}", job_name);

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