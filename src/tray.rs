use tray_item::{IconSource, TrayItem};
use std::sync::mpsc;

pub enum TrayMessage {
    Quit,
    Show,
}

pub fn create_tray() -> Result<(TrayItem, mpsc::Receiver<TrayMessage>), Box<dyn std::error::Error>> {
    let mut tray = TrayItem::new("JNotify", IconSource::Resource("bell"))
        .map_err(|e| format!("Failed to create tray icon: {}", e))?;

    tray.add_label("Jenkins Notification App")
        .map_err(|e| format!("Failed to add label: {}", e))?;

    let (tx, rx) = mpsc::sync_channel(1);

    let show_tx = tx.clone();
    tray.add_menu_item("Show", move || {
        show_tx.send(TrayMessage::Show).unwrap();
    }).map_err(|e| format!("Failed to add show menu: {}", e))?;

    tray.inner_mut().add_separator()
        .map_err(|e| format!("Failed to add separator: {}", e))?;

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(TrayMessage::Quit).unwrap();
    }).map_err(|e| format!("Failed to add quit menu: {}", e))?;

    Ok((tray, rx))
}