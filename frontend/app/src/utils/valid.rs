

pub fn address(addy: &String) -> bool {
    let alpha = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string();
    let mut ok = true;
    for c in addy.chars() { 
        if !alpha.contains(c) {
            ok = false;
        }
    }
    ok
}

pub fn amount_input(amt: &String, max: u64) -> bool {
    if amt=="" {
        return true
    }
    match amt.parse::<u64>() {
        Ok(n)=> {
            if n==0 {
                return false
            } else if n>2779530283277761 { // max supply
                return false
            } else if n>max {
                return false
            } else {
                return true
            }
        },
        Err(_e)=> false
    }
}

pub fn process_ip(ip: String) -> String {
    let mut val = ip.clone();
    if val.chars().last()==Some('/') {
        val.pop();
    }
    val
}