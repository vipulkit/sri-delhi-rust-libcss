
extern mod std;
extern mod parserutils;
extern mod parserutils_filter;
//extern mod test;

use parserutils::*;
use parserutils_filter::*;
use core::vec::*;

fn main()
{				
	let mut parser = lpu();
	let (filterInstance, filterResult) : (Option<~lpu_filter> , parserutils_error) = lpu_filter(parser, ~"UTF-8");
	let mut filter: ~lpu_filter;
	match(filterResult)
	{
		PARSERUTILS_OK   => {
				io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>lpu_filter>>Pass:  Filter created");
				filter = filterInstance.unwrap();
				match(filter.filter_set_encoding(~"UTF-8"))
				{
					PARSERUTILS_OK=>io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>filter_set_encoding>>Pass: "),
					_=>io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>filter_set_encoding>>fail: ")
				}
				let mut inbuf:~[u8]= (~"hell\xc2\xa0o!").to_bytes();
				let mut outbuf:~[u8]=~[];
				let mut processedLen:uint;
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return1>>fail: ");
					}
				}

				if eq(outbuf,"hell\xc2\xa0o!".to_bytes())
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf1>>pass: ");
				}
				else
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf1>>fail: ");
				}

				//filter.parserutils__filter_reset();





				 inbuf= (~"hello!").to_bytes();
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 2>>fail: ");
					}
				}

				if eq(outbuf,"hello!".to_bytes())
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf2>>pass: ");
				}
				else
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf2>>fail: ");
				}

				//filter.parserutils__filter_reset();



				inbuf= (~"hell\x96o!").to_bytes();
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 3>>fail: ");
					}
				}

				if eq(outbuf,"hell\xef\xbf\xbdo!".to_bytes())
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf3>>pass: ");
				}
				else
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf3>>fail: ");
				}

				//filter.parserutils__filter_reset();




				inbuf= (~"hell\xc2\xa0o!").to_bytes();
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						io::println(fmt!("outbuf=%?",outbuf));
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 4>>fail: ");
					}
				}
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf = copy processed_chunk.outbuf;
						io::println(fmt!("outbuf=%?",outbuf));
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 4>>fail: ");
					}
				}

				if eq(outbuf,"hell\xc2\xa0o!".to_bytes())
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf4>>pass: ");
				}
				else
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf4>>fail: ");
				}

				/*match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_reset4>>pass: "),
					_=> io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_reset>>fail: ")
				}*/





				inbuf= (~"hell\xc2\xc2o!").to_bytes();
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 5>>fail: ");
					}
				}
				outbuf=~[];
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 5>>fail: ");
					}
				}

				if eq(outbuf,"hell\xef\xbf\xbd\xef\xbf\xbdo!".to_bytes())
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf5>>pass: ");
				}
				else
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf5>>fail: ");
				}

				/*match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_reset5>>pass: "),
					_=> io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_reset5>>fail: ")
				}*/





				inbuf= (~"hell\xc2\xa0\xc2\xa1o!").to_bytes();
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-5).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 6>>fail: ");
					}
				}
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 6>>fail: ");
					}
				}

				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 6>>fail: ");
					}
				}

				if eq(outbuf,"hell\xc2\xa0\xc2\xa1o!".to_bytes())
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf6>>pass: ");
				}
				else
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf6>>fail: ");
				}

				/*match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_reset6>>pass: "),
					_=> io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_reset6>>fail: ")
				}*/






				inbuf= (~"hell\xe2\x80\xa2o!").to_bytes();
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-4).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 7>>fail: ");
					}
				}
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf.slice(0,inbuf.len()-3).to_owned())) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 7>>fail: ");
					}
				}
				 outbuf=~[];
				
				match(filter.parserutils__filter_process_chunk(inbuf)) { 
					(processed_chunk , PARSERUTILS_OK) => {
						outbuf += processed_chunk.outbuf;
						processedLen = processed_chunk.len_processed as uint;
					},
					(_ , y) => {
						io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk return 7>>fail: ");
					}
				}

				if eq(outbuf,"hell\xe2\x80\xa2o!".to_bytes())
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf7>>pass: ");
				}
				else
				{
					io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_process_chunk outbuf7>>fail: ");
				}

				/*match(filter.parserutils__filter_reset())
				{
					PARSERUTILS_OK => io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_reset7>>pass: "),
					_=> io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>parserutils__filter_reset7>>fail: ")
				}
*/

				
		}
		_  => {
				io::println("test file>> test_parser_utils_filter>> file to test>> parser_utils_filter.rs>> functn>>lpu_filter>>Fail : Filter not created");
		}
	}
}
