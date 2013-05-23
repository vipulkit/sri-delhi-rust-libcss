extern mod std;
extern mod parserutils;

use core::vec::*;
use parserutils::charset::aliases::*;
use parserutils::input::parserutils_filter::*;

fn main(){                  

    let mut Alias = alias();
    
    let mut (filterinstance,filterResult) = parserutils_filter(Alias, ~"UTF-8");

    match(filterResult){
        PARSERUTILS_OK   => {                               
                let mut Filter = filterinstance.unwrap();

                match(Filter.filter_set_encoding(~"UTF-8")) {
                    PARSERUTILS_OK  =>  {},
                    _   =>  assert!(false)          
                }

                let mut inbuf:~[u8]= (~"hell\xc2\xa0o!").to_bytes();
                let mut outbuf:~[u8]=~[];
                //let mut processedLen:uint;

                match(Filter.parserutils__filter_process_chunk(inbuf)) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {                        
                        assert!(false)
                    }
                }

                if eq(outbuf,"hell\xc2\xa0o!".to_bytes()){
                    
                }
                else{                   
                    assert!(false)
                }

                Filter.parserutils__filter_reset();

                inbuf = (~"hello!").to_bytes();
                outbuf = ~[];
                
                match(Filter.parserutils__filter_process_chunk(inbuf)) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {                    
                        assert!(false)
                    }
                }

                if !eq(outbuf,"hello!".to_bytes()){                  
                    assert!(false);
                }

                Filter.parserutils__filter_reset();


                inbuf= (~"hell\x96o!").to_bytes();
                outbuf=~[];
                
                match(Filter.parserutils__filter_process_chunk(inbuf)) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {                        
                        assert!(false);
                    }
                }

                if !eq(outbuf,"hell\xef\xbf\xbdo!".to_bytes()){                  
                    assert!(false);
                }

                Filter.parserutils__filter_reset();

                inbuf = (~"hell\xc2\xa0o!").to_bytes();
                outbuf = ~[];
                
                match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {                        
                        assert!(false);
                    }
                }
                match(Filter.parserutils__filter_process_chunk(inbuf)) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf = copy processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {                        
                        assert!(false);   
                    }
                }

                if !eq(outbuf,"hell\xc2\xa0o!".to_bytes()){                  
                    assert!(false);  
                }

                match(Filter.parserutils__filter_reset()){
                    PARSERUTILS_OK => {},                                                                    
                    _ => assert!(false)
                }

                inbuf= (~"hell\xc2\xc2o!").to_bytes();
                outbuf=~[];
                
                match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {                        
                        assert!(false);
                    }
                }

                outbuf=~[];
                match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {                        
                        assert!(false);  
                    }
                }

                if !eq(outbuf,"hell\xef\xbf\xbd\xef\xbf\xbdo!".to_bytes()){                  
                    assert!(false)  
                }

                match(Filter.parserutils__filter_reset()){
                    PARSERUTILS_OK => {},                                                                    
                    _ => assert!(false)
                }

                inbuf= (~"hell\xc2\xa0\xc2\xa1o!").to_bytes();
                outbuf=~[];
                
                match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-5).to_owned())) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {    
                        assert!(false)
                    }
                }

                outbuf=~[];
                
                match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {    
                        assert!(false)
                    }
                }

                outbuf=~[];
                
                match(Filter.parserutils__filter_process_chunk(inbuf)) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {
                        assert!(false)
                    }
                }

                if !eq(outbuf,"hell\xc2\xa0\xc2\xa1o!".to_bytes()){
                    assert!(false);
                }

                match(Filter.parserutils__filter_reset()) {
                    PARSERUTILS_OK => {},                                                              
                    _ => assert!(false)
                }


                inbuf= (~"hell\xe2\x80\xa2o!").to_bytes();
                outbuf=~[];
                
                match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-4).to_owned())) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                        //processedLen = processed_chunk.len_processed as uint;
                    },
                    (_ , _) => {            
                        assert!(false);
                    }
                }
                outbuf=~[];
                
                match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));
                    },
                    (_ , _) => {
                        assert!(false);
                    }
                }
                outbuf=~[];
                
                match(Filter.parserutils__filter_process_chunk(inbuf)) { 
                    (processed_chunk , PARSERUTILS_OK) => {
                        outbuf += processed_chunk.outbuf;
                        io::println(fmt!("outbuf=%?",outbuf));                                   
                    },
                    (_ , _) => {
                        assert!(false);
                    }
                }

                if !eq(outbuf,"hell\xe2\x80\xa2o!".to_bytes()) {
                    assert!(false);
                }

                match(Filter.parserutils__filter_reset()){
                    PARSERUTILS_OK =>   {},                                                               
                    _          =>   assert!(false)
                }               
        }
        _  => {
                assert!(false)
        }
    }
}
