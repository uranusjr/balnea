extern crate balnea;

fn main() {
    let app = balnea::App::new();

    let win = app.create_window("Demo App: Empty Window");
    win.show();

    app.main_loop();
}
