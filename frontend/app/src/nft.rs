use std::collections::HashMap;
// use std::collections::hash_map::Entry;

fn main() {

    let mut nft1 = NFT{
        name: String::from("Oregon Token"),
        _symbol: String::from("ORT"),
        _uri: String::from("https://something.com"),
    };
    let mut nft2 = NFT{
        name: String::from("California Token"),
        _symbol: String::from("CAT"),
        _uri: String::from("https://something.com"),
    };
    
    let alice = "alice";
    let bob = "bob";
    
    let mut db = DB{
        balances: HashMap::new(),
    };
    // give all to alice
    db.mint(alice, nft1.name().to_string());
    db.mint(alice, nft2.name().to_string());

    // alice sends 2 to bob
    db.transfer(alice, bob, nft1.name().to_string());
    
    println!("alice: {:?}", db.balance_of(alice));
    println!("bob: {:?}", db.balance_of(bob));
    
    db._burn(alice, nft2.name().to_string());
    
    println!("alice: {:?}", db.balance_of(alice));

    println!("yo yo {}", "yo")
}

struct NFT {
    name: String,
    _symbol: String,
    _uri: String
}
impl NFT {
     pub fn name(&mut self) ->&str {
        return self.name.as_str()
    }
    pub fn _symbol(&mut self) ->&str {
        return self._symbol.as_str()
    }
    pub fn _uri(&mut self) ->&str {
        return self._uri.as_str()
    }
}

struct DB {
    balances: HashMap<String, Vec<String>>,
}
impl DB {
    fn balance_of(&mut self, addy:&str) -> Vec<String> {
        match self.balances.get(&addy.to_string()) {
            Some(val) => { return val.to_vec() }, // .into_iter().collect(), //.into_iter(),
            _ => return Vec::new(),
        }
    }
    pub fn transfer(&mut self, from:&str, to:&str, tid:String) ->bool {
        let mut balance_from = self.balance_of(&from);
        if !balance_from.iter().any(|x| x==&tid) || tid=="" {
            return false
        }
        
        // remove it from here
        balance_from.retain(|x| x!=&tid);
        
        // push it
        let mut balance_to = self.balance_of(&to);
        balance_to.push(tid);
        
        self.balances.insert(from.to_string(), balance_from);
        self.balances.insert(to.to_string(), balance_to);
        true
    }
    fn mint(&mut self, to:&str, tid:String) {
        let mut balance_to = self.balance_of(to);
        // let mut vec = Vec::new();
        if !balance_to.iter().any(|i| i==&tid) {
            balance_to.push(tid);
        }
        
        self.balances.insert(to.to_string(), balance_to);
    }
    fn _burn(&mut self, from:&str, tid:String) ->bool {
        let mut balance_from = self.balance_of(from);
        if !balance_from.iter().any(|x| x==&tid) || tid=="" {
            return false
        }
        
        // remove it from here
        balance_from.retain(|x| x!=&tid);
        
        self.balances.insert(from.to_string(), balance_from);
        true
    }
    // fn _update(&mut self, addy:&str, tid:&str) {
    //     match self.balances.entry(addy.to_string()) {
    //         Entry::Occupied(mut entry)  => { entry.get_mut().push(tid.to_string()); },
    //         Entry::Vacant(entry)        => { entry.insert(vec!(tid.to_string())); },
    //     }
    // }
}