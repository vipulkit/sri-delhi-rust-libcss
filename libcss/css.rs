use libwapcaplet::wapcaplet::*;
use parserutils::input::inputstream::*;

// libcss uses
use charset::csdetect::*;
use lex::lexer::*;
use parse::language::*;
use parse::parse::*;
use stylesheet::*;
use utils::errors::*;
use parse::propstrings::*;

pub struct css {
    stylesheet:uint,
    parser:~css_parser,
    // lwc_ref:Option<~lwc>,
}

enum css_params_version {
    CSS_PARAMS_VERSION_1 = 1
}

pub struct css_params {
        /** ABI version of this structure */
    params_version : css_params_version,

    /** The language level of the stylesheet */
    level: css_language_level,

    /** The charset of the stylesheet data, or NULL to detect */
    charset : Option<~str>,
    /** URL of stylesheet */
    url : ~str,
    /** Title of stylesheet */
    title : ~str,

    /** Permit quirky parsing of stylesheet */
    allow_quirks : bool,
    /** This stylesheet is an inline style */
    inline_style : bool,

    /** URL resolution function */
    resolve : css_url_resolution_fn,

    /** Import notification function */
    import : Option<css_import_notification_fn>,

    /** Colour resolution function */
    color : Option<css_color_resolution_fn>,

    /** Font resolution function */
    font : Option<css_font_resolution_fn>,

}

impl css {

    pub fn css_create(stylesheet_vector:&mut ~[css_stylesheet], params: &css_params) -> ~css {
       
        // assert!(!(propstrings_instance.is_some());
        //assert!((propstrings_instance.is_none() || lwc_instance.is_none()));
                                                     
        // create inputstream
        let (inputstream_option, _) =  
            match params.charset.clone() {
                None => inputstream(None, None ,Some(@css__charset_extract)),
                Some(charset) => inputstream(Some(charset), Some(CSS_CHARSET_DICTATED as int), Some(@css__charset_extract))
            };
        
        lwc();

        // create lexer
        
        let lexer = css_lexer::css__lexer_create(inputstream_option.unwrap());

        // create stylesheet
        let stylesheet = css_stylesheet {
            selectors:css_selector_hash::css__selector_hash_create(),       
            rule_count:0,                        
            rule_list:None,   
            last_rule:None,   
            disabled:false,                          
            url:params.url.clone(),                               
            title:params.title.clone(),                             
            level:params.level,               
            quirks_allowed:params.allow_quirks,                    
            quirks_used:false,                       
            inline_style:params.inline_style,                      
            string_vector:~[],
            resolve : params.resolve, 
            import : params.import, 
            font : params.font,   
            color: params.color,
            css_rule_list : ~[],
            css_selectors_list : ~[] 
        };

        stylesheet_vector.push(stylesheet);
        let stylesheet_index = stylesheet_vector.len() -1;
        // create language
        let language = css_language(stylesheet_index);

        // create parser
        let parser = match params.inline_style {
            false => css_parser::css__parser_create( language, lexer),
            true => css_parser::css__parser_create_for_inline_style( language, lexer)
        }; 

        // let mut lwc_ref = if lwc_instance.is_none() {
        //     ~lwc()
        // }
        // else {
        //     lwc_instance.unwrap()
        // };
        
        // let propstrings_ref = if propstrings_instance.is_none() {
        //     css_propstrings::css_propstrings(lwc_ref)
        // }
        // else {
        //     propstrings_instance.unwrap()
        // };           

        ~css {
            parser:parser.unwrap(),
            stylesheet:stylesheet_index,
            // lwc_ref: Some(lwc_ref),
            // propstrings_ref : Some(propstrings_ref)   
        }
    }

    /**
    * #Description:
    *   Append source data to a stylesheet.
    
    * #Arguments:
    *  'data' - The data to append.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css_stylesheet_append_data(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type],  propstrings_ref: &css_propstrings , data:~[u8]) -> css_error {
        self.parser.css__parser_parse_chunk(stylesheet_vector, css_rule_data_list, unsafe {lwc_ref.get_mut_ref()}, propstrings_ref, data)
    }

    /**
    * #Description:
    *   Flag that the last of a stylesheet's data has been seen.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success,
                      CSS_IMPORTS_PENDING if there are imports pending,
                      appropriate error otherwise.
    */
    pub fn css_stylesheet_data_done(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], propstrings_ref: &css_propstrings) -> css_error {
        let error = self.parser.css__parser_completed(stylesheet_vector, css_rule_data_list, unsafe {lwc_ref.get_mut_ref()}, propstrings_ref);
        match error {
            CSS_OK=>{},
            err => {
                return err ;
            }
        }


        let mut ptr = stylesheet_vector[self.stylesheet].rule_list ;
        loop {
            match ptr {
                None=>{
                    return CSS_OK ;
                },
                Some(rule)=>{
                    match css_rule_data_list[rule].rule_type {
                        CSS_RULE_IMPORT=>{
                            if css_rule_data_list[rule].rule_import.get_mut_ref().sheet.is_none() {
                                return CSS_IMPORTS_PENDING ;
                            }
                            else {
                                ptr = stylesheet_vector[self.stylesheet].css_rule_list[stylesheet_vector[self.stylesheet].css__stylesheet_get_base_rule(css_rule_data_list, rule)].next;
                                loop ;
                            }
                        },
                        CSS_RULE_UNKNOWN=>{
                            ptr = stylesheet_vector[self.stylesheet].css_rule_list[stylesheet_vector[self.stylesheet].css__stylesheet_get_base_rule(css_rule_data_list, rule)].next;
                            loop ;
                        },
                        CSS_RULE_CHARSET=>{
                            ptr = stylesheet_vector[self.stylesheet].css_rule_list[stylesheet_vector[self.stylesheet].css__stylesheet_get_base_rule(css_rule_data_list, rule)].next;
                            loop ;
                        },
                        _=>{
                            break ;
                        }
                    }
                }
            }
        }

        CSS_OK
    }

    /**
    * #Description:
    *   Set a stylesheet's disabled state.
    
    * #Arguments:
    *  'disabled' - The new disabled state.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css_stylesheet_set_disabled(&mut self, stylesheet_vector:&mut ~[css_stylesheet], disabled:bool ) -> css_error {

        stylesheet_vector[self.stylesheet].disabled = disabled;
        CSS_OK
    }

    /**
    * #Description:
    *   Get disabled status of a stylesheet.
    
    * #Return Value:
    *   '(css_error,~bool)' - (CSS_OK , disabled state flag).
    */
    pub fn css_stylesheet_get_disabled(&mut self, stylesheet_vector:&mut ~[css_stylesheet]) -> (css_error,bool) {

        (CSS_OK,stylesheet_vector[self.stylesheet].disabled)
    }

    /**
    * #Description:
    *   Determine whether quirky parsing was permitted on a stylesheet.
    
    * #Return Value:
    *   '(css_error,~bool)' - (CSS_OK , quirks allowed flag).
    */
    pub fn css_stylesheet_quirks_allowed(&mut self, stylesheet_vector:&mut ~[css_stylesheet]) -> (css_error,bool) {

        (CSS_OK,stylesheet_vector[self.stylesheet].quirks_allowed)
    }

    /**
    * #Description:
    *   Determine whether quirky parsing was used on a stylesheet.
    
    * #Return Value:
    *   '(css_error,~bool)' - (CSS_OK , quirks used flag).
    */
    pub fn css_stylesheet_used_quirks(&mut self, stylesheet_vector:&mut ~[css_stylesheet]) -> (css_error,bool) {

        (CSS_OK,stylesheet_vector[self.stylesheet].quirks_used)
    }

    /**
    * #Description:
    *   Retrieve the title associated with a stylesheet.
    
    * #Return Value:
    *   '(css_error,~str)' - (CSS_OK , title).
    */
    pub fn css_stylesheet_get_title(&mut self, stylesheet_vector:&mut ~[css_stylesheet]) -> (css_error,~str) {

        (CSS_OK,stylesheet_vector[self.stylesheet].title.clone())
    }

    /**
    * #Description:
    *   Retrieve the URL associated with a stylesheet.
    
    * #Return Value:
    *   '(css_error,~str)' - (CSS_OK , url).
    */
    pub fn css_stylesheet_get_url(&mut self, stylesheet_vector:&mut ~[css_stylesheet]) -> (css_error,~str) {

        (CSS_OK,stylesheet_vector[self.stylesheet].url.clone())
    }

    /**
    * #Description:
    *   Retrieve the language level of a stylesheet.
    
    * #Return Value:
    *   '(css_error,css_language_level)' - (CSS_OK , level).
    */
    pub fn css_stylesheet_get_language_level(&mut self, stylesheet_vector:&mut ~[css_stylesheet]) -> 
                                    (css_error,css_language_level) {

        (CSS_OK,stylesheet_vector[self.stylesheet].level)  
    }

    /**
    * #Description:
    *   Retrieve the next pending import for the parent stylesheet.
    * The client must resolve the absolute URL of the imported stylesheet,
    * using the parent's URL as the base. It must then fetch the imported
    * stylesheet, and parse it to completion, including fetching any stylesheets
    * it may import. The resultant sheet must then be registered with the
    * parent using css_stylesheet_register_import().
    *
    * The client must then call this function again, to determine if there
    * are any further imports for the parent stylesheet, and, if so,
    * process them as described above.
    *
    * If the client is unable to fetch an imported stylesheet, it must
    * register an empty stylesheet with the parent in its place.
    
    * #Return Value:
    *   '(css_error,Option<~str>,Option<u64>)' - (CSS_OK, URL of imported stylesheet, applicable media types for 
                                                        imported stylesheet) on success, 
                                                (appropriate error, None, None) otherwise.
    */
    pub fn css_stylesheet_next_pending_import(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type]) -> 
                                (css_error,Option<~str>,Option<u64>) {

        let mut ptr = stylesheet_vector[self.stylesheet].rule_list ;
        loop {
            match ptr {
                None=> {
                    break ;
                },
                Some(current_rule) => {
                    match css_rule_data_list[current_rule].rule_type {
                        CSS_RULE_IMPORT=>{
                            if css_rule_data_list[current_rule].rule_import.get_mut_ref().sheet.is_none() {
                                return (CSS_OK,Some(css_rule_data_list[current_rule].rule_import.get_mut_ref().url.clone()),
                                                        Some(css_rule_data_list[current_rule].rule_import.get_mut_ref().media));
                            }
                            else {
                                ptr = stylesheet_vector[self.stylesheet].css_rule_list[stylesheet_vector[self.stylesheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                                loop ;
                            }
                        },
                        CSS_RULE_CHARSET =>{
                            ptr = stylesheet_vector[self.stylesheet].css_rule_list[stylesheet_vector[self.stylesheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                            loop;
                        },
                        CSS_RULE_UNKNOWN =>{
                            ptr = stylesheet_vector[self.stylesheet].css_rule_list[stylesheet_vector[self.stylesheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                            loop;
                        },
                        _=> {
                            break ;
                        }
                    }
                }
            }
        }
        (CSS_INVALID,None,None) 
    }

    /**
    * #Description:
    *   Register an imported stylesheet with its parent.
    
    * #Arguments:
    *  'import' - Imported sheet.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success, CSS_INVALID if there are no outstanding imports, appropriate error otherwise.
    */
    pub fn css_stylesheet_register_import(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], import:Option<uint>) 
        -> css_error {
            debug!("Entering: css_stylesheet_register_import");

        if import.is_none() {
            // debug!("Entering: import.is_none()");
            return CSS_BADPARM ;
        }

        let mut ptr = stylesheet_vector[self.stylesheet].rule_list ;
        loop {
            match ptr {
                None=> {
                    break ;
                },
                Some(current_rule) => {
                    match css_rule_data_list[current_rule].rule_type {

                        CSS_RULE_IMPORT=>{
                            if css_rule_data_list[current_rule].rule_import.get_mut_ref().sheet.is_none() {
                                css_rule_data_list[current_rule].rule_import.get_mut_ref().sheet = import ;
                                return CSS_OK ;
                            }
                            else {
                                ptr = stylesheet_vector[self.stylesheet].css_rule_list[stylesheet_vector[self.stylesheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                                loop ;
                            }
                        },
                        CSS_RULE_CHARSET =>{
                            ptr = stylesheet_vector[self.stylesheet].css_rule_list[stylesheet_vector[self.stylesheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                            loop;
                        },
                        CSS_RULE_UNKNOWN =>{
                            ptr = stylesheet_vector[self.stylesheet].css_rule_list[stylesheet_vector[self.stylesheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                            loop;
                        },
                        _=> {
                            break ;
                        }
                    }
                }
            }
        }
        CSS_INVALID 
    }

}
