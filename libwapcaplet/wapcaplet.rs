#[link(name = "wapcaplet", vers = "1.0")];
#[crate_type = "lib"];

extern mod extra;

use std::hashmap::HashMap;
use std::str;
use std::clone::Clone;

pub struct lwc_string {
    id: uint,
    string: ~str,
    insensitive: Option<uint>
}

// implementing clone for lwc_string
impl Clone for lwc_string {
    [#inline]
    pub fn clone(&self) -> lwc_string {
        lwc_string{
            id:self.id,
            string:self.string.clone(),
            insensitive:self.insensitive
        }
    }
}

pub struct lwc {
    priv map: HashMap<~str, uint>,
    priv vect:~[lwc_string]
}

// implementing clone for lwc
impl Clone for lwc {
    [#inline]
    pub fn clone(&self) -> lwc {
        lwc{
            map: self.map.clone(),
            vect: self.vect.clone()
        }
    }
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
    fn to_lower(string:&str) -> ~str{
        let mut lower : ~[u8] = ~[];
        let len = string.len();
		lower.reserve(len);
		let mut c = 0;
		let mut ch : u8;
		while c < len {
			ch = string[c] as u8;
			unsafe {
				if (ch > 64 && ch < 91) {
					lower.push_fast(ch + 32);
				} else {
					lower.push_fast(ch);
				}
			}	
			c += 1;
        }
        str::from_bytes_owned(lower)
    }

    #[inline]
    pub fn lwc_intern_string(&mut self, val: &str) -> lwc_string {

        match self.map.find_equiv(&val) {
            Some(&idx) => {
                return self.vect[idx].clone();
            },
            None => (),
        }
        
        let new_idx = self.vect.len();
        let val = val.to_owned();
        
        self.map.insert(val.clone(), new_idx);

        let new_lwc_string = lwc_string {
            id:new_idx,
            string: val,
            insensitive: None
        };

        self.vect.push(new_lwc_string.clone());
        new_lwc_string
    }


    #[inline]
    pub fn lwc_string_isequal(&mut self, str1: &mut lwc_string , str2: &mut lwc_string) -> bool {
        str1.id == str2.id
    }

    #[inline]
    pub fn lwc_string_caseless_isequal(&mut self, str1: &mut lwc_string , str2: &mut lwc_string) ->bool {
        				
        if (str1.insensitive.is_none()) {
			self.lwc_intern_caseless_string(str1);
        }
		
        if (str2.insensitive.is_none()) {
            self.lwc_intern_caseless_string(str2);
        }

        (str1.insensitive.get() == str2.insensitive.get())
    }

	#[inline]
    pub fn lwc_intern_caseless_string(&mut self , string: &mut lwc_string) {
        if (string.insensitive.is_some()) {
            return;
        }

        let val = lwc::to_lower(string.string);
		
		match self.map.find_equiv(&val) {
            Some(&idx) => {
                string.insensitive = Some(idx);
				return;
            },
            None => {}	
        }
        
        let new_idx = self.vect.len();
		let val = val.to_owned();
		self.map.insert(val.clone(), new_idx);	

		let new_insensitive = lwc_string {
			id:new_idx,
			string: val,
			insensitive: Some(new_idx)
		};
		
		self.vect.push(new_insensitive);
		string.insensitive = Some(new_idx);	
    }	

    
    #[inline]
    pub fn lwc_intern_substring(&mut self , substring_to_intern: &mut lwc_string , ssoffset: u32, sslen: u32) -> Option<lwc_string> {
        
        if (substring_to_intern.string.len() <= ssoffset as uint) || (substring_to_intern.string.len() <= (ssoffset+sslen) as uint) {
            None
        }
        else{
            Some(self.lwc_intern_string((substring_to_intern.string.slice(ssoffset as uint , (ssoffset+sslen) as uint))))
        }
    }

} // impl wapcaplet
    
#[inline]
pub fn lwc_string_length(string:& lwc_string) -> uint {
    string.string.len()
}
    
#[inline]
pub fn lwc_string_data(string:& lwc_string) -> ~str {
    string.string.clone()
}

pub fn lwc()->lwc {
     lwc {
        map: HashMap::new(),
        vect: ~[]
    }
}


pub static mut lwc_ref : Option<lwc>  = None;

