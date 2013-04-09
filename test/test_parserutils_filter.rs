
extern mod std;
extern mod parserutils;
extern mod parserutils_filter;

use parserutils::*;
use parserutils_filter::*;

fn main()
{				
	let mut parser = lpu();
	let (filterInstance, filterResult) : (Option<~lpu_filter> , parserutils_filter_result) = lpu_filter(parser, ~"UTF-8");
	
	match(filterResult)
	{
		PARSERUTILS_FILTER_CREATE_OK   => {
											io::println("Pass:  Filter created");
											}
		_                              => {
											io::println("Fail : Filter not created");
											}
	}
}
