#include <iconv.h>
#include <stdio.h>
#include <string.h>
#include <inttypes.h>
#include <errno.h>



void showHex(const char * s , int len ) 
{	int i = 0 ;
	printf ("\n Hex - ");
	for( i=0 ; i<len ; ++i ) {
		printf ("-%d-",*(s+i));
	}
	printf ("-- \n ");
}

extern uint64_t rust_iconv_open ( const char * to_code, const char * from_code )
{
	uint64_t result ;
	void * handle ;
	handle = iconv_open(to_code,from_code);
	result = handle ;
	printf("\n Opened rust iconv =%lld=%lld=%s=%s= \n",result,handle, to_code , from_code );
	return result ;
}



extern size_t rust_iconv(uint64_t handle , char ** inbuf , size_t * insize , char ** outbuf , size_t * outsize , int *error_num)
{
	char * buf = malloc(150);
	memset(buf,0,150);
	
	char * ibuf = malloc(150);
	memset(ibuf,0,150);
	memcpy(ibuf,*inbuf,8);


	printf("\n Rust iconv call =%lld=%d=%d= \n",handle,*insize,*outsize );
	showHex(*inbuf, *insize) ;
	showHex(*outbuf,*outbuf);
    void * res = handle ;
	size_t result =  iconv(res,&ibuf,8,&buf,150);
	printf("\n After Rust iconv call =%lld=%s=%d=%s=%d= \n",result, buf , *insize, buf , *outsize );
	fflush(stdout);

	if (result==(size_t)-1) {
		if (errno==E2BIG)
			*error_num = 1;
		else if (errno==EILSEQ)
			*error_num = 2;
		else
			*error_num = 3;

	}
	else {
		*error_num = 0 ;
	}

	return result ;
}


extern int rust_iconv_close(uint64_t handle)
{
	void * res = handle ;
	printf("\n ICONV_WRAPPER::Calling Rust Iconv Close =%lld=%lld= \n",handle,res);
	return iconv_close(res);
}
