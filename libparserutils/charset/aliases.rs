extern mod std;

use core::io::Reader;
use core::io::ReaderUtil;
use core::hashmap::linear::LinearMap;
use core::vec::*;
use std::arc;

pub struct parserutils_charset_aliases_canon {
    mib_enum:u16,
    name_len:u16,
    name: ~str
}

pub enum parserutils_error {
    PARSERUTILS_OK = 0,
    PARSERUTILS_BADPARAM = 1,
    PARSERUTILS_NOMEM = 2,
    PARSERUTILS_EOF = 3,
    PARSERUTILS_BADENCODING = 4,
    PARSERUTILS_NEEDDATA = 5,
    PARSERUTILS_INVALID = 6,
    PARSERUTILS_ICONV_ERROR = 8,
    PARSERUTILS_SUCCESS = 9
}

pub struct alias {
    // these two data structures together can be used for (mibenum -> canonical name) conversion
    canonical_name_list: ~[~str],
    mibenum_map: ~LinearMap<u16,uint>,
    // this data structure can be used for (canonical name/alias -> mibenum) conversion
    alias_map: ~LinearMap<~str,u16>
}

pub fn memcmp(str1 : &~[u8] , str2 : &[u8] , len : uint ) -> int {
    let mut i : uint = 0 ;
    while ( i<len ) {
        if str1[i] != str2[i] {
            return ( (str1[i]-str2[i]) as int) ;
        }
        i = i+1 ; 
    }
    0
}

impl alias {

    fn read_aliases(&mut self) {

        let mut line_number=1;

        let Aliases = &[&"# > Unicode:Files.Aliases", &"# Mapping of character set encoding names to their canonical form", &"#", &"# Lines starting with a \'#\' are comments, blank lines are ignored.", &"#", &"# Based on http://www.iana.org/assignments/character-sets and", &"# http://www.iana.org/assignments/ianacharset-mib", &"#", &"# Canonical Form\tMIBenum\t\tAliases...", &"#", &"US-ASCII\t\t3\t\tiso-ir-6 ANSI_X3.4-1986 ISO_646.irv:1991 ASCII ISO646-US ANSI_X3.4-1968 us IBM367 cp367 csASCII", &"ISO-10646-UTF-1\t\t27\t\tcsISO10646UTF1", &"ISO_646.basic:1983\t28\t\tref csISO646basic1983", &"INVARIANT\t\t29\t\tcsINVARIANT", &"ISO_646.irv:1983\t30\t\tiso-ir-2 irv csISO2IntlRefVersion", &"BS_4730\t\t\t20\t\tiso-ir-4 ISO646-GB gb uk csISO4UnitedKingdom", &"NATS-SEFI\t\t31\t\tiso-ir-8-1 csNATSSEFI", &"NATS-SEFI-ADD\t\t32\t\tiso-ir-8-2 csNATSSEFIADD", &"NATS-DANO\t\t33\t\tiso-ir-9-1 csNATSDANO", &"NATS-DANO-ADD\t\t34\t\tiso-ir-9-2 csNATSDANOADD", &"SEN_850200_B\t\t35\t\tiso-ir-10 FI ISO646-FI ISO646-SE se csISO10Swedish", &"SEN_850200_C\t\t21\t\tiso-ir-11 ISO646-SE2 se2 csISO11SwedishForNames", &"KS_C_5601-1987\t\t36\t\tiso-ir-149 KS_C_5601-1989 KSC_5601 korean csKSC56011987", &"ISO-2022-KR\t\t37\t\tcsISO2022KR", &"EUC-KR\t\t\t38\t\tcsEUCKR EUCKR", &"ISO-2022-JP\t\t39\t\tcsISO2022JP", &"ISO-2022-JP-2\t\t40\t\tcsISO2022JP2", &"ISO-2022-CN\t\t104", &"ISO-2022-CN-EXT\t\t105", &"JIS_C6220-1969-jp\t41\t\tJIS_C6220-1969 iso-ir-13 katakana x0201-7 csISO13JISC6220jp", &"JIS_C6220-1969-ro\t42\t\tiso-ir-14 jp ISO646-JP csISO14JISC6220ro", &"IT\t\t\t22\t\tiso-ir-15 ISO646-IT csISO15Italian", &"PT\t\t\t43\t\tiso-ir-16 ISO646-PT csISO16Portuguese", &"ES\t\t\t23\t\tiso-ir-17 ISO646-ES csISO17Spanish", &"greek7-old\t\t44\t\tiso-ir-18 csISO18Greek7Old", &"latin-greek\t\t45\t\tiso-ir-19 csISO19LatinGreek", &"DIN_66003\t\t24\t\tiso-ir-21 de ISO646-DE csISO21German", &"NF_Z_62-010_(1973)\t46\t\tiso-ir-25 ISO646-FR1 csISO25French", &"Latin-greek-1\t\t47\t\tiso-ir-27 csISO27LatinGreek1", &"ISO_5427\t\t48\t\tiso-ir-37 csISO5427Cyrillic", &"JIS_C6226-1978\t\t49\t\tiso-ir-42 csISO42JISC62261978", &"BS_viewdata\t\t50\t\tiso-ir-47 csISO47BSViewdata", &"INIS\t\t\t51\t\tiso-ir-49 csISO49INIS", &"INIS-8\t\t\t52\t\tiso-ir-50 csISO50INIS8", &"INIS-cyrillic\t\t53\t\tiso-ir-51 csISO51INISCyrillic", &"ISO_5427:1981\t\t54\t\tiso-ir-54 ISO5427Cyrillic1981", &"ISO_5428:1980\t\t55\t\tiso-ir-55 csISO5428Greek", &"GB_1988-80\t\t56\t\tiso-ir-57 cn ISO646-CN csISO57GB1988", &"GB_2312-80\t\t57\t\tiso-ir-58 chinese csISO58GB231280", &"NS_4551-1\t\t25\t\tiso-ir-60 ISO646-NO no csISO60DanishNorwegian csISO60Norwegian1", &"NS_4551-2\t\t58\t\tISO646-NO2 iso-ir-61 no2 csISO61Norwegian2", &"NF_Z_62-010\t\t26\t\tiso-ir-69 ISO646-FR fr csISO69French", &"videotex-suppl\t\t59\t\tiso-ir-70 csISO70VideotexSupp1", &"PT2\t\t\t60\t\tiso-ir-84 ISO646-PT2 csISO84Portuguese2", &"ES2\t\t\t61\t\tiso-ir-85 ISO646-ES2 csISO85Spanish2", &"MSZ_7795.3\t\t62\t\tiso-ir-86 ISO646-HU hu csISO86Hungarian", &"JIS_C6226-1983\t\t63\t\tiso-ir-87 x0208 JIS_X0208-1983 csISO87JISX0208", &"greek7\t\t\t64\t\tiso-ir-88 csISO88Greek7", &"ASMO_449\t\t65\t\tISO_9036 arabic7 iso-ir-89 csISO89ASMO449", &"iso-ir-90\t\t66\t\tcsISO90", &"JIS_C6229-1984-a\t67\t\tiso-ir-91 jp-ocr-a csISO91JISC62291984a", &"JIS_C6229-1984-b\t68\t\tiso-ir-92 ISO646-JP-OCR-B jp-ocr-b csISO92JISC62991984b", &"JIS_C6229-1984-b-add\t69\t\tiso-ir-93 jp-ocr-b-add csISO93JIS62291984badd", &"JIS_C6229-1984-hand\t70\t\tiso-ir-94 jp-ocr-hand csISO94JIS62291984hand", &"JIS_C6229-1984-hand-add\t71\t\tiso-ir-95 jp-ocr-hand-add csISO95JIS62291984handadd", &"JIS_C6229-1984-kana\t72\t\tiso-ir-96 csISO96JISC62291984kana", &"ISO_2033-1983\t\t73\t\tiso-ir-98 e13b csISO2033", &"ANSI_X3.110-1983\t74\t\tiso-ir-99 CSA_T500-1983 NAPLPS csISO99NAPLPS", &"ISO-8859-1\t\t4\t\tiso-ir-100 ISO_8859-1 ISO_8859-1:1987 latin1 l1 IBM819 CP819 csISOLatin1 8859_1 ISO8859-1", &"ISO-8859-2\t\t5\t\tiso-ir-101 ISO_8859-2 ISO_8859-2:1987 latin2 l2 csISOLatin2 8859_2 ISO8859-2", &"T.61-7bit\t\t75\t\tiso-ir-102 csISO102T617bit", &"T.61-8bit\t\t76\t\tT.61 iso-ir-103 csISO103T618bit", &"ISO-8859-3\t\t6\t\tiso-ir-109 ISO_8859-3 ISO_8859-3:1988 latin3 l3 csISOLatin3 8859_3 ISO8859-3", &"ISO-8859-4\t\t7\t\tiso-ir-110 ISO_8859-4 ISO_8859-4:1988 latin4 l4 csISOLatin4 8859_4 ISO8859-4", &"ECMA-cyrillic\t\t77\t\tiso-ir-111 KOI8-E csISO111ECMACyrillic", &"CSA_Z243.4-1985-1\t78\t\tiso-ir-121 ISO646-CA csa7-1 ca csISO121Canadian1", &"CSA_Z243.4-1985-2\t79\t\tiso-ir-122 ISO646-CA2 csa7-2 csISO122Canadian2", &"CSA_Z243.4-1985-gr\t80\t\tiso-ir-123 csISO123CSAZ24341985gr", &"ISO-8859-6\t\t9\t\tiso-ir-127 ISO_8859-6 ISO_8859-6:1987 ECMA-114 ASMO-708 arabic csISOLatinArabic", &"ISO-8859-6-E\t\t81\t\tcsISO88596E ISO_8859-6-E", &"ISO-8859-6-I\t\t82\t\tcsISO88596I ISO_8859-6-I", &"ISO-8859-7\t\t10\t\tiso-ir-126 ISO_8859-7 ISO_8859-7:1987 ELOT_928 ECMA-118 greek greek8 csISOLatinGreek 8859_7 ISO8859-7", &"T.101-G2\t\t83\t\tiso-ir-128 csISO128T101G2", &"ISO-8859-8\t\t11\t\tiso-ir-138 ISO_8859-8 ISO_8859-8:1988 hebrew csISOLatinHebrew 8859_8 ISO8859-8", &"ISO-8859-8-E\t\t84\t\tcsISO88598E ISO_8859-8-E", &"ISO-8859-8-I\t\t85\t\tcsISO88598I ISO_8859-8-I", &"CSN_369103\t\t86\t\tiso-ir-139 csISO139CSN369103", &"JUS_I.B1.002\t\t87\t\tiso-ir-141 ISO646-YU js yu csISO141JUSIB1002", &"ISO_6937-2-add\t\t14\t\tiso-ir-142 csISOTextComm", &"IEC_P27-1\t\t88\t\tiso-ir-143 csISO143IECP271", &"ISO-8859-5\t\t8\t\tiso-ir-144 ISO_8859-5 ISO_8859-5:1988 cyrillic csISOLatinCyrillic 8859_5 ISO8859-5", &"JUS_I.B1.003-serb\t89\t\tiso-ir-146 serbian csISO146Serbian", &"JUS_I.B1.003-mac\t90\t\tmacedonian iso-ir-147 csISO147Macedonian", &"ISO-8859-9\t\t12\t\tiso-ir-148 ISO_8859-9 ISO_8859-9:1989 latin5 l5 csISOLatin5 8859_9 ISO8859-9", &"greek-ccitt\t\t91\t\tiso-ir-150 csISO150 csISO150GreekCCITT", &"NC_NC00-10:81\t\t92\t\tcuba iso-ir-151 ISO646-CU csISO151Cuba", &"ISO_6937-2-25\t\t93\t\tiso-ir-152 csISO6937Add", &"GOST_19768-74\t\t94\t\tST_SEV_358-88 iso-ir-153 csISO153GOST1976874", &"ISO_8859-supp\t\t95\t\tiso-ir-154 latin1-2-5 csISO8859Supp", &"ISO_10367-box\t\t96\t\tiso-ir-155 csISO10367Box", &"ISO-8859-10\t\t13\t\tiso-ir-157 l6 ISO_8859-10:1992 csISOLatin6 latin6 8859_10 ISO8859-10", &"latin-lap\t\t97\t\tlap iso-ir-158 csISO158Lap", &"JIS_X0212-1990\t\t98\t\tx0212 iso-ir-159 csISO159JISX02121990", &"DS_2089\t\t\t99\t\tDS2089 ISO646-DK dk csISO646Danish", &"us-dk\t\t\t100\t\tcsUSDK", &"dk-us\t\t\t101\t\tcsDKUS", &"JIS_X0201\t\t15\t\tX0201 csHalfWidthKatakana", &"KSC5636\t\t\t102\t\tISO646-KR csKSC5636", &"ISO-10646-UCS-2\t\t1000\t\tcsUnicode UCS-2 UCS2", &"ISO-10646-UCS-4\t\t1001\t\tcsUCS4 UCS-4 UCS4", &"DEC-MCS\t\t\t2008\t\tdec csDECMCS", &"hp-roman8\t\t2004\t\troman8 r8 csHPRoman8", &"macintosh\t\t2027\t\tmac csMacintosh MACROMAN MAC-ROMAN X-MAC-ROMAN", &"IBM037\t\t\t2028\t\tcp037 ebcdic-cp-us ebcdic-cp-ca ebcdic-cp-wt ebcdic-cp-nl csIBM037", &"IBM038\t\t\t2029\t\tEBCDIC-INT cp038 csIBM038", &"IBM273\t\t\t2030\t\tCP273 csIBM273", &"IBM274\t\t\t2031\t\tEBCDIC-BE CP274 csIBM274", &"IBM275\t\t\t2032\t\tEBCDIC-BR cp275 csIBM275", &"IBM277\t\t\t2033\t\tEBCDIC-CP-DK EBCDIC-CP-NO csIBM277", &"IBM278\t\t\t2034\t\tCP278 ebcdic-cp-fi ebcdic-cp-se csIBM278", &"IBM280\t\t\t2035\t\tCP280 ebcdic-cp-it csIBM280", &"IBM281\t\t\t2036\t\tEBCDIC-JP-E cp281 csIBM281", &"IBM284\t\t\t2037\t\tCP284 ebcdic-cp-es csIBM284", &"IBM285\t\t\t2038\t\tCP285 ebcdic-cp-gb csIBM285", &"IBM290\t\t\t2039\t\tcp290 EBCDIC-JP-kana csIBM290", &"IBM297\t\t\t2040\t\tcp297 ebcdic-cp-fr csIBM297", &"IBM420\t\t\t2041\t\tcp420 ebcdic-cp-ar1 csIBM420", &"IBM423\t\t\t2042\t\tcp423 ebcdic-cp-gr csIBM423", &"IBM424\t\t\t2043\t\tcp424 ebcdic-cp-he csIBM424", &"IBM437\t\t\t2011\t\tcp437 437 csPC8CodePage437", &"IBM500\t\t\t2044\t\tCP500 ebcdic-cp-be ebcdic-cp-ch csIBM500", &"IBM775\t\t\t2087\t\tcp775 csPC775Baltic", &"IBM850\t\t\t2009\t\tcp850 850 csPC850Multilingual", &"IBM851\t\t\t2045\t\tcp851 851 csIBM851", &"IBM852\t\t\t2010\t\tcp852 852 csPCp852", &"IBM855\t\t\t2046\t\tcp855 855 csIBM855", &"IBM857\t\t\t2047\t\tcp857 857 csIBM857", &"IBM860\t\t\t2048\t\tcp860 860 csIBM860", &"IBM861\t\t\t2049\t\tcp861 861 cp-is csIBM861", &"IBM862\t\t\t2013\t\tcp862 862 csPC862LatinHebrew", &"IBM863\t\t\t2050\t\tcp863 863 csIBM863", &"IBM864\t\t\t2051\t\tcp864 csIBM864", &"IBM865\t\t\t2052\t\tcp865 865 csIBM865", &"IBM866\t\t\t2086\t\tcp866 866 csIBM866", &"IBM868\t\t\t2053\t\tCP868 cp-ar csIBM868", &"IBM869\t\t\t2054\t\tcp869 869 cp-gr csIBM869", &"IBM870\t\t\t2055\t\tCP870 ebcdic-cp-roece ebcdic-cp-yu csIBM870", &"IBM871\t\t\t2056\t\tCP871 ebcdic-cp-is csIBM871", &"IBM880\t\t\t2057\t\tcp880 EBCDIC-Cyrillic csIBM880", &"IBM891\t\t\t2058\t\tcp891 csIBM891", &"IBM903\t\t\t2059\t\tcp903 csIBM903", &"IBM904\t\t\t2060\t\tcp904 904 csIBBM904", &"IBM905\t\t\t2061\t\tCP905 ebcdic-cp-tr csIBM905", &"IBM918\t\t\t2062\t\tCP918 ebcdic-cp-ar2 csIBM918", &"IBM1026\t\t\t2063\t\tCP1026 csIBM1026", &"EBCDIC-AT-DE\t\t2064\t\tcsIBMEBCDICATDE", &"EBCDIC-AT-DE-A\t\t2065\t\tcsEBCDICATDEA", &"EBCDIC-CA-FR\t\t2066\t\tcsEBCDICCAFR", &"EBCDIC-DK-NO\t\t2067\t\tcsEBCDICDKNO", &"EBCDIC-DK-NO-A\t\t2068\t\tcsEBCDICDKNOA", &"EBCDIC-FI-SE\t\t2069\t\tcsEBCDICFISE", &"EBCDIC-FI-SE-A\t\t2070\t\tcsEBCDICFISEA", &"EBCDIC-FR\t\t2071\t\tcsEBCDICFR", &"EBCDIC-IT\t\t2072\t\tcsEBCDICIT", &"EBCDIC-PT\t\t2073\t\tcsEBCDICPT", &"EBCDIC-ES\t\t2074\t\tcsEBCDICES", &"EBCDIC-ES-A\t\t2075\t\tcsEBCDICESA", &"EBCDIC-ES-S\t\t2076\t\tcsEBCDICESS", &"EBCDIC-UK\t\t2077\t\tcsEBCDICUK", &"EBCDIC-US\t\t2078\t\tcsEBCDICUS", &"UNKNOWN-8BIT\t\t2079\t\tcsUnknown8BiT", &"MNEMONIC\t\t2080\t\tcsMnemonic", &"MNEM\t\t\t2081\t\tcsMnem", &"VISCII\t\t\t2082\t\tcsVISCII", &"VIQR\t\t\t2083\t\tcsVIQR", &"KOI8-R\t\t\t2084\t\tcsKOI8R", &"KOI8-U\t\t\t2088", &"IBM00858\t\t2089\t\tCCSID00858 CP00858 PC-Multilingual-850+euro", &"IBM00924\t\t2090\t\tCCSID00924 CP00924 ebcdic-Latin9--euro", &"IBM01140\t\t2091\t\tCCSID01140 CP01140 ebcdic-us-37+euro", &"IBM01141\t\t2092\t\tCCSID01141 CP01141 ebcdic-de-273+euro", &"IBM01142\t\t2093\t\tCCSID01142 CP01142 ebcdic-dk-277+euro ebcdic-no-277+euro", &"IBM01143\t\t2094\t\tCCSID01143 CP01143 ebcdic-fi-278+euro ebcdic-se-278+euro", &"IBM01144\t\t2095\t\tCCSID01144 CP01144 ebcdic-it-280+euro", &"IBM01145\t\t2096\t\tCCSID01145 CP01145 ebcdic-es-284+euro", &"IBM01146\t\t2097\t\tCCSID01146 CP01146 ebcdic-gb-285+euro", &"IBM01147\t\t2098\t\tCCSID01147 CP01147 ebcdic-fr-297+euro", &"IBM01148\t\t2099\t\tCCSID01148 CP01148 ebcdic-international-500+euro", &"IBM01149\t\t2100\t\tCCSID01149 CP01149 ebcdic-is-871+euro", &"Big5-HKSCS\t\t2101", &"IBM1047\t\t\t2102\t\tIBM-1047", &"PTCP154\t\t\t2103\t\tcsPTCP154 PT154 CP154 Cyrillic-Asian", &"Amiga-1251\t\t2104\t\tAmi1251 Amiga1251 Ami-1251", &"KOI7-switched\t\t2105", &"UNICODE-1-1\t\t1010\t\tcsUnicode11", &"SCSU\t\t\t1011", &"UTF-7\t\t\t1012", &"UTF-16BE\t\t1013", &"UTF-16LE\t\t1014", &"UTF-16\t\t\t1015", &"CESU-8\t\t\t1016\t\tcsCESU-8", &"UTF-32\t\t\t1017", &"UTF-32BE\t\t1018", &"UTF-32LE\t\t1019", &"BOCU-1\t\t\t1020\t\tcsBOCU-1", &"UNICODE-1-1-UTF-7\t103\t\tcsUnicode11UTF7", &"UTF-8\t\t\t106\t\tUNICODE-1-1-UTF-8 UNICODE-2-0-UTF-8 utf8", &"ISO-8859-13\t\t109\t\t8859_13 ISO8859-13", &"ISO-8859-14\t\t110\t\tiso-ir-199 ISO_8859-14:1998 ISO_8859-14 latin8 iso-celtic l8 8859_14 ISO8859-14", &"ISO-8859-15\t\t111\t\tISO_8859-15 Latin-9 8859_15 ISO8859-15", &"ISO-8859-16\t\t112\t\tiso-ir-226 ISO_8859-16:2001 ISO_8859-16 latin10 l10", &"GBK\t\t\t113\t\tCP936 MS936 windows-936", &"GB18030\t\t\t114", &"OSD_EBCDIC_DF04_15\t115", &"OSD_EBCDIC_DF03_IRV\t116", &"OSD_EBCDIC_DF04_1\t117", &"JIS_Encoding\t\t16\t\tcsJISEncoding", &"Shift_JIS\t\t17\t\tMS_Kanji csShiftJIS X-SJIS Shift-JIS", &"EUC-JP\t\t\t18\t\tcsEUCPkdFmtJapanese Extended_UNIX_Code_Packed_Format_for_Japanese EUCJP", &"Extended_UNIX_Code_Fixed_Width_for_Japanese\t19\t\tcsEUCFixWidJapanese", &"ISO-10646-UCS-Basic\t1002\t\tcsUnicodeASCII", &"ISO-10646-Unicode-Latin1\t1003\t\tcsUnicodeLatin1 ISO-10646", &"ISO-Unicode-IBM-1261\t1005\t\tcsUnicodeIBM1261", &"ISO-Unicode-IBM-1268\t1006\t\tcsUnicodeIBM1268", &"ISO-Unicode-IBM-1276\t1007\t\tcsUnicodeIBM1276", &"ISO-Unicode-IBM-1264\t1008\t\tcsUnicodeIBM1264", &"ISO-Unicode-IBM-1265\t1009\t\tcsUnicodeIBM1265", &"ISO-8859-1-Windows-3.0-Latin-1\t2000\t\tcsWindows30Latin1", &"ISO-8859-1-Windows-3.1-Latin-1\t2001\t\tcsWindows31Latin1", &"ISO-8859-2-Windows-Latin-2\t2002\t\tcsWindows31Latin2", &"ISO-8859-9-Windows-Latin-5\t2003\t\tcsWindows31Latin5", &"Adobe-Standard-Encoding\t2005\t\tcsAdobeStandardEncoding", &"Ventura-US\t\t2006\t\tcsVenturaUS", &"Ventura-International\t2007\t\tcsVenturaInternational", &"PC8-Danish-Norwegian\t2012\t\tcsPC8DanishNorwegian", &"PC8-Turkish\t\t2014\t\tcsPC8Turkish", &"IBM-Symbols\t\t2015\t\tcsIBMSymbols", &"IBM-Thai\t\t2016\t\tcsIBMThai", &"HP-Legal\t\t2017\t\tcsHPLegal", &"HP-Pi-font\t\t2018\t\tcsHPPiFont", &"HP-Math8\t\t2019\t\tcsHPMath8", &"Adobe-Symbol-Encoding\t2020\t\tcsHPPSMath", &"HP-DeskTop\t\t2021\t\tcsHPDesktop", &"Ventura-Math\t\t2022\t\tcsVenturaMath", &"Microsoft-Publishing\t2023\t\tcsMicrosoftPublishing", &"Windows-31J\t\t2024\t\tcsWindows31J", &"GB2312\t\t\t2025\t\tcsGB2312 EUC-CN EUCCN CN-GB", &"Big5\t\t\t2026\t\tcsBig5 BIG-FIVE BIG-5 CN-BIG5 BIG_FIVE x-x-big5", &"windows-1250\t\t2250\t\tCP1250 MS-EE", &"windows-1251\t\t2251\t\tCP1251 MS-CYRL", &"windows-1252\t\t2252\t\tCP1252 MS-ANSI", &"windows-1253\t\t2253\t\tCP1253 MS-GREEK", &"windows-1254\t\t2254\t\tCP1254 MS-TURK", &"windows-1255\t\t2255", &"windows-1256\t\t2256\t\tCP1256 MS-ARAB", &"windows-1257\t\t2257\t\tCP1257 WINBALTRIM", &"windows-1258\t\t2258", &"TIS-620\t\t\t2259", &"HZ-GB-2312\t\t2085", &"", &"# Additional encodings not defined by IANA", &"", &"# Arbitrary allocations", &"#CP737\t\t\t3001", &"#CP853\t\t\t3002", &"#CP856\t\t\t3003", &"CP874\t\t\t3004\t\tWINDOWS-874", &"#CP922\t\t\t3005", &"#CP1046\t\t\t3006", &"#CP1124\t\t\t3007", &"#CP1125\t\t\t3008\t\tWINDOWS-1125", &"#CP1129\t\t\t3009", &"#CP1133\t\t\t3010\t\tIBM-CP1133", &"#CP1161\t\t\t3011\t\tIBM-1161 IBM1161 CSIBM1161", &"#CP1162\t\t\t3012\t\tIBM-1162 IBM1162 CSIBM1162", &"#CP1163\t\t\t3013\t\tIBM-1163 IBM1163 CSIBM1163", &"#GEORGIAN-ACADEMY\t3014", &"#GEORGIAN-PS\t\t3015", &"#KOI8-RU\t\t3016", &"#KOI8-T\t\t\t3017", &"#MACARABIC\t\t3018\t\tX-MAC-ARABIC MAC-ARABIC", &"#MACCROATIAN\t\t3019\t\tX-MAC-CROATIAN MAC-CROATIAN", &"#MACGREEK\t\t3020\t\tX-MAC-GREEK MAC-GREEK", &"#MACHEBREW\t\t3021\t\tX-MAC-HEBREW MAC-HEBREW", &"#MACICELAND\t\t3022\t\tX-MAC-ICELAND MAC-ICELAND", &"#MACROMANIA\t\t3023\t\tX-MAC-ROMANIA MAC-ROMANIA", &"#MACTHAI\t\t3024\t\tX-MAC-THAI MAC-THAI", &"#MACTURKISH\t\t3025\t\tX-MAC-TURKISH MAC-TURKISH", &"#MULELAO-1\t\t3026", &"CP949\t\t\t3027\t\tWINDOWS-949", &"", &"# From Unicode Lib", &"ISO-IR-182\t\t4000", &"ISO-IR-197\t\t4002", &"ISO-2022-JP-1\t\t4008", &"MACCYRILLIC\t\t4009\t\tX-MAC-CYRILLIC MAC-CYRILLIC", &"MACUKRAINE\t\t4010\t\tX-MAC-UKRAINIAN MAC-UKRAINIAN", &"MACCENTRALEUROPE\t4011\t\tX-MAC-CENTRALEURROMAN MAC-CENTRALEURROMAN", &"JOHAB\t\t\t4012", &"ISO-8859-11\t\t4014\t\tiso-ir-166 ISO_8859-11 ISO8859-11 8859_11", &"X-CURRENT\t\t4999\t\tX-SYSTEM", &"X-ACORN-LATIN1\t\t5001", &"X-ACORN-FUZZY\t\t5002", &""];

        for Aliases.each |&line| {
            if (!str::starts_with(line,"#") && line.len()>0) {
                //io::println(fmt!("read_aliases:: line_number == %?, line == %?", line_number, line));
                let mut alias_entry_columns : ~[~str] = ~[];
                for str::each_split_str_nonempty(line,"\t") |column| {
                    alias_entry_columns.push(column.to_owned());
                } 
                
                // first column is canonical name
                let canonical_name = alias_entry_columns[0].to_lower();
                // second column is mibenum
                let mibenum = u16::from_str(alias_entry_columns[1]).unwrap();
                
                // add the canonical name to the list of canonical names
                self.canonical_name_list.push(copy canonical_name);
                // insert <mibenum, index of canonical name> into mibenum_map
                self.mibenum_map.insert(mibenum,line_number-1);
                // insert <canonical_name, mibenum> into alias_map
                self.alias_map.insert(canonical_name, mibenum);

                // optionally, the third column has other aliases
                if (alias_entry_columns.len() > 2) {
                    let mut aliases : ~[~str] = ~[];
                    for str::each_split_str_nonempty(alias_entry_columns[2]," ") |alias| {
                        aliases.push(alias.to_lower());
                    } 
                    // insert <alias, mibenum> into alias_map
                    for aliases.each |&alias| {
                        self.alias_map.insert(alias.to_lower(), mibenum);
                    }
                }
                line_number=line_number+1;
            }
        }
    }

    pub fn parserutils__charset_alias_canonicalise(&self, alias: ~str) -> Option<parserutils_charset_aliases_canon> {       
        match self.alias_map.find(&alias.to_lower()) {         
            None => None,                   
            Some(temp_mib_enum) => {
                match self.mibenum_map.find(temp_mib_enum) {
                    None => None,                   
                    Some(canonical_name_list_index) => {
                        if (*canonical_name_list_index < self.canonical_name_list.len()) {
                            
                            let temp_name = copy (self.canonical_name_list[*canonical_name_list_index]);
                            let temp_name_len = temp_name.len() as u16;
                            Some( parserutils_charset_aliases_canon {
                                    mib_enum: *temp_mib_enum,
                                    name: temp_name,
                                    name_len: temp_name_len
                                }
                            )
                        }
                        else {
                            None
                        }
                    }
                }
            }
        }
    }

    pub fn parserutils_charset_mibenum_from_name(&self, alias: ~str) -> u16 {
        match self.alias_map.find(&alias.to_lower()) {
            None => 0 ,
            Some(mib_enum) => *mib_enum
        }
    }

    pub fn parserutils_charset_mibenum_to_name(&self, mibenum: u16)-> Option<~str> {
        match self.mibenum_map.find(&(mibenum)) {
            None => None,
            Some (canonical_name_list_index) => {
                if canonical_name_list_index < &self.canonical_name_list.len() {
                    Some(copy self.canonical_name_list[*canonical_name_list_index])
                }
                else {
                    None
                }
            }
        }
    }
    
} //impl alias

pub fn alias() -> arc::ARC<~alias> {
    let mut new_alias = ~alias {
        canonical_name_list : ~[],
        mut mibenum_map : ~LinearMap::new(),
        mut alias_map : ~LinearMap::new()
    };

    new_alias.read_aliases();
    arc::ARC(new_alias)
}
