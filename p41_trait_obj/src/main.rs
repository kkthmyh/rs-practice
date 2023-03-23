use std::str;

use p41_trait_obj::Botton;
use p41_trait_obj::Draw;
use p41_trait_obj::Screen;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // todo
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Botton {
                width: 10,
                height: 5,
                label: "submit".to_string(),
            }),
            Box::new(SelectBox {
                width: 10,
                height: 20,
                options: vec!["yes".to_string(), "no".to_string()],
            }),
        ],
    };

    screen.run();
}
