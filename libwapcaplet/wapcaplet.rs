#[link(name = "wapcaplet", vers = "1.0")];
#[crate_type = "lib"];

extern mod extra;

use std::hashmap::HashMap;
use std::str;

pub struct lwc_string {
    id: uint,
    string: @str,
    insensitive: @mut Option<uint>
}

pub struct lwc {
    priv map: @mut HashMap<@str, uint>,
    priv vect: @mut ~[@lwc_string]
}

impl lwc {

    #[inline]
    pub fn dolower(c: u8 ) -> u8 {
        if (c > 64 && c < 91) {
              return (c  + 32) ;
        }
        c
    }

    #[inline]
    fn to_lower(string:@str) -> @str{
        let mut lower : ~[u8] = ~[];
        for string.bytes_iter().advance |c| {
            if (c > 64 && c < 91) {
                lower.push(c + 32);
            } else {
                lower.push(c);
            }
        }
        str::from_bytes_owned(lower).to_managed()
    }

    #[inline]
    pub fn lwc_intern_string(&self, val: &str) -> @lwc_string {

        match self.map.find_equiv(&val) {
            Some(&idx) => {
                return self.vect[idx];
            },
            None => (),
        }
        
        let new_idx = self.vect.len();
        let val = val.to_managed();
        
        self.map.insert(val, new_idx);

        let new_lwc_string = @lwc_string {
            id:new_idx,
            string: val,
            insensitive: @mut None
        };

        self.vect.push(new_lwc_string);
        new_lwc_string
    }

    #[inline]
    pub fn lwc_intern_string_managed(&self, val: @str) -> @lwc_string {
        let new_idx = self.vect.len();
        
        let find_idx = self.map.find_or_insert(val, new_idx); 
        

        if (*find_idx != new_idx) {
            self.vect[*find_idx]
        }
        else{
            let new_lwc_string = @lwc_string {
                id:new_idx,
                string: val,
                insensitive: @mut None
            };

            self.vect.push(new_lwc_string);
            new_lwc_string
        }
    }

    

    #[inline]
    pub fn lwc_string_isequal(&self, str1: @lwc_string , str2: @lwc_string) -> bool {
        str1.id == str2.id
    }

    #[inline]
    pub fn lwc_string_caseless_isequal(&self, str1: @lwc_string , str2: @lwc_string) ->bool {
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


    #[inline]
    pub fn lwc_intern_caseless_string(&self , string: @lwc_string) {
        if (string.insensitive.is_some()) {
            return;
        }

        let val = lwc::to_lower(string.string);
        let new_idx = self.vect.len();
        
        let find_idx = self.map.find_or_insert(val, new_idx); 
        
        if (*find_idx != new_idx) {
            *string.insensitive = Some(*find_idx);
        }
        else{
            let new_insensitive = @lwc_string {
                id:new_idx,
                string: val,
                insensitive: @mut None
            };

            self.vect.push(new_insensitive);
            *string.insensitive = Some(new_idx);
        }   
    }

    #[inline]
    pub fn lwc_intern_substring(&self , substring_to_intern: @lwc_string , ssoffset: u32, sslen: u32) -> Option<@lwc_string> {
        
        if (substring_to_intern.string.len() <= ssoffset as uint) || (substring_to_intern.string.len() <= (ssoffset+sslen) as uint) {
            None
        }
        else{
            Some(self.lwc_intern_string((substring_to_intern.string.slice(ssoffset as uint , (ssoffset+sslen) as uint)).to_owned()))
        }
    }

} // impl wapcaplet
    
#[inline]
pub fn lwc_string_length(string: @lwc_string) -> uint {
    string.string.len()
}
    
#[inline]
pub fn lwc_string_data(string: @lwc_string) -> @str {
    string.string
}

pub fn lwc()->@lwc {
    @lwc {
        map: @mut HashMap::new(),
        vect: @mut ~[]
    }
}
