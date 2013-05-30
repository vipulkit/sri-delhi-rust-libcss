
use std::arc;
use wapcaplet::*;

use include::types::*;
use include::font_face::*;
use bytecode::bytecode::*;
use utils::errors::*;
use select::common::*;
use select::dispatch::*;
use stylesheet::*;

use core::managed::*;

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
    font_family:~str,
    media:u64,

    ua_font_faces:css_select_font_faces_list,
    user_font_faces:css_select_font_faces_list,
    author_font_faces:css_select_font_faces_list
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

    // pub fn css_select_style(&mut self) -> css_error {

    // }

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
                                font_family:~str) 
                                -> (css_error,Option<@mut css_select_font_faces_results>) {

        if(font_family.len()==0) {
            return (CSS_BADPARM,None) ;
        }

        let state = @mut css_select_font_faces_state {
            font_family:font_family,
            media:media,

            ua_font_faces:css_select_font_faces_list{font_faces:~[]},
            user_font_faces:css_select_font_faces_list{font_faces:~[]},
            author_font_faces:css_select_font_faces_list{font_faces:~[]}
        };

        /* Iterate through the top-level stylesheets, selecting font-faces
         * from those which apply to our current media requirements and
         * are not disabled */
        for self.sheets.each |select_sheet| {

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

    pub fn select_font_faces_from_sheet(&self,
                                        sheet:@mut css_stylesheet,
                                        origin: css_origin,
                                        state:@mut css_select_font_faces_state)
                                        -> css_error {

        CSS_OK
    }

    pub fn _select_font_face_from_rule(rule:@mut css_rule_font_face,
                                    origin: css_origin,
                                    state:@mut css_select_font_faces_state) 
                                    -> css_error {


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