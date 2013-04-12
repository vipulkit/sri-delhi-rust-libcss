
extern mod std;
extern mod parserutils;
extern mod parserutils_filter;
extern mod test;

use std::arc;
use parserutils::*;
use parserutils_filter::*;
use core::vec::*;
use test::*;

fn main()
{					
	let mut parser : arc::ARC<~lpu> = lpu();
	let (filterInstance, filterResult) : (Option<~lpu_filter> , parserutils_error) = lpu_filter(parser, ~"UTF-8");
	let mut filter: ~lpu_filter;

	// Log file 
	let mut test_logger = result::unwrap(test_report(&"temp_log.csv"));

	match(filterResult)
	{
		PARSERUTILS_OK   => {								
				test_logger.info( ~"parserutils", ~"parser_utils_filter.rs", ~"lpu_filter", ~"", ~"Filter is created");

				filter = filterInstance.unwrap();

				match(filter.filter_set_encoding(~"UTF-8"))
				{
					   SERUTILS_OK  =>  test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"filter_set_encoding", ~"UTF-8", ~""),				
				}

				let mut inbuf:~[u8]= (~"hell\xc2\xa0o!").to_bytes();
				let mut outbuf:~[u8]=~[];
				let mut processedLen:uint;

				match(filter.parserutils__filter_process_chunk(inbuf))
				 { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0o!", ~"");
					}
				}

				if eq(outbuf,"hell\xc2\xa0o!".to_bytes())
				{
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk outbuf1", ~"hell\xc2\xa0o!", ~"");
				}
				else
				{					
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk outbuf1", ~"hell\xc2\xa0o!", ~"");
				}

				filter.parserutils__filter_reset();

				 inbuf = (~"hello!").to_bytes();
				 outbuf = ~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {					
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hello!", ~"");
					}
				}

				if eq(outbuf,"hello!".to_bytes())
				{					
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk outbuf2", ~"hello!", ~"");
				}
				else
				{					
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk outbuf2", ~"hello!", ~"");
				}

				filter.parserutils__filter_reset();


				inbuf= (~"hell\x96o!").to_bytes();
				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\x96o!", ~"");
					}
				}

				if eq(outbuf,"hell\xef\xbf\xbdo!".to_bytes())
				{					
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk outbuf3", ~"hell\x96o!", ~"");
				}
				else
				{					
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk outbuf3", ~"hell\x96o!", ~"");
				}

				filter.parserutils__filter_reset();

				inbuf = (~"hell\xc2\xa0o!").to_bytes();
				outbuf = ~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						io::println(fmt!("outbuf=%?",outbuf));
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0o!", ~"");
					}
				}
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf = copy processed_chunk.outbuf;
						io::println(fmt!("outbuf=%?",outbuf));
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0o!", ~"");	
					}
				}

				if eq(outbuf,"hell\xc2\xa0o!".to_bytes())
				{					
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0o!", ~"");	
				}
				else
				{					
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0o!", ~"");	
				}

				match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"", ~""),																	
					    _          => test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"", ~"")
				}

				inbuf= (~"hell\xc2\xc2o!").to_bytes();
				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xc2o!", ~"");
					}
				}
				outbuf=~[];
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xc2o!", ~"");	
					}
				}

				if eq(outbuf,"hell\xef\xbf\xbd\xef\xbf\xbdo!".to_bytes())
				{					
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xef\xbf\xbd\xef\xbf\xbdo!", ~"");	
				}
				else
				{				
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xef\xbf\xbd\xef\xbf\xbdo!", ~"");	
				}

				match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"", ~""),																	
					    _          => test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"", ~"")
				}


				inbuf= (~"hell\xc2\xa0\xc2\xa1o!").to_bytes();
				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-5).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {	
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0\xc2\xa1o!", ~"");
					}
				}

				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {	
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0\xc2\xa1o!", ~"");
					}
				}

				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0\xc2\xa1o!", ~"");
					}
				}

				if eq(outbuf,"hell\xc2\xa0\xc2\xa1o!".to_bytes())
				{
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0\xc2\xa1o!", ~"");
				}
				else
				{
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xc2\xa0\xc2\xa1o!", ~"");
				}

				match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"", ~""),																
					    _          => test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"", ~"")
				}


				inbuf= (~"hell\xe2\x80\xa2o!").to_bytes();
				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-4).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {			
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xe2\x80\xa2o!", ~"");
					}
				}
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {	
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xe2\x80\xa2o!", ~"");
					}
				}
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xe2\x80\xa2o!", ~"");
					}
				}

				if eq(outbuf,"hell\xe2\x80\xa2o!".to_bytes())
				{
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xe2\x80\xa2o!", ~"");
				}
				else
				{
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"hell\xe2\x80\xa2o!", ~"");
				}

				match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => 	test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"", ~""),																	
					    _          => 	test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"", ~"")
				}				
		}
		_  => {
				test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"lpu_filter", ~"", ~"Filter not created");
			  }
	}
}
