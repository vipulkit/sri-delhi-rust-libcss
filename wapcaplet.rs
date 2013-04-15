#[link(name = "wapcaplet", vers = "1.0")];
#[crate_type = "lib"];

extern mod std;

use std::arc;
use core::ptr;

pub struct lwc_string {
	string: ~str,
	length: uint,
	refcnt: u32,
	hash: u32,
	is_case_insensitive: bool
}

pub struct lwc {
	bucketVector: ~([~[arc::RWARC<~lwc_string>]])
}

impl lwc {

	pub fn dolower(&self , c: u8 ) -> char {
		if (c >= 'A' as u8 && c <= 'Z' as u8) {
			  return (c as char + 'a' - 'A');
		}
		else {
			return c as char;
		}
	}

	pub fn lwc_calculate_hash(&self , string: &str) -> u32{
		let mut z: u32 = 0x811c9dc5;
	    let mut i: uint = 0;
	    let mut string_index = str::char_len(string);
	    while string_index>0 {
	        z = z*0x01000193;
	        z = (z^(string[i]) as u32);
	        string_index = string_index-1;
	        i = i+1; 
	    }
	    z = z%4091;
	    z
	}

	pub fn lwc_calculate_lcase_hash(&self , string: &str) -> u32 {
		let mut z: u32 = 0x811c9dc5;
		let mut i: uint = 0;
		let mut string_index = str::char_len(string);

		while string_index>0 {
			z = z*0x01000193;
			i = i+1;
			z = (z^self.dolower(string[i]) as u32);
	        string_index = string_index-1;
		}
		z = z%4091;
		z
	}

	pub fn lwc_lcase_strncmp(&self , s1: &str, s2: &str, n: uint) -> int {
		let mut i: uint = 0;
	    let mut t = n;
	    while t>0 {
	    	t = t-1;
	    	i = i+1;
	        if s1[i] != self.dolower(s2[i]) as u8 {
	            return 1;
	        }
	    }
	    return 0;
	}

	pub fn lwc_lcase_memcpy(&self , target: &str, source: &str, n: uint) {
		let mut i: uint = 0;
		let mut t =n;
		let mut str1: ~str = str::from_slice(source);
	    let mut str2: ~str = str::from_slice(target);
	    while t>0 {
	        t = t - 1;
	        i = i + 1;
	        str2[i] = self.dolower(str1[i]) as u8;
	    }
	}

	pub fn lwc_intern_string(&mut self, string_to_intern: ~str) -> arc::RWARC<~lwc_string> {
		self.__lwc_intern(string_to_intern, false)
	}

	fn __lwc_intern(&mut self , string_to_intern: ~str, case_insensitive:bool) -> arc::RWARC<~lwc_string> {
		let hash_value = 
			match (case_insensitive) {
			false=> self.lwc_calculate_hash(string_to_intern),
			true=> self.lwc_calculate_lcase_hash(string_to_intern)
		};
	
		let len = str::char_len(string_to_intern);
		let mut vector_index = self.bucketVector[hash_value].len();

		let copy_of_string_to_intern = copy string_to_intern;

		let lwc_string_to_intern = arc::RWARC(~lwc_string {
			string: string_to_intern , 
			length: len ,
			refcnt: 1 , 
			hash: hash_value , 
			is_case_insensitive:case_insensitive
		});
		
		if vector_index == 0 {
			vec::push(&mut self.bucketVector[hash_value], lwc_string_to_intern.clone());
			return lwc_string_to_intern;
		}
		else {
			while vector_index>0 {
				let mut found_flag = false;
				
				do self.bucketVector[hash_value][vector_index-1].write |l| {
					if ((*l).string == copy_of_string_to_intern) {
						(*l).refcnt += 1;
						found_flag = true;
					}
				}
				
				if (found_flag) {
					return self.bucketVector[hash_value][vector_index-1].clone();
				}
				
				vector_index = vector_index-1;
				
			}	
			vec::push(&mut self.bucketVector[hash_value], lwc_string_to_intern.clone());
			return lwc_string_to_intern;
		}
	}
	
	pub fn lwc_intern_substring(&mut self , substring_to_intern: arc::RWARC<~lwc_string> , ssoffset: u32, sslen: u32) -> arc::RWARC<~lwc_string> {
		do substring_to_intern.read |l| {
			self.lwc_intern_string((str::slice(l.string , ssoffset as uint , (ssoffset+sslen) as uint)).to_owned())
		}
	}

	pub fn lwc_string_ref(&mut self , string_to_ref: arc::RWARC<~lwc_string>) -> arc::RWARC<~lwc_string> {
		do string_to_ref.read |l| {
			let hash_value = l.hash;

			let mut vector_index = self.bucketVector[hash_value].len();

			while vector_index > 0 {
				let mut found_flag = false;
				do self.bucketVector[hash_value][vector_index-1].write |i_l| {
					if ((*i_l).string == (*l).string) {
						(*i_l).refcnt += 1;
						found_flag = true;
					}
				}
				if found_flag {
					break;
				}
				vector_index = vector_index - 1;
			}
		}
		
		
		string_to_ref.clone()
	}

	pub fn lwc_string_unref(&mut self , string_to_unref: arc::RWARC<~lwc_string>) {
		do string_to_unref.write |l| {

			let hash_value = l.hash;
			let mut vector_index = self.bucketVector[hash_value].len();

			let mut remove_flag = false;
			
			while vector_index > 0 {
				let mut found_flag = false;
				
				do self.bucketVector[hash_value][vector_index-1].write |i_l| {
					if ((*i_l).string == (*l).string) {
						found_flag = true;

						if ((*i_l).refcnt > 1) {
							(*i_l).refcnt -= 1;
						}
						else {
							remove_flag = true;
						}
					}
				}

				if (found_flag) {
					break;	
				}

				vector_index = vector_index - 1;
			}

			if (remove_flag) {
				vec::remove(&mut self.bucketVector[hash_value] , vector_index - 1);
			}
		}
	}

	pub fn lwc_string_isequal(str1: arc::RWARC<~lwc_string> , str2: arc::RWARC<~lwc_string>) ->bool {
		do str1.read |s1| {
			do str2.read |s2| {
				ptr::ref_eq(s1,s2)
			}
		}
	}

	pub fn lwc_intern_caseless_string(&mut self , string_to_intern: arc::RWARC<~lwc_string>) ->  arc::RWARC<~lwc_string> {
		do string_to_intern.read |s| {
			self.__lwc_intern(copy s.string, true)
		}
		
	}

	pub fn lwc_string_caseless_isequal(str1: arc::RWARC<~lwc_string> , str2: arc::RWARC<~lwc_string>) ->bool {
		do str1.read |s1| {
			do str2.read |s2| {
				s1.is_case_insensitive && s2.is_case_insensitive
			}
		}
	}

	pub fn lwc_string_length(string: arc::RWARC<~lwc_string>) -> uint {
		do string.read |s| {
			s.string.len()
		}
	}

	pub fn lwc_string_hash_value(string: arc::RWARC<~lwc_string>) -> u32 {
		do string.read |s| {
			s.hash
		}
	}

	pub fn lwc_string_data(string: arc::RWARC<~lwc_string>) -> ~str {
		do string.read |s| {
			copy s.string
		}
	}
}

pub fn lwc()->~lwc {
	
	let mut tempBucketVector: ~([~[arc::RWARC<~lwc_string>]]) = ~[];
	for uint::range(0, 4091) |_| {
		let bucket:~[arc::RWARC<~lwc_string>] = ~[];
		tempBucketVector.push(bucket);
	}

	~lwc {
		bucketVector:tempBucketVector
	}
}

