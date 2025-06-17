mod notification;
mod jenkins_api;

use notification::send_fail_notification;
use jenkins_api::get_job_data;

#[tokio::main]
async fn main() {
    // println!("Obecny katalog roboczy: {:?}", std::env::current_dir());
    // send_fail_notification("PNR Analyzer Branches", "FAILURE");
    
    match get_job_data("MockJob").await {
        Ok(summary) => {
            println!("Build zakończony: {}", summary.status);
            println!("Czas trwania: {} sekund", summary.duration);
            println!("Start: {}", summary.start_time);
        }
        Err(e) => {
            eprintln!("Błąd pobierania danych z Jenkinsa: {}", e);
        }
    }

}