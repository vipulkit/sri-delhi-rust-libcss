use std::libc::{c_int, size_t};
use std::vec::*;
use std::vec::raw::*;

struct  chunk_result {
    outbuf : ~[u8] ,
    len_processed : u64 ,
    err_state : int , 
    ret_val : u64
}

mod iconv_wrapper {

    use std::libc::*;

    extern {
        fn AllocateBuffer(bytes:c_int) -> *u8 ;
        fn DeallocateBuffer(buffer:*u8)  ;
        fn rust_iconv_open( tocode: * u8 , fromcode : * u8) -> u64;
        fn rust_iconv(s: u64, inbuf : **u8 , insize : * size_t , outbuf : ** u8 , outsize : * size_t , error : *c_int ) -> size_t ;
        fn rust_iconv_close(s: u64) -> c_int ;
    }
}

/* Safe functions */


pub fn riconv_initialized(hnd : u64) -> bool {
    let nptr : u64 = -1 as u64;
    hnd != nptr
}

pub fn riconv_initialize() -> u64 {
    -1 as u64
}

#[fixed_stack_segment]
pub fn safe_riconv_open( tocode: ~str , fromcode : ~str ) -> u64 {
    unsafe {
        let tobytes = tocode.into_bytes() ;
        let frombytes = fromcode.into_bytes();
        iconv_wrapper::rust_iconv_open( to_ptr(tobytes) ,  to_ptr(frombytes) ) 
    }
}

#[fixed_stack_segment]
pub fn safe_riconv (hnd : u64, inbuf : &[u8] ) -> (~[u8], u64, int) {
    unsafe {
        let err : c_int = 0 ;
        if inbuf.len()==0 {
            (~[], 0, 0)
        }
        else {
            let c_insize : size_t = inbuf.len() as  u64;
            let c_outsize : size_t = (c_insize+1)*4 ;

            let c_output : *u8 = iconv_wrapper::AllocateBuffer( c_outsize as c_int );

            let c_inargs = c_insize ;
            
            let c_ouargs = c_outsize ;
            let c_oubuf = c_output ;

            iconv_wrapper::rust_iconv(hnd , &to_ptr(inbuf) /*c_input*/, &c_inargs  , &c_output , &c_ouargs , &err ) ;
            let outbuf = from_buf(c_oubuf,(c_outsize- c_ouargs)as uint); ;
            let len_processed=c_insize-c_inargs ;
           
            iconv_wrapper::DeallocateBuffer(c_oubuf);
            (outbuf, len_processed, err as int)
        }
    }
}

#[fixed_stack_segment]
pub fn safe_riconv_close(hnd: u64) -> int {
    unsafe {
        // io::println(fmt!("\n ICONV_CRATE::In safe_riconv_close called = %? ", hnd));
        iconv_wrapper::rust_iconv_close(hnd) as int
    }
}




