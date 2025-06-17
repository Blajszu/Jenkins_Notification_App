pub(crate) use crate::notification::send_fail_notification;
pub(crate) use crate::jenkins_api::BuildSummary;
pub(crate) fn change_handler(builds: Vec<BuildSummary>) {
    println!{"test"};
    let failed_jobs: Vec<&str> = builds
        .iter()
        .filter(|build| build.status == "FAILURE")
        .map(|build| build.name.as_str())
        .collect();

    if !failed_jobs.is_empty() {
        send_fail_notification(failed_jobs, "FAILURE");
    }
}
