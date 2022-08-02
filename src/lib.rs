mod random;
mod snake;

use js_sys::Function;
use snake::SnakeGame;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{closure, prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{console, window, HtmlDivElement, HtmlElement};

thread_local! {

    static GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(20, 20)));

    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        let game = GAME.with(|game| game.clone());
        move || {
            game.borrow_mut().tick();
            render();
        }
    }) as Box<dyn FnMut()>);
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting...".into());

    let game = Rc::new(RefCell::new(SnakeGame::new(20, 20)));

    TICK_CLOSURE.with(|tick_closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                500,
            )
            .unwrap_throw();
    });

    render();
}

pub fn render() {
    let document = window().unwrap_throw().document().unwrap_throw();

    let root_container = window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .get_element_by_id("root")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();

    root_container.set_inner_html("");

    let height = GAME.with(|game| game.borrow().height);
    let width = GAME.with(|game| game.borrow().width);

    root_container
        .style()
        .set_property("display", "inline-grid")
        .unwrap_throw();

    root_container
        .style()
        .set_property(
            "grid-template",
            &format!("repeat({}, auto) / repeat({}, auto)", height, width),
        )
        .unwrap_throw();

    root_container
        .style()
        .set_property("grid-column-gap", "30px")
        .unwrap_throw();

    root_container
        .style()
        .set_property("grid-row-gap", "5px")
        .unwrap_throw();

    for y in 0..height {
        for x in 0..width {
            let pos = (x, y);
            let field_element = document
                .create_element("div")
                .unwrap_throw()
                .dyn_into::<HtmlDivElement>()
                .unwrap_throw();

            field_element.set_inner_text({
                if pos == GAME.with(|game| game.borrow().food) {
                    "🍎"
                } else if GAME.with(|game| game.borrow().snake.contains(&pos)) {
                    "🟩"
                } else {
                    "⬜️"
                }
            });

            root_container.append_child(&field_element).unwrap_throw();
        }
    }
}
