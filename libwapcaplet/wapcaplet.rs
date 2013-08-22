#[link(name = "wapcaplet", vers = "1.0")];
#[crate_type = "lib"];

extern mod extra;

use std::hashmap::HashMap;
use std::str;
use std::clone::Clone;

priv struct lwc_string {
    id: uint,
    string: ~str,
    insensitive: Option<uint>
}

// implementing clone for  
impl Clone for lwc_string {  
    fn clone(&self) -> lwc_string {     
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
    #[inline]  
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
    pub fn lwc_intern_string(&mut self, val: &str) -> uint {

        match self.map.find_equiv(&val) {
            Some(&idx) => {
                return idx;
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

        self.vect.push(new_lwc_string);
        new_idx
    }


    #[inline]
    pub fn lwc_string_isequal(&mut self, str1: uint , str2: uint) -> bool {
        str1 == str2
    }

    #[inline]
    pub fn lwc_string_caseless_isequal(&mut self, str1: uint , str2: uint) ->bool {
        				
        if (self.vect[str1].insensitive.is_none()) {
			self.lwc_intern_caseless_string(str1);
        }
		
        if (self.vect[str2].insensitive.is_none()) {
            self.lwc_intern_caseless_string(str2);
        }

        (self.vect[str1].insensitive.get() == self.vect[str2].insensitive.get())
    }

	#[inline]
    pub fn lwc_intern_caseless_string(&mut self , string: uint) {
        if (self.vect[string].insensitive.is_some()) {
            return;
        }

        let val = lwc::to_lower(self.vect[string].string);
		
		match self.map.find_equiv(&val) {
            Some(&idx) => {
                self.vect[string].insensitive = Some(idx);
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
		self.vect[string].insensitive = Some(new_idx);	
    }	

    
    #[inline]
    pub fn lwc_intern_substring(&mut self , substring_to_intern: uint , ssoffset: u32, sslen: u32) -> Option<uint> {
        
        if (self.vect[substring_to_intern].string.len() <= ssoffset as uint) || (self.vect[substring_to_intern].string.len() <= (ssoffset+sslen) as uint) {
            None
        }
        else{
            let slice_string = self.vect[substring_to_intern].string.slice(ssoffset as uint , (ssoffset+sslen) as uint).to_owned();
            Some(self.lwc_intern_string(slice_string))
        }
    }

    #[inline]
    pub fn lwc_string_length(&self, string:uint) -> uint {
        self.vect[string].string.len()
    }
        
    #[inline]
    pub fn lwc_string_data(&self, string:uint) -> ~str {
        self.vect[string].string.clone()
    }

    
} // impl wapcaplet




pub fn create_lwc_instance() {
    unsafe{
        if lwc_ref.is_none() { 
            lwc_ref=Some(lwc())
        }
    }
}

priv fn lwc()->lwc {
    return lwc {
        map: HashMap::new(),
        vect: ~[]
    }
}

pub static mut lwc_ref : Option<lwc>  = None;

