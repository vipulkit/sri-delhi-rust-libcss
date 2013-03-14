#[link(name = "wapcaplet", vers = "1.0")];
#[crate_type = "lib"];

use core::managed;

pub struct lwc_string {
	mut string: @str,
	mut length: uint,
	mut refcnt: u32,
	mut hash: u32,
	mut is_case_insensitive: bool
}

pub struct lwc {
	mut bucketVector: @[mut (~[@lwc_string]) * 4091]
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

	pub fn lwc_intern_string(&self, string_to_intern: &str) -> @lwc_string {
		self.__lwc_intern(string_to_intern, false)
	}

	fn __lwc_intern(&self , string_to_intern: &str, case_insensitive:bool) -> @lwc_string {
		let hash_value = 
			match (case_insensitive) {
			false=> self.lwc_calculate_hash(string_to_intern),
			true=> self.lwc_calculate_lcase_hash(string_to_intern)
		};
	
		let len = str::char_len(string_to_intern);
		let mut vector_index = self.bucketVector[hash_value].len();
		let string_to_intern_managed = string_to_intern.to_managed();

		let lwc_string_to_intern = @lwc_string {
			string: string_to_intern_managed , 
			length: len ,
			refcnt: 1 , 
			hash: hash_value , 
			is_case_insensitive:case_insensitive
		};
		
		if vector_index == 0 {
			vec::push(&mut self.bucketVector[hash_value], lwc_string_to_intern);
			return lwc_string_to_intern;
		}
		else {
			while vector_index>0 {
				if self.bucketVector[hash_value][vector_index-1].string == string_to_intern_managed {
					self.bucketVector[hash_value][vector_index-1].refcnt += 1;
					return self.bucketVector[hash_value][vector_index-1];
				}
				vector_index = vector_index-1;
			}
			vec::push(&mut self.bucketVector[hash_value], lwc_string_to_intern);
			return lwc_string_to_intern;
		}
	}
	
	pub fn lwc_intern_substring(&self , substring_to_intern: @lwc_string , ssoffset: u32, sslen: u32) -> @lwc_string {
		self.lwc_intern_string(str::slice(substring_to_intern.string , ssoffset as uint , (ssoffset+sslen) as uint))
	}

	pub fn lwc_string_ref(&self , string_to_ref: @lwc_string) -> @lwc_string {
		let hash_value = string_to_ref.hash;
		let mut vector_index = self.bucketVector[hash_value].len();

		while vector_index > 0 {
			if (self.bucketVector[hash_value][vector_index - 1].string.len() == string_to_ref.string.len())&&(str::contains(string_to_ref.string ,(self.bucketVector[hash_value][vector_index - 1].string) )) {
				string_to_ref.refcnt += 1;
			}
			vector_index = vector_index - 1;
		}
		string_to_ref
	}

	pub fn lwc_string_unref(&self , string_to_unref: @lwc_string) {
		let hash_value = string_to_unref.hash;
		let mut vector_index = self.bucketVector[hash_value].len();

		while vector_index > 0 {
			if (self.bucketVector[hash_value][vector_index - 1].string.len() == string_to_unref.string.len())&&(str::contains(string_to_unref.string ,(self.bucketVector[hash_value][vector_index - 1].string) )) {
				if string_to_unref.refcnt!= 1 {
					string_to_unref.refcnt -= 1;

				}
				else {
					vec::remove(&mut self.bucketVector[hash_value] , (vector_index - 1) as uint);
				}
				break;
			}
			vector_index = vector_index - 1;
		}
	}

	pub fn lwc_string_isequal(&self , str1: @lwc_string , str2: @lwc_string) ->bool {
		managed::ptr_eq(str1 , str2)
	}

	pub fn lwc_intern_caseless_string(&self , string_to_intern: @lwc_string) ->@lwc_string {
		self.__lwc_intern(string_to_intern.string, true)
	}

	pub fn lwc_string_caseless_isequal(&self , string_to_cmp1: @lwc_string , string_to_cmp2: @lwc_string) ->bool {
		(string_to_cmp1.is_case_insensitive && string_to_cmp2.is_case_insensitive)
	}

	static pub fn lwc_string_length( string: @lwc_string) -> uint {
		string.length
	}

	pub fn lwc_string_hash_value(&self , string: @lwc_string) -> u32 {
		string.hash
	}

	static pub fn lwc_string_data(string: @lwc_string) -> @str {
		string.string
	}
}

pub fn lwc()->@lwc {
	@lwc {
		bucketVector:@[mut ~[], ..4091]
	}
}

