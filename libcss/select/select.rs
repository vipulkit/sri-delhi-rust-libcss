
//use extra::arc;
use wapcaplet::*;

use include::properties::*;
use include::types::*;
use include::font_face::*;

use bytecode::bytecode::*;

use utils::errors::*;

use select::common::*;
use select::dispatch::*;

use stylesheet::*;

use std::ptr::*;
use std::libc::*;
use std::cast::*;
use std::vec::from_elem;
//use extra::arc;


static IMPORT_STACK_SIZE : int = 256 ;

/*
 * Container for stylesheet selection info
 */
pub struct css_select_sheet {
    sheet:uint,
    origin:css_origin,
    media:u64
}

/*
 * CSS selection context
 */
pub struct css_select_ctx {
    sheets:~[~css_select_sheet],
    /* Useful interned strings */
    lwc_ref: ~lwc,
    universal:Option<uint>,
    first_child:Option<uint>,
    link:Option<uint>,
    visited:Option<uint>,
    hover:Option<uint>,
    active:Option<uint>,
    focus:Option<uint>,
    nth_child:Option<uint>,
    nth_last_child:Option<uint>,
    nth_of_type:Option<uint>,
    nth_last_of_type:Option<uint>,
    last_child:Option<uint>,
    first_of_type:Option<uint>,
    last_of_type:Option<uint>,
    only_child:Option<uint>,
    only_of_type:Option<uint>,
    root:Option<uint>,
    empty:Option<uint>,
    target:Option<uint>,
    lang:Option<uint>,
    enabled:Option<uint>,
    disabled:Option<uint>,
    checked:Option<uint>,
    first_line:Option<uint>,
    first_letter:Option<uint>,
    before:Option<uint>,
    after:Option<uint>
}

/*
 * Font face selection state
 */
pub struct css_select_font_faces_state {
    font_family:Option<uint>,
    media:u64,

    ua_font_faces:~[~css_font_face],
    user_font_faces:~[~css_font_face],
    author_font_faces: ~[~css_font_face]
}

pub enum source_type {
    CSS_SELECT_RULE_SRC_ELEMENT,
    CSS_SELECT_RULE_SRC_CLASS,
    CSS_SELECT_RULE_SRC_ID,
    CSS_SELECT_RULE_SRC_UNIVERSAL
}

pub struct css_select_rule_source {
    source:source_type,
    source_class:u32
}
//////////////////////////////////////////////////////////////////
// Start of CSS Selector internal functions
//////////////////////////////////////////////////////////////////
impl css_select_ctx {

    /**
    * Create a selection context
    * #Return Value:
	* 'css_select_ctx' - Pointer to created context.
    */
    pub fn css_select_ctx_create(lwc_ref : ~lwc) -> ~css_select_ctx {
        let mut result = ~css_select_ctx {
            sheets:~[],
            lwc_ref: lwc_ref,
            universal:None,
            first_child:None,
            link:None,
            visited:None,
            hover:None,
            active:None,
            focus:None,
            nth_child:None,
            nth_last_child:None,
            nth_of_type:None,
            nth_last_of_type:None,
            last_child:None,
            first_of_type:None,
            last_of_type:None,
            only_child:None,
            only_of_type:None,
            root:None,
            empty:None,
            target:None,
            lang:None,
            enabled:None,
            disabled:None,
            checked:None,
            first_line:None,
            first_letter:None,
            before:None,
            after:None
        };

        result.intern_strings();
        return result 
    }


    /**
    * Append a stylesheet to a selection context
	* #Arguments:
	*  'self' - The selection context to append to.
    *  'sheet'  - Stylestylesheet_vector[sheet]. 
    *  'origin' - Origin of the stylesheet_vector[sheet].
    *  'media' - Vector of tokens to process.
    * #Return Value:
	* 'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css_select_ctx_append_sheet(&mut self,
                                    stylesheet_vector: &mut ~[css_stylesheet],
                                    sheet:uint,
                                    origin:css_origin,
                                    media:u64) 
                                    -> css_error {
        //let n_sheets = self.sheets.len();
        //debug!(fmt!("Entering css_select_ctx_append_sheet")) ;
        self.css_select_ctx_insert_sheet(stylesheet_vector, sheet, origin, media)
    }

    /**
    * Insert a stylesheet into a selection context
	* #Arguments:
	*  'self' - The selection context to insert to.
    *  'sheet'  - Stylestylesheet_vector[sheet]. 
    *  'index' - Index in context to insert stylesheet_vector[sheet].
    *  'origin' - Origin of the stylesheet_vector[sheet].
    *  'media' - Vector of tokens to process.
    * #Return Value:
	* 'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css_select_ctx_insert_sheet(&mut self,
                                    stylesheet_vector: &mut ~[css_stylesheet],
                                    csheet:uint,
                                    //index:uint,
                                    corigin:css_origin,
                                    cmedia:u64) 
                                    -> css_error {

        //debug!(fmt!("Entering css_select_ctx_insert_sheet")) ;
        /* Inline styles cannot be inserted into a selection context */
        if (stylesheet_vector[csheet].inline_style) {
            return CSS_INVALID ;
        }
    
        /* Index must be in the range [0, n_sheets]
         * The latter being equivalent to append */
        // if index > self.sheets.len()    {
        //     return CSS_INVALID;
        // }   
            
        let select_sheet = ~css_select_sheet{
            sheet:csheet,
            origin:corigin,
            media:cmedia
        };

        self.sheets.push(select_sheet);
        CSS_OK
    }

    /**
    * Remove a sheet from a selection context
	* #Arguments:
	*  'self' - The selection context to remove from.
    *  'sheet'  - Sheet to remove. 
    * #Return Value:
	* 'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css_select_ctx_remove_sheet(&mut self, csheet:uint)-> css_error {

        debug!(fmt!("Entering css_select_ctx_remove_sheet")) ;
        let mut i = self.sheets.len() ;
        while (i>0) {
            i = i - 1 ;
            if ( self.sheets[i].sheet == csheet ) {
                self.sheets.remove(i);
                return CSS_OK ;
            }
        }
        CSS_INVALID
        
    }

    
    /**
    * Count the number of top-level sheets in a selection context
	* #Arguments:
	*  'self' - The selection context to consider.
    * #Return Value:
	* 'uint' - Count of sheets.
    */
    pub fn css_select_ctx_count_sheets(&mut self) -> uint {
        //debug!(fmt!("Entering css_select_ctx_count_sheets")) ;
        self.sheets.len()
    }

    /**
    * Retrieve a sheet from a selection context
	* #Arguments:
	*  'self' - The selection context to look in.
	*  'index' - Index in context to look.
    * #Return Value:
	* '(css_error,Option<uint>)' - (CSS_OK,Some(sheet)) on success, appropriate (error,None) otherwise.
    */
    pub fn css_select_ctx_get_sheet(&mut self, index:uint) 
                                -> (css_error,Option<uint>) {

        //debug!(fmt!("Entering css_select_ctx_get_sheet")) ;
        if ( index >= self.sheets.len() ) {
            return (CSS_INVALID,None) ;
        }

        (CSS_OK,Some(self.sheets[index].sheet))
    } 

    /**
	* Select a style for the given node
	* #Arguments:
    *  'ctx'    - Selection context to use.
    *  'node'  - Node to select style for. 
    *  'media' - Currently active media types.
    *  'inline_style' - Corresponding inline style for node, or NULL.
    *  'handler' - Dispatch table of handler functions.
    * #Return Value:
	* '(css_error,Option<@mut css_select_results>' - (CSS_OK, results) on success, (appropriate error, None) otherwise.
    * #Post condition:
	*   ctx is updated with the next token to process.
    *   If the input is invalid, then ctx remains unchanged.
	* #Description:
	* In computing the style, no reference is made to the parent node's
	* style. Therefore, the resultant computed style is not ready for
	* immediate use, as some properties may be marked as inherited.
	* Use css_computed_style_compose() to obtain a fully computed style.
	*
	* This two-step approach to style computation is designed to allow
	* the client to store the partially computed style and efficiently
	* update the fully computed style for a node when layout changes.
	*/
    pub fn css_select_style(&mut self,
                                stylesheet_vector: &mut ~[css_stylesheet], 
                                css_rule_data_list:&mut ~[~css_rule_data_type],
                                node:*c_void,
                                media:u64,
                                inline_style:Option<uint>,
                                handler:~css_select_handler,
								pw:*c_void) 
                                -> (css_error,Option<~css_select_results>) {

        //debug!(fmt!("Entering css_select_style")) ;
        if( node == null() || handler.handler_version != (CSS_SELECT_HANDLER_VERSION_1  as uint) ) {
            return (CSS_BADPARM,None) ;
        }
        let mut i : int  ;
        let mut j :int;
        let mut error : css_error ;
        //let mut results : Option<@mut css_select_results>  ;
        let mut parent : *c_void = null() ;

        let mut state: ~css_select_state = ~css_select_state {
            node:node,
            media:media,       
            results:~css_select_results{ 
                styles:~[None,None,None,None,None] 
            },   
            current_pseudo:CSS_PSEUDO_ELEMENT_NONE,  
            computed:CSS_PSEUDO_ELEMENT_NONE as uint,
            handler:Some(handler), 
            pw:pw,   
            sheet:None,   
            current_origin:CSS_ORIGIN_UA,  
            current_specificity:0,   
            element:css_qname{ 
                name:self.lwc_ref.lwc_intern_string("") , 
                ns:self.lwc_ref.lwc_intern_string("") 
            },
            id:self.lwc_ref.lwc_intern_string(""),
            classes:~[],
            n_classes:0,             
            reject_cache:~[],       
            next_reject:128-1,             
            props: from_elem(CSS_N_PROPERTIES as uint, None) 
        };
		
        /* Base element style is guaranteed to exist */
        state.results.styles[0] = (Some(css_computed_style_create()));

        error = ((state.handler.get_ref().parent_node))(node, &mut parent);
        match error {
            CSS_OK=>{},
            x =>  {
                return (x,None) ;
            }
        }

        /* Get node's name */
        error = ((state.handler.get_ref().node_name))(node, &mut state.element);
        match error {
            CSS_OK=>{},
            x =>  {
                return (x,None) ;
            }
        }

        /* Get node's ID, if any */
        error = ((state.handler.get_ref().node_id))(&mut self.lwc_ref, pw, node, &mut state.id);
        match error {
            CSS_OK=>{},
            x =>  {
                return (x,None) ;
            }
        }

        /* Get node's classes, if any */
        /* \todo Do we really want to force the client to allocate a new array 
         * every time we call this? It seems hugely inefficient, given they can 
         * cache the data. */
        error = ((state.handler.get_ref().node_classes))(&mut self.lwc_ref, pw, node, 
                &mut (state.classes));
        match error {
            CSS_OK=>{},
            x =>  {
                return (x,None) ;
            }
        }

        /* Iterate through the top-level stylesheets, selecting styles
         * from those which apply to our current media requirements and
         * are not disabled */
        i=0;
        let sheets_len = self.sheets.len() as int;
        //let (out,in): (Port<~>, Chan<~>) = stream();
		//let (out1,in1): (Port<~[int]>, Chan<~[int]>) = stream();
		
		while i< sheets_len {
            
            if( self.sheets[i].media & media ) != 0 && 
                stylesheet_vector[self.sheets[i].sheet].disabled == false {
                    //debug!(fmt!("css_select_style : selecting from sheet ")) ;
                    
				self.select_from_sheet(stylesheet_vector, css_rule_data_list, self.sheets[i].sheet, self.sheets[i].origin, &mut state);  
				match error {
					CSS_OK=>{},
					x =>  {
						return (x,None) ;
					}
				}
						
            }

            i += 1 ;
        }
       
	   /* Consider any inline style for the node */
        if (inline_style.is_some()) {
            //debug!(fmt!("css_select_style : considerng inline style")) ;
            let sel = stylesheet_vector[inline_style.expect("")].rule_list;

            /* Sanity check style */
            if (stylesheet_vector[inline_style.expect("")].rule_count != 1 ){
                 return (CSS_INVALID,None) ;
            }
            
            match sel {
                None=>{
                    return (CSS_INVALID,None) ;
                },
                Some(r) => {
                    match css_rule_data_list[r].rule_type {
                        CSS_RULE_SELECTOR=>{
                            // Complete 

                            /* No bytecode if input was empty or wholly invalid */
                            if(css_rule_data_list[r].rule_selector.get_mut_ref().style.is_some()){
                                /* Inline style applies to base element only */
                                state.current_pseudo = CSS_PSEUDO_ELEMENT_NONE;
                                state.computed = CSS_PSEUDO_ELEMENT_NONE as uint;


                                error = css_select_ctx::cascade_style(stylesheet_vector, css_rule_data_list[r].rule_selector.get_mut_ref().style.get_mut_ref(), 
                                                        &mut state);
                                match error {
                                    CSS_OK=>{},
                                    x =>  {
                                        return (x,None) ;
                                    }
                                }
                            }
                        },
                        _=>{
                            return (CSS_INVALID,None) ;
                        }
                    }
                }
            }
        }

        /* Take account of presentational hints and fix up any remaining
         * unset properties. */

        /* Base element */
        state.current_pseudo = CSS_PSEUDO_ELEMENT_NONE;
        state.computed = CSS_PSEUDO_ELEMENT_NONE as uint;
        i = 0 ;
        while (i<(CSS_N_PROPERTIES as int)) {
            //debug!(fmt!("css_select_style : setting initial hint of property =%?=",i)) ;
            //let prop = state.props[i][CSS_PSEUDO_ELEMENT_NONE as uint];

            /* Apply presentational hints if the property is unset or 
             * the existing property value did not come from an author 
             * stylesheet or a user sheet using !important. */
            if (state.props[i].is_none() || state.props[i].get_ref()[CSS_PSEUDO_ELEMENT_NONE as uint].get_ref().set == false ||
                    (state.props[i].get_ref()[CSS_PSEUDO_ELEMENT_NONE as uint].get_ref().origin != (CSS_ORIGIN_AUTHOR as u8) &&
                    state.props[i].get_ref()[CSS_PSEUDO_ELEMENT_NONE as uint].get_ref().important == false)) {
                error = css_select_ctx::set_hint(&mut state, i as u32);
                match error {
                    CSS_OK=>{},
                    x =>  {
                        return (x,None) ;
                    }
                }
            }

            /* If the property is still unset or it's set to inherit 
             * and we're the root element, then set it to its initial 
             * value. */
            if ( state.props[i].is_none() || state.props[i].get_ref()[CSS_PSEUDO_ELEMENT_NONE as uint].get_ref().set == false || 
                    (parent == null() && 
                    state.props[i].get_ref()[CSS_PSEUDO_ELEMENT_NONE as uint].get_ref().inherit == true)) {
                error = css_select_ctx::set_initial(&mut state, i as uint, 
                        CSS_PSEUDO_ELEMENT_NONE, parent);
                match error {
                    CSS_OK=>{},
                    x =>  {
                        return (x,None) ;
                    }
                }
            }
            i += 1;
        }

        /* Pseudo elements, if any */
        j = (CSS_PSEUDO_ELEMENT_NONE as int) + 1;
        while ( j < (CSS_PSEUDO_ELEMENT_COUNT as int) ) {
            //debug!(fmt!("css_select_style : pseudo element of property =%?=",j)) ;
            unsafe {
                state.current_pseudo = transmute(j);
            }
						
            if state.results.styles[j].is_some() {
                state.computed = j as uint;
            } 
            else {
                j += 1; 
                loop;
            }
            
			
            /* Skip non-existent pseudo elements */
            // if (state.computed == NULL)
            //     continue;
            i = 0 ;
            while (i < (CSS_N_PROPERTIES as int) ) {
                //let prop = state.props[i][j];
                
                //debug!(fmt!("css_select_style : property =%?=%?="j,i)) ;
                /* If the property is still unset then set it 
                 * to its initial value. */
                if (state.props[i].is_none() || state.props[i].get_ref()[j].get_ref().set == false) {
                    error = css_select_ctx::set_initial(&mut state, i as uint, unsafe { transmute(j)}, parent);
                    match error {
                        CSS_OK=>{},
                        x =>  {
                            return (x,None) ;
                        }
                    }
                }
                i += 1 ;
            }
            j += 1;
        }

        /* If this is the root element, then we must ensure that all
         * length values are absolute, display and float are correctly 
         * computed, and the default border-{top,right,bottom,left}-color 
         * is set to the computed value of color. */
        if (parent == null()) {
            /* Only compute absolute values for the base element */
            error = css__compute_absolute_values(None,
                    state.results.styles[CSS_PSEUDO_ELEMENT_NONE as uint].get_mut_ref(),
                    state.handler.get_ref().compute_font_size);
            match error {
                CSS_OK=>{},
                x =>  {
                    return (x,None) ;
                }
            }
        }

        (CSS_OK,Some(state.results))
    }


    /**
    * Destroy a selection result set
	* #Arguments:
	*  'results' - Result set to destroy.
    */
    pub fn css_select_results_destroy(results: &mut ~[css_select_results] ) {
        //debug!(fmt!("Entering css_select_results_destroy")) ;
        results.clear() ;
        
    }

    /**
    * Search a selection context for defined font faces
	* #Arguments:
	*  'self' - The selection context to look in.
	*  'media' - Currently active media types.
	*  'font_family' - Font family to search for.
    * #Return Value:
	* '(css_error,Option<@mut css_select_font_faces_results>)' - (
			CSS_OK,Some(css_select_font_faces_results)) on success, appropriate (error,None) otherwise.
    */
    pub fn css_select_font_faces(&mut self,
                                stylesheet_vector:&mut ~[css_stylesheet],
                                css_rule_data_list:&mut ~[~css_rule_data_type],
                                media:u64,
                                font_family:uint) 
                                -> (css_error,Option<~css_select_font_faces_results>) {

        //debug!(fmt!("Entering css_select_font_faces")) ;
        if( self.lwc_ref.lwc_string_length(font_family) == 0 ) {
            return (CSS_BADPARM,None) ;
        }

        let state = ~css_select_font_faces_state {
            font_family:Some(font_family),
            media:media,

            ua_font_faces:~[],
            user_font_faces:~[],
            author_font_faces:~[]
        };

        let mut ua_font_faces:~[~css_font_face] = ~[];
        let mut user_font_faces:~[~css_font_face] = ~[];
        let mut author_font_faces: ~[~css_font_face] = ~[];
        /* Iterate through the top-level stylesheets, selecting font-faces
         * from those which apply to our current media requirements and
         * are not disabled */
        let mut i = self.sheets.len() ;
        while (i>0) { 
            i -= 1 ;
            
            if ((self.sheets[i].media & media) != 0 ) && 
                (stylesheet_vector[self.sheets[i].sheet].disabled == false ) {

                let error = self.select_font_faces_from_sheet(stylesheet_vector, css_rule_data_list, self.sheets[i].sheet,
                                                        self.sheets[i].origin, 
                                                        media, 
                                                        font_family, 
                                                        &mut ua_font_faces, 
                                                        &mut user_font_faces,
                                                        &mut author_font_faces);
                match error {
                    CSS_OK=>{} ,
                    x => {
                        return (x,None) ;
                    }
                }
            }
        }
          
        let mut results = ~css_select_font_faces_results {
                font_faces:~[]
        };
          
		/* We found some matching faces.  Make a results structure with
		 * the font faces in priority order. */
        if (state.ua_font_faces.len() > 0) {
            results.font_faces.push(ua_font_faces); 
        }
        if (state.user_font_faces.len() > 0) {
		  results.font_faces.push(user_font_faces);
        }
        if (state.author_font_faces.len() > 0) {
		  results.font_faces.push(author_font_faces);
        }
     
        (CSS_OK,Some(results))
    }


    /******************************************************************************
     * Selection engine internals below here                                      *
     ******************************************************************************/
    #[inline]
    pub fn intern_strings(&mut self) {
        
                    /* Universal selector */
        self.universal = Some(self.lwc_ref.lwc_intern_string("*"));

        /* Pseudo classes */
        self.first_child = Some(self.lwc_ref.lwc_intern_string("first-child"));
        self.link = Some(self.lwc_ref.lwc_intern_string("link"));
        self.visited = Some(self.lwc_ref.lwc_intern_string("visited"));
        self.hover = Some(self.lwc_ref.lwc_intern_string("hover"));
        self.active = Some(self.lwc_ref.lwc_intern_string("active"));
        self.focus = Some(self.lwc_ref.lwc_intern_string("focus"));
        self.nth_child = Some(self.lwc_ref.lwc_intern_string("nth-child"));
        self.nth_last_child = Some(self.lwc_ref.lwc_intern_string("nth-last-child"));
        self.nth_of_type = Some(self.lwc_ref.lwc_intern_string("nth-of-type"));
        self.nth_last_of_type = Some(self.lwc_ref.lwc_intern_string("nth-last-of-type"));
        self.last_child = Some(self.lwc_ref.lwc_intern_string("last-child"));
        self.first_of_type = Some(self.lwc_ref.lwc_intern_string("first-of-type"));
        self.last_of_type = Some(self.lwc_ref.lwc_intern_string("last-of-type"));
        self.only_child = Some(self.lwc_ref.lwc_intern_string("only-child"));
        self.only_of_type = Some(self.lwc_ref.lwc_intern_string("only-of-type"));
        self.root = Some(self.lwc_ref.lwc_intern_string("root"));
        self.empty = Some(self.lwc_ref.lwc_intern_string("empty"));
        self.target = Some(self.lwc_ref.lwc_intern_string("target"));
        self.lang = Some(self.lwc_ref.lwc_intern_string("lang"));
        self.enabled = Some(self.lwc_ref.lwc_intern_string("enabled"));
        self.disabled = Some(self.lwc_ref.lwc_intern_string("disabled"));
        self.checked = Some(self.lwc_ref.lwc_intern_string("checked"));

        /* Pseudo elements */
        self.first_line = Some(self.lwc_ref.lwc_intern_string("first-line"));
        self.first_letter = Some(self.lwc_ref.lwc_intern_string("first-letter"));
        self.before = Some(self.lwc_ref.lwc_intern_string("before"));
        self.after = Some(self.lwc_ref.lwc_intern_string("after"));

    }

    #[inline]
    pub fn set_hint(state:&mut ~css_select_state, prop:u32) -> css_error {
        
        //debug!(fmt!("Entering set_hint")) ;
        /* Retrieve this property's hint from the client */
        let (error,hint_option) = (state.handler.get_ref().node_presentational_hint)(state.node, prop);
        match error {
            CSS_OK => {},
            CSS_PROPERTY_NOT_SET => return CSS_OK, 
            x => return x
        } 

        /* Hint defined -- set it in the result */
        let mut hint = hint_option.unwrap();
        let error =  (prop_dispatch[prop as uint].set_from_hint)(&mut hint, 
                                            state.results.styles[state.computed].get_mut_ref());

        match error {
            CSS_OK => {},
            x => {
                return x ;
            }
        }
        
        if (state.props[prop].is_none()) {
            let pstate = ~prop_state{
                specificity:0,
                set:true,
                origin:CSS_ORIGIN_AUTHOR as u8,
                important:false,
                inherit:(hint.status == 0)   
            };  
            let mut prop_vec: ~[Option<~prop_state>] = from_elem(CSS_PSEUDO_ELEMENT_COUNT as uint,None);
            prop_vec[CSS_PSEUDO_ELEMENT_NONE as uint] = Some(pstate);
            state.props[prop] = Some(prop_vec);
        }  
        else {
            /* Keep selection state in sync with reality */
            state.props[prop].get_mut_ref()[CSS_PSEUDO_ELEMENT_NONE as uint].get_mut_ref().set = true;
            state.props[prop].get_mut_ref()[CSS_PSEUDO_ELEMENT_NONE as uint].get_mut_ref().specificity = 0;
            state.props[prop].get_mut_ref()[CSS_PSEUDO_ELEMENT_NONE as uint].get_mut_ref().origin = CSS_ORIGIN_AUTHOR as u8;
            state.props[prop].get_mut_ref()[CSS_PSEUDO_ELEMENT_NONE as uint].get_mut_ref().important = false;
            state.props[prop].get_mut_ref()[CSS_PSEUDO_ELEMENT_NONE as uint].get_mut_ref().inherit = (hint.status == 0);
        }
            
        return CSS_OK;
    }

    #[inline]
    pub fn set_initial(state :&mut ~css_select_state, prop : uint, pseudo : css_pseudo_element,
        parent: *c_void) -> css_error {

        //debug!(fmt!("Entering set_initial")) ;
        let mut error : css_error; 

        /* Do nothing if this property is inherited (the default state 
         * of a clean computed style is for everything to be set to inherit)
         *
         * If the node is tree root and we're dealing with the base element, 
         * everything should be defaulted.
         */

        if dispatch_table::get_inherited(prop) == 0 || match pseudo { CSS_PSEUDO_ELEMENT_NONE => true, _ => false} &&
            parent == null() {
            
            let group : prop_group = dispatch_table::get_group(prop);

            /* Remaining properties are neither inherited nor 
             * already set. Thus, we set them to their initial 
             * values here. Except, however, if the property in 
             * question resides in one of the extension blocks and 
             * the extension block has yet to be allocated. In that 
             * case, we do nothing and leave it to the property 
             * accessors to return the initial values for the 
             * property.
             */

            match group {
                GROUP_NORMAL => {
                    error = (prop_dispatch[prop].initial)(state);
                    match error {
                        CSS_OK => {},
                        _=> {
                            return error;
                        }
                    }
                }

                GROUP_UNCOMMON => {
                    match state.results.styles[state.computed].get_ref().uncommon {
                        None => {},
                        Some(_) => {
                            error = (prop_dispatch[prop].initial)(state);
                            match error {
                                CSS_OK => {},
                                _=> {
                                    return error;
                                }
                            }
                        }
                    }
                }
                
                GROUP_PAGE => {
                    match state.results.styles[state.computed].get_ref().page {
                        None => {},
                        Some(_) => {
                            error = (prop_dispatch[prop].initial)(state);
                            match error {
                                CSS_OK => {},
                                _=> {
                                    return error;
                                }
                            }
                        }
                    }
                }
                
                GROUP_AURAL => {
                    match state.results.styles[state.computed].get_ref().aural {
                        None => {},
                        Some(_) => {
                            error = (prop_dispatch[prop].initial)(state);
                            match error {
                                CSS_OK => {},
                                _=> {
                                    return error;
                                }
                            }
                        }
                    }
                }
                
            }
        }
        CSS_OK
    }

    pub fn select_from_sheet(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], sheet : uint, origin : css_origin, state :&mut ~css_select_state) -> css_error{

        //debug!(fmt!("Entering select_from_sheet")) ;
        let mut s:Option<uint> = Some(sheet);
        let mut rule : Option<uint> = stylesheet_vector[s.expect("")].rule_list;
        let mut sp : u32 = 0;
        let mut import_stack : ~[css_rule_data_index] = ~[];

        loop{
            /* Find first non-charset rule, if we're at the list head */
            if compare_css_rdt(rule, stylesheet_vector[s.unwrap()].rule_list) {
                while rule.is_some() && compare_CSS_RULE_TYPEs(Some(&css_rule_data_list[rule.unwrap()]), CSS_RULE_CHARSET) {
                    rule = stylesheet_vector[sheet].get_css_rule_next(css_rule_data_list, rule.expect(""));
                }
            }
            if rule.is_some() && compare_CSS_RULE_TYPEs(Some(&css_rule_data_list[rule.unwrap()]), CSS_RULE_IMPORT) {
                /* Current rule is an import */
		    let mut import_sheet : Option<uint> = None;
                let mut import_media:u64 = 0;
                match css_rule_data_list[rule.unwrap()].rule_type {
                    CSS_RULE_IMPORT => {
                        import_media = css_rule_data_list[rule.unwrap()].rule_import.get_mut_ref().media;
                        import_sheet = css_rule_data_list[rule.unwrap()].rule_import.get_mut_ref().sheet;
                    },
                    _=> {},
                }

                if import_sheet.is_some() && ((import_media & state.media) != 0) {
                    /* It's applicable, so process it */

                    import_stack.push(rule.unwrap());

                    s = import_sheet;
                    rule = stylesheet_vector[s.expect("")].rule_list;
                }
                else {
                    /* Not applicable; skip over it */
                    rule = stylesheet_vector[sheet].get_css_rule_next(css_rule_data_list, rule.expect(""));
                }
            }
            else {
                /* Gone past import rules in this sheet */
                let mut error : css_error;

                /* Process this sheet */
                state.sheet = s;
                state.current_origin = origin;

                error = self.match_selectors_in_sheet(stylesheet_vector, css_rule_data_list, s.expect(""), state);
                match error {
                    CSS_OK => {
                        if sp > 0 {
                            sp -= 1;
                            rule = stylesheet_vector[sheet].get_css_rule_next(css_rule_data_list, import_stack[sp]);
                            if stylesheet_vector[sheet].get_stylesheet_parent(css_rule_data_list, import_stack[sp]) {
                                s = Some(sheet)
                            }
                            else {
                                s = None
                            }

                        }
                        else {
                            s = None;
                        }
                    },
                    _=> { 
                        return error;
                    }
                }
            }
            
            if s.is_none() {
                break;
            }
 
        }

        CSS_OK
    }

    #[inline]
    pub fn _rule_applies_to_media(stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], sheet:uint, rule: Option<css_rule_data_index>, media:u64) -> bool {

        //debug!(fmt!("Entering _rule_applies_to_media")) ;
        let mut applies : bool = true;
        let mut ancestor = rule;

        loop {  
            match ancestor {
                None=>{
                    break ;
                },
                Some(ancestor_rule)=> {
                    match css_rule_data_list[ancestor_rule].rule_type {
                        CSS_RULE_MEDIA=>{
                            if( ( css_rule_data_list[ancestor_rule].rule_media.get_mut_ref().media & media ) == 0 ) {
                                applies = false ;
                                return applies ;
                            }

                            if !stylesheet_vector[sheet].css_rule_list[css_rule_data_list[ancestor_rule].rule_media.get_mut_ref().base].parent_stylesheet {
                                ancestor = stylesheet_vector[sheet].css_rule_list[css_rule_data_list[ancestor_rule].rule_media.get_mut_ref().base].parent_rule ;
                            }
                            else {
                                ancestor = None ;
                            }
                            loop ;
                        },
                        _ => {
                            let ancestor_base = stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, ancestor_rule);
                            if !stylesheet_vector[sheet].css_rule_list[ancestor_base].parent_stylesheet {
                                ancestor = stylesheet_vector[sheet].css_rule_list[ancestor_base].parent_rule ;
                            }
                            else {
                                ancestor = None ;
                            }
                            loop ;
                        }
                    }
                }
            }
        }
        applies
    }

    pub fn _select_font_face_from_rule(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], sheet :uint,
                                    rule:css_rule_data_index,
                                    origin: css_origin,
                                    media: u64,
                                    font_family: uint,
                                    ua_font_faces:&mut ~[~css_font_face],
                                    user_font_faces:&mut ~[~css_font_face],
                                    author_font_faces:&mut ~[~css_font_face]) 
                                    -> css_error {

        //debug!(fmt!("Entering _select_font_face_from_rule")) ;

        if ( css_select_ctx::_rule_applies_to_media(stylesheet_vector, css_rule_data_list, sheet, Some(rule), media) ) {

            if ( css_rule_data_list[rule].rule_font_face.get_ref().font_face.is_none() || 
                css_rule_data_list[rule].rule_font_face.get_ref().font_face.get_ref().font_family.is_none()
                ) {
                return CSS_BADPARM ;
            }

            let res : bool = self.lwc_ref.lwc_string_isequal(css_rule_data_list[rule].rule_font_face.get_ref().font_face.get_ref().font_family.expect(""), font_family) ;

            if ( res ) {
				match (origin) {
					CSS_ORIGIN_UA => {
						ua_font_faces.push(css_rule_data_list[rule].rule_font_face.get_ref().font_face.get_ref().clone());
					},
					CSS_ORIGIN_USER => {
						user_font_faces.push(css_rule_data_list[rule].rule_font_face.get_ref().font_face.get_ref().clone());
					},
					CSS_ORIGIN_AUTHOR => {
						author_font_faces.push(css_rule_data_list[rule].rule_font_face.get_ref().font_face.get_ref().clone());
					}
				}
            }
        }
        CSS_OK
    }

    pub fn select_font_faces_from_sheet(&mut self,
                                        stylesheet_vector:&mut ~[css_stylesheet],
                                        css_rule_data_list: &mut ~[~css_rule_data_type],
                                        sheet:uint,
                                        origin: css_origin,
                                        media: u64,
                                        font_family: uint,
                                        ua_font_faces:&mut ~[~css_font_face],
                                        user_font_faces:&mut ~[~css_font_face],
                                        author_font_faces:&mut ~[~css_font_face] )
                                        -> css_error {

        //debug!(fmt!("Entering select_font_faces_from_sheet")) ;
        let mut s = Some(sheet) ;
        let mut rule = stylesheet_vector[s.expect("")].rule_list;
        let mut sp : u32 = 0 ;
        let mut import_stack : ~[css_rule_data_index] = ~[];
        import_stack.reserve_at_least(IMPORT_STACK_SIZE as uint) ;

        let mut ptr = rule ;
        while ( s.is_some() ) {
            loop {
                match ptr {
                    None=> { 
                        break ;
                    },
                    Some(current_rule) => {
                        match css_rule_data_list[current_rule].rule_type {
                            CSS_RULE_CHARSET =>{
                                ptr = stylesheet_vector[sheet].css_rule_list[stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                                loop;
                            },
                            _=> {
                                break ;
                            }
                        }
                    }
                }
            }
            match ptr {
                None=> {
                    /* Find next sheet to process */
                    if (sp > 0) {
                        sp -= 1;
                        ptr = stylesheet_vector[sheet].css_rule_list[stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, import_stack[sp])].next;
                        s = if stylesheet_vector[sheet].css_rule_list[stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, import_stack[sp])].parent_stylesheet { Some(sheet)} else { None} ;
                    } 
                    else {
                        s = None;
                    }
                },
                Some(current_rule) => {
                    match css_rule_data_list[current_rule].rule_type {
                        CSS_RULE_CHARSET =>{
                            ptr = stylesheet_vector[sheet].css_rule_list[stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                        },
                        CSS_RULE_IMPORT => {
                            /* Current rule is an import */
                            if ( css_rule_data_list[current_rule].rule_import.get_mut_ref().sheet.is_some() && 
                                ((css_rule_data_list[current_rule].rule_import.get_mut_ref().media & media) != 0) ) {
                                if ( sp >= IMPORT_STACK_SIZE as u32) {
                                    return CSS_NOMEM ;
                                }
                                import_stack[sp] = current_rule ;
                                sp += 1;
                                s = css_rule_data_list[current_rule].rule_import.get_mut_ref().sheet ;
                                rule = stylesheet_vector[s.expect("")].rule_list ;
                                ptr = rule ;
                            }
                            else {
                                ptr = stylesheet_vector[sheet].css_rule_list[stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                            }
                        },
                        CSS_RULE_FONT_FACE => {
                            let error : css_error = self._select_font_face_from_rule(
                                                            stylesheet_vector,
                                                            css_rule_data_list,
                                                            sheet,
                                                            current_rule,
                                                            origin,
                                                            media,
                                                            font_family,
                                                            ua_font_faces,
                                                            user_font_faces,
                                                            author_font_faces
                                                            );
                            match error {
                                CSS_OK=>{},
                                x => { 
                                    return x ;
                                }
                            }

                            ptr = stylesheet_vector[sheet].css_rule_list[stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                        },
                        _=> {
                            ptr = stylesheet_vector[sheet].css_rule_list[stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                        }
                    }
                }
            }
        }

        CSS_OK
    }

    #[inline]
    pub fn _selectors_pending(node: Option<uint>, id: Option<uint>,
                classes: &~[Option<uint>], 
                univ: Option<uint>) -> bool {

        //debug!(fmt!("Entering _selectors_pending")) ;
        let mut pending : bool = false;
        match node {
            None => {}
            Some(_) => {
                pending = true;
            }
        }
        match id {
            None => {}
            Some(_) => {
                pending = true;
            }
        }
        match univ {
            None => {}
            Some(_) => {
                pending = true;
            }
        }

        let mut z = 0 ;
        let z_len = classes.len();
	    while z<z_len {        
            match classes[z] {
                None => {}
                Some(_) => {
                    pending = true;
                }
            }
            z += 1;
        }

        pending
    }

    #[inline]
    pub fn _selector_less_specific(stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list: &mut ~[~css_rule_data_type], sheet: uint, refer:Option<uint>, 
                                cand:Option<uint>) 
                                -> bool {

        //debug!(fmt!("Entering _selector_less_specific")) ;
        let mut result : bool;

        if (cand.is_none()) {
            return false;
        }

        if (refer.is_none()) {
            return true;
        }

        /* Sort by specificity */
        if (stylesheet_vector[sheet].css_selectors_list[cand.unwrap()].specificity < stylesheet_vector[sheet].css_selectors_list[refer.unwrap()].specificity) {
            result = true;
        } 
        else if (stylesheet_vector[sheet].css_selectors_list[refer.unwrap()].specificity < stylesheet_vector[sheet].css_selectors_list[cand.unwrap()].specificity) {
            result = false;
        } 
        else {

            if( stylesheet_vector[sheet].css_selectors_list[cand.unwrap()].rule.is_none() || stylesheet_vector[sheet].css_selectors_list[refer.unwrap()].rule.is_none() ) {
                fail!(~"_selector_less_specific:Base rule cannot be null");
            }
            let cand_base = stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, stylesheet_vector[sheet].css_selectors_list[cand.unwrap()].rule.unwrap()) ;
            let refer_base = stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, stylesheet_vector[sheet].css_selectors_list[refer.unwrap()].rule.unwrap()) ;
            /* Then by rule index -- earliest wins */
            if (stylesheet_vector[sheet].css_rule_list[cand_base].index < stylesheet_vector[sheet].css_rule_list[refer_base].index) {
                result = true;
            }
            else {
                result = false;
            }
        }

        result
    }

    #[inline]
    pub fn _selector_next(stylesheet_vector:&mut ~[css_stylesheet], 
                            css_rule_data_list: &mut ~[~css_rule_data_type],
                            sheet: uint,
                            node: Option<uint>, 
                            id: Option<uint>,
                            classes: &~[Option<uint>], 
                            univ: Option<uint>) 
                            -> Option<uint> {

        //debug!(fmt!("Entering _selector_next")) ;
        let mut ret : Option<uint> = None;

        if (css_select_ctx::_selector_less_specific(stylesheet_vector, css_rule_data_list, sheet, ret, node)) {
            ret = Some(node.expect(""));
        }

        if (css_select_ctx::_selector_less_specific(stylesheet_vector, css_rule_data_list, sheet, ret, id)) {
            ret = Some(id.expect(""));
        }

        if (css_select_ctx::_selector_less_specific(stylesheet_vector, css_rule_data_list, sheet, ret, univ)) {
            ret = Some(univ.expect(""));
        }

        let mut i : uint = 0;
		let classes_len : uint = classes.len();
        while i < classes_len {
            if (css_select_ctx::_selector_less_specific(stylesheet_vector, css_rule_data_list, sheet, ret,classes[i])){
                ret = Some(classes[i].expect(""));
            }
            i += 1;
        }
        ret
    }

    pub fn _rule_good_for_element_name(selector:&mut css_selector, lwc_ref:&mut ~lwc,
        src:&mut css_select_rule_source, state:&css_select_state) -> bool {
        /* If source of rule is element or universal hash, we know the
         * element name is a match.  If it comes from the class or id hash,
         * we have to test for a match */
        //debug!(fmt!("Entering _rule_good_for_element_name")) ;
        if (match src.source { 
            CSS_SELECT_RULE_SRC_ID | CSS_SELECT_RULE_SRC_CLASS => true,
            _ => false }) {
            
            if ( lwc_ref.lwc_string_length(selector.data[0].qname.name) != 1 ||
                   lwc_ref.lwc_string_data(selector.data[0].qname.name)[0] != ('*' as u8) ) {
                
                if selector.data[0].qname.name == state.element.name {
                    return false;
                }
            }
        }    
        return true;
    }        

    pub fn match_selectors_in_sheet(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], sheet : uint, 
                                    state :&mut ~css_select_state) -> css_error {
    
        //debug!(fmt!("Entering match_selectors_in_sheet")) ;
        let mut node_selectors_hash_entry : Option<uint> = None ;
        let mut node_selectors_option : Option<uint> = None ;
        let mut id_selectors_hash_entry : Option<uint> = None ;
        let mut id_selectors_option : Option<uint> = None ;
        let mut class_selectors_hash_entry : ~[Option<uint>] = ~[];
        let mut class_selectors_option_list : ~[Option<uint>] = ~[] ;
        let mut univ_selectors_hash_entry : Option<uint> = None ;
        let mut univ_selectors_option : Option<uint> = None ;
        //let mut error : css_error ;

        /* Find hash chain that applies to current node */
        let (sel,error) = stylesheet_vector[sheet].css__selector_hash_find(&mut self.lwc_ref, state.element.name);
        match error {
            CSS_OK => {},
            err => {
                return err;
            }
        }
        if sel.is_some() {
            node_selectors_hash_entry = sel;
            node_selectors_option = Some(stylesheet_vector[sheet].selectors.hash_entry_list[sel.unwrap()].selector) ;
        }

        if ( state.classes.len() != 0 ) {
             /* Find hash chains for node classes */
			//debug!(fmt!("state.classes=%?",state.classes));
			
            let mut z = 0 ;
            let z_len = state.classes.len();
            while z<z_len {
                let (sel_class,error) = stylesheet_vector[sheet].css__selector_hash_find_by_class(&mut self.lwc_ref, state.classes[z]);
                match error {
                    CSS_OK => {},
                    err => {
                        return err;
                    }
                }
				
                if sel_class.is_some() {
                    class_selectors_hash_entry.push(sel_class) ;
                    class_selectors_option_list.push(Some(stylesheet_vector[sheet].selectors.hash_entry_list[sel_class.unwrap()].selector)) ;
                }
                z += 1;
            }
        }
		
		//debug!(fmt!("state.id=%?, state.id.len=%?", state.id, state.id.len()));
				
        if ( self.lwc_ref.lwc_string_length(state.id) != 0 ) {
            /* Find hash chain for node ID */
            let (sel_id,error) = stylesheet_vector[sheet].css__selector_hash_find_by_id(&mut self.lwc_ref, state.id);
            match error {
                CSS_OK => {},
                err => {
                    return err;
                }
            }
            if sel_id.is_some() {
                id_selectors_hash_entry = sel_id ;
                id_selectors_option = Some(stylesheet_vector[sheet].selectors.hash_entry_list[sel_id.unwrap()].selector) ;
            }
        }

        /* Find hash chain for universal selector */
        let (sel_univ,error) = stylesheet_vector[sheet].selectors.css__selector_hash_find_universal();
        match error {
            CSS_OK => {},
            err => {
                return err;
            }
        }
        if sel_univ.is_some() {
            univ_selectors_hash_entry = sel_univ ;
            univ_selectors_option = Some(stylesheet_vector[sheet].selectors.hash_entry_list[sel_univ.unwrap()].selector) ;
        }

        // /* Process matching selectors, if any */
        while ( css_select_ctx::_selectors_pending(node_selectors_option, 
                                                    id_selectors_option, 
                                                    &class_selectors_option_list,
                                                    univ_selectors_option) ) {
            let mut selector : uint ;

            /*Selectors must be matched in ascending order of specificity
             * and rule index. (c.f. css__outranks_existing())
             *
             * Pick the least specific/earliest occurring selector.
             */
            let o_selector = css_select_ctx::_selector_next(
                                    stylesheet_vector, 
                                    css_rule_data_list,
                                    sheet,
                                    node_selectors_option, 
                                    id_selectors_option,
                                    &class_selectors_option_list, 
                                    univ_selectors_option );

            if o_selector.is_none() {
                fail!(~"Error getting selector next ") ;
            }
            selector = o_selector.expect("") ; 
            /* Ignore any selectors contained in rules which are a child 
             * of an @media block that doesn't match the current media 
             * requirements. */
            if (css_select_ctx::_rule_applies_to_media(stylesheet_vector, css_rule_data_list, sheet, stylesheet_vector[sheet].css_selectors_list[selector].rule, state.media)) {
                let error = self.match_selector_chain(stylesheet_vector, css_rule_data_list, sheet, Some(selector), state);
                match error {
                    CSS_OK => {},
                    err => {
                        return err;
                    }
                }
            }

            /* Advance to next selector in whichever chain we extracted 
             * the processed selector from. */
            if ( node_selectors_option.is_some() &&
                selector == node_selectors_option.expect("") ) {
                let (node_next_hash,error) = 
                        stylesheet_vector[sheet]._iterate_elements(&mut self.lwc_ref, node_selectors_hash_entry.expect(""));

                match error {
                    CSS_OK => {},
                    err => {
                        return err;
                    }
                }

                if node_next_hash.is_some() {
                    node_selectors_hash_entry = node_next_hash;
                    node_selectors_option = Some(stylesheet_vector[sheet].selectors.hash_entry_list[node_next_hash.unwrap()].selector) ;
                }
                else {
                    node_selectors_option = None ;
                }
            } 
            else if (   id_selectors_option.is_some() &&
                        selector ==  id_selectors_option.expect("") ){
                let (id_next_hash,error) = 
                            stylesheet_vector[sheet]._iterate_ids(&mut self.lwc_ref, id_selectors_hash_entry.expect(""));

                match error {
                    CSS_OK => {},
                    err => {
                        return err;
                    }
                }

                if id_next_hash.is_some() {
                    id_selectors_hash_entry = id_next_hash;
                    id_selectors_option = Some(stylesheet_vector[sheet].selectors.hash_entry_list[id_next_hash.unwrap()].selector) ;
                }
                else {
                    id_selectors_option = None ;
                }
            } 
            else if (   univ_selectors_option.is_some() &&
                        selector == univ_selectors_option.expect("") ){
                let (univ_next_hash,error) = 
                            stylesheet_vector[sheet].selectors._iterate_universal(univ_selectors_hash_entry.unwrap());

                match error {
                    CSS_OK => {},
                    err => {
                        return err;
                    }
                }

                if univ_next_hash.is_some() {
                    univ_selectors_hash_entry = univ_next_hash;
                    univ_selectors_option = Some(stylesheet_vector[sheet].selectors.hash_entry_list[univ_next_hash.unwrap()].selector);
                }
                else {
                    univ_selectors_option = None ;
                }
            } 
            else {
                let mut i = 0 ;
                let n_classes = class_selectors_option_list.len()  ;
                while i < n_classes  {
                    if ( class_selectors_option_list[i].is_some() &&
                         selector == class_selectors_option_list[i].expect("")) {
                        let (class_next_hash,error) = 
                                        stylesheet_vector[sheet]._iterate_classes(
                                                    &mut self.lwc_ref, class_selectors_hash_entry[i].unwrap());

                        match error {
                            CSS_OK => {},
                            err => {
                                return err;
                            }
                        }

                        if class_next_hash.is_some() {
                            class_selectors_hash_entry[i] = class_next_hash;
                            class_selectors_option_list[i] = Some(stylesheet_vector[sheet].selectors.hash_entry_list[class_next_hash.unwrap()].selector);
                        }
                        else {
                            class_selectors_option_list[i] = None;
                        }
                        break;
                    }
					i = i + 1;
                }
            }

            match error {
                CSS_OK => {},
                err => {
                    return err;
                }
            }
        }

        CSS_OK
    }
    pub fn update_reject_cache(stylesheet_vector:&mut ~[css_stylesheet], sheet:uint, state:&mut ~css_select_state, comb:css_combinator,
                                s:uint) {

        //debug!(fmt!("Entering update_reject_cache")) ;
        let mut  next_detail : Option<& ~css_selector_detail> = None;

		if (stylesheet_vector[sheet].css_selectors_list[s].data.len() > 1 ) {
			next_detail = Some(&stylesheet_vector[sheet].css_selectors_list[s].data[1]);
		}

		if ( (state.next_reject < 0) ||

			(match comb {   
				CSS_COMBINATOR_ANCESTOR => { false },
				_=>{
					true
				}
			})   ||

			(next_detail.is_none()) ||

			(if (stylesheet_vector[sheet].css_selectors_list[s].data.len() > 2) {
				true
			} 
			else {
				false
			}) ||

			(match next_detail.expect("").selector_type {   
				CSS_SELECTOR_CLASS=> { false },         
				CSS_SELECTOR_ID=>{false},
				_=>{
					true  
				}
			}) 
		) {

			return ;
		}
 
        /* Insert */
        let item : reject_item = reject_item{
            value: next_detail.expect("").qname.name ,
            sel_type: next_detail.expect("").selector_type
        };
        state.reject_cache[state.next_reject] = Some(item) ;
        state.next_reject -= 1;
    }

    pub fn match_named_combinator(&mut self, stylesheet_vector:&mut ~[css_stylesheet], sheet: uint, combinator_type:css_combinator,
        selector:uint, state:&mut ~css_select_state, 
        node:*c_void, next_node:*mut *c_void) -> css_error {

        //debug!(fmt!("Entering match_named_combinator")) ;
        let mut n = node;
        let mut error:css_error;

        loop {
            let match_result = &mut false;

            /* Find candidate node */
            match combinator_type {
                CSS_COMBINATOR_ANCESTOR => {
                    error = (state.handler.get_ref().named_ancestor_node)( 
                            &mut self.lwc_ref, n, stylesheet_vector[sheet].css_selectors_list[selector].data[0].qname, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }   
                CSS_COMBINATOR_PARENT => {
                    error = (state.handler.get_ref().named_parent_node)( 
                            &mut self.lwc_ref, n, stylesheet_vector[sheet].css_selectors_list[selector].data[0].qname, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }    
                CSS_COMBINATOR_SIBLING => {
                    error = (state.handler.get_ref().named_sibling_node)( 
                            &mut self.lwc_ref, n, stylesheet_vector[sheet].css_selectors_list[selector].data[0].qname, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }    
                    
                CSS_COMBINATOR_GENERIC_SIBLING => {
                    error = (state.handler.get_ref().named_generic_sibling_node)(
                            &mut self.lwc_ref, n, stylesheet_vector[sheet].css_selectors_list[selector].data[0].qname, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }        
                CSS_COMBINATOR_NONE => {}
                    
            }

            if n != null() {
                /* Match its details */
                //debug!("");
                error = self.match_details(n, stylesheet_vector[sheet].css_selectors_list[selector].data, state, match_result, None);
                match error {
                    CSS_OK => {},
                    err => return err
                }
                
                //debug!(fmt!("match_result=%?", match_result));
                /* If we found a match, use it */
                if (*match_result == true){
                    break   
                }
                    

                /* For parent and sibling selectors, only adjacent 
                 * nodes are valid. Thus, if we failed to match, 
                 * give up. */
                match combinator_type { 
                    CSS_COMBINATOR_PARENT | CSS_COMBINATOR_SIBLING => {
                        n = null();   
                    },
                    _  => {}
                }    
                    
            }

            if n == null() {
                break
            }
        }

        unsafe { *next_node = n };

        return CSS_OK;
    }
    pub fn match_selector_chain(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], sheet:uint, selector:Option<uint>,
                            state:&mut ~css_select_state) -> css_error {

        //debug!(fmt!("Entering match_selector_chain")) ;
        let mut s = selector;
        let mut node = state.node;
        let match_b : &mut bool = &mut false;
        let mut may_optimise = true;
        let rejected_by_cache : &mut bool = &mut true ;
        let mut pseudo : css_pseudo_element = CSS_PSEUDO_ELEMENT_NONE ;
        let mut error : css_error ;
        let universal_string = self.universal.expect("") ;
          
        
        /* Match the details of the first selector in the chain. 
         *
         * Note that pseudo elements will only appear as details of
         * the first selector in the chain, as the parser will reject
         * any selector chains containing pseudo elements anywhere 
         * else.
         */
      
        error = self.match_details(node, (stylesheet_vector[sheet].css_selectors_list[s.unwrap()].data) , state, match_b, Some(&mut pseudo) );
       
        match error {
            CSS_OK => {},
            err => { 
                return err ;
            }
        }

        /* Details don't match, so reject selector chain */
        if (*match_b == false) {
            return CSS_OK;
        }

        
		/* Iterate up the selector chain, matching combinators */
		loop {
			let mut next_node : *c_void = null();

			/* Consider any combinator on this selector */
			if ( (stylesheet_vector[sheet].css_selectors_list[s.expect("")].data.len() > 0 ) && 
				 ( match stylesheet_vector[sheet].css_selectors_list[s.expect("")].data[0].combinator_type { 
					CSS_COMBINATOR_NONE=>{false},
					_=>{true} }
				 ) && 
				 (stylesheet_vector[sheet].css_selectors_list[s.expect("")].combinator.is_some() ) &&
				 (self.universal.is_some() ) &&
				 (stylesheet_vector[sheet].css_selectors_list[stylesheet_vector[sheet].css_selectors_list[s.expect("")].combinator.expect("")].data[0].qname.name != 
				  universal_string) ) {

				/* Named combinator */
				
					may_optimise &= match stylesheet_vector[sheet].css_selectors_list[s.expect("")].data[0].combinator_type {
						CSS_COMBINATOR_ANCESTOR=> { true },
						CSS_COMBINATOR_PARENT=>{ true },
						_=>{ false }
					} ;
				

				error = self.match_named_combinator(stylesheet_vector, sheet, stylesheet_vector[sheet].css_selectors_list[s.expect("")].data[0].combinator_type, 
					   stylesheet_vector[sheet].css_selectors_list[s.expect("")].combinator.expect(""), state, node, &mut next_node);
				match error {
					CSS_OK => {},
					err => { 
						return err ;
					}
				}

				/* No match for combinator, so reject selector chain */
				if (next_node == null() ) {
					return CSS_OK;
				}
			} 
			else if ( (stylesheet_vector[sheet].css_selectors_list[s.expect("")].data.len() > 0 ) &&
					( match stylesheet_vector[sheet].css_selectors_list[s.expect("")].data[0].combinator_type { 
						CSS_COMBINATOR_NONE=>{false},
						_=>{true} }
					) ) {

				/* Universal combinator */
				
					may_optimise &= match stylesheet_vector[sheet].css_selectors_list[s.expect("")].data[0].combinator_type {
						CSS_COMBINATOR_ANCESTOR=> { true },
						CSS_COMBINATOR_PARENT=>{ true },
						_=>{ false }
					} ;
				

				error = self.match_universal_combinator(stylesheet_vector, sheet, stylesheet_vector[sheet].css_selectors_list[s.expect("")].data[0].combinator_type, 
												stylesheet_vector[sheet].css_selectors_list[s.expect("")].combinator.expect(""), state, node, 
												may_optimise, rejected_by_cache,
												&mut next_node);
				match error {
					CSS_OK => {},
					err => { 
						return err ;
					}
				}

				/* No match for combinator, so reject selector chain */
				if (next_node == null()) {
					if ( may_optimise && (s.expect("") == selector.expect("")) &&
							*rejected_by_cache == false) {
						css_select_ctx::update_reject_cache(stylesheet_vector, sheet, state, 
												stylesheet_vector[sheet].css_selectors_list[s.expect("")].data[0].combinator_type,
												stylesheet_vector[sheet].css_selectors_list[s.expect("")].combinator.expect(""));
					}

					return CSS_OK;
				}
			}

			/* Details matched, so progress to combining selector */
			s = stylesheet_vector[sheet].css_selectors_list[s.expect("")].combinator;
			node = next_node;

			if s.is_none() {
				break;
			}
             
        }
        /* If we got here, then the entire selector chain matched, so cascade */
        state.current_specificity = stylesheet_vector[sheet].css_selectors_list[selector.expect("")].specificity;

        /* No bytecode if rule body is empty or wholly invalid */
        if ( stylesheet_vector[sheet].css_selectors_list[selector.expect("")].rule.is_none() ) {
            return CSS_OK;
        }

         /* No bytecode if rule body is empty or wholly invalid */
        let x_rule = stylesheet_vector[sheet].css_selectors_list[selector.expect("")].rule.unwrap();
        
        if css_rule_data_list[x_rule].rule_type as uint != CSS_RULE_SELECTOR as uint {
            return CSS_OK ;
        }    
        
        let rule = css_rule_data_list[x_rule].rule_selector.get_mut_ref();


        if ( rule.style.is_none() ) {
            return CSS_OK ;
        }
 
		if( state.results.styles.len() <= pseudo as uint ) {
			return CSS_INVALID ;
		}

		/* Ensure that the appropriate computed style exists */
		if ( state.results.styles[pseudo as uint].is_none() ) {
			state.results.styles[pseudo as uint] = Some(css_computed_style_create()); 
		}
      
        state.current_pseudo = pseudo;
        state.computed = pseudo as uint;


        css_select_ctx::cascade_style( stylesheet_vector, rule.style.get_mut_ref() , state)
    }

    pub fn match_universal_combinator(&mut self, stylesheet_vector:&mut ~[css_stylesheet], sheet: uint, combinator_type:css_combinator,
        selector:uint, state:&mut ~css_select_state,
        node:*c_void, may_optimise:bool, rejected_by_cache:&mut bool,
        next_node:*mut *c_void) -> css_error  {
        
        //debug!(fmt!("Entering match_universal_combinator")) ;
        let mut n:*c_void = node;
        //println(fmt!("n = %?", n));
		if ( n == null()){
			//debug!("Node Is Null");
		}

        let mut error:css_error;
		//Block for handling css_selectors_list borrow
        {
            let mut next_detail:Option<&~css_selector_detail> = None; 
            
            
            if (stylesheet_vector[sheet].css_selectors_list[selector].data.len() > 1){
                next_detail = Some(&stylesheet_vector[sheet].css_selectors_list[selector].data[1]);   
            }
                
            *rejected_by_cache = false;

            /* Consult reject cache first */
            if (may_optimise && 
                (match combinator_type { CSS_COMBINATOR_ANCESTOR | CSS_COMBINATOR_PARENT => true, _ => false }) && 
                match next_detail { Some(_) => true, None => false } &&
                (match next_detail.get_ref().selector_type { CSS_SELECTOR_CLASS | CSS_SELECTOR_ID => true, _ => false})) {

                let mut reject = state.next_reject + 1;
                let last : int = (state.reject_cache.len() -1) as int ;

                while (reject <= last) {
                    /* Perform pessimistic matching (may hurt quirks) */
                    if ((state.reject_cache[reject]).get_ref().sel_type as uint == next_detail.get_ref().selector_type as uint) &&
                       ((state.reject_cache[reject]).get_ref().value ==next_detail.get_ref().qname.name ) {
                        
                        /* Found it: can't match */
                        unsafe { *next_node = null() };
                        *rejected_by_cache = true;
                        return CSS_OK;
                    }

                    reject += 1;
                }
            }
        }
            
        loop {
            let mut match_result = false;

            /* Find candidate node */
            match (combinator_type) {
                CSS_COMBINATOR_ANCESTOR | 
                CSS_COMBINATOR_PARENT => {
					//println(fmt!("n = %?", n));
                    error = (state.handler.get_ref().parent_node)(n, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }
                CSS_COMBINATOR_SIBLING |
                CSS_COMBINATOR_GENERIC_SIBLING => {
                    error = (state.handler.get_ref().sibling_node)(n, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }
                CSS_COMBINATOR_NONE => {}
            }

            if (n != null()) {
                /* Match its details */
                let length = stylesheet_vector[sheet].css_selectors_list[selector].data.len();
                error = self.match_details(n, stylesheet_vector[sheet].css_selectors_list[selector].data.mut_slice(1,length), state, &mut match_result, None);
                match error {
                    CSS_OK => {},
                    err => return err
                }

                /* If we found a match, use it */
                if (match_result == true){
                    break   
                }

                /* For parent and sibling selectors, only adjacent
                 * nodes are valid. Thus, if we failed to match,
                 * give up. */
                
                match combinator_type { 
                    CSS_COMBINATOR_PARENT | CSS_COMBINATOR_SIBLING => {
                        n = null();   
                    },
                    _  => {}
                }    

                if n == null() {
                break
            }    
        }
    } 

        unsafe { *next_node = n };

        return CSS_OK;
    }

    pub fn match_details(&mut self,  node:*c_void, 
        detail :&mut [~css_selector_detail], state :&mut ~css_select_state, 
        matched : &mut bool, pseudo_element : Option<&mut css_pseudo_element>) -> css_error {

        //debug!(fmt!("Entering match_details")) ;
        let mut error : css_error ;
        let mut pseudo : css_pseudo_element = CSS_PSEUDO_ELEMENT_NONE;
        let mut index:uint = 0;

        /* Skip the element selector detail, which is always first.
         * (Named elements are handled by match_named_combinator, so the
         * element selector detail always matches here.) */

        if(detail.len() > 1){
            index += 1;
        }
        else {
            index = -1;
        }

        /* We match by default (if there are no details other than the element
         * selector, then we must match) */
        *matched = true;

        //** \todo Some details are easier to test than others (e.g. dashmatch 
        // * actually requires looking at data rather than simply comparing 
        // * pointers). Should we consider sorting the detail list such that the 
        // * simpler details come first (and thus the expensive match routines 
        // * can be avoided unless absolutely necessary)? 

        while index != -1 {
            error = self.match_detail(node, detail[index], state, matched, &mut pseudo);
            match error {
                CSS_OK => {}
                _=> {
                    return error;
                }
            }
			
            if !(*matched) {
                return CSS_OK;
            }

            if(detail.len() -1 > index){
                index += 1;
            }
            else {
                index = -1;
            }
        }
        
        /* Return the applicable pseudo element, if required */
        match pseudo_element {
            Some(value) =>{
                *value = pseudo ;
            },
            None => {}
        }
		
        CSS_OK
    }
    
    #[inline]
    pub fn match_nth(a:i32  , b:i32 , count:i32) -> bool {

        //debug!(fmt!("Entering match_nth")) ;
        if (a == 0) {
            return (count == b);
        } 
        else {
            let  delta : i32 = count - b;

            /* (count - b) / a is positive or (count - b) is 0 */
            if (((delta > 0) == (a > 0)) || delta == 0) {
                /* (count - b) / a is integer */
                return (delta % a == 0);
            }

            return false;
        }
    }

    pub fn match_detail(&mut self, node:*c_void, 
            detail:&mut css_selector_detail, state:&mut ~css_select_state, 
            matched:&mut bool, pseudo_element:&mut css_pseudo_element) -> css_error {

        //debug!(fmt!("Entering match_detail")) ;
        let mut is_root = false;
        let mut error = CSS_OK;
        let lwc_name = detail.qname.name.clone();
    
		
		
        match (detail.selector_type) {
            CSS_SELECTOR_ELEMENT => {
                if (detail.negate) {
                    /* Only need to test this inside not(), since
                     * it will have been considered as a named node
                     * otherwise. */
                    error = (state.handler.get_ref().node_has_name)(&mut self.lwc_ref, state.pw, node,
                            detail.qname, matched);
                }
            }
            CSS_SELECTOR_CLASS => {
                error = (state.handler.get_ref().node_has_class)(&mut self.lwc_ref, state.pw, node,
                        lwc_name , matched);
            }       
            CSS_SELECTOR_ID => {
                error = (state.handler.get_ref().node_has_id)(&mut self.lwc_ref, state.pw, node,
                        lwc_name , matched);
            }
            CSS_SELECTOR_PSEUDO_CLASS => {
                error = (state.handler.get_ref().node_is_root)( node, &mut is_root);
                match error {
                    CSS_OK => {},
                    _=> {
                        return error;
                    }
                }
								
				if (is_root == false && 
					   self.lwc_ref.lwc_string_isequal(lwc_name , self.first_child.get_ref().clone() ) ) { 

                    let num_before:&mut i32 =&mut 0;

                    error = (state.handler.get_ref().node_count_siblings)( 
                            &mut self.lwc_ref, node, false, false, num_before);

                    match error {
                        CSS_OK => {
                            if  (*num_before == 0) {
                                *matched = true
                            }
                            else {
                                *matched = false
                            }
                        },
                        _=> {}
                    }
                }
                else if (is_root == false && 
							self.lwc_ref.lwc_string_isequal(lwc_name , self.nth_child.expect("") )
					   ) { 
                    let num_before:&mut i32 =&mut 0;

                    error = (state.handler.get_ref().node_count_siblings)( 
                            &mut self.lwc_ref, node, false, false, num_before);
                
                    match error {
                        CSS_OK => {
                            let a = detail.a;
                            let b = detail.b;

                            *matched = css_select_ctx::match_nth(a, b, *num_before + 1);
                        },
                        _ => {}
                    }
                }
                else if (is_root == false && 
						  self.lwc_ref.lwc_string_isequal(lwc_name , self.nth_last_child.expect("") )
						)  { 
                    let mut num_after:i32 = 0;

                    error = (state.handler.get_ref().node_count_siblings)( 
                            &mut self.lwc_ref, node, false, true, &mut num_after);
                    
                    match error {
                        CSS_OK => {
                            let a = detail.a;
                            let b = detail.b;

                            *matched = css_select_ctx::match_nth(a, b, num_after + 1);
                        },
                        _ => {}
                    }
                }
                else if (is_root == false && 
                        self.lwc_ref.lwc_string_isequal(lwc_name , self.nth_of_type.expect("") ) ) { 
                    let mut num_before:i32 = 0;

                    error = (state.handler.get_ref().node_count_siblings)( 
                            &mut self.lwc_ref, node, true, false, &mut num_before);
                    
                    match error {
                        CSS_OK => {
                            let a = detail.a;
                            let b = detail.b;

                            *matched = css_select_ctx::match_nth(a, b, num_before + 1);
                        },
                        _ => {}
                    }
                } 
                else if (is_root == false && 
                        self.lwc_ref.lwc_string_isequal(lwc_name , self.nth_last_of_type.expect("") ) ) { 
                    let mut num_after:i32 = 0;

                    error = (state.handler.get_ref().node_count_siblings)( 
                            &mut self.lwc_ref, node, true, true, &mut num_after);
                    
                    match error {
                        CSS_OK => {
                            let a = detail.a;
                            let b = detail.b;

                            *matched = css_select_ctx::match_nth(a, b, num_after + 1);
                        },
                        _ => {}
                    }
                } else if (is_root == false &&
                        self.lwc_ref.lwc_string_isequal(lwc_name , self.last_child.expect("") ) ) { 
                    let mut num_after:i32 = 0;

                    error = (state.handler.get_ref().node_count_siblings)(
                            &mut self.lwc_ref, node, false, true, &mut num_after);
                    match error {
                                CSS_OK => {
                                    if  (num_after == 0) {
                                        *matched = true
                                    }
                                    else {
                                        *matched = false
                                    }
                                },
                                _=> {}
                            }
                } 
                else if (is_root == false &&
                        self.lwc_ref.lwc_string_isequal(lwc_name , self.first_of_type.get_ref().clone() ) ) { 
                    let mut num_before:i32 = 0;


                    error = (state.handler.get_ref().node_count_siblings)( 
                            &mut self.lwc_ref, node, true, false, &mut num_before);
                    
                    match error {
                        CSS_OK => {
                            if  (num_before == 0) {
                                *matched = true
                            }
                            else {
                                *matched = false
                            }
                        },
                        _=> {}
                    }
                } 
                else if (is_root == false &&
                        self.lwc_ref.lwc_string_isequal(lwc_name , self.last_of_type.get_ref().clone() ) ) { 
                    let mut num_after:i32 = 0;

                    error = (state.handler.get_ref().node_count_siblings)( 
                            &mut self.lwc_ref, node, true, true, &mut num_after);
                
                    match error {
                        CSS_OK => {
                            if  (num_after == 0) {
                                *matched = true
                            }
                            else {
                                *matched = false
                            }
                        },
                        _=> {}
                    }
                }
                else if (is_root == false && 
                        self.lwc_ref.lwc_string_isequal(lwc_name , self.only_child.expect("") ) ) { 
                    
                    let mut num_before = 0;
                    let mut num_after = 0;

                    error = (state.handler.get_ref().node_count_siblings)( 
                            &mut self.lwc_ref, node, false, false, &mut num_before);
                    
                    match error {
                        CSS_OK => {
                            error = (state.handler.get_ref().node_count_siblings)(
                                 &mut self.lwc_ref, node, false, true, &mut num_after);
                                    
                            match error {
                                CSS_OK => {
                                    if  (num_before == 0) && 
                                            (num_after == 0) {
                                        *matched = true
                                    }
                                    else {
                                        *matched = false
                                    }
                                },
                                _=> {}
                            }       
                        }
                        _ => {}
                    }
                } 
                else if (is_root == false && 
                        self.lwc_ref.lwc_string_isequal(lwc_name , self.only_of_type.expect("") ) ) { 
                
                    let mut num_before = 0;
                    let mut num_after = 0;

                    error = (state.handler.get_ref().node_count_siblings)( 
                            &mut self.lwc_ref, node, true, false, &mut num_before);
                
                    if (match error { CSS_OK => true, _  => false}) {
                        error = (state.handler.get_ref().node_count_siblings)(
                                    &mut self.lwc_ref, node, true, true, &mut num_after);
                
                        match error {
                            CSS_OK => {
                                if  (num_before == 0) && 
                                            (num_after == 0) {
                                        *matched = true
                                }
                                else {
                                    *matched = false
                                }
                            },
                            _=> {}
                        }       
                    }
                } 
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.root.expect("") ) ) { 
                    *matched = is_root;
                } 
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.empty.expect("") ) ) {
                    error = (state.handler.get_ref().node_is_empty)(
                            node, matched);
                } 
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.link.expect("") ) ) { 
                    error = (state.handler.get_ref().node_is_link)(
                            node, matched);
                }
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.visited.expect("") ) ) { 
                    error = (state.handler.get_ref().node_is_visited)(
                            node, matched);
                }
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.hover.expect("") ) ) { 
                    error = (state.handler.get_ref().node_is_hover)(
                            node, matched);
                }
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.active.expect("") ) ) { 
                    error = (state.handler.get_ref().node_is_active)(
                            node, matched);
                } 
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.focus.expect("") ) ) { 
                    error = (state.handler.get_ref().node_is_focus)(
                            node, matched);
                } 
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.target.expect("") ) ) { 
                    error = (state.handler.get_ref().node_is_target)(
                            node, matched);
                }
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.lang.expect("") ) ) { 
                    error = (state.handler.get_ref().node_is_lang)(
                            node, (detail.string).get_ref().clone(), matched);
                }
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.enabled.expect("") ) ) { 
                    error = (state.handler.get_ref().node_is_enabled)(
                            node, matched);
                }
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.disabled.expect("") ) ) { 
                    error = (state.handler.get_ref().node_is_disabled)(
                            node, matched);
                }
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name , self.checked.expect("") ) ) { 
                    error = (state.handler.get_ref().node_is_checked)(
                            node, matched);
                }
                else {
                    *matched = false;
                }
            }
            CSS_SELECTOR_PSEUDO_ELEMENT => {
                *matched = true;
                if ( self.lwc_ref.lwc_string_isequal(lwc_name,  self.first_line.expect("") ) ) { 
                    *pseudo_element = CSS_PSEUDO_ELEMENT_FIRST_LINE;
                } 
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name, self.first_letter.expect("") ) ){ 
                    *pseudo_element = CSS_PSEUDO_ELEMENT_FIRST_LETTER;
                } 
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name,  self.before.expect("") ) ) { 
                    *pseudo_element = CSS_PSEUDO_ELEMENT_BEFORE;
                } 
                else if ( self.lwc_ref.lwc_string_isequal(lwc_name, self.after.expect("") ) ) { 
                    *pseudo_element = CSS_PSEUDO_ELEMENT_AFTER;
                } 
                else {
                    *matched = false;
                }
            }
            CSS_SELECTOR_ATTRIBUTE => {
                error = (state.handler.get_ref().node_has_attribute)(&mut self.lwc_ref, node,
                        detail.qname, matched);
            }
            CSS_SELECTOR_ATTRIBUTE_EQUAL => {
                error = (state.handler.get_ref().node_has_attribute_equal)( 
                        &mut self.lwc_ref, node, detail.qname, detail.string.get_ref().clone(), 
                        matched);
            }
            CSS_SELECTOR_ATTRIBUTE_DASHMATCH => {
                error = (state.handler.get_ref().node_has_attribute_dashmatch)(
                        &mut self.lwc_ref, node, detail.qname, detail.string.get_ref().clone(),
                        matched);
            }
            CSS_SELECTOR_ATTRIBUTE_INCLUDES => {
                error = (state.handler.get_ref().node_has_attribute_includes)( 
                        &mut self.lwc_ref, node, detail.qname, detail.string.get_ref().clone(),
                        matched);
            }
            CSS_SELECTOR_ATTRIBUTE_PREFIX => {
                error = (state.handler.get_ref().node_has_attribute_prefix)(
                        &mut self.lwc_ref, node, detail.qname, detail.string.get_ref().clone(),
                        matched);
            }
            CSS_SELECTOR_ATTRIBUTE_SUFFIX => {
                error = (state.handler.get_ref().node_has_attribute_suffix)(
                        &mut self.lwc_ref, node, detail.qname,detail.string.get_ref().clone(),
                        matched);
            }
            CSS_SELECTOR_ATTRIBUTE_SUBSTRING => {
                error = (state.handler.get_ref().node_has_attribute_substring)(
                        &mut self.lwc_ref, node, detail.qname,detail.string.get_ref().clone(),
                        matched);
            }
        }

        /* Invert match, if the detail requests it */
        if ( (match error { CSS_OK => true, _ => false} )&& detail.negate){
            *matched = !*matched;
        }
        return error
    }

    pub fn cascade_style(stylesheet_vector:&mut ~[css_stylesheet], style:&mut ~css_style, state:&mut ~css_select_state) -> css_error {
        let s = style;

        //debug!(fmt!("Entering cascade_style")) ;
		//debug!(fmt!("s_used=%?, s_len=%?", s.used, s.bytecode.len())) ;
		
        let bytecode_len = s.bytecode.len();
        while s.used < bytecode_len {
            let mut op: u32;
            let mut error : css_error ;
            let opv = peek_bytecode(s);

            advance_bytecode(s);

            op = getOpcode(opv) as u32;
            //debug!(fmt!("op=%?, opv=%?, op_m=%?", op, opv, op as uint));
            error =  (prop_dispatch[op as uint].cascade)(stylesheet_vector, opv, s, state);

            match error {
                CSS_OK => {},
                x => {
                    return x ;
                }
            }
        }

        CSS_OK
    }

}


//////////////////////////////////////////////////////////////////
