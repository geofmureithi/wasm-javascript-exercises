use hirola::prelude::*;

fn counter<'a>(_app: &'a HirolaApp) -> Dom {
    let count = Signal::new(5);
    let incerement = count.mut_callback(|c, _| *c + 1);

    html! {
        <div>
            <button on:click=incerement>
                <span>"Increment"</span>
            </button>
            <span>{count.get()}</span>
        </div>

    }
}
fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let _body = document.body().unwrap();

    let app = HirolaApp::new();
    app.render_to_string(counter);
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    #[wasm_bindgen_test]
    fn counter_renders() {
        let app = HirolaApp::new();
        let res = app.render_to_string(counter);
        assert_eq!("<div><button><span>Increment</span></button><span>5</span></div>", &res);
    }
}