#include <iconv.h>
#include <stdio.h>
#include <string.h>
#include <inttypes.h>


extern uint64_t rust_iconv_open ( const char * to_code, const char * from_code )
{
	uint64_t result ;
	void * handle ;
	handle = iconv_open(to_code,from_code);
	result = handle ;
	printf("\n Opened rust iconv =%lld=%lld=%s=%s= \n",result,handle, to_code , from_code );
	return result ;
}


extern size_t rust_iconv(uint64_t handle , char ** inbuf , size_t * insize , char ** outbuf , size_t * outsize)
{
	char * buf = malloc(150);
	memset(buf,0,150);
	
	char * ibuf = malloc(150);
	memset(ibuf,0,150);
	memcpy(ibuf,*inbuf,8);

	printf("\n Rust iconv call =%d=%s=%d=%s=%d= \n",handle, ibuf , *insize, buf , *outsize );
    void * res = handle ;
	size_t result =  iconv(res,&ibuf,8,&buf,150);
	printf("\n After Rust iconv call =%lld=%s=%d=%s=%d= \n",result, buf , *insize, buf , *outsize );
	fflush(stdout);
	return result ;
}


extern int rust_iconv_close(uint64_t handle)
{
	void * res = handle ;
	printf("\n ICONV_WRAPPER::Calling Rust Iconv Close =%lld=%lld= \n",handle,res);
	return iconv_close(res);
}
