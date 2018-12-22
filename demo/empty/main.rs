extern crate balnea;

fn main() {
    balnea::run(|app| {
        let win = app.create_window("Demo App: Empty Window");
        win.show();
    });
}
