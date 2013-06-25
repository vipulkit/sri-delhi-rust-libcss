
use std::arc;
use wapcaplet::*;

use include::properties::*;
use include::types::*;
use include::font_face::*;
use bytecode::bytecode::*;
use utils::errors::*;
use select::common::*;
use select::dispatch::*;
use stylesheet::*;

use core::managed::*;

static IMPORT_STACK_SIZE : int = 256 ;

/*
 * Container for stylesheet selection info
 */
pub struct css_select_sheet {
    sheet:@mut css_stylesheet,
    origin:css_origin,
    media:u64
}

/*
 * CSS selection context
 */
struct css_select_ctx {
    sheets:~[@mut css_select_sheet],
    lwc_instance:arc::RWARC<~lwc>,
    /* Useful interned strings */
    universal:Option<arc::RWARC<~lwc_string>>,
    first_child:Option<arc::RWARC<~lwc_string>>,
    link:Option<arc::RWARC<~lwc_string>>,
    visited:Option<arc::RWARC<~lwc_string>>,
    hover:Option<arc::RWARC<~lwc_string>>,
    active:Option<arc::RWARC<~lwc_string>>,
    focus:Option<arc::RWARC<~lwc_string>>,
    nth_child:Option<arc::RWARC<~lwc_string>>,
    nth_last_child:Option<arc::RWARC<~lwc_string>>,
    nth_of_type:Option<arc::RWARC<~lwc_string>>,
    nth_last_of_type:Option<arc::RWARC<~lwc_string>>,
    last_child:Option<arc::RWARC<~lwc_string>>,
    first_of_type:Option<arc::RWARC<~lwc_string>>,
    last_of_type:Option<arc::RWARC<~lwc_string>>,
    only_child:Option<arc::RWARC<~lwc_string>>,
    only_of_type:Option<arc::RWARC<~lwc_string>>,
    root:Option<arc::RWARC<~lwc_string>>,
    empty:Option<arc::RWARC<~lwc_string>>,
    target:Option<arc::RWARC<~lwc_string>>,
    lang:Option<arc::RWARC<~lwc_string>>,
    enabled:Option<arc::RWARC<~lwc_string>>,
    disabled:Option<arc::RWARC<~lwc_string>>,
    checked:Option<arc::RWARC<~lwc_string>>,
    first_line:Option<arc::RWARC<~lwc_string>>,
    first_letter:Option<arc::RWARC<~lwc_string>>,
    before:Option<arc::RWARC<~lwc_string>>,
    after:Option<arc::RWARC<~lwc_string>>
}

/*
 * Container for selected font faces
 */
pub struct css_select_font_faces_list {
    font_faces:~[@mut css_font_face]
}

/*
 * Font face selection state
 */
pub struct css_select_font_faces_state {
    font_family:Option<arc::RWARC<~lwc_string>>,
    media:u64,

    ua_font_faces:css_select_font_faces_list,
    user_font_faces:css_select_font_faces_list,
    author_font_faces:css_select_font_faces_list
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
    pub fn css_select_ctx_create() -> ~css_select_ctx {
        
        let mut result = ~css_select_ctx {
            sheets:~[],
            lwc_instance:lwc(), // create lwc

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
    *  'sheet'  - Stylesheet. 
    *  'origin' - Origin of the sheet.
    *  'media' - Vector of tokens to process.
    * #Return Value:
	* 'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css_select_ctx_append_sheet(&mut self,
                                    sheet:@mut css_stylesheet,
                                    origin:css_origin,
                                    media:u64) 
                                    -> css_error {
        //let n_sheets = self.sheets.len();
        io::println(fmt!("Entering css_select_ctx_append_sheet")) ;
        self.css_select_ctx_insert_sheet(sheet,origin,media)
    }

    /**
    * Insert a stylesheet into a selection context
	* #Arguments:
	*  'self' - The selection context to insert to.
    *  'sheet'  - Stylesheet. 
    *  'index' - Index in context to insert sheet.
    *  'origin' - Origin of the sheet.
    *  'media' - Vector of tokens to process.
    * #Return Value:
	* 'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css_select_ctx_insert_sheet(&mut self,
                                    csheet:@mut css_stylesheet,
                                    //index:uint,
                                    corigin:css_origin,
                                    cmedia:u64) 
                                    -> css_error {

        io::println(fmt!("Entering css_select_ctx_insert_sheet")) ;
        /* Inline styles cannot be inserted into a selection context */
        if (csheet.inline_style) {
            return CSS_INVALID ;
        }
    
        /* Index must be in the range [0, n_sheets]
         * The latter being equivalent to append */
        // if index > self.sheets.len()    {
        //     return CSS_INVALID;
        // }   
            
        let mut select_sheet = @mut css_select_sheet{
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
    pub fn css_select_ctx_remove_sheet(&mut self, csheet:@mut css_stylesheet)-> css_error {

        io::println(fmt!("Entering css_select_ctx_remove_sheet")) ;
        let mut i = self.sheets.len() ;
        while (i>0) {
            i = i - 1 ;
            if ( mut_ptr_eq(self.sheets[i].sheet,csheet) ) {
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
        io::println(fmt!("Entering css_select_ctx_count_sheets")) ;
        self.sheets.len()
    }

    /**
    * Retrieve a sheet from a selection context
	* #Arguments:
	*  'self' - The selection context to look in.
	*  'index' - Index in context to look.
    * #Return Value:
	* '(css_error,Option<@mut css_stylesheet>)' - (CSS_OK,Some(sheet)) on success, appropriate (error,None) otherwise.
    */
    pub fn css_select_ctx_get_sheet(&mut self, index:uint) 
                                -> (css_error,Option<@mut css_stylesheet>) {

        io::println(fmt!("Entering css_select_ctx_get_sheet")) ;
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
	* '(css_error,Option<css_select_results>' - (CSS_OK, results) on success, (appropriate error, None) otherwise.
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
                                node:*libc::c_void,
                                media:u64,
                                inline_style:Option<@mut css_stylesheet>,
                                handler:@mut css_select_handler,
								pw:*libc::c_void) 
                                -> (css_error,Option<css_select_results>) {

        io::println(fmt!("Entering css_select_style")) ;
        if( node == ptr::null() || handler.handler_version != (CSS_SELECT_HANDLER_VERSION_1  as uint) ) {
            return (CSS_BADPARM,None) ;
        }
        let mut i : int  ;
        let mut j :int;
        let mut error : css_error ;
        //let mut results : Option<css_select_results>  ;
        let mut parent : *libc::c_void = ptr::null() ;

        let mut state: @mut css_select_state = @mut css_select_state {
            node:node,
            media:media,       
            results:css_select_results{ 
                styles:~[] 
            },    
            current_pseudo:CSS_PSEUDO_ELEMENT_NONE,  
            computed:css_computed_style_create(),   
            handler:Some(handler), 
            pw:pw,   
            sheet:None,   
            current_origin:CSS_ORIGIN_UA,  
            current_specificity:0,   
            element:css_qname{ 
                name:~"" , 
                ns:~"" 
            },
            id:~"",
            classes:~[],
            n_classes:0,             
            reject_cache: ~[],       
            next_reject:128-1,             
            props: ~[] 
        };
        for uint::range(0,128) |_| {
            state.reject_cache.push(None);
        }

        for uint::range(0,(CSS_N_PROPERTIES as uint)) |_| {
            let mut prop_vec : ~[@mut prop_state] = ~[] ;
            for uint::range(0,(CSS_PSEUDO_ELEMENT_COUNT as uint)) |_| {
                let mut pstate = @mut prop_state{
                    specificity:0,
                    set:false,
                    origin:0,
                    important:false,
                    inherit:false    
                };
                prop_vec.push(pstate);
            }
            state.props.push(prop_vec);
        }

        i = CSS_PSEUDO_ELEMENT_COUNT as int ;
        while (i>0) {
            state.results.styles.push(None) ;
            i -= 1;
        }

        /* Base element style is guaranteed to exist */
        state.results.styles[0] = (Some(css_computed_style_create()));

        error = (*(handler.parent_node))(node, &mut parent);
        match error {
            CSS_OK=>{},
            x =>  {
                return (x,None) ;
            }
        }

        /* Get node's name */
        error = (*(handler.node_name))(node, &mut state.element);
        match error {
            CSS_OK=>{},
            x =>  {
                return (x,None) ;
            }
        }

        /* Get node's ID, if any */
        error = (*(handler.node_id))(pw, node, &mut state.id);
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
        error = (*(handler.node_classes))(pw, node, 
                &mut state.classes);
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
        while(i < (self.sheets.len() as int) ) {
            let mut s = self.sheets[i] ;
            if( s.media & media ) != 0 && 
                s.sheet.disabled == false {
                    io::println(fmt!("css_select_style : selecting from sheet ")) ;
                    error = self.select_from_sheet(s.sheet, 
                              s.origin, state);  
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
            io::println(fmt!("css_select_style : considerng inline style")) ;
            let mut  sel = 
                        inline_style.get().rule_list;

            /* Sanity check style */
            if (inline_style.get().rule_count != 1 ){
                 return (CSS_INVALID,None) ;
            }
            
            match sel {
                None=>{
                    return (CSS_INVALID,None) ;
                },
                Some(r) => {
                    match r {
                        RULE_SELECTOR(r_sel)=>{
                            // Complete 

                            /* No bytecode if input was empty or wholly invalid */
                            if(r_sel.style.is_some()){
                                /* Inline style applies to base element only */
                                state.current_pseudo = CSS_PSEUDO_ELEMENT_NONE;
                                state.computed = state.results.styles[
                                        CSS_PSEUDO_ELEMENT_NONE as uint].get();

                                error = css_select_ctx::cascade_style(r_sel.style.get(), 
                                                        state);
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
        state.computed = state.results.styles[CSS_PSEUDO_ELEMENT_NONE as uint].get();
        i = 0 ;
        while (i<(CSS_N_PROPERTIES as int)) {
            io::println(fmt!("css_select_style : setting initial hint of property =%?=",i)) ;
            let mut prop2 = copy state.props[i] ;
            let mut prop = prop2[CSS_PSEUDO_ELEMENT_NONE as uint];

            /* Apply presentational hints if the property is unset or 
             * the existing property value did not come from an author 
             * stylesheet or a user sheet using !important. */
            if (prop.set == false ||
                    (prop.origin != (CSS_ORIGIN_AUTHOR as u8) &&
                    prop.important == false)) {
                error = css_select_ctx::set_hint(state, i as u32);
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
            if (prop.set == false || 
                    (parent == ptr::null() && 
                    prop.inherit == true)) {
                error = css_select_ctx::set_initial(state, i as uint, 
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
            io::println(fmt!("css_select_style : pseudo element of property =%?=",j)) ;
            state.current_pseudo = unsafe { cast::transmute(j)};
			let computed_opt = state.results.styles[j];
			
			match computed_opt {
				Some(T) => {
					state.computed = T;
				}
				None => {
					j += 1; 
					loop;
				}
			}

            /* Skip non-existent pseudo elements */
            // if (state.computed == NULL)
            //     continue;
            i = 0 ;
            while (i < (CSS_N_PROPERTIES as int) ) {
                let mut prop = state.props[i][j];
                
                io::println(fmt!("css_select_style : property =%?=%?="j,i)) ;
                /* If the property is still unset then set it 
                 * to its initial value. */
                if (prop.set == false) {
                    error = css_select_ctx::set_initial(state, i as uint, unsafe { cast::transmute(j)}, parent);
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
        if (parent == ptr::null()) {
            /* Only compute absolute values for the base element */
            error = css__compute_absolute_values(None,
                    state.results.styles[CSS_PSEUDO_ELEMENT_NONE as uint].get(),
                    handler.compute_font_size);
            match error {
                CSS_OK=>{},
                x =>  {
                    return (x,None) ;
                }
            }
        }

        (CSS_OK,Some(copy state.results))
    }


    /**
    * Destroy a selection result set
	* #Arguments:
	*  'results' - Result set to destroy.
    */
    pub fn css_select_results_destroy(results: &mut ~[@mut css_select_results] ) {
        io::println(fmt!("Entering css_select_results_destroy")) ;
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
                                media:u64,
                                font_family:arc::RWARC<~lwc_string>) 
                                -> (css_error,Option<@mut css_select_font_faces_results>) {

        io::println(fmt!("Entering css_select_font_faces")) ;
        if( lwc_string_length(font_family.clone()) == 0 ) {
            return (CSS_BADPARM,None) ;
        }

        let state = @mut css_select_font_faces_state {
            font_family:Some(font_family.clone()),
            media:media,

            ua_font_faces:css_select_font_faces_list{font_faces:~[]},
            user_font_faces:css_select_font_faces_list{font_faces:~[]},
            author_font_faces:css_select_font_faces_list{font_faces:~[]}
        };

        /* Iterate through the top-level stylesheets, selecting font-faces
         * from those which apply to our current media requirements and
         * are not disabled */
        let mut i = self.sheets.len() ;
        while (i>0) { 
            i -= 1 ;
            let mut select_sheet = self.sheets[i] ;
            if ((select_sheet.media & media) != 0 ) && 
                (select_sheet.sheet.disabled == false ) {

                let error = self.select_font_faces_from_sheet(select_sheet.sheet,
                                                        select_sheet.origin,state);
                match error {
                    CSS_OK=>{} ,
                    x => {
                        return (x,None) ;
                    }
                }
            }
        }
          
        let results = @mut css_select_font_faces_results{
                    font_faces:~[]
            };

        unsafe{    
            let n_font_faces = state.ua_font_faces.font_faces.len() + 
                state.user_font_faces.font_faces.len() +
                state.author_font_faces.font_faces.len();

            if (n_font_faces > 0) {
                /* We found some matching faces.  Make a results structure with
                 * the font faces in priority order. */
                results.font_faces.push_all(state.ua_font_faces.font_faces);
                results.font_faces.push_all(state.user_font_faces.font_faces);
                results.font_faces.push_all(state.author_font_faces.font_faces);    
            }
        }    

        (CSS_OK,Some(results))
    }


    /******************************************************************************
     * Selection engine internals below here                                      *
     ******************************************************************************/
    pub fn intern_strings(&mut self) {
        
        do self.lwc_instance.clone().write |l| {

            /* Universal selector */
            self.universal = Some(l.lwc_intern_string(~"*"));

            /* Pseudo classes */
            self.first_child = Some(l.lwc_intern_string(~"first_child"));
            self.link = Some(l.lwc_intern_string(~"link"));
            self.visited = Some(l.lwc_intern_string(~"visited"));
            self.hover = Some(l.lwc_intern_string(~"hover"));
            self.active = Some(l.lwc_intern_string(~"active"));
            self.focus = Some(l.lwc_intern_string(~"focus"));
            self.nth_child = Some(l.lwc_intern_string(~"nth_child"));
            self.nth_last_child = Some(l.lwc_intern_string(~"nth_last_child"));
            self.nth_of_type = Some(l.lwc_intern_string(~"nth_of_type"));
            self.nth_last_of_type = Some(l.lwc_intern_string(~"nth_last_of_type"));
            self.last_child = Some(l.lwc_intern_string(~"last_child"));
            self.first_of_type = Some(l.lwc_intern_string(~"first_of_type"));
            self.last_of_type = Some(l.lwc_intern_string(~"last_of_type"));
            self.only_child = Some(l.lwc_intern_string(~"only_child"));
            self.only_of_type = Some(l.lwc_intern_string(~"only_of_type"));
            self.root = Some(l.lwc_intern_string(~"root"));
            self.empty = Some(l.lwc_intern_string(~"empty"));
            self.target = Some(l.lwc_intern_string(~"target"));
            self.lang = Some(l.lwc_intern_string(~"lang"));
            self.enabled = Some(l.lwc_intern_string(~"enabled"));
            self.disabled = Some(l.lwc_intern_string(~"disabled"));
            self.checked = Some(l.lwc_intern_string(~"checked"));

            /* Pseudo elements */
            self.first_line = Some(l.lwc_intern_string(~"first_line"));
            self.first_letter = Some(l.lwc_intern_string(~"first_letter"));
            self.before = Some(l.lwc_intern_string(~"before"));
            self.after = Some(l.lwc_intern_string(~"after"));
        }   
    }

    pub fn set_hint(state:@mut css_select_state, prop:u32) -> css_error {
        
        io::println(fmt!("Entering set_hint")) ;
        /* Retrieve this property's hint from the client */
        let (error,hint_option) = (*state.handler.get().node_presentational_hint)(state.node, prop);
        match error {
            CSS_OK => {},
            CSS_PROPERTY_NOT_SET => return CSS_OK, 
            x => return x
        } 

        /* Hint defined -- set it in the result */
        let mut dispatch_hint = dispatch_table::get_set_from_hint_ptr(prop as uint) ;
        let hint = hint_option.unwrap();
        let error =  dispatch_hint(hint, state.computed);

        match error {
            CSS_OK => {},
            x => {
                return x ;
            }
        }
        
        /* Keep selection state in sync with reality */
        state.props[prop][CSS_PSEUDO_ELEMENT_NONE as uint].set = true;
        state.props[prop][CSS_PSEUDO_ELEMENT_NONE as uint].specificity = 0;
        state.props[prop][CSS_PSEUDO_ELEMENT_NONE as uint].origin = CSS_ORIGIN_AUTHOR as u8;
        state.props[prop][CSS_PSEUDO_ELEMENT_NONE as uint].important = false;
        state.props[prop][CSS_PSEUDO_ELEMENT_NONE as uint].inherit = (hint.status == 0);

        return CSS_OK;
    }

    pub fn set_initial(state : @mut css_select_state, prop : uint, pseudo : css_pseudo_element,
        parent: *libc::c_void) -> css_error {

        io::println(fmt!("Entering set_initial")) ;
        let mut error : css_error; 

        /* Do nothing if this property is inherited (the default state 
         * of a clean computed style is for everything to be set to inherit)
         *
         * If the node is tree root and we're dealing with the base element, 
         * everything should be defaulted.
         */

        if dispatch_table::get_inherited(prop) == 0 || match pseudo { CSS_PSEUDO_ELEMENT_NONE => true, _ => false} &&
            parent == ptr::null() {
            
            let mut group : prop_group = dispatch_table::get_group(prop);

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
                    error = (dispatch_table::get_initial_ptr(prop))(state);
                    match error {
                        CSS_OK => {},
                        _=> {
                            return error;
                        }
                    }
                }

                GROUP_UNCOMMON => {
                    match state.computed.uncommon {
                        None => {},
                        Some(_) => {
                            error = (dispatch_table::get_initial_ptr(prop))(state);
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
                    match state.computed.page {
                        None => {},
                        Some(_) => {
                            error = (dispatch_table::get_initial_ptr(prop))(state);
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
                    match state.computed.aural {
                        None => {},
                        Some(_) => {
                            error = (dispatch_table::get_initial_ptr(prop))(state);
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

    pub fn select_from_sheet(&mut self, sheet : @mut css_stylesheet, origin : css_origin, state : @mut css_select_state) -> css_error{

        io::println(fmt!("Entering select_from_sheet")) ;
        let mut s:Option<@mut css_stylesheet> = Some(sheet);
        let mut rule : Option<CSS_RULE_DATA_TYPE> = s.get().rule_list;
        let mut sp : u32 = 0;
        let mut import_stack : ~[CSS_RULE_DATA_TYPE] = ~[];

        loop{
            /* Find first non-charset rule, if we're at the list head */
            if compare_css_rdt(rule, s.get().rule_list){
                while rule.is_some() && compare_css_rule_types(rule, CSS_RULE_CHARSET) {
                    rule = get_css_rule_next(rule.get());
                }
            }
            if rule.is_some() && compare_css_rule_types(rule, CSS_RULE_IMPORT) {
                /* Current rule is an import */
		    let mut import_sheet : Option<@mut css_stylesheet> = None;
                let mut import_media:u64 = 0;
                match rule.get() {
                    RULE_IMPORT(x) => {
                        import_media = x.media;
                        import_sheet = x.sheet;
                    },
                    _=> {},
                }

                if import_sheet.is_some() && ((import_media & state.media) != 0) {
                    /* It's applicable, so process it */

                    import_stack.push(rule.get());

                    s = import_sheet;
                    rule = s.get().rule_list;
                }
                else {
                    /* Not applicable; skip over it */
                    rule = get_css_rule_next(rule.get());
                }
            }
            else {
                /* Gone past import rules in this sheet */
                let mut error : css_error;

                /* Process this sheet */
                state.sheet = s;
                state.current_origin = origin;

                error = self.match_selectors_in_sheet(s.get(), state);
                match error {
                    CSS_OK => {
                        if sp > 0 {
                            sp -= 1;
                            rule = get_css_rule_next(import_stack[sp]);
                            s = get_stylesheet_parent(import_stack[sp]);
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

    pub fn _rule_applies_to_media(rule: Option<CSS_RULE_DATA_TYPE>, media:u64) -> bool {

        io::println(fmt!("Entering _rule_applies_to_media")) ;
        let mut applies : bool = true;
        let mut ancestor = rule;

        loop {  
            match ancestor {
                None=>{
                    break ;
                },
                Some(ancestor_rule)=> {
                    match ancestor_rule {
                        RULE_MEDIA(r)=>{
                            if( ( r.media & media ) == 0 ) {
                                applies = false ;
                                return applies ;
                            }

                            if r.base.parent_stylesheet.is_none() {
                                ancestor = r.base.parent_rule ;
                            }
                            else {
                                ancestor = None ;
                            }
                            loop ;
                        },
                        _ => {
                            let mut ancestor_base = css_stylesheet::css__stylesheet_get_base_rule(ancestor_rule);
                            if ancestor_base.parent_stylesheet.is_none() {
                                ancestor = ancestor_base.parent_rule ;
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

    pub fn _select_font_face_from_rule(&mut self,
                                    rule:@mut css_rule_font_face,
                                    origin: css_origin,
                                    state:@mut css_select_font_faces_state) 
                                    -> css_error {

        io::println(fmt!("Entering _select_font_face_from_rule")) ;                                
        if ( css_select_ctx::_rule_applies_to_media(Some(RULE_FONT_FACE(rule)), state.media) ) {

            if ( rule.font_face.is_none() || 
                rule.font_face.get().font_family.is_none() || 
                state.font_family.is_none() ) {
                return CSS_BADPARM ;
            }

            let mut res : bool = false ;
            do self.lwc_instance.read |lwc_ins| {
                res = lwc_ins.lwc_string_isequal(rule.font_face.get().font_family.swap_unwrap(),
                                                    state.font_family.swap_unwrap() ) ;
            }
            if ( res ) {
                let mut faces = @mut css_select_font_faces_list{
                    font_faces:~[]
                };
                unsafe {
                    match (origin) {
                        CSS_ORIGIN_UA => {
                            state.ua_font_faces.font_faces.push(rule.font_face.get());
                            faces.font_faces.push_all(state.ua_font_faces.font_faces);
                        },
                        CSS_ORIGIN_USER => {
                            state.user_font_faces.font_faces.push(rule.font_face.get());
                            faces.font_faces.push_all(state.user_font_faces.font_faces);
                        },
                        CSS_ORIGIN_AUTHOR => {
                            state.author_font_faces.font_faces.push(rule.font_face.get());
                            faces.font_faces.push_all(state.author_font_faces.font_faces);
                        }
                    }
                }
            }
        }
        CSS_OK
    }

    pub fn select_font_faces_from_sheet(&mut self,
                                        sheet:@mut css_stylesheet,
                                        origin: css_origin,
                                        state:@mut css_select_font_faces_state)
                                        -> css_error {

        io::println(fmt!("Entering select_font_faces_from_sheet")) ;
        let mut s = Some(sheet) ;
        let mut rule = s.get().rule_list;
        let mut sp : u32 = 0 ;
        let mut import_stack : ~[CSS_RULE_DATA_TYPE] = ~[];
        vec::reserve_at_least(&mut import_stack,IMPORT_STACK_SIZE as uint) ;

        let mut ptr = rule ;
        while ( s.is_some() ) {
            loop {
                match ptr {
                    None=> { 
                        break ;
                    },
                    Some(current_rule) => {
                        match current_rule {
                            RULE_CHARSET(_) =>{
                                ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
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
                        ptr = css_stylesheet::css__stylesheet_get_base_rule(import_stack[sp]).next;
                        s = css_stylesheet::css__stylesheet_get_base_rule(import_stack[sp]).parent_stylesheet;
                    } 
                    else {
                        s = None;
                    }
                },
                Some(current_rule) => {
                    match current_rule {
                        RULE_CHARSET(_) =>{
                            ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
                        },
                        RULE_IMPORT(x) => {
                            /* Current rule is an import */
                            if ( x.sheet.is_some() && 
                                ((x.media & state.media) != 0) ) {
                                if ( sp >= IMPORT_STACK_SIZE as u32) {
                                    return CSS_NOMEM ;
                                }
                                import_stack[sp] = current_rule ;
                                sp += 1;
                                s = x.sheet ;
                                rule = s.get().rule_list ;
                                ptr = rule ;
                            }
                            else {
                                ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
                            }
                        },
                        RULE_FONT_FACE(x) => {
                            let mut error : css_error = self._select_font_face_from_rule(
                                                            x,
                                                            origin,
                                                            state);
                            match error {
                                CSS_OK=>{},
                                x => { 
                                    return x ;
                                }
                            }

                            ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
                        },
                        _=> {
                            ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
                        }
                    }
                }
            }
        }

        CSS_OK
    }

    pub fn _selectors_pending(node: Option<@mut css_selector>, id: Option<@mut css_selector>,
                classes: &~[Option<@mut css_selector>], 
                univ: Option<@mut css_selector>) -> bool {

        io::println(fmt!("Entering _selectors_pending")) ;
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

	for classes.each |&ele| {        
            match ele {
                None => {}
                Some(_) => {
                    pending = true;
                }
            }
        }

        pending
    }

    pub fn _selector_less_specific(refer:Option<@mut css_selector>, 
                                cand:Option<@mut css_selector>) 
                                -> bool {

        io::println(fmt!("Entering _selector_less_specific")) ;
        let mut result : bool;

        if (cand.is_none()) {
            return false;
        }

        if (refer.is_none()) {
            return true;
        }

        /* Sort by specificity */
        if (cand.get().specificity < refer.get().specificity) {
            result = true;
        } 
        else if (refer.get().specificity < cand.get().specificity) {
            result = false;
        } 
        else {

            if( cand.get().rule.is_none() || refer.get().rule.is_none() ) {
                fail!(~"_selector_less_specific:Base rule cannot be null");
            }
            let mut cand_base = css_stylesheet::css__stylesheet_get_base_rule(cand.get().rule.get()) ;
            let mut refer_base = css_stylesheet::css__stylesheet_get_base_rule(refer.get().rule.get()) ;
            /* Then by rule index -- earliest wins */
            if (cand_base.index < refer_base.index) {
                result = true;
            }
            else {
                result = false;
            }
        }

        result
    }

    pub fn _selector_next(node: Option<@mut css_selector>, 
                            id: Option<@mut css_selector>,
                            classes: &~[Option<@mut css_selector>], 
                            univ: Option<@mut css_selector>) 
                            -> Option<@mut css_selector> {

        io::println(fmt!("Entering _selector_next")) ;
        let mut ret : Option<@mut css_selector> = None;

        if (css_select_ctx::_selector_less_specific(ret, node)) {
            ret = Some(node.get());
        }

        if (css_select_ctx::_selector_less_specific(ret, id)) {
            ret = Some(id.get());
        }

        if (css_select_ctx::_selector_less_specific(ret, univ)) {
            ret = Some(univ.get());
        }

        let mut i : uint = 0;
        while i < classes.len() {
            if (css_select_ctx::_selector_less_specific(classes[i], ret)){
                ret = Some(classes[i].get());
            }
            i += 1;
        }
        ret
    }

    pub fn _rule_good_for_element_name(selector:@mut css_selector,
        src:@mut css_select_rule_source, state:@mut css_select_state) -> bool {
        /* If source of rule is element or universal hash, we know the
         * element name is a match.  If it comes from the class or id hash,
         * we have to test for a match */
        io::println(fmt!("Entering _rule_good_for_element_name")) ;
        if (match src.source { 
            CSS_SELECT_RULE_SRC_ID | CSS_SELECT_RULE_SRC_CLASS => true,
            _ => false }) {
            
            if (unsafe {selector.data[0].qname.name.len()} != 1 ||
                   selector.data[0].qname.name[0] != '*' as u8) {
                
                if selector.data[0].qname.name != state.element.name {
                    return false;
                }
            }
        }    
        return true;
    }        

    pub fn match_selectors_in_sheet(&mut self, sheet : @mut css_stylesheet, 
                                    state : @mut css_select_state) -> css_error {
    
        io::println(fmt!("Entering match_selectors_in_sheet")) ;
        let mut node_selectors_hash_entry : Option<@mut hash_entry> = None ;
        let mut node_selectors_option : Option<@mut css_selector> = None ;
        let mut id_selectors_hash_entry : Option<@mut hash_entry> = None ;
        let mut id_selectors_option : Option<@mut css_selector> = None ;
        let mut class_selectors_hash_entry : ~[Option<@mut hash_entry>] = ~[];
        let mut class_selectors_option_list : ~[Option<@mut css_selector>] = ~[] ;
        let mut univ_selectors_hash_entry : Option<@mut hash_entry> = None ;
        let mut univ_selectors_option : Option<@mut css_selector> = None ;
        //let mut error : css_error ;

        /* Find hash chain that applies to current node */
        let mut (sel,error) = sheet.selectors.css__selector_hash_find(copy state.element.name);
        match error {
            CSS_OK => {},
            err => {
                return err;
            }
        }
        if sel.is_some() {
            node_selectors_hash_entry = sel;
            node_selectors_option = Some(sel.get().selector) ;
        }

        if ( unsafe {state.classes.len() != 0} ) {
             /* Find hash chains for node classes */

            for state.classes.each_mut |&sclass| {
                let mut (sel_class,error) = sheet.selectors.css__selector_hash_find_by_class(copy sclass);
                match error {
                    CSS_OK => {},
                    err => {
                        return err;
                    }
                }
                if sel_class.is_some() {
                    class_selectors_hash_entry.push(sel_class) ;
                    class_selectors_option_list.push(Some(sel_class.get().selector)) ;
                }
            }
        }

        if ( unsafe { state.id.len() != 0 } ) {
            /* Find hash chain for node ID */
            let mut (sel_id,error) = sheet.selectors.css__selector_hash_find_by_id(copy state.id);
            match error {
                CSS_OK => {},
                err => {
                    return err;
                }
            }
            if sel_id.is_some() {
                id_selectors_hash_entry = sel_id ;
                id_selectors_option = Some(sel_id.get().selector) ;
            }
        }

        /* Find hash chain for universal selector */
        let mut (sel_univ,error) = sheet.selectors.css__selector_hash_find_universal();
        match error {
            CSS_OK => {},
            err => {
                return err;
            }
        }
        if sel_univ.is_some() {
            univ_selectors_hash_entry = sel_univ ;
            univ_selectors_option = Some(sel_univ.get().selector) ;
        }

        // /* Process matching selectors, if any */
        while ( css_select_ctx::_selectors_pending(node_selectors_option, 
                                                    id_selectors_option, 
                                                    &class_selectors_option_list,
                                                    univ_selectors_option) ) {
            let mut selector : @mut css_selector ;

            /*Selectors must be matched in ascending order of specificity
             * and rule index. (c.f. css__outranks_existing())
             *
             * Pick the least specific/earliest occurring selector.
             */
            let mut o_selector = css_select_ctx::_selector_next(
                                    node_selectors_option, 
                                    id_selectors_option,
                                    &class_selectors_option_list, 
                                    univ_selectors_option );

            if o_selector.is_none() {
                fail!(~"Error getting selector next ") ;
            }
            selector = o_selector.get() ; 
            /* Ignore any selectors contained in rules which are a child 
             * of an @media block that doesn't match the current media 
             * requirements. */
            if (css_select_ctx::_rule_applies_to_media(selector.rule, state.media)) {
                error = self.match_selector_chain(Some(selector), state);
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
                mut_ptr_eq( selector, node_selectors_option.get() ) ) {
                let mut (node_next_hash,error) = 
                        css_selector_hash::_iterate_elements(node_selectors_hash_entry.get());

                match error {
                    CSS_OK => {},
                    err => {
                        return err;
                    }
                }

                if node_next_hash.is_some() {
                    node_selectors_hash_entry = node_next_hash;
                    node_selectors_option = Some(node_next_hash.get().selector) ;
                }
                else {
                    node_selectors_option = None ;
                }
            } 
            else if (   id_selectors_option.is_some() &&
                        mut_ptr_eq(selector, id_selectors_option.get() ) ){
                let mut (id_next_hash,error) = 
                            css_selector_hash::_iterate_ids(id_selectors_hash_entry.get());

                match error {
                    CSS_OK => {},
                    err => {
                        return err;
                    }
                }

                if id_next_hash.is_some() {
                    id_selectors_hash_entry = id_next_hash;
                    id_selectors_option = Some(id_next_hash.get().selector) ;
                }
                else {
                    id_selectors_option = None ;
                }
            } 
            else if (   univ_selectors_option.is_some() &&
                        mut_ptr_eq(selector, univ_selectors_option.get() ) ){
                let mut (univ_next_hash,error) = 
                            css_selector_hash::_iterate_universal(univ_selectors_hash_entry.get());

                match error {
                    CSS_OK => {},
                    err => {
                        return err;
                    }
                }

                if univ_next_hash.is_some() {
                    univ_selectors_hash_entry = univ_next_hash;
                    univ_selectors_option = Some(univ_next_hash.get().selector);
                }
                else {
                    univ_selectors_option = None ;
                }
            } 
            else {
                let mut i = 0 ;
                //let mut j = class_selectors_option_list.len()  ;
                while i < class_selectors_option_list.len()  {
                    if ( class_selectors_option_list[i].is_some() &&
                         mut_ptr_eq(selector, class_selectors_option_list[i].get()) ) {
                        let mut (class_next_hash,error) = 
                                        css_selector_hash::_iterate_classes(
                                                    class_selectors_hash_entry[i].get());

                        match error {
                            CSS_OK => {},
                            err => {
                                return err;
                            }
                        }

                        if class_next_hash.is_some() {
                            class_selectors_hash_entry[i] = class_next_hash;
                            class_selectors_option_list[i] = Some(class_next_hash.get().selector);
                        }
                        else {
                            class_selectors_option_list[i] = None;
                        }
                        break;
                    }
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
    pub fn update_reject_cache(state: @mut css_select_state, comb:css_combinator,
                                s:@mut css_selector) {

        io::println(fmt!("Entering update_reject_cache")) ;
        let mut  next_detail : Option<@mut css_selector_detail> = None;

        unsafe {
            if (s.data.len() > 1 ) {
                next_detail = Some(s.data[1]);
            }

            if ( (state.next_reject < 0) ||

                (match comb {   
                    CSS_COMBINATOR_ANCESTOR => { false },
                    _=>{
                        true
                    }
                })   ||

                (next_detail.is_none()) ||

                (if (s.data.len() > 2) {
                    true
                } 
                else {
                    false
                }) ||

                (match next_detail.get().selector_type {   
                    CSS_SELECTOR_CLASS=> { false },         
                    CSS_SELECTOR_ID=>{false},
                    _=>{
                        true  
                    }
                }) 
            ) {

                return ;
            }
        }

        /* Insert */
        let mut item : reject_item = reject_item{
            value: copy next_detail.get().qname.name ,
            sel_type: next_detail.get().selector_type
        };
        state.reject_cache[state.next_reject] = Some(item) ;
        state.next_reject -= 1;
    }

    pub fn match_named_combinator(&mut self, combinator_type:css_combinator,
        selector:@mut css_selector, state:@mut css_select_state, 
        node:*libc::c_void, next_node:*mut *libc::c_void) -> css_error {

        io::println(fmt!("Entering match_named_combinator")) ;
        let detail :~[@mut css_selector_detail] = copy selector.data;
        let mut n = node;
        let mut error:css_error;

        loop {
            let mut match_result = false;

            /* Find candidate node */
            match combinator_type {
                CSS_COMBINATOR_ANCESTOR => {
                    error = (*state.handler.unwrap().named_ancestor_node)( 
                            n, &mut selector.data[0].qname, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }   
                CSS_COMBINATOR_PARENT => {
                    error = (*state.handler.unwrap().named_parent_node)( 
                            n, &mut selector.data[0].qname, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }    
                CSS_COMBINATOR_SIBLING => {
                    error = (*state.handler.unwrap().named_sibling_node)( 
                            n, &mut selector.data[0].qname, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }    
                    
                CSS_COMBINATOR_GENERIC_SIBLING => {
                    error = (*state.handler.unwrap().named_generic_sibling_node)(
                            n, &mut selector.data[0].qname, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }        
                CSS_COMBINATOR_NONE => {}
                    
            }

            if n != ptr::null() {
                /* Match its details */
                error = self.match_details(n, detail, state, @mut match_result, None);
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
                        n = ptr::null();   
                    },
                    _  => {}
                }    
                    
            }

            if n == ptr::null() {
                break
            }
        }

        unsafe { *next_node = n };

        return CSS_OK;
    }

    pub fn match_selector_chain(&mut self, selector:Option<@mut css_selector>,
                            state:@mut css_select_state) -> css_error {

        io::println(fmt!("Entering match_selector_chain")) ;
        let mut s = selector;
        let mut node = state.node;
        let mut match_b : @mut bool = @mut false;
        let mut may_optimise = true;
        let mut rejected_by_cache : @mut bool = @mut true ;
        let mut pseudo : css_pseudo_element = CSS_PSEUDO_ELEMENT_NONE ;
        let mut error : css_error ;
        let mut universal_string = self.universal.swap_unwrap().clone() ;
        self.universal = Some(universal_string.clone()) ;     
        
        /* Match the details of the first selector in the chain. 
         *
         * Note that pseudo elements will only appear as details of
         * the first selector in the chain, as the parser will reject
         * any selector chains containing pseudo elements anywhere 
         * else.
         */
        unsafe {
            error = self.match_details(node, (s.get().data) , state, match_b, Some(@mut pseudo) );
        }
        match error {
            CSS_OK => {},
            err => { 
                return err ;
            }
        }

        /* Details don't match, so reject selector chain */
        if (match_b == @mut false) {
            return CSS_OK;
        }

        unsafe {
            /* Iterate up the selector chain, matching combinators */
            while ( s.is_some() ) {
                let mut next_node : *libc::c_void = ptr::null();

                /* Consider any combinator on this selector */
                if ( (s.get().data.len() > 0 ) && 
                     ( match s.get().data[0].combinator_type { 
                        CSS_COMBINATOR_NONE=>{false},
                        _=>{true} }
                     ) && 
                     (s.get().combinator.is_some() ) &&
                     (self.universal.is_some() ) &&
                     (s.get().combinator.get().data[0].qname.name != 
                      lwc_string_data(universal_string.clone()) )  ) {

                    /* Named combinator */
                    
                        may_optimise &= match s.get().data[0].combinator_type {
                            CSS_COMBINATOR_ANCESTOR=> { true },
                            CSS_COMBINATOR_PARENT=>{ true },
                            _=>{ false }
                        } ;
                    

                    error = self.match_named_combinator(s.get().data[0].combinator_type, 
                           s.get().combinator.get(), state, node, &mut next_node);
                    match error {
                        CSS_OK => {},
                        err => { 
                            return err ;
                        }
                    }

                    /* No match for combinator, so reject selector chain */
                    if (next_node == ptr::null() ) {
                        return CSS_OK;
                    }
                } 
                else if ( (s.get().data.len() > 0 ) &&
                        ( match s.get().data[0].combinator_type { 
                            CSS_COMBINATOR_NONE=>{false},
                            _=>{true} }
                        ) ) {

                    /* Universal combinator */
                    
                        may_optimise &= match s.get().data[0].combinator_type {
                            CSS_COMBINATOR_ANCESTOR=> { true },
                            CSS_COMBINATOR_PARENT=>{ true },
                            _=>{ false }
                        } ;
                    

                    error = self.match_universal_combinator(s.get().data[0].combinator_type, 
                                                    s.get().combinator.get(), state, node, 
                                                    may_optimise, rejected_by_cache,
                                                    &mut next_node);
                    match error {
                        CSS_OK => {},
                        err => { 
                            return err ;
                        }
                    }

                    /* No match for combinator, so reject selector chain */
                    if (next_node == ptr::null()) {
                        if (may_optimise && mut_ptr_eq(s.get(),selector.get()) &&
                                rejected_by_cache == @mut false) {
                            css_select_ctx::update_reject_cache(state, 
                                                    s.get().data[0].combinator_type,
                                                    s.get().combinator.get());
                        }

                        return CSS_OK;
                    }
                }

                /* Details matched, so progress to combining selector */
                s = s.get().combinator;
                node = next_node;
            } 
        }
        /* If we got here, then the entire selector chain matched, so cascade */
        state.current_specificity = selector.get().specificity;

        /* No bytecode if rule body is empty or wholly invalid */
        if ( selector.get().rule.is_none() ) {
            return CSS_OK;
        }

         /* No bytecode if rule body is empty or wholly invalid */
        let mut rule = match selector.get().rule.get() {
            RULE_SELECTOR(x)=>{
                x
            },
            _=> {
                return CSS_OK ;
            }
        } ;

        if ( rule.style.is_none() ) {
            return CSS_OK ;
        }

        unsafe {
            if( state.results.styles.len() <= pseudo as uint ) {
                return CSS_INVALID ;
            }

            /* Ensure that the appropriate computed style exists */
            if ( state.results.styles[pseudo as uint].is_none() ) {
                state.results.styles[pseudo as uint] = Some(css_computed_style_create()); 
            }
        }

        state.current_pseudo = pseudo;
        state.computed = state.results.styles[pseudo as uint].get();

        css_select_ctx::cascade_style( rule.style.get() , state)
    }

    pub fn match_universal_combinator(&mut self, combinator_type:css_combinator,
        selector:@mut css_selector, state:@mut css_select_state,
        node:*libc::c_void, may_optimise:bool, rejected_by_cache:@mut bool,
        next_node:*mut *libc::c_void) -> css_error  {
        
        io::println(fmt!("Entering match_universal_combinator")) ;
        let detail :~[@mut css_selector_detail] = copy selector.data;
        let mut n:*libc::c_void = node;
        let mut next_detail:Option<@mut css_selector_detail> = None; 
        let mut error:css_error;
        
        if (detail.len() > 1){
            next_detail = Some(detail[1]);   
        }
            
        *rejected_by_cache = false;

        /* Consult reject cache first */
        if (may_optimise && 
            (match combinator_type { CSS_COMBINATOR_ANCESTOR | CSS_COMBINATOR_PARENT => true, _ => false }) && 
            match next_detail { Some(_) => true, None => false } &&
            (match next_detail.get().selector_type { CSS_SELECTOR_CLASS | CSS_SELECTOR_ID => true, _ => false})) {

            let mut reject = state.next_reject + 1;
            let last : int = (unsafe { state.reject_cache.len()} -1) as int ;

            while (reject <= last) {
                /* Perform pessimistic matching (may hurt quirks) */
                if ((copy state.reject_cache[reject]).get().sel_type as uint == next_detail.get().selector_type as uint) &&
                   ((copy state.reject_cache[reject]).get().value==next_detail.get().qname.name) {
                    
                    /* Found it: can't match */
                    unsafe { *next_node = ptr::null() };
                    *rejected_by_cache = true;
                    return CSS_OK;
                }

                reject += 1;
            }
        }

        loop {
            let mut match_result = false;

            /* Find candidate node */
            match (combinator_type) {
                CSS_COMBINATOR_ANCESTOR | 
                CSS_COMBINATOR_PARENT => {
                    error = (*state.handler.get().parent_node)(n, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }
                CSS_COMBINATOR_SIBLING |
                CSS_COMBINATOR_GENERIC_SIBLING => {
                    error = (*state.handler.get().sibling_node)(n, &mut n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }
                CSS_COMBINATOR_NONE => {}
            }

            if (n != ptr::null()) {
                /* Match its details */
                error = self.match_details(n, vec::slice(detail,1,detail.len()), state, @mut match_result, None);
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
                        n = ptr::null();   
                    },
                    _  => {}
                }    

                if n == ptr::null() {
                break
            }    
            }
        } 

        unsafe { *next_node = n };

        return CSS_OK;
    }

    pub fn match_details(&mut self, node:*libc::c_void, 
        detail :&[@mut css_selector_detail], state : @mut css_select_state, 
        matched : @mut bool, pseudo_element : Option<@mut css_pseudo_element>) -> css_error {

        io::println(fmt!("Entering match_details")) ;
        let mut error : css_error ;
        let mut pseudo : @mut css_pseudo_element = @mut CSS_PSEUDO_ELEMENT_NONE;
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
            error = self.match_detail(node, detail[index], state, matched, pseudo);
            match error {
                CSS_OK => {}
                _=> {
                    return error;
                }
            }

            if !(*matched) {
                return CSS_OK;
            }

            if(copy detail.len() -1 > index){
                index += 1;
            }
            else {
                index = -1;
            }
        }
        
        /* Return the applicable pseudo element, if required */
        match pseudo_element {
            Some(value) =>{
                value = pseudo ;
            },
            None => {}
        }
 
        CSS_OK
    }
    

    pub fn match_nth(a:i32  , b:i32 , count:i32) -> bool {

        io::println(fmt!("Entering match_nth")) ;
        if (a == 0) {
            return (count == b);
        } 
        else {
            let mut delta : i32 = count - b;

            /* (count - b) / a is positive or (count - b) is 0 */
            if (((delta > 0) == (a > 0)) || delta == 0) {
                /* (count - b) / a is integer */
                return (delta % a == 0);
            }

            return false;
        }
    }

    pub fn match_detail(&mut self, node:*libc::c_void, 
            detail:@mut css_selector_detail, state:@mut css_select_state, 
            matched:@mut bool, pseudo_element:@mut css_pseudo_element) -> css_error {

        io::println(fmt!("Entering match_detail")) ;
        let mut is_root = false;
        let mut error = CSS_OK;
        let lwc_name = do self.lwc_instance.clone().write |l|{
            l.lwc_intern_string(copy detail.qname.name)
        };

        match (detail.selector_type) {
            CSS_SELECTOR_ELEMENT => {
                if (detail.negate) {
                    /* Only need to test this inside not(), since
                     * it will have been considered as a named node
                     * otherwise. */
                    error = (*state.handler.get().node_has_name)(state.pw, node,
                            copy detail.qname, matched);
                }
            }
            CSS_SELECTOR_CLASS => {
                error = (*state.handler.get().node_has_class)(state.pw, node,
                        lwc_name.clone(), matched);
            }       
            CSS_SELECTOR_ID => {
                error = (*state.handler.get().node_has_id)(state.pw, node,
                        lwc_name.clone(), matched);
            }
            CSS_SELECTOR_PSEUDO_CLASS => {
                error = (*state.handler.get().node_is_root)( node, @mut is_root);
                match error {
                    CSS_OK => {},
                    _=> {
                        return error;
                    }
                }
        
                if (is_root == false && 
                         do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.first_child.get_ref().clone())} ) { 

                    let mut num_before:i32 = 0;

                    error = (*state.handler.get().node_count_siblings)( 
                            node, false, false, @mut num_before);

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
                        do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.nth_child.get_ref().clone())} ) { 
                    let mut num_before:i32 = 0;

                    error = (*state.handler.get().node_count_siblings)( 
                            node, false, false, @mut num_before);
                
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
                        do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.nth_last_child.get_ref().clone())} ) { 
                    let mut num_after:i32 = 0;

                    error = (*state.handler.get().node_count_siblings)( 
                            node, false, true, @mut num_after);
                    
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
                        do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.nth_of_type.get_ref().clone())} ) { 
                    let mut num_before:i32 = 0;

                    error = (*state.handler.get().node_count_siblings)( 
                            node, true, false, @mut num_before);
                    
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
                        do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.nth_last_of_type.get_ref().clone())} ) { 
                    let mut num_after:i32 = 0;

                    error = (*state.handler.get().node_count_siblings)( 
                            node, true, true, @mut num_after);
                    
                    match error {
                        CSS_OK => {
                            let a = detail.a;
                            let b = detail.b;

                            *matched = css_select_ctx::match_nth(a, b, num_after + 1);
                        },
                        _ => {}
                    }
                } else if (is_root == false &&
                        do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.last_child.get_ref().clone())} ) { 
                    let mut num_after:i32 = 0;

                    error = (*state.handler.get().node_count_siblings)(
                            node, false, true, @mut num_after);
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
                        do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.first_of_type.get_ref().clone())} ) { 
                    let mut num_before:i32 = 0;

                    error = (*state.handler.get().node_count_siblings)( 
                            node, true, false, @mut num_before);
                    
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
                        do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.last_of_type.get_ref().clone())} ) { 
                    let mut num_after:i32 = 0;

                    error = (*state.handler.get().node_count_siblings)( 
                            node, true, true, @mut num_after);
                
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
                        do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.only_child.get_ref().clone())} ) { 
                    
                    let mut num_before = 0;
                    let mut num_after = 0;

                    error = (*state.handler.get().node_count_siblings)( 
                            node, false, false, @mut num_before);
                    
                    match error {
                        CSS_OK => {
                            error = (*state.handler.get().node_count_siblings)(
                                 node, false, true, @mut num_after);
                                    
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
                        do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.only_of_type.get_ref().clone())} ) { 
                
                    let mut num_before = 0;
                    let mut num_after = 0;

                    error = (*state.handler.get().node_count_siblings)( 
                            node, true, false, @mut num_before);
                
                    if (match error { CSS_OK => true, _  => false}) {
                        error = (*state.handler.get().node_count_siblings)(
                                    node, true, true, @mut num_after);
                
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
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.root.get_ref().clone())} ) { 
                    *matched = is_root;
                } 
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.empty.get_ref().clone())} ) {
                    error = (*state.handler.get().node_is_empty)(
                            node, matched);
                } 
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.link.get_ref().clone())} ) { 
                    error = (*state.handler.get().node_is_link)(
                            node, matched);
                }
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.visited.get_ref().clone())} ) { 
                    error = (*state.handler.get().node_is_visited)(
                            node, matched);
                }
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.hover.get_ref().clone())} ) { 
                    error = (*state.handler.get().node_is_hover)(
                            node, matched);
                }
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.active.get_ref().clone())} ) { 
                    error = (*state.handler.get().node_is_active)(
                            node, matched);
                } 
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.focus.get_ref().clone())} ) { 
                    error = (*state.handler.get().node_is_focus)(
                            node, matched);
                } 
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.target.get_ref().clone())} ) { 
                    error = (*state.handler.get().node_is_target)(
                            node, matched);
                }
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.lang.get_ref().clone())} ) { 
                    error = (*state.handler.get().node_is_lang)(
                            node, (copy detail.string).unwrap(), matched);
                }
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.enabled.get_ref().clone())} ) { 
                    error = (*state.handler.get().node_is_enabled)(
                            node, matched);
                }
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.disabled.get_ref().clone())} ) { 
                    error = (*state.handler.get().node_is_disabled)(
                            node, matched);
                }
                else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.checked.get_ref().clone())} ) { 
                    error = (*state.handler.get().node_is_checked)(
                            node, matched);
                }
                else {
                    *matched = false;
                }
            }
            CSS_SELECTOR_PSEUDO_ELEMENT => {
                *matched = true;

                if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.first_line.get_ref().clone())} ) { 
                    *pseudo_element = CSS_PSEUDO_ELEMENT_FIRST_LINE;
                } else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.first_letter.get_ref().clone())} ) { 
                    *pseudo_element = CSS_PSEUDO_ELEMENT_FIRST_LETTER;
                } else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.before.get_ref().clone())} ) { 
                    *pseudo_element = CSS_PSEUDO_ELEMENT_BEFORE;
                } else if (do self.lwc_instance.clone().read |l| { l.lwc_string_isequal(lwc_name.clone(), self.after.get_ref().clone())} ) { 
                    *pseudo_element = CSS_PSEUDO_ELEMENT_AFTER;
                } else {
                    *matched = false;
                }
            }
            CSS_SELECTOR_ATTRIBUTE => {
                error = (*state.handler.get().node_has_attribute)( node,
                        copy detail.qname, matched);
            }
            CSS_SELECTOR_ATTRIBUTE_EQUAL => {
                error = (*state.handler.get().node_has_attribute_equal)( 
                        node, copy detail.qname, (copy detail.string).unwrap(), 
                        matched);
            }
            CSS_SELECTOR_ATTRIBUTE_DASHMATCH => {
                error = (*state.handler.get().node_has_attribute_dashmatch)(
                        node, copy detail.qname, (copy detail.string).unwrap(),
                        matched);
            }
            CSS_SELECTOR_ATTRIBUTE_INCLUDES => {
                error = (*state.handler.get().node_has_attribute_includes)( 
                        node, copy detail.qname, (copy detail.string).unwrap(),
                        matched);
            }
            CSS_SELECTOR_ATTRIBUTE_PREFIX => {
                error = (*state.handler.get().node_has_attribute_prefix)(
                        node, copy detail.qname, (copy detail.string).unwrap(),
                        matched);
            }
            CSS_SELECTOR_ATTRIBUTE_SUFFIX => {
                error = (*state.handler.get().node_has_attribute_suffix)(
                        node, copy detail.qname,(copy detail.string).unwrap(),
                        matched);
            }
            CSS_SELECTOR_ATTRIBUTE_SUBSTRING => {
                error = (*state.handler.get().node_has_attribute_substring)(
                        node, copy detail.qname,(copy  detail.string).unwrap(),
                        matched);
            }
        }

        /* Invert match, if the detail requests it */
        if ( (match error { CSS_OK => true, _ => false} )&& detail.negate){
            *matched = !*matched;
        }
        return error
    }

    pub fn cascade_style(style:@mut css_style, state:@mut css_select_state) -> css_error {
        let mut s = style;

        io::println(fmt!("Entering cascade_style")) ;

		unsafe{
			io::println(fmt!("s_used=%?, s_len=%?", s.used, s.bytecode.len())) ;
		}	
        while (unsafe { s.used  < s.bytecode.len()} ) {
            let mut op: u32;
            let mut error : css_error ;
            let mut opv = peek_bytecode(s);

            advance_bytecode(s);

            op = getOpcode(opv) as u32;
            io::println(fmt!("op=%?, opv=%?, op_m=%?", op, opv, op as uint));
			let mut dispatch_cascade = dispatch_table::get_cascade_ptr(op as uint) ;
            error =  dispatch_cascade(opv, s, state);

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
