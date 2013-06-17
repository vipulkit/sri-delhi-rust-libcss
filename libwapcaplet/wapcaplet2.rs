#[link(name = "wapcaplet2", vers = "2.0")];
#[crate_type = "lib"];


/*
 *  High Performance Libwapcaplet for string internment
 *  Performance/space parameters can be adjusted according to requirements
 *  Test-cases are checked in ../test/libwapcaplet/* 
 */

extern mod std;
use std::arc;

// convert string elements to lower-type for case-insensitive matching
#[inline(always)]
pub fn dolower(c: u8 ) -> u8 {
    if (c > 64 && c < 91) {
          return (c  + 32) ;
    }
    c
}

// Generate hash for strings in normal and lower-case scenarios in one-go
// Initial values and calucation derived from ...
#[inline(always)]
pub fn lwc_calculate_hash(string: &str) -> (u64,u64) {
    let mut z: u64 = 0x811c9dc5;                // 
    let mut iz : u64 = 0x811c9dc5 ;
    let string_index = str::char_len(string);
    for uint::range(0, string_index) |i| { 
        z *= 0x01000193;
        iz *= 0x01000193;
        iz ^= (dolower(string[i]) as u64);
        z ^= (string[i] as u64) ;
    }
    (z,iz)
}

// Interned string reference structure
pub struct lwc_string{

	priv hash_case:u64,
	priv hash_icase:u64,
	priv position:uint,
	priv slot_num : uint
}

// Slot bucket containing vector of hashes, lower-case string hashes and 
// position of corresponding string in the buck array.
pub struct hash_buck {
	priv hashes:~[u64],
	priv lhashes:~[u64],
	priv position:~[uint]
}

// Lwc instance
pub struct lwc {
	// Vector of hash_buck with length of 0xFFFF , 
	// index of this vector are termed as slots for ref/unref.
	priv hash_buck : ~[hash_buck],

	// data strings that are already refed
    priv buck: ~[~str],

    // Keeping locations of string already unrefed 
    // so that new strings can take their place

    // otherwise shifting all elements down from 
    // current position to the length will be costly
	priv unref_locations : ~[uint]    
}

// Single threaded version of the lwc
pub fn lwc_single_threaded() -> ~lwc {
    let mut t_hash :  ~[hash_buck] = ~[] ;
    for uint::range(0, 65537) |_| {
		let mut lh = hash_buck{ 
			hashes:~[] , 
			lhashes:~[] ,
			position:~[]
		} ;
        t_hash.push(lh);
    }
    let mut result = ~lwc{
        buck:~[],
        hash_buck:t_hash,
        unref_locations:~[]
    };    
    result
}

// thread safe version of lwc
pub fn lwc_thread_safe() -> arc::RWARC<~lwc> {
    let mut t_hash :  ~[hash_buck] = ~[] ;
    for uint::range(0, 65536) |_| {
		let mut lh = hash_buck{ 
			hashes:~[] , 
			lhashes:~[] ,
			position:~[]
		} ;
        t_hash.push(lh);
    }
    let mut result = ~lwc{
        buck:~[],
        hash_buck:t_hash,
        unref_locations:~[]
    };    
    arc::RWARC(result)
}


// Is strings are equal
#[inline(always)]
pub fn lwc_string_isequal(first:lwc_string , second:lwc_string) -> bool {
	(first.position==second.position)
}

impl lwc {
	pub fn lwc_string_ref(&mut self,data:&str) -> lwc_string {
		self.lwc_intern_string(data)
	}

	pub fn lwc_string_unref(&mut self, string:lwc_string) -> bool {
		let pos = string.position ;
		let slot = string.slot_num ;
		let mut j = 0 ;
		let mut found : bool = false ;

		for self.hash_buck[slot].position.eachi |i,&elem| {
			if elem == pos {
				found = true ;
				j = i ;
			}
		}

		if found {
			// remove j th element from hashes , lhashes , and position 
			// -- not calling vector::remove because , this in turn  
			// will shift all elements from the current position to the length

			// so replacing current elemnt with last element
			// and poping up the last element in the vector.
			let len = self.hash_buck[slot].position.len() - 1 ;
			self.hash_buck[slot].position[j] = self.hash_buck[slot].position[len] ;
			self.hash_buck[slot].hashes[j] = self.hash_buck[slot].hashes[len] ;
			self.hash_buck[slot].lhashes[j] = self.hash_buck[slot].lhashes[len] ;
			self.hash_buck[slot].position.pop() ;
			self.hash_buck[slot].hashes.pop() ;
			self.hash_buck[slot].lhashes.pop() ;
			// Seting string as empty.
			self.buck[pos] = ~"" ;
			true
		}
		else {
			// Not found
			false
		}
	}

	// String interment
	pub fn lwc_intern_string(&mut self,data:&str) -> lwc_string {
		let (hash,lhash)  = lwc_calculate_hash(data) ;
		let slot : uint = (lhash & 0xFFFF ) as uint ;
		let final_lhash = (lhash & 0xFFFFFFFFFFFF0000) | ((data.len() as u8) as u64) ; 
		let mut pos : uint = 0 ;

		for self.hash_buck[slot].hashes.eachi |i,&elem| {
			//io::println(fmt!("Hash Bucket is =%?=",self.hash_buck[slot]));
			if elem == hash {
				if self.hash_buck[slot].lhashes[i] == final_lhash {
					pos = self.hash_buck[slot].position[i] ;
					if str::eq_slice(data,self.buck[pos]) {
						//io::println(fmt!("    Found  =%?=%?=%?=",hash,slot,final_lhash));
						//io::println(fmt!("Strings are =%?=%?=",data,self.buck[pos]));
						return lwc_string{hash_case:hash,hash_icase:final_lhash,position:pos,slot_num:slot} ;
					}
				}
			}
		}
		self.hash_buck[slot].hashes.push(hash);
		self.hash_buck[slot].lhashes.push(final_lhash);
		if self.unref_locations.len() ==0 {
			pos = self.buck.len() ;
			self.buck.push(data.to_owned());
		}
		else {
			pos = self.unref_locations.pop() ;
			self.buck[pos] = data.to_owned() ;
		}
		self.hash_buck[slot].position.push(pos) ;
		//io::println(fmt!("Not Found  =%?=%?=%?=",hash,slot,final_lhash));
		//io::println(fmt!("Strings inserted is =%?=",data));
		return lwc_string{hash_case:hash,hash_icase:lhash,position:pos,slot_num:slot} ;
	}

	// Is strings are caseless equal
	pub fn lwc_string_caseless_isequal(&self,first:lwc_string , second:lwc_string) -> bool {
		//io::println(fmt!("Entering caseless isequal"));
		if first.position == second.position {
			return true ;
		}
		io::println(fmt!("check1 caseless isequal"));
		if first.hash_icase != second.hash_icase {
			//io::println(fmt!("hashes matched are =%?=%?=",first.hash_icase,second.hash_icase)) ;
			return false ;
		}
		let mut len1 = self.buck[first.position].len() ;
		let mut len2 = self.buck[second.position].len() ;
		//io::println(fmt!("check2 caseless isequal"));
		if len1 != len2 {
			return false ;
		}
		io::println(fmt!("check3 caseless isequal"));
		while len1 !=0 {
			len1 -= 1;
			if dolower(self.buck[first.position][len1]) == dolower(self.buck[second.position][len1]) {
				//io::println(fmt!("character matched are =%?=%?=",self.buck[first.position][len1] ,self.buck[second.position][len1] )) ;
			}
			else {
				return false ;
			}
		}
		true 
	}

	#[inline(always)]
	pub fn lwc_string_data(&self, string:lwc_string) -> ~str {
		copy self.buck[string.position]
	}

	#[inline(always)]
	pub fn lwc_string_length(&self,string:lwc_string) -> uint {
		self.buck[string.position].len()
	}
}

// implementing clone for
pub impl<'self> lwc_string {
    fn clone(&self) -> lwc_string {
        lwc_string{
        	hash_case:self.hash_case,
        	hash_icase:self.hash_icase,
        	position:self.position,
        	slot_num:self.slot_num
        }
    }
}