

pub fn url() -> String {
    let window = web_sys::window().unwrap();
    let port = window.location().port().unwrap();
    if port=="8000" {
        return "http://localhost:3889/".to_string()
    }
    "/".to_string()
}

pub fn is_shimmer_node() -> bool {
    // let window = web_sys::window().unwrap();
    // let port = window.location().port().unwrap();
    // if port=="3888" {
    //     return true
    // }
    // false
    true
}