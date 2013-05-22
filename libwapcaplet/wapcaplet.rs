#[link(name = "wapcaplet", vers = "1.0")];
#[crate_type = "lib"];

extern mod std;

use std::arc;
use core::ptr;
use core::pipes::{GenericPort, Chan, Port};
use core::pipes::stream;
use core::comm::PortSet;

pub struct lwc_string {
    string: ~str,
    length: uint,
    refcnt: u32,
    hash: u32,
    is_case_insensitive: bool,
    case_insensitive: Option<arc::RWARC<~lwc_string>>
}


pub struct lwc {
    priv bucketVector: ~([~[arc::RWARC<~lwc_string>]])
}

pub impl lwc {

    priv fn dolower(c: u8 ) -> char {
        if (c >= 'A' as u8 && c <= 'Z' as u8) {
              return (c as char + 'a' - 'A');
        }
        else {
            return c as char;
        }
    }

    priv fn lwc_calculate_hash(string: &str) -> u32 {
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

    priv fn lwc_calculate_lcase_hash(string: &str) -> u32 {
        let mut z: u32 = 0x811c9dc5;
        let mut i: uint = 0;
        let mut string_index = str::char_len(string);

        while string_index>0 {
            z = z*0x01000193;
            z = (z^lwc::dolower(string[i]) as u32);
            string_index = string_index-1;
            i = i+1;
        }
        z = z%4091;
        z
    }

    priv fn lwc_lcase_strncmp(s1: &str, s2: &str, n: uint) -> int {
        let mut i: uint = 0;
        let mut t = n;
        while t>0 {
            t = t-1;
            i = i+1;
            if s1[i] != lwc::dolower(s2[i]) as u8 {
                return 1;
            }
        }
        return 0;
    }

    priv fn lwc_lcase_memcpy(target: &str, source: &str, n: uint) {
        let mut i: uint = 0;
        let mut t =n;
        let mut str1: ~str = str::from_slice(source);
        let mut str2: ~str = str::from_slice(target);
        while t>0 {
            t = t - 1;
            i = i + 1;
            str2[i] = lwc::dolower(str1[i]) as u8;
        }
    }


    pub fn lwc_intern_string_vector(&mut self, string_list: ~[~str]) -> ~[arc::RWARC<~lwc_string>] {
        
        let mut num_threads : uint = 8;
        let num_strings = string_list.len();
        if (num_strings < num_threads) {
            num_threads = num_strings;
        }
        let size_of_slice : uint = (num_strings / num_threads) + 1;

        //io::println(fmt!("num_threads, num_strings, size_of_slice : %? %? %?", num_threads, num_strings, size_of_slice));

        let mut interned_string_list : ~[arc::RWARC<~lwc_string>] = ~[];
        let dummy_lwc_string = arc::RWARC(~lwc_string {
            string: ~"" , 
            length: 0 ,
            refcnt: 0 , 
            hash: 0 , 
            is_case_insensitive : false,
            case_insensitive: None
        });

        do vec::grow_fn(&mut interned_string_list, num_strings) |_| {
            dummy_lwc_string.clone()
        };

        // io::println(fmt!("len of interned_string_list: %?", interned_string_list.len()));

        let string_list_arc = arc::ARC(string_list);
        let p:PortSet<(uint, u32)> = PortSet();
        
        let mut thread_number = 0;
        
        loop {
            
            let (child_to_parent_port, child_to_parent_channel):
                (Port<(uint, u32)>, Chan<(uint, u32)>)
                = stream();
            
            p.add(child_to_parent_port);

            let string_list_arc_clone = string_list_arc.clone();
            let current_thread_number = thread_number;

            do task::spawn {
                let start_index = current_thread_number * size_of_slice;
                let mut end_index = (current_thread_number + 1) * size_of_slice - 1;
                if end_index >= num_strings {
                    end_index = num_strings-1;
                }

                // io::println(fmt!("current_thread_number, start_index, end_index: %? %? %?", current_thread_number, start_index, end_index));

                let mut send_count = 0;

                for uint::range(start_index, end_index+1) |index| {
                    let hash_value = lwc::lwc_calculate_hash(arc::get(&string_list_arc_clone)[index]);
                    //io::println(fmt!("sending (index,hash_value): (%?,%?)", index, hash_value));
                    child_to_parent_channel.send((index, hash_value));
                    send_count += 1;
                }
                // io::println(fmt!("current_thread_number, send_count: %? %?", current_thread_number, send_count));
            }
            
            thread_number += 1;
            
            if (thread_number > num_threads-1) {
                break;
            }
            
        }

        for uint::range(0,num_strings) |_/*recv_count*/| {
            let mut (index, hash_value) = p.recv();

            // io::println(fmt!("recv_count: %?", recv_count));
            // io::println(fmt!("receiving (index,hash_value): (%?,%?)", index, hash_value));

            let mut vector_index = self.bucketVector[hash_value].len();

            let copy_of_string_to_intern = copy arc::get(&string_list_arc)[index];
            let len = copy_of_string_to_intern.len();

            let lwc_string_to_intern = arc::RWARC(~lwc_string {
                string: copy copy_of_string_to_intern , 
                length: len ,
                refcnt: 1 , 
                hash: hash_value , 
                is_case_insensitive : false,
                case_insensitive: None
            });
            
            if vector_index == 0 {
                vec::push(&mut self.bucketVector[hash_value], lwc_string_to_intern.clone());
                interned_string_list[index] = lwc_string_to_intern;
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
                        interned_string_list[index] = self.bucketVector[hash_value][vector_index-1].clone();
                    }
                    
                    vector_index = vector_index-1;
                    
                }   
                vec::push(&mut self.bucketVector[hash_value], lwc_string_to_intern.clone());
                interned_string_list[index] = lwc_string_to_intern;
            }
        }

        interned_string_list
    }

    pub fn lwc_intern_string(&mut self, string_to_intern: ~str) -> arc::RWARC<~lwc_string> {
        self.__lwc_intern(string_to_intern, false)
    }

    priv fn __lwc_intern(&mut self , string_to_intern: ~str, case_insensitive:bool) -> arc::RWARC<~lwc_string> {
        
        let mut string_to_intern_actual : ~str; //= ~"";
        let string_to_intern_lcase = str::to_lower(string_to_intern);
        let mut hash_value : u32;//= 0u32;
        let mut is_case_insensitive : bool;//= case_insensitive;

        match (case_insensitive) {
            false=> {
                hash_value = lwc::lwc_calculate_hash(string_to_intern);
                string_to_intern_actual = string_to_intern;
                if (string_to_intern_actual==string_to_intern_lcase) {
                    is_case_insensitive = true;
                }
                else {
                    is_case_insensitive = false;
                }
            }
            true=> { 
                hash_value = lwc::lwc_calculate_lcase_hash(string_to_intern);
                string_to_intern_actual = string_to_intern_lcase;
                is_case_insensitive = true;
            }
        };
        

        let len = str::char_len(string_to_intern_actual);
        let mut vector_index = self.bucketVector[hash_value].len();

        let copy_of_string_to_intern = copy string_to_intern_actual;

        let lwc_string_to_intern = arc::RWARC(~lwc_string {
            string: string_to_intern_actual , 
            length: len ,
            refcnt: 1 , 
            hash: hash_value , 
            is_case_insensitive : is_case_insensitive,
            case_insensitive: None
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

    pub fn lwc_string_isequal(&self, str1: arc::RWARC<~lwc_string> , str2: arc::RWARC<~lwc_string>) ->bool {
        do str1.read |s1| {
            do str2.read |s2| {
                ptr::ref_eq(s1,s2)
            }
        }
    }

    pub fn lwc_intern_caseless_string(&mut self , string_to_intern: arc::RWARC<~lwc_string>) ->  arc::RWARC<~lwc_string> {
        let mut string = ~"";
        do string_to_intern.read |s| {
            string = copy s.string;
        }
        self.__lwc_intern(string, true)
    }


    pub fn lwc_string_caseless_isequal(&mut self, str1: arc::RWARC<~lwc_string> , str2: arc::RWARC<~lwc_string>) ->bool {
            
        let mut retVal: bool = false;   
        do str1.read |s1| {
            do str2.read |s2| {
                retVal = ptr::ref_eq(s1,s2);
            }
        }

        if retVal {
            return true;
        }

        let mut s1_c: Option<arc::RWARC<~lwc_string>> = None;
        let mut s2_c: Option<arc::RWARC<~lwc_string>> = None; 

        let mut string = ~"";

        let mut case_insensitive_is_none = false;
        let mut is_case_insensitive = false;

        do str1.read |s| {
            is_case_insensitive = (*s).is_case_insensitive;
            if !is_case_insensitive && (*s).case_insensitive.is_none() {
                string = copy s.string;
                case_insensitive_is_none = true;
            }
        }

        if (case_insensitive_is_none) {
                case_insensitive_is_none = false;
                let temp = self.__lwc_intern(copy string, true);
                s1_c = Some(temp.clone());
                do str1.write |s|  {
                    (*s).case_insensitive = Some(temp.clone());
            }
        }
        else if (is_case_insensitive) {
            is_case_insensitive = false;
            s1_c = Some(str1.clone());
        }
        else {
            do str1.write |s|  {
                let temp = (*s).case_insensitive.swap_unwrap();
                (*s).case_insensitive = Some(temp.clone());
                s1_c = Some(temp.clone());
            }
        }
        
        do str2.read |s| {
            is_case_insensitive = (*s).is_case_insensitive;
            if !is_case_insensitive && (*s).case_insensitive.is_none() {
                string = copy s.string;
                case_insensitive_is_none = true;
            }
        }

        if (case_insensitive_is_none) {
                let temp = self.__lwc_intern(string, true);
                s2_c = Some(temp.clone());
                do str2.write |s|  {
                    (*s).case_insensitive = Some(temp.clone());
            }
        }
        else if (is_case_insensitive) {
            // is_case_insensitive = false;
            s1_c = Some(str2.clone());
        }
        else {
            do str2.write |s|  {
                let temp = (*s).case_insensitive.swap_unwrap();
                (*s).case_insensitive = Some(temp.clone());
                s2_c = Some(temp.clone());
            }
        }

        let s1 = s1_c.unwrap();
        let s2 = s2_c.unwrap();

        self.lwc_string_isequal(s1,s2)
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

pub fn lwc()->arc::RWARC<~lwc> {
    
    let mut tempBucketVector: ~([~[arc::RWARC<~lwc_string>]]) = ~[];
    for uint::range(0, 4091) |_| {
        let bucket:~[arc::RWARC<~lwc_string>] = ~[];
        tempBucketVector.push(bucket);
    }

    arc::RWARC(~lwc {
        bucketVector:tempBucketVector
    })
}

