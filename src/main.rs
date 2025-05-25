mod notification;
use notification::send_fail_notification;

fn main() {
    println!("Obecny katalog roboczy: {:?}", std::env::current_dir());
    send_fail_notification("PNR Analyzer Branches");
}