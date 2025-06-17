mod change_handler;
mod notification;
mod jenkins_api;
mod builds_info_checker;

use std::thread;
use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

pub(crate) use crate::change_handler::change_handler;
pub(crate) use crate::builds_info_checker::check_builds;
use crate::builds_info_checker::check_builds_failed;

enum Message {
    Quit,
    Green,
    Red,
}

fn main() {

    let _ = std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = check_builds_failed(change_handler).await {
                eprintln!("Error in check_builds: {}", e);
            }
        });
    });

    let mut tray = TrayItem::new(
        "JNotify",
        IconSource::Resource("bell"),
    )
        .unwrap();

    tray.add_label("Jenkins Notification App").unwrap();

    tray.add_menu_item("Show", || {
        println!("show!");
    })
        .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
        .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            _ => {}
        }
    }
}