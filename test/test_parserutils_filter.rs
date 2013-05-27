
extern mod std;
extern mod parserutils;

extern mod test;

use std::arc;
use core::vec::*;
use parserutils::charset::aliases::*;
use parserutils::input::parserutils_filter::*;
use parserutils::utils::errors::*;


use test::*;

fn main(){					

	let mut external_argument : ~str = ~"";
	let mut Alias = alias();
	
	let mut (filterinstance,filterResult) = parserutils_filter(Alias, ~"UTF-8");


	// Log file 
	let mut test_logger = result::unwrap(test_report(&"Unit_test_report.csv"));

	match(filterResult){
		PARSERUTILS_OK   => {								
				test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"lpu_filter", ~"filter creation with mibenum UTF8",~"Filter is to be created", ~"Filter is created",~"");

				let mut Filter = filterinstance.unwrap();

				match(Filter.filter_set_encoding(~"UTF-8"))	{
					   PARSERUTILS_OK  =>  test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"filter_set_encoding", ~"UTF8",~"PARSERUTILS_OK", ~"PARSERUTILS_OK", ~""),
					              _    =>  test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"filter_set_encoding", ~"UTF8",~"PARSERUTILS_OK", ~"Non PARSERUTILS_OK", ~""),			
				}

				let mut inbuf:~[u8]= (~"hell\xc2\xa0o!").to_bytes();
				let mut outbuf:~[u8]=~[];
				//let mut processedLen:uint;

				match(Filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk",  ~"checking return type for error",~"PARSERUTILS_OK",~"Error condition",~"");
					}
				}

				if eq(outbuf,"hell\xc2\xa0o!".to_bytes()){
					test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"hell\xc2\xa0o!", ~"hell\xc2\xa0o!", str::from_bytes(outbuf), ~"");
				}
				else{					
					test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"hell\xc2\xa0o!", ~"hell\xc2\xa0o!", str::from_bytes(outbuf), ~"");
				}

				Filter.parserutils__filter_reset();

				 inbuf = (~"hello!").to_bytes();
				 outbuf = ~[];
				
				match(Filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {					
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line2.1", ~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK", ~"");
					}
				}

				if eq(outbuf,"hello!".to_bytes()){					
					test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"hello!", ~"hello!", str::from_bytes(outbuf),~"");
				}
				else{					
					test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"hello!", ~"hello!", str::from_bytes(outbuf),~"");
				}

				Filter.parserutils__filter_reset();


				inbuf= (~"hell\x96o!").to_bytes();
				outbuf=~[];
				
				match(Filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line3.1", ~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK", ~"");
					}
				}

				if eq(outbuf,"hell\xef\xbf\xbdo!".to_bytes()){					
					test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"hell\xef\xbf\xbdo!", ~"hell\xef\xbf\xbdo!", str::from_bytes(outbuf),~"");
				}
				else{					
					test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"hell\xef\xbf\xbdo!", ~"hell\xef\xbf\xbdo!", str::from_bytes(outbuf),~"");
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
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line4.1", ~"PARSERUTILS_OK", ~"Non PARSERUTILS_OK", ~"");
					}
				}
				match(Filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf = copy processed_chunk.outbuf;
						io::println(fmt!("outbuf=%?",outbuf));
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line4.2", ~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK",~"");	
					}
				}

				if eq(outbuf,"hell\xc2\xa0o!".to_bytes()){					
					test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0o!", ~"hell\xc2\xa0o!", str::from_bytes(outbuf), ~"");	
				}
				else{					
					test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0o!", ~"hell\xc2\xa0o!", str::from_bytes(outbuf), ~"");	
				}

				match(Filter.parserutils__filter_reset()){
					PARSERUTILS_OK => test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~"PARSERUTILS_OK", ~" PARSERUTILS_OK", ~""),																	
					    _          => test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter",~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK", ~"")
				}

				inbuf= (~"hell\xc2\xc2o!").to_bytes();
				outbuf=~[];
				
				match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line5.1", ~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK", ~"");
					}
				}

				outbuf=~[];
				match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line5.2", ~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK", ~"");	
					}
				}

				if eq(outbuf,"hell\xef\xbf\xbd\xef\xbf\xbdo!".to_bytes()){					
					test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xef\xbf\xbd\xef\xbf\xbdo!", ~"hell\xef\xbf\xbd\xef\xbf\xbdo!", str::from_bytes(outbuf), ~"");	
				}
				else{				
					test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xef\xbf\xbd\xef\xbf\xbdo!", ~"hell\xef\xbf\xbd\xef\xbf\xbdo!", str::from_bytes(outbuf), ~"");	
				}

				match(Filter.parserutils__filter_reset()){
					PARSERUTILS_OK => test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~"PARSERUTILS_OK", ~" PARSERUTILS_OK", ~""),																	
					    _          => test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK", ~"")
				}

				inbuf= (~"hell\xc2\xa0\xc2\xa1o!").to_bytes();
				outbuf=~[];
				
				match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-5).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {	
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0\xc2\xa1o!test_parserUtils_filter_line6.1", ~"hell\xc2\xa0\xc2\xa1o!", str::from_bytes(outbuf), ~"");
					}
				}

				outbuf=~[];
				
				match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {	
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0\xc2\xa1o!test_parserUtils_filter_line6.2", ~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK",~"");
					}
				}

				outbuf=~[];
				
				match(Filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0\xc2\xa1o!_test_parserUtils_filter_line6.3", ~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK",~"");
					}
				}

				if eq(outbuf,"hell\xc2\xa0\xc2\xa1o!".to_bytes()){
					test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0\xc2\xa1o!", ~"hell\xc2\xa0\xc2\xa1o!", str::from_bytes(outbuf), ~"");
				}
				else{
					test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0\xc2\xa1o!", ~"hell\xc2\xa0\xc2\xa1o!", str::from_bytes(outbuf), ~"");
				}

				match(Filter.parserutils__filter_reset()){
					PARSERUTILS_OK => test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter",~"PARSERUTILS_OK", ~"PARSERUTILS_OK", ~""),																
					    _          => test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK",~"")
				}


				inbuf= (~"hell\xe2\x80\xa2o!").to_bytes();
				outbuf=~[];
				
				match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-4).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {			
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xe2\x80\xa2o!_test_parserUtils_filter_line7.1", ~"PARSERUTILS_OK", ~" Non PARSERUTILS_OK",~"");
					}
				}
				outbuf=~[];
				
				match(Filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
					},
					(_ , _) => {
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xe2\x80\xa2o!_test_parserUtils_filter_line7.2", ~"PARSERUTILS_OK", ~"PARSERUTILS_OK", ~"");
					}
				}
				outbuf=~[];
				
				match(Filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;									
					},
					(_ , _)	=> {
						test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xe2\x80\xa2o!_ est_parserUtils_filter_line7.3", ~"PARSERUTILS_OK", ~"Non PARSERUTILS_OK", ~"");
					}
				}

				if eq(outbuf,"hell\xe2\x80\xa2o!".to_bytes()){
					test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xe2\x80\xa2o!", ~"hell\xe2\x80\xa2o!", str::from_bytes(outbuf),~"");
				}
				else{
					test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xe2\x80\xa2o!", ~"hell\xe2\x80\xa2o!", str::from_bytes(outbuf),~"");
				}

				match(Filter.parserutils__filter_reset()){
					PARSERUTILS_OK => 	test_logger.pass( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~"PARSERUTILS_OK", ~"PARSERUTILS_OK", ~""),																
					    _          => 	test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter",  ~"PARSERUTILS_OK", ~"Non PARSERUTILS_OK",~"")
				}				
		}
		_  => {
				test_logger.fail( ~"test_parserUtils_filter.rs", copy external_argument, ~"parserutils", ~"parser_utils_filter.rs", ~"lpu_filter", ~"", ~"", ~"", ~"Filter not created");
			  }
	}
}
