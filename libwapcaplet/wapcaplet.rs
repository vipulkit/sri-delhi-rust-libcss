#[link(name = "wapcaplet", vers = "1.0")];
#[crate_type = "lib"];

extern mod extra;

use std::str;
use std::hashmap::HashMap;

pub struct lwc_string {
    id: uint,
    string: ~str,
    insensitive: Option<uint>
}

pub struct lwc {
    priv map: @mut HashMap<~str, uint>,
    priv idx: uint
}

impl lwc {

    #[inline]
    pub fn dolower(c: u8 ) -> u8 {
        if (c > 64 && c < 91) {
              return (c  + 32) ;
        }
        c
    }

    fn to_lower(string:&str) -> ~str{
        let mut lower : ~[u8] = ~[];
        for string.bytes_iter().advance |c| {
            if (c > 64 && c < 91) {
                lower.push(c + 32);
            } else {
                lower.push(c);
            }
        }
        str::from_bytes_owned(lower)
    }

    pub fn lwc_intern_string(&mut self, val: &str) -> @mut lwc_string {

        match self.map.find_equiv(&val) {
            Some(&idx) => {
                return 
                    @mut lwc_string {
                        id:idx,
                        string: val.to_owned(),
                        insensitive: None
                    }
            },
            None => (),
        }

        self.idx = self.idx + 1;
        
        let val = val.to_owned();
        self.map.insert(copy val, self.idx);
        
        @mut lwc_string {
            id:self.idx,
            string: val,
            insensitive: None
        }
    }

    

    pub fn lwc_string_isequal(&self, str1: @mut lwc_string , str2: @mut lwc_string) -> bool {
        str1.id == str2.id
    }

    pub fn lwc_string_caseless_isequal(&mut self, str1: @mut lwc_string , str2: @mut lwc_string) ->bool {
        if (str1.id == str2.id) {
            return true;
        }

        if (str1.insensitive.is_none()) {
            self.lwc_intern_caseless_string(str1);
        }

        if (str2.insensitive.is_none()) {
            self.lwc_intern_caseless_string(str2);
        }

        (str1.insensitive.get() == str2.insensitive.get())
    }


    pub fn lwc_intern_caseless_string(&mut self , string: @mut lwc_string) {
        if (string.insensitive.is_some()) {
            return;
        }

        let val = lwc::to_lower(string.string);

        match self.map.find_equiv(&val) {
            Some(&idx) => {
                string.insensitive = Some(idx);
                return;
            },
            None => (),
        }

        self.idx = self.idx + 1;
        self.map.insert(val, self.idx);

        string.insensitive = Some(self.idx);
    }

    pub fn lwc_intern_substring(&mut self , substring_to_intern: @mut lwc_string , ssoffset: u32, sslen: u32) -> Option<@mut lwc_string> {
        
        if (substring_to_intern.string.len() <= ssoffset as uint) || (substring_to_intern.string.len() <= (ssoffset+sslen) as uint) {
            None
        }
        else{
            Some(self.lwc_intern_string((substring_to_intern.string.slice(ssoffset as uint , (ssoffset+sslen) as uint)).to_owned()))
        }
    }

} // impl wapcaplet

pub fn lwc_string_length(string: @mut lwc_string) -> uint {
    string.string.len()
}

pub fn lwc_string_data(string: @mut lwc_string) -> ~str {
    copy string.string
}

pub fn lwc()->@mut lwc {
    @mut lwc {
        map: @mut HashMap::new(),
        idx: 0,
    }
}
