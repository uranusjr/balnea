# Balnea

OS native cross-platform GUI toolkit for Rust.

## Examples

### A toy

This creates a window with a button inside. Clicking the button prints `Hello!` to the console.

```rust
extern crate balnea;

fn main() {
    let app = balnea::App("First app", "com.uranusjr.helloworld");

    let bx = balnea::Box::new();
    let button = balnea_button!("Hello world", |_| { println!("Hello!"); });
    button.style().set_paddings(50);
    bx.add(button);

    let win = app.create_window(bx);
    win.show();

    app.main_loop();
}
```

## Design

Each visual components is called a widget. A box is a special widget that can contain other widgets. Call `Box::add` to add widgets into it.

Any widget can be passed to `App::create_window` to be rendered as a window. This associates the widget, and all child widgets it holds with the application.

Each widget contains a style member that can be used to control how it is layed out in its parent in a flexbox-like logic.


## Ideas

* Adopt ideas from [Toga].
* Use [Yoga] internally for layout?

[Toga]: https://pybee.org/toga/
[Yoga]: https://github.com/facebook/yoga
