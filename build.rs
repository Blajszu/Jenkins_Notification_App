extern crate embed_resource;

fn main() {
    let _ = embed_resource::compile("resources.rc", embed_resource::NONE);
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
