use std::time::Duration;
use tokio::time;
use crate::jenkins_api::{BuildSummary, get_job_data};

fn get_builds_names_from_file(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = std::env::current_dir().unwrap().join(file_path);
    let content = std::fs::read_to_string(path)?;
    let builds: Vec<String> = content.lines().map(|line| line.to_string()).collect();
    Ok(builds)
}

pub async fn get_all_builds_data() -> Result<Vec<BuildSummary>, Box<dyn std::error::Error>> {
    let builds = get_builds_names_from_file("src\\builds.txt")?;
    if builds.is_empty() {
        return Err("No builds found in the file.".into());
    }

    let mut build_summaries:Vec<BuildSummary> = Vec::new();

    for build in builds {
        match get_job_data(&build).await {
            Ok(summary) => {
                build_summaries.push(summary);
            }
            Err(e) => {
                return Err(format!("Error fetching data for build '{}': {}", build, e).into());
            }
        }
    }

    Ok(build_summaries)
}

pub async fn check_builds<F>(callback: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(Vec<BuildSummary>) + Send + 'static,
{
    let mut previous_builds = get_all_builds_data().await?;

    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        match get_all_builds_data().await {
            Ok(current_builds) => {

                if previous_builds != current_builds {
                    callback(current_builds.clone());
                    previous_builds = current_builds;
                }
            }
            Err(e) => eprintln!("Error checking builds: {}", e),
        }
    }
}