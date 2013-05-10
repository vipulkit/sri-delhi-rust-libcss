extern mod std;

use core::io::Reader;
use core::io::ReaderUtil;
use core::hashmap::linear::LinearMap;
use core::vec::*;
use std::arc;

pub struct parserutils_charset_aliases_canon {
    mib_enum:u16,
    name_len:u16,
    name: ~str
}

pub enum parserutils_error {
    PARSERUTILS_OK = 0,
    PARSERUTILS_BADPARAM = 1,
    PARSERUTILS_NOMEM = 2,
    PARSERUTILS_EOF = 3,
    PARSERUTILS_BADENCODING = 4,
    PARSERUTILS_NEEDDATA = 5,
    PARSERUTILS_INVALID = 6,
    PARSERUTILS_ICONV_ERROR = 8,
    PARSERUTILS_SUCCESS = 9
}

pub struct alias {
    // these two data structures together can be used for mibenum->canonical name conversion
    canonical_name_list: ~[~str],
    mibenum_map: ~LinearMap<u16,uint>,
    // this data structure can be used for name (canonnical/alias) ->mibenum conversion
    alias_map: ~LinearMap<~str,u16>
}

pub fn memcmp(str1 : &~[u8] , str2 : &[u8] , len : uint ) -> int {
    let mut i : uint = 0 ;
    while ( i<len ) {
        if str1[i] != str2[i] {
            return ( (str1[i]-str2[i]) as int) ;
        }
        i = i+1 ; 
    }
    0
}

impl alias {

    fn read_aliases(&mut self) {
        let aliases_file_reader: @Reader = (&io::file_reader(&Path(&"Aliases"))).unwrap();

        let mut line_number=1;

        while !aliases_file_reader.eof() {
            let line = aliases_file_reader.read_line();
            if (!str::starts_with(line,"#") && line.len()>0) {
                let mut alias_entry_columns : ~[~str] = ~[];
                for str::each_split_str_nonempty(line,"\t") |column| {
                    alias_entry_columns.push(column.to_owned());
                } 
                
                // first column is canonical name
                let canonical_name = copy alias_entry_columns[0];
                // second column is mibenum
                let mibenum = u16::from_str(alias_entry_columns[1]).unwrap();
                
                // add the canonical name to the list of canonical names
                self.canonical_name_list.push(copy canonical_name);
                // insert <mibenum, index of canonical name> into mibenum_map
                self.mibenum_map.insert(mibenum,line_number-1);
                // insert <canonical_name, mibenum> into alias_map
                self.alias_map.insert(canonical_name, mibenum);

                // optionally, the third column has other aliases
                if (alias_entry_columns.len() > 2) {
                    //let aliases=str::split_str_nonempty(alias_entry_columns[2]," ");
                    let mut aliases : ~[~str] = ~[];
                    for str::each_split_str_nonempty(alias_entry_columns[2]," ") |alias| {
                        aliases.push(alias.to_owned());
                    } 
                    // insert <alias, mibenum> into alias_map
                    for aliases.each |&alias| {
                        self.alias_map.insert(alias.to_lower(), mibenum);
                    }
                }
                line_number=line_number+1;
            }
        }
    }

    pub fn parserutils__charset_alias_canonicalise(&self, alias: ~str) -> Option<parserutils_charset_aliases_canon> {       
        match self.alias_map.find(&alias) {         
            None => None,                   
            Some(temp_mib_enum) => {
                match self.mibenum_map.find(temp_mib_enum) {
                    None => None,                   
                    Some(canonical_name_list_index) => {
                        if (*canonical_name_list_index < self.canonical_name_list.len()) {
                            
                            let temp_name = copy (self.canonical_name_list[*canonical_name_list_index]);
                            let temp_name_len = temp_name.len() as u16;
                            Some( parserutils_charset_aliases_canon {
                                    mib_enum: *temp_mib_enum,
                                    name: temp_name,
                                    name_len: temp_name_len
                                }
                            )
                        }
                        else {
                            None
                        }
                    }
                }
            }
        }
    }

    pub fn parserutils_charset_mibenum_from_name(&self, alias: ~str) -> u16 {
        match self.alias_map.find(&alias) {
            None => 0 ,
            Some(mib_enum) => *mib_enum
        }
    }

    pub fn parserutils_charset_mibenum_to_name(&self, mibenum: u16)-> Option<~str> {
        match self.mibenum_map.find(&(mibenum)) {
            None => None,
            Some (canonical_name_list_index) => {
                if canonical_name_list_index < &self.canonical_name_list.len() {
                    Some(copy self.canonical_name_list[*canonical_name_list_index])
                }
                else {
                    None
                }
            }
        }
    }
    
} //impl alias

pub fn alias() -> arc::ARC<~alias> {
    let mut new_alias = ~alias {
        canonical_name_list : ~[],
        mut mibenum_map : ~LinearMap::new(),
        mut alias_map : ~LinearMap::new()
    };

    new_alias.read_aliases();
    arc::ARC(new_alias)
}
