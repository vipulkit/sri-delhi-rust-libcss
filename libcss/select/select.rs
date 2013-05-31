
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
     *
     * \return Pointer to created context
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
     *
     * \param self   The selection context to append to
     * \param sheet   The sheet to append
     * \param origin  Origin of the sheet
     * \param media   Media types to which the sheet applies
     * \return CSS_OK on success, appropriate error otherwise
     */
    pub fn css_select_ctx_append_sheet(&mut self,
                                    sheet:@mut css_stylesheet,
                                    origin:css_origin,
                                    media:u64) 
                                    -> css_error {
        let n_sheets = self.sheets.len();
        self.css_select_ctx_insert_sheet(sheet, n_sheets, origin,media)
    }

    /**
     * Insert a stylesheet into a selection context
     * 
     * \param self   The selection context to insert into
     * \param sheet  Sheet to insert
     * \param index  Index in context to insert sheet
     * \param origin  Origin of the sheet
     * \param media   Media types to which the sheet applies
     * \return CSS_OK on success, appropriate error otherwise
     */
    pub fn css_select_ctx_insert_sheet(&mut self,
                                    csheet:@mut css_stylesheet,
                                    index:uint,
                                    corigin:css_origin,
                                    cmedia:u64) 
                                    -> css_error {

    
        /* Inline styles cannot be inserted into a selection context */
        if (csheet.inline_style) {
            return CSS_INVALID ;
        }
    
        /* Index must be in the range [0, n_sheets]
         * The latter being equivalent to append */
        if index > self.sheets.len()    {
            return CSS_INVALID;
        }   
            
        let mut select_sheet = @mut css_select_sheet{
            sheet:csheet,
            origin:corigin,
            media:cmedia
        };

        self.sheets.insert(index, select_sheet);
        CSS_OK
    }

    /**
     * Remove a sheet from a selection context
     *
     * \param self   The selection context to remove from
     * \param sheet  Sheet to remove
     * \return CSS_OK on success, appropriate error otherwise
     */
    pub fn css_select_ctx_remove_sheet(&mut self, csheet:@mut css_stylesheet)-> css_error {

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
     *
     * \param self   The selection context to consider
     * \return Count of sheets
     */
    pub fn css_select_ctx_count_sheets(&mut self) -> uint {

        self.sheets.len()
    }

    /**
     * Retrieve a sheet from a selection context
     *
     * \param self   The selection context to look in
     * \param index  Index in context to look
     * \return (CSS_OK,Some(sheet)) on success, appropriate (error,None) otherwise
     */
    pub fn css_select_ctx_get_sheet(&mut self, index:uint) 
                                -> (css_error,Option<@mut css_stylesheet>) {

        if ( index >= self.sheets.len() ) {
            return (CSS_INVALID,None) ;
        }

        (CSS_OK,Some(self.sheets[index].sheet))
    } 

    pub fn css_select_style(&mut self,
                                node:*libc::c_void,
                                media:u64,
                                inline_style:Option<@mut css_stylesheet>,
                                handler:@mut css_select_handler) 
                                -> (css_error,Option<css_select_results>) {

        if( node == ptr::null() || handler.handler_version != (CSS_SELECT_HANDLER_VERSION_1  as uint) ) {
            return (CSS_BADPARM,None) ;
        }
        let mut i = 0 ;
        let mut j = 0 ;
        let mut error : css_error ;
        let mut results : Option<css_select_results> = None ;
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
            props: ~[~[]] 
        };
        for uint::range(0,CSS_N_PROPERTIES as uint) |outer| {
            let mut prop_vec : ~[@mut prop_state] = ~[] ;
            for uint::range(0,CSS_PSEUDO_ELEMENT_COUNT as uint) |inner| {
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
        }

        /* Base element style is guaranteed to exist */
        state.results.styles.push(Some(css_computed_style_create()));

        error = (*(handler.parent_node))(node, parent);
        match error {
            CSS_OK=>{},
            x =>  {
                return (x,None) ;
            }
        }

        /* Get node's name */
        error = (*(handler.node_name))(node, copy state.element);
        match error {
            CSS_OK=>{},
            x =>  {
                return (x,None) ;
            }
        }

        /* Get node's ID, if any */
        error = (*(handler.node_id))(node, copy state.id);
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
        error = (*(handler.node_classes))(node, 
                copy state.classes);
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
                    error = self.select_from_sheet(Some(s.sheet), 
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
            let mut prop = 
                    state.props[i][CSS_PSEUDO_ELEMENT_NONE as uint];

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
                        CSS_PSEUDO_ELEMENT_NONE, None);
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
            state.current_pseudo = unsafe { cast::transmute(j)};
            state.computed = state.results.styles[j].get();

            /* Skip non-existent pseudo elements */
            // if (state.computed == NULL)
            //     continue;
            i = 0 ;
            while (i < (CSS_N_PROPERTIES as int) ) {
                let mut prop = state.props[i][j];

                /* If the property is still unset then set it 
                 * to its initial value. */
                if (prop.set == false) {
                    error = css_select_ctx::set_initial(state, i as uint, unsafe { cast::transmute(j)}, None);
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
     *
     * \param results  Result set to destroy
     */
    pub fn css_select_results_destroy(results: &mut ~[@mut css_select_results] ) {
        results.clear() ;
        
    }

    /**
     * Search a selection context for defined font faces
     *
     * \param self         Selection context
     * \param media        Currently active media types
     * \param font_family  Font family to search for
     * \return (CSS_OK,Some(css_select_font_faces_results)) on success, appropriate (error,None) otherwise.
     */
    pub fn css_select_font_faces(&mut self,
                                media:u64,
                                font_family:arc::RWARC<~lwc_string>) 
                                -> (css_error,Option<@mut css_select_font_faces_results>) {

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
        
        
        /* Retrieve this property's hint from the client */
        let (error,hint_option) = (*state.handler.unwrap().node_presentational_hint)(state.node, prop);
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
        parent: Option<@mut css_computed_style>) -> css_error {

        let mut error : css_error = CSS_OK; 
        let mut is_pseudo_and_parent_none : bool = false;
        match pseudo {
            CSS_PSEUDO_ELEMENT_NONE => {
                match parent {
                    None => {
                        is_pseudo_and_parent_none = true;
                    }
                    Some(_) => {}
                }
            }
            _=> {}
        }

        if dispatch_table::get_inherited(prop) == 0 || is_pseudo_and_parent_none{
            let mut group : prop_group = dispatch_table::get_group(prop);
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

    pub fn select_from_sheet(&mut self, sheet : Option<@mut css_stylesheet>,origin : css_origin, state : &mut css_select_state) -> css_error{
        let mut s_option : Option<@mut css_stylesheet> = sheet;
        let mut rule : Option<CSS_RULE_DATA_TYPE> = None;
        let mut sp : u32 = 0;
        let mut import_stack : ~[Option<CSS_RULE_DATA_TYPE>] = ~[];
        loop{
            let mut s : @mut css_stylesheet;
            match s_option {
                None => { 
                    break;
                },
                Some(T) => { s = T;}
            }

            rule = s.rule_list;
            if compare_css_rdt(rule, s.rule_list){
                while !rule.is_none() && compare_css_rule_types(rule, CSS_RULE_IMPORT) {
                    rule = get_css_rule_next(rule);
                }
            }
            if !rule.is_none() && compare_css_rule_types(rule, CSS_RULE_IMPORT) {
                let mut import_sheet : Option<@mut css_stylesheet> = None;
                let mut import_media:u64 = 0;
                match rule {
                    None => {},
                    Some(T) => {
                        match T {
                           RULE_IMPORT(x) => {
                            import_media = x.media;
                            import_sheet = x.sheet;
                           },
                           _=> {},
                        }
                    }
                }

                if !import_sheet.is_none() && (import_media & state.media) != 0 {
                    if sp >= 256 {
                        return CSS_NOMEM;
                    }

                    import_stack.push(rule);
                    match import_sheet {
                        None => {},
                        Some(T) => {
                            s = T;
                        }
                    }

                    rule = s.rule_list;
                }
                else {
                    rule = get_css_rule_next(rule);
                }
            }
            else {
                let mut error : css_error ;
                state.sheet = Some(s);
                state.current_origin = origin;
                error = self.match_selectors_in_sheet(s, state);
                match error {
                    CSS_OK => {
                        if sp > 0 {
                            sp -= 1;
                            rule = get_css_rule_next(import_stack[sp]);
                            s_option = get_stylesheet_parent(import_stack[sp]);
                        }
                        else {
                            s_option = None;
                        }
                    },
                    _=> { 
                        return error;
                    }
                }
            }
        }

        CSS_OK
    }

    pub fn _rule_applies_to_media(rule: Option<CSS_RULE_DATA_TYPE>, media:u64) -> bool {

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
                            faces.font_faces.push_all(state.ua_font_faces.font_faces);
                        },
                        CSS_ORIGIN_USER => {
                            faces.font_faces.push_all(state.user_font_faces.font_faces);
                        },
                        CSS_ORIGIN_AUTHOR => {
                            faces.font_faces.push_all(state.author_font_faces.font_faces);
                        }
                    }
                }
                faces.font_faces.push(rule.font_face.get());
            }
        }
        CSS_OK
    }

    pub fn select_font_faces_from_sheet(&mut self,
                                        sheet:@mut css_stylesheet,
                                        origin: css_origin,
                                        state:@mut css_select_font_faces_state)
                                        -> css_error {

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
                        rule = css_stylesheet::css__stylesheet_get_base_rule(import_stack[sp]).next;
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

    pub fn _selectors_pending(node: Option<Option<css_selector>>, id: Option<Option<css_selector>>,
                classes: ~[Option<Option<css_selector>>], n_classes : uint, 
                univ: Option<Option<css_selector>>) -> bool {

        let mut pending : bool = false;
        match node {
            None => {}
            Some(T) => {
                match T {
                    None => {
                        pending |= false;
                    }
                    Some(_) => {
                        pending |= true;
                    }
                }
            }
        }

        match id {
            None => {}
            Some(T) => {
                match T {
                    None => {
                        pending |= false;
                    }
                    Some(_) => {
                        pending |= true;
                    }
                }
            }
        }

        match univ {
            None => {}
            Some(T) => {
                match T {
                    None => {
                        pending |= false;
                    }
                    Some(_) => {
                        pending |= true;
                    }
                }
            }
        }

        let mut i : uint = 0;
        while i < n_classes {
            match copy classes[i] {
                None => {}
                Some(T) => {
                    match T {
                        None => {
                            pending |= false;
                        }
                        Some(_) => {
                            pending |= true;
                        }
                    }
                }
            }

            i += 1;
        }

        pending
    }

    pub fn _selector_less_specific(refer:@mut css_selector, cand:@mut css_selector) -> bool {

        let mut result : bool = true;

        // if (cand == NULL)
        //  return false;

        // if (ref == NULL)
        //  return true;

        /* Sort by specificity */
        if (cand.specificity < refer.specificity) {
            result = true;
        } 
        else if (refer.specificity < cand.specificity) {
            result = false;
        } 
        else {

            if( cand.rule.is_none() || refer.rule.is_none() ) {
                fail!(~"_selector_less_specific:Base rule cannot be null");
            }
            let mut cand_base = css_stylesheet::css__stylesheet_get_base_rule(cand.rule.get()) ;
            let mut refer_base = css_stylesheet::css__stylesheet_get_base_rule(refer.rule.get()) ;
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

    pub fn _selector_next(node: Option<@mut css_selector>, id: Option<@mut css_selector>,
                classes: ~[Option<@mut css_selector>], n_classes : uint, 
                univ: Option<@mut css_selector>) -> Option<@mut css_selector> {

        let mut ret : Option<@mut css_selector> = None;

        match node {
            None => {}
            Some(T) => {
                ret = Some(T);
            }
        }

        match id {
            None => {}
            Some(I) => {
                match ret {
                    None => {}
                    Some(R) => {
                        if css_select_ctx::_selector_less_specific(R, I) {
                            ret = Some(I);
                        }
                    }
                }
            }
        }

        match univ {
            None => {}
            Some(I) => {
                match ret {
                    None => {}
                    Some(R) => {
                        if css_select_ctx::_selector_less_specific(R, I) {
                            ret = Some(I);
                        }
                    }
                }
            }
        }

        let mut i : uint = 0;
        while i < n_classes {
            match copy classes[i] {
                None => {}
                Some(T) => {
                    match ret {
                        None => {}
                        Some(R) => {
                            if css_select_ctx::_selector_less_specific(R, T) {
                                ret = Some(T);
                            }
                        }
                    }
                }
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

    // Note: pending implementation
    pub fn match_selectors_in_sheet(&mut self, sheet : @mut css_stylesheet, state : &mut css_select_state) -> css_error {
        CSS_OK
    }

    pub fn update_reject_cache(state: @mut css_select_state, comb:css_combinator,
                                s:@mut css_selector) {

        let mut  next_detail : Option<@mut css_selector_detail> = None;

        unsafe {
            if (s.data.len() > 1 ) {
                next_detail = Some(s.data[1]);
            }

            if (state.next_reject < 0 || s.data.len() > 2 ) { 
                return;
            }
        }

        if( next_detail.is_none() ) {
            return ;
        }

        match comb {
            CSS_COMBINATOR_ANCESTOR => {},
            _=>{
                return ;
            }
        }

        match next_detail.get().selector_type {
            CSS_SELECTOR_CLASS=> {},
            CSS_SELECTOR_ID=>{},
            _=>{
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

        let detail :~[@mut css_selector_detail] = copy selector.data;
        let mut n = node;
        let mut error:css_error;

        loop {
            let mut match_result = false;

            /* Find candidate node */
            match combinator_type {
                CSS_COMBINATOR_ANCESTOR => {
                    error = (*state.handler.unwrap().named_ancestor_node)( 
                            n, &mut selector.data[0].qname, &n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }   
                CSS_COMBINATOR_PARENT => {
                    error = (*state.handler.unwrap().named_parent_node)( 
                            n, &mut selector.data[0].qname, &n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }    
                CSS_COMBINATOR_SIBLING => {
                    error = (*state.handler.unwrap().named_sibling_node)( 
                            n, &mut selector.data[0].qname, &n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }    
                    
                CSS_COMBINATOR_GENERIC_SIBLING => {
                    error = (*state.handler.unwrap().named_generic_sibling_node)(
                            n, &mut selector.data[0].qname, &n);
                    match error {
                        CSS_OK => {},
                        err => return err
                    }
                }        
                CSS_COMBINATOR_NONE => {}
                    
            }

            let mut index : uint = 0;
            if n != ptr::null() {
                /* Match its details */
                error = self.match_details(n, copy detail, state, @mut match_result, None, @mut index);
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

    pub fn match_detail(&mut self, node:*libc::c_void, 
        detail :~[@mut css_selector_detail], state : @mut css_select_state, 
        matched : @mut bool, pseudo_element : Option<css_pseudo_element>, index: @mut uint) -> css_error {

        CSS_OK
    }

    pub fn match_details(&mut self, node:*libc::c_void, 
        detail :~[@mut css_selector_detail], state : @mut css_select_state, 
        matched : @mut bool, pseudo_element : Option<css_pseudo_element>, index: @mut uint) -> css_error {

        let mut error : css_error = CSS_OK;
        let mut pseudo : css_pseudo_element = CSS_PSEUDO_ELEMENT_NONE;
        if(detail.len() > *index){
            *index += 1;
        }
        else {
            *index = -1;
        }

        *matched = true;

        while *index != -1 {
            error = self.match_detail(node, copy detail, state, matched, Some(pseudo), index);
            match error {
                CSS_OK => {}
                _=> {
                    return error;
                }
            }

            if !(*matched) {
                return CSS_OK;
            }

            if(copy detail.len() > *index){
                *index += 1;
            }
            else {
                *index = -1;
            }
        }

        error
    }
    

    pub fn match_nth(a:i32  , b:i32 , count:i32) -> bool {
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

    pub fn cascade_style(style:@mut css_style, state:@mut css_select_state) -> css_error {
        let mut s = style;

        while (s.used > 0) {
            let mut op: u32;
            let mut error : css_error ;
            let mut opv = peek_bytecode(s);

            advance_bytecode(s);

            op = getOpcode(opv) as u32;

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
