mod fail_handler;
mod notification;
mod jenkins_api;
mod builds_info_checker;
mod tray;
mod gui;

use crate::tray::{TrayMessage, create_tray};

pub(crate) use crate::fail_handler::handle_fail;
use crate::builds_info_checker::check_builds_failed;
pub(crate) use crate::gui::show;

#[tokio::main]
async fn main() {
    let _ = std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = check_builds_failed(handle_fail).await {
                eprintln!("Error in check_builds: {}", e);
            }
        });
    });

    let (_tray, rx) = match create_tray() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to create tray: {}", e);
            return;
        }
    };

    loop {
        match rx.recv() {
            Ok(TrayMessage::Quit) => {
                break;
            },
            Ok(TrayMessage::Show) => {
                show().await.expect("Failed to show GUI");
            },
            Err(e) => {
                eprintln!("Error receiving tray message: {}", e);
                break;
            }
        }
    }
}