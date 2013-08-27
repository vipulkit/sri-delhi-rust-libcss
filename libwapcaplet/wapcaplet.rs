#[link(name = "wapcaplet", vers = "1.0")];
#[crate_type = "lib"];

extern mod extra;

use std::hashmap::HashMap;
use std::str;
use std::clone::Clone;
use std::comm::* ;

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




// pub fn create_lwc_instance() {
//     unsafe{
//         if lwc_ref.is_none() { 
//             lwc_ref=Some(lwc())
//         }
//     }
// }

// priv fn lwc()->~lwc {
//     return ~lwc {
//         map: HashMap::new(),
//         vect: ~[]
//     }
// }

// pub static mut lwc_data_ref : Option<~lwc>  = None;

/////////////////////////////////////////////////////////////////////////////////////////




pub enum to_lwc {
    C_INTERN(~str),
    C_GET_LENGTH(uint),
    C_GET_DATA(uint),
    C_INTERN_SUBSTRING(uint,uint,uint),
    C_INTERN_CASELESS(uint),
    C_IS_CASELESS_EQUAL(uint,uint),
    C_TERMINATE
}

pub enum from_lwc {
    R_INTERN(uint),
    R_GET_LENGTH(uint),
    R_GET_DATA(~str),
    R_INTERN_SUBSTRING(uint),
    R_INTERN_CASELESS(uint),
    R_IS_CASELESS_EQUAL(bool),
    R_TERMINATE  // Not Required
}

pub static mut lwc_ref : Option<@mut lwc_wrapper>  = None;

pub struct lwc_wrapper {
    thread_handle : SharedChan< (to_lwc,SharedChan<from_lwc>) > ,
    thread_port : Port<(to_lwc,SharedChan<from_lwc>)>
}

pub fn create_lwc_thread() {
    unsafe{
        if lwc_ref.is_none() { 
            // create task  and assign handle to it 

            // initialize lwc_wrapper here and assign it to global variable
            let (port, chan): (Port<(to_lwc,SharedChan<from_lwc>)>, Chan<(to_lwc,SharedChan<from_lwc>)>) = stream();
            let schan = SharedChan::new(chan);
            let mut lwc_wrapper : @mut lwc_wrapper = @mut lwc_wrapper {
                thread_handle:schan.clone(),
                thread_port:port
            };

            ::lwc_ref = Some(lwc_wrapper);
            
            // when call enters the lwc_thread , lwc_thread will use it
            do spawn {
                lwc_thread() ;
            }
        }
    }
}

pub fn lwc_thread() {
    // do work here
    let mut lwc_container = ~lwc {
        map: HashMap::new(),
        vect: ~[]
    };

    let mut lwc_wrapper = unsafe { ::lwc_ref.get() } ;
    loop {
        let (message, send_port) = lwc_wrapper.thread_port.recv() ;
        match message {
            C_INTERN(x)=>{ 
                match lwc_container.map.find_equiv(&x) {
                    Some(&idx) => {
                        send_port.send(R_INTERN(idx));
                    },
                    None => {},
                }
                
                let new_idx = lwc_container.vect.len();
                let x = x.to_owned();
                
                lwc_container.map.insert(x.clone(), new_idx);

                let new_lwc_string = lwc_string {
                    id:new_idx,
                    string: x,
                    insensitive: None
                };

                lwc_container.vect.push(new_lwc_string);
                send_port.send(R_INTERN(new_idx));
                loop ; 
            },
            C_GET_LENGTH(x)=>{ 
                send_port.send(R_GET_LENGTH(lwc_container.vect[x].string.len()));
                loop ; 
            },
            C_GET_DATA(x)=>{ 
                send_port.send(R_GET_DATA(lwc_container.vect[x].string.clone()));
                loop ; 
            },
            C_INTERN_SUBSTRING(x,y,z)=>{ 
                if (lwc_container.vect[x].string.len() <= y as uint) || (lwc_container.vect[x].string.len() <= (y+z) as uint) {
                    send_port.send(R_INTERN_SUBSTRING(-1));
                }
                else{
                    let slice_string = lwc_container.vect[x].string.slice(y as uint , (y+z) as uint).to_owned();
                    send_port.send(R_INTERN_SUBSTRING(lwc_container.lwc_intern_string(slice_string)));
                }
                loop ; 
            },
            C_INTERN_CASELESS(x)=>{ 
                if (lwc_container.vect[x].insensitive.is_some()) {
                    send_port.send(R_INTERN_CASELESS(lwc_container.vect[x].insensitive.get()));
                }

                let val = lwc_wrapper::to_lower(lwc_container.vect[x].string);
                
                match lwc_container.map.find_equiv(&val) {
                    Some(&idx) => {
                        lwc_container.vect[x].insensitive = Some(idx);
                        send_port.send(R_INTERN_CASELESS(idx));
                    },
                    None => {}  
                }
                
                let new_idx = lwc_container.vect.len();
                let val = val.to_owned();
                lwc_container.map.insert(val.clone(), new_idx);  

                let new_insensitive = lwc_string {
                    id:new_idx,
                    string: val,
                    insensitive: Some(new_idx)
                };
                
                lwc_container.vect.push(new_insensitive);
                lwc_container.vect[x].insensitive = Some(new_idx);  
                send_port.send(R_INTERN_CASELESS(new_idx));
                loop ; 
            },
            C_IS_CASELESS_EQUAL(x,y)=>{ 
                loop ; 
            },
            C_TERMINATE=> {
                break ;
            }
        }
    }
    // Set global handle to none , and leave from the threads
    unsafe { ::lwc_ref = None; }
}


impl lwc_wrapper {

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
    pub fn lwc_intern_string(&mut self, val: ~str) -> int {

        let thread = self.thread_handle.clone();        
        let to_msg : to_lwc = C_INTERN(val) ;
        let (port, chan): (Port<from_lwc>, Chan<from_lwc>) = stream();
        let chan = SharedChan::new(chan);

        thread.send((to_msg,chan.clone()));

        let result : from_lwc = port.recv() ;

        match result{
            R_INTERN(x)=>{ x as int },
            _=>{ -1 }
        }
    }


    #[inline]
    pub fn lwc_string_isequal(&mut self, str1: uint , str2: uint) -> bool {
        str1 == str2
    }

    #[inline]
    pub fn lwc_string_caseless_isequal(&mut self, str1: uint , str2: uint) ->bool {
        
        let thread = self.thread_handle.clone();         
        let to_msg : to_lwc = C_IS_CASELESS_EQUAL(str1,str2) ;
        let (port, chan): (Port<from_lwc>, Chan<from_lwc>) = stream();
        let chan = SharedChan::new(chan);

        thread.send((to_msg,chan.clone()));

        let result : from_lwc = port.recv() ;

        match result{
            R_IS_CASELESS_EQUAL(x)=>{ x },
            _=>{ false }
        }              
    }

    #[inline]
    pub fn lwc_intern_caseless_string(&mut self , string: uint) -> int {

        let thread = self.thread_handle.clone();        
        let to_msg : to_lwc = C_INTERN_CASELESS(string) ;
        let (port, chan): (Port<from_lwc>, Chan<from_lwc>) = stream();
        let chan = SharedChan::new(chan);

        thread.send((to_msg,chan.clone()));

        let result : from_lwc = port.recv() ;

        match result{
            R_INTERN_CASELESS(x)=>{ x as int },
            _=>{ -1 }
        }
    }   

    
    #[inline]
    pub fn lwc_intern_substring(&mut self , substring_to_intern: uint , ssoffset: u32, sslen: u32) -> int {
        
        let thread = self.thread_handle.clone();         
        let to_msg : to_lwc = C_INTERN_SUBSTRING(substring_to_intern,ssoffset as uint,sslen as uint) ;
        let (port, chan): (Port<from_lwc>, Chan<from_lwc>) = stream();
        let chan = SharedChan::new(chan);

        thread.send((to_msg,chan.clone()));

        let result : from_lwc = port.recv() ;

        match result{
            R_INTERN_SUBSTRING(x)=>{ x as int },
            _=>{ -1 }
        }
    }

    #[inline]
    pub fn lwc_string_length(&mut self, string:uint) -> int {

        let thread = self.thread_handle.clone();          
        let to_msg : to_lwc = C_GET_LENGTH(string) ;
        let (port, chan): (Port<from_lwc>, Chan<from_lwc>) = stream();
        let chan = SharedChan::new(chan);

        thread.send((to_msg,chan.clone()));

        let result : from_lwc = port.recv() ;

        match result{
            R_GET_LENGTH(x)=>{ x as int },
            _=>{ -1 }
        }
    }
        
    #[inline]
    pub fn lwc_string_data(&self, string:uint) -> ~str {

        let thread = self.thread_handle.clone();          
        let to_msg : to_lwc = C_GET_DATA(string) ;
        let (port, chan): (Port<from_lwc>, Chan<from_lwc>) = stream();
        let chan = SharedChan::new(chan);

        thread.send((to_msg,chan.clone()));

        let result : from_lwc = port.recv() ;

        match result{
            R_GET_DATA(x)=>{ x },
            _=>{ ~"" }
        }
    }

    
} // impl wapcaplet

/////////////////////////////////////////////////////////////////////////////////////////