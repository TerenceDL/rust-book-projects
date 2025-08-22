use gui_app::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "SelectBox {}x{} ({} options)",
            self.width, self.height, self.options.len()
        );
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec!["Yes".into(), "Maybe".into(), "No".into()],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "OK".into(),
            }),
        ],
    };
    screen.run();
}

