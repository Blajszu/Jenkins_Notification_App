pub(crate) use crate::notification::send_fail_notification;
pub(crate) use crate::jenkins_api::BuildSummary;
pub(crate) fn handle_fail(builds: Vec<BuildSummary>) {
    let failed_jobs: Vec<&str> = builds
        .iter()
        .map(|build| build.name.as_str())
        .collect();

    if !failed_jobs.is_empty() {
        send_fail_notification(failed_jobs, "FAILURE");
    }
}
