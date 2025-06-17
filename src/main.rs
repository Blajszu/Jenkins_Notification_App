use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

enum Message {
    Quit,
    Green,
    Red,
}

fn main() {
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