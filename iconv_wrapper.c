#include <iconv.h>
#include <stdio.h>
#include <string.h>
#include <inttypes.h>
#include <errno.h>



void showHex(const char * message , const char * s , int len ) 
{	int i = 0 ;
	printf ("\n %s Dec is - ",message);
	for( i=0 ; i<len ; ++i ) {
		printf ("-%d-",*(s+i));
	}
	printf ("-- \n ");
}

extern  char * AllocateBuffer(int bytes) 
{
	return (char*)calloc(bytes+4,1);
}

extern void DeallocateBuffer( char * buffer) 
{
	if (buffer>0) 
		free(buffer);
	buffer=0 ;
}

extern uint64_t rust_iconv_open ( const char * to_code, const char * from_code )
{
	uint64_t result ;
	void * handle ;
	handle = iconv_open("UTF-32BE",from_code);
	result = handle ;
	printf("\n Opened rust iconv =%lld=%lld=%s=%s= \n",result,handle, to_code , from_code );
	return result ;
}



extern size_t rust_iconv(uint64_t handle , char ** inbuf , size_t * insize , char ** outbuf , size_t * outsize , int *error_num)
{
	size_t result ; 
	if ((inbuf==NULL)||(*inbuf==NULL)) {
		result = iconv(handle, NULL,0,NULL,0) ;
	}
	else {
		//printf("\n Rust iconv call =%lld=%d=%d= \n",handle,*insize,*outsize );
		//fflush(stdout);
		//showHex("Input-",*inbuf, *insize) ;
		//showHex("Output-",*outbuf,*outsize);
		//fflush(stdout);

	    void * res = handle ;
	   
	    //char * iptr = *inbuf ;
	    //char * optr = *outbuf ;
	    //size_t ibytes = *insize ;
	    //size_t obytes = *outsize ; 

		result = iconv(res,inbuf,insize,outbuf,outsize);
		//size_t result = iconv(res,&iptr,ibytes,&optr,outsize);
		//printf("\n After Rust iconv call =%lld=%lld=%lld=%lld= \n",result, handle , *insize, *outsize );
		//showHex("Output After Processing-",optr,16);

		//fflush(stdout);
	}

	if (result==(size_t)-1) {
		if (errno==E2BIG)
			*error_num = 1;
		else if (errno==EILSEQ)
			*error_num = 2;
		else if (errno==EINVAL)
			*error_num = 3;
		else
			*error_num = 4;

	}
	else {
		*error_num = 0 ;
	}
	//printf("\n Error code is =%ud= \n",error_num );
	return result ;
}


extern int rust_iconv_close(uint64_t handle)
{
	void * res = handle ;
	printf("\n ICONV_WRAPPER::Calling Rust Iconv Close =%lld=%lld= \n",handle,res);
	return iconv_close(res);
}
