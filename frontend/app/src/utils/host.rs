

pub fn url() -> String {
    if get_port()=="8000" { // dev
        return "http://localhost:3889/".to_string()
    }
    "/".to_string() // prod
}

// docker on shimmer node
pub fn is_shimmer_node() -> bool {
    if get_port()=="3888" {
        return true
    }
    false
}

// go build on shimmer node
pub fn is_shimmer_node_no_docker() -> bool {
    if get_port()=="3887" {
        return true
    }
    false
}

fn get_port() -> String {
    let window = web_sys::window().unwrap();
    let port = window.location().port().unwrap();
    return port
}