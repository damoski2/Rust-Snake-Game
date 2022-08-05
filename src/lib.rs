mod random;
mod snake;

use js_sys::Function;
use snake::Direction;
use snake::SnakeGame;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{closure, prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{console, window, HtmlDivElement, HtmlElement, KeyboardEvent};

thread_local! {

    static GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(15, 15)));

    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        let game = GAME.with(|game| game.clone());
        move || {
            game.borrow_mut().tick();
            render();
        }
    }) as Box<dyn FnMut()>);

    static HANDLE_KEYDOWN: Closure<dyn FnMut(KeyboardEvent)> =
    Closure::wrap(Box::new(|evt: KeyboardEvent| GAME.with(|game| {
      let direction = match &evt.key()[..] {
        "ArrowUp" => Some(Direction::Up),
        "ArrowRight" => Some(Direction::Right),
        "ArrowDown" => Some(Direction::Down),
        "ArrowLeft" => Some(Direction::Left),
        _ => None,
      };

      if let Some(direction) = direction {
        game.borrow_mut().change_direction(direction);
      }
    })) as Box<dyn FnMut(KeyboardEvent)>)

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
                200,
            )
            .unwrap_throw();
    });

    HANDLE_KEYDOWN.with(|handle_keydown| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback(
                "keydown",
                handle_keydown.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });

    render();
}

pub fn render() {
    GAME.with(|game| {
        let game = game.borrow();
        let document = window().unwrap_throw().document().unwrap_throw();
        let root_container = document
            .get_element_by_id("root")
            .unwrap_throw()
            .dyn_into::<HtmlElement>()
            .unwrap_throw();

        root_container.set_inner_html("");

        let width = game.width;
        let height = game.height;

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

        /*    
        
         root_container
            .style()
            .set_property("width", "100%")
            .unwrap_throw();

        root_container
            .style()
            .set_property("height", "80vh")
            .unwrap_throw();

        root_container
                   .style()
                   .set_property("grid-column-gap", "30px")
                   .unwrap_throw();

               root_container
                   .style()
                   .set_property("grid-row-gap", "5px")
                   .unwrap_throw();
        */
        for y in 0..height {
            for x in 0..width {
                let pos = (x, y);
                let field_element = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap_throw();

                field_element.set_class_name("field");

                field_element.set_inner_text({
                    if pos == game.food {
                        "🍎"
                    } else if game.snake.get(0) == Some(&pos) {
                        "❇️"
                    } else if game.snake.contains(&pos) {
                        "🟩"
                    } else {
                        " "
                    }
                });

                root_container.append_child(&field_element).unwrap_throw();
            }
        }
    });
}

/*
      root_container
        .style()
        .set_property("grid-column-gap", "30px")
        .unwrap_throw();

    root_container
        .style()
        .set_property("grid-row-gap", "5px")
        .unwrap_throw();
*/
