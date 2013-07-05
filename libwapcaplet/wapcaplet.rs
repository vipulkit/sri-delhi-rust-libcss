#[link(name = "wapcaplet", vers = "1.0")];
#[crate_type = "lib"];

extern mod extra;

use std::{uint, borrow};

pub struct lwc_string {
    string: ~str,
    hash: u32,
    insensitive: Option<@mut lwc_string>
}

pub struct lwc {
    priv buckets: ~([~[@mut lwc_string]])
}

impl lwc {

    #[inline(always)]
    pub fn dolower(c: u8 ) -> u8 {
        if (c > 64 && c < 91) {
              return (c  + 32) ;
        }
        c
    }

    #[inline(always)]
    pub fn lwc_calculate_hash(string: &str) -> u32 {
        let mut z: u32 = 0x811c9dc5;
        let string_index = string.char_len();
        for uint::range(0, string_index) |i| { 
            z *= 0x01000193;
            z ^= (string[i] as u32) ;
        }
        z%4091
    }

    #[inline(always)]
    priv fn lwc_calculate_lcase_hash(string: &str) -> u32 {
        let mut z: u32 = 0x811c9dc5;
        let string_index = string.char_len();
        for uint::range(0, string_index) |i| { 
            z *= 0x01000193;
            z ^=  (lwc::dolower(string[i]) as u32) ;
        }
        z%4091
    }

    pub fn lwc_intern_string(&mut self, string_to_intern: &str) -> @mut lwc_string {

        let hash_value = lwc::lwc_calculate_hash(string_to_intern);
        let string_to_intern_actual = string_to_intern.to_owned();
        
        let vector_index = self.buckets[hash_value].len();
        
        if vector_index != 0 {
            for uint::range(0, vector_index) |j| {
                if (self.buckets[hash_value][j].hash==hash_value) && (self.buckets[hash_value][j].string == string_to_intern_actual) {
                    return self.buckets[hash_value][j];
                }       
            }

            let lwc_string_to_intern = @mut lwc_string {
                string: string_to_intern_actual , 
                hash: hash_value , 
                insensitive: None
            };
            self.buckets[hash_value].push(lwc_string_to_intern.clone());
            return lwc_string_to_intern;
        }
        else  {
            let lwc_string_to_intern = @mut lwc_string {
                string: string_to_intern_actual , 
                hash: hash_value , 
                insensitive: None
            };
            self.buckets[hash_value].push(lwc_string_to_intern.clone());
            return lwc_string_to_intern;
        }
    }

    priv fn __lwc_intern(&mut self , string_to_intern: &str, insensitive:bool) -> @mut lwc_string {
        //io::println(fmt!("lwc_intern_string:: Timestamp 1 == %.3f usec", std::time::precise_time_ns() as float/ 1000f));
        let mut string_to_intern_actual : ~str; //= ~"";
        let mut hash_value : u32;//= 0u32;

        match (insensitive) {
            false=> {
                hash_value = lwc::lwc_calculate_hash(string_to_intern);
                string_to_intern_actual = string_to_intern.to_owned();
            }
            true=> { 
                hash_value = lwc::lwc_calculate_lcase_hash(string_to_intern);
                string_to_intern_actual = string_to_intern.to_ascii().to_lower().to_str_ascii();
            }
        };
        
        //io::println(fmt!("lwc_intern_string:: Timestamp 2 == %.3f usec", std::time::precise_time_ns() as float/ 1000f));
        let mut vector_index = self.buckets[hash_value].len();

        let copy_of_string_to_intern = copy string_to_intern_actual;
        //io::println(fmt!("lwc_intern_string:: Timestamp 2.5 == %.3f usec", std::time::precise_time_ns() as float/ 1000f));
        let lwc_string_to_intern = @mut lwc_string {
            string: string_to_intern_actual , 
            hash: hash_value , 
            insensitive: None
        };
        
        //io::println(fmt!("lwc_intern_string:: Timestamp 3 == %.3f usec", std::time::precise_time_ns() as float/ 1000f));
        if vector_index == 0 {
            self.buckets[hash_value].push(lwc_string_to_intern.clone());
            //io::println(fmt!("lwc_intern_string:: Timestamp 4 == %.3f usec", std::time::precise_time_ns() as float/ 1000f));
            return lwc_string_to_intern;
        }
        else {
            while vector_index>0 {
                
                if (self.buckets[hash_value][vector_index-1].string == copy_of_string_to_intern) {
                    return self.buckets[hash_value][vector_index-1];
                }
                
                vector_index = vector_index-1;
                
            }   
            self.buckets[hash_value].push(lwc_string_to_intern);

            //io::println(fmt!("lwc_intern_string:: Timestamp 5 == %.3f usec", std::time::precise_time_ns() as float/ 1000f));
            return lwc_string_to_intern;
        }

    }
    
    pub fn lwc_intern_substring(&mut self , substring_to_intern: @mut lwc_string , ssoffset: u32, sslen: u32) -> Option<@mut lwc_string> {
        //io::println("Inside lwc_intern_substring");
        if (substring_to_intern.string.len() <= ssoffset as uint) || (substring_to_intern.string.len() <= (ssoffset+sslen) as uint) {
            None
        }
        else{
            Some(self.lwc_intern_string((substring_to_intern.string.slice(ssoffset as uint , (ssoffset+sslen) as uint)).to_owned()))
        }
    }

    pub fn lwc_string_isequal(&self, str1: @mut lwc_string , str2: @mut lwc_string) -> bool {
        borrow::ref_eq(str1,str2)
    }

    pub fn lwc_intern_caseless_string(&mut self , lwc_string_instance: @mut lwc_string) {

        let caseless_string = self.__lwc_intern(lwc_string_data(lwc_string_instance), true);

        lwc_string_instance.insensitive = Some(caseless_string);

    }


    pub fn lwc_string_caseless_isequal(&mut self, str1: @mut lwc_string , str2: @mut lwc_string) ->bool {
            
        let return_value = borrow::ref_eq(str1,str2);

        if return_value {
            return true;
        }

        if (str1.insensitive.is_none()) {
            self.lwc_intern_caseless_string(str1);
        }

        if (str2.insensitive.is_none()) {
            self.lwc_intern_caseless_string(str2);
        }

        self.lwc_string_isequal(str1.insensitive.get(), str2.insensitive.get())
    }

} // impl wapcaplet

pub fn lwc_string_length(string: @mut lwc_string) -> uint {
    string.string.len()
}

pub fn lwc_string_hash_value(string: @mut lwc_string) -> u32 {
    string.hash
}

pub fn lwc_string_data(string: @mut lwc_string) -> ~str {
    copy string.string
}

pub fn lwc()->@mut lwc {
    
    let mut temp_buckets: ~([~[@mut lwc_string]]) = ~[];
    for uint::iterate(0, 4091) |_| {
        let bucket:~[@mut lwc_string] = ~[];
        temp_buckets.push(bucket);
    }

    @mut lwc {
        buckets:temp_buckets
    }
}

