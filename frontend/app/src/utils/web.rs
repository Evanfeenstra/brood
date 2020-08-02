
pub fn coopy(text: &str){
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let clip = navigator.clipboard();
    clip.write_text(text);
}

