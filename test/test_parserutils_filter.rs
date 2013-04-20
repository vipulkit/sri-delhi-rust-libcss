
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
					   PARSERUTILS_OK  =>  test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"filter_set_encoding", ~"test_parserUtils_filter", ~"UTF-8"),
					   _=>	test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"filter_set_encoding", ~"test_parserUtils_filter", ~"UTF-8 not created"),			
				}

				let mut inbuf:~[u8]= (~"hell\xc2\xa0o!").to_bytes();
				let mut outbuf:~[u8]=~[];
				//let mut processedLen:uint;

				match(filter.parserutils__filter_process_chunk(inbuf))
				 { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter", ~"hell\xc2\xa0o!");
					}
				}

				if eq(outbuf,"hell\xc2\xa0o!".to_bytes())
				{
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"test_parserUtils_filter_line1.1", ~"hell\xc2\xa0o!");
				}
				else
				{					
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"test_parserUtils_filter_line1.2", ~"hell\xc2\xa0o!");
				}

				filter.parserutils__filter_reset();

				 inbuf = (~"hello!").to_bytes();
				 outbuf = ~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {					
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line2.1", ~"hello!");
					}
				}

				if eq(outbuf,"hello!".to_bytes())
				{					
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"test_parserUtils_filter_line2.2", ~"hello!");
				}
				else
				{					
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"test_parserUtils_filter_line2.3", ~"hello!");
				}

				filter.parserutils__filter_reset();


				inbuf= (~"hell\x96o!").to_bytes();
				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line3.1", ~"hell\x96o!");
					}
				}

				if eq(outbuf,"hell\xef\xbf\xbdo!".to_bytes())
				{					
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"test_parserUtils_filter_line3.2", ~"hell\x96o!");
				}
				else
				{					
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk ", ~"test_parserUtils_filter_line3.3", ~"hell\x96o!");
				}

				filter.parserutils__filter_reset();

				inbuf = (~"hell\xc2\xa0o!").to_bytes();
				outbuf = ~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						io::println(fmt!("outbuf=%?",outbuf));
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line4.1", ~"hell\xc2\xa0o!");
					}
				}
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf = copy processed_chunk.outbuf;
						io::println(fmt!("outbuf=%?",outbuf));
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line4.2", ~"hell\xc2\xa0o!");	
					}
				}

				if eq(outbuf,"hell\xc2\xa0o!".to_bytes())
				{					
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line4.3", ~"hell\xc2\xa0o!");	
				}
				else
				{					
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line4.4", ~"hell\xc2\xa0o!");	
				}

				match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~""),																	
					    _          => test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~"")
				}

				inbuf= (~"hell\xc2\xc2o!").to_bytes();
				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line5.1", ~"hell\xc2\xc2o!");
					}
				}
				outbuf=~[];
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {						
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line5.2", ~"hell\xc2\xc2o!");	
					}
				}

				if eq(outbuf,"hell\xef\xbf\xbd\xef\xbf\xbdo!".to_bytes())
				{					
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line5.3", ~"hell\xef\xbf\xbd\xef\xbf\xbdo!");	
				}
				else
				{				
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line5.4", ~"hell\xef\xbf\xbd\xef\xbf\xbdo!");	
				}

				match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~""),																	
					    _          => test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~"")
				}


				inbuf= (~"hell\xc2\xa0\xc2\xa1o!").to_bytes();
				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-5).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {	
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line6.1", ~"hell\xc2\xa0\xc2\xa1o!");
					}
				}

				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {	
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line6.2", ~"hell\xc2\xa0\xc2\xa1o!");
					}
				}

				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line6.3", ~"hell\xc2\xa0\xc2\xa1o!");
					}
				}

				if eq(outbuf,"hell\xc2\xa0\xc2\xa1o!".to_bytes())
				{
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line6.4", ~"hell\xc2\xa0\xc2\xa1o!");
				}
				else
				{
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line6.5", ~"hell\xc2\xa0\xc2\xa1o!");
				}

				match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~""),																
					    _          => test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~"")
				}


				inbuf= (~"hell\xe2\x80\xa2o!").to_bytes();
				outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-4).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {			
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line7.1", ~"hell\xe2\x80\xa2o!");
					}
				}
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {	
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line7.2", ~"hell\xe2\x80\xa2o!");
					}
				}
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						//processedLen = processed_chunk.len_processed as uint;
					},
					(_ , _) => {
						test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line7.3", ~"hell\xe2\x80\xa2o!");
					}
				}

				if eq(outbuf,"hell\xe2\x80\xa2o!".to_bytes())
				{
					test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line7.4", ~"hell\xe2\x80\xa2o!");
				}
				else
				{
					test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_process_chunk", ~"test_parserUtils_filter_line7.5", ~"hell\xe2\x80\xa2o!");
				}

				match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => 	test_logger.pass( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~""),																	
					    _          => 	test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"parserutils__filter_reset", ~"test_parserUtils_filter", ~"")
				}				
		}
		_  => {
				test_logger.fail( ~"parserutils", ~"parser_utils_filter.rs", ~"lpu_filter", ~"", ~"Filter not created");
			  }
	}
}
