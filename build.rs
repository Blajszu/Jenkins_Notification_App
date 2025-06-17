extern crate embed_resource;

fn main() {
    embed_resource::compile("tray-example.rc", embed_resource::NONE);
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
