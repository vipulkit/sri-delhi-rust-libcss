use std::libc::*;


struct  chunk_result {
    outbuf : ~[u8] ,
    len_processed : u64 ,
    err_state : int , 
    ret_val : u64
}

mod iconv_wrapper {
 extern {
        fn AllocateBuffer(bytes:c_int) -> *u8 ;
        fn DeallocateBuffer(buffer:*u8)  ;
        fn rust_iconv_open( tocode: * u8 , fromcode : * u8) -> u64;
        fn rust_iconv(s: u64, inbuf : **u8 , insize : * size_t , outbuf : ** u8 , outsize : * size_t , error : * int ) -> size_t ;
        fn rust_iconv_close(s: u64) -> c_int ;
    }
}

/* Safe functions */
pub fn copy_rust_to_c(r_ptr : ~[u8] , c_ptr : *u8 , len : uint ) {
    // Caution :: Allocate C array before copying 
    unsafe {
        let dptr = ::cast::transmute_mut_unsafe(c_ptr);
        cast::forget(dptr);
        ptr::copy_memory(dptr, vec::raw::to_ptr(r_ptr), len);
    }
}

pub fn riconv_initialized(hnd : u64) -> bool {
    unsafe {
        let nptr : u64 = -1 as u64;
        // io::println(fmt!("\n ICONV_CRATE::In riconv_initialized = %? = %? ", nptr,hnd));
        hnd != nptr
    }
}

pub fn riconv_initialize() -> u64 {
    unsafe {
        // io::println(fmt!("\n ICONV_CRATE::In riconv_initialize  "));
        -1 as u64
    }
}

pub fn safe_riconv_open( tocode: &str , fromcode : &str ) -> u64 {
    unsafe {
        let tobytes = str::to_bytes(tocode) ;
        let frombytes = str::to_bytes(fromcode) ;
        iconv_wrapper::rust_iconv_open( vec::raw::to_ptr(tobytes) ,  vec::raw::to_ptr(frombytes) ) 
    }
}

pub fn safe_riconv (hnd : u64, inbuf : &[u8] ) -> (~[u8], u64, int) {
    unsafe {
        let mut err : int = 0 ;
        if inbuf.len()==0 {
            (~[], 0, 0)
        }
        else {
            let c_insize : size_t = inbuf.len() as  u64;
            let c_outsize : size_t = (c_insize+1)*4 ;

            let mut c_output : *u8 = iconv_wrapper::AllocateBuffer( c_outsize as libc::c_int );

            let mut c_inargs = c_insize ;
            
            let mut c_ouargs = c_outsize ;
            let mut c_oubuf = c_output ;

            iconv_wrapper::rust_iconv(hnd , &vec::raw::to_ptr(inbuf) /*c_input*/, &c_inargs  , &c_output , &c_ouargs , &err ) ;
            let outbuf = vec::from_buf(c_oubuf,(c_outsize- c_ouargs)as uint); ;
            let len_processed=c_insize-c_inargs ;
           
            iconv_wrapper::DeallocateBuffer(c_oubuf);
            (outbuf, len_processed, err)
        }
    }
}

pub fn safe_riconv_close(hnd: u64) -> int {
    unsafe {
        // io::println(fmt!("\n ICONV_CRATE::In safe_riconv_close called = %? ", hnd));
        iconv_wrapper::rust_iconv_close(hnd) as int
    }
}




