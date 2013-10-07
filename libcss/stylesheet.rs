use std::managed::*;
use wapcaplet::*;

use bytecode::bytecode::*;

use include::font_face::*;
use include::properties::*;
use include::types::*;
use utils::errors::*;

/**
 * Stylesheet language level -- defines parsing rules and supported properties
 */

pub enum css_language_level {
    CSS_LEVEL_1                 = 0,    /**< CSS 1 */
    CSS_LEVEL_2                 = 1,    /**< CSS 2 */
    CSS_LEVEL_21                = 2,    /**< CSS 2.1 */
    CSS_LEVEL_3                 = 3     //< CSS 3
}
static  CSS_LEVEL_DEFAULT : css_language_level = CSS_LEVEL_21 ;

pub enum css_selector_type {
    CSS_SELECTOR_ELEMENT,
    CSS_SELECTOR_CLASS,
    CSS_SELECTOR_ID,
    CSS_SELECTOR_PSEUDO_CLASS,
    CSS_SELECTOR_PSEUDO_ELEMENT,
    CSS_SELECTOR_ATTRIBUTE,
    CSS_SELECTOR_ATTRIBUTE_EQUAL,
    CSS_SELECTOR_ATTRIBUTE_DASHMATCH,
    CSS_SELECTOR_ATTRIBUTE_INCLUDES,
    CSS_SELECTOR_ATTRIBUTE_PREFIX,
    CSS_SELECTOR_ATTRIBUTE_SUFFIX,
    CSS_SELECTOR_ATTRIBUTE_SUBSTRING
}

pub enum css_combinator {
    CSS_COMBINATOR_NONE,
    CSS_COMBINATOR_ANCESTOR,
    CSS_COMBINATOR_PARENT,
    CSS_COMBINATOR_SIBLING,
    CSS_COMBINATOR_GENERIC_SIBLING
} 


pub enum css_selector_detail_value_type {
    CSS_SELECTOR_DETAIL_VALUE_STRING,
    CSS_SELECTOR_DETAIL_VALUE_NTH
} 

static CSS_SPECIFICITY_A:uint=0x01000000;
static CSS_SPECIFICITY_B:uint=0x00010000;
static CSS_SPECIFICITY_C:uint=0x00000100;
static CSS_SPECIFICITY_D:uint=0x00000001;

pub struct size {
    size: i32,
    unit: css_unit
}

pub struct line_height {
    size: i32,
    unit: css_unit
}

pub struct css_system_font {
    style: css_font_style_e,
    variant: css_font_variant_e,
    weight: css_font_weight_e,
    size: size,
    line_height: line_height,
    family: uint
}

pub type css_fixed = i32;

pub type css_url_resolution_fn = @extern fn (base:&str, rel:uint) -> (css_error,Option<uint>);
pub type css_font_resolution_fn = @extern fn (name: uint) -> (css_error , Option<css_system_font>);
pub type css_import_notification_fn =  @extern fn(url:&str, media:&mut u64) -> css_error;
pub type css_color_resolution_fn = @extern fn (name: uint) -> (Option<u32> , css_error);



static CSS_STYLE_DEFAULT_SIZE : uint = 16 ;


// /**< Qualified name of selector */
pub struct css_qname {  
    name:uint,
    ns:uint
}

pub struct css_selector_detail {
    qname:~css_qname,                     /**< Interned name */
    selector_type:css_selector_type,     /**< The type of selector  */
    combinator_type:css_combinator,      /**< The combinator type */
    value_type:css_selector_detail_value_type,  /**<   Value of selector  */
    negate:bool,                        /**< Detail match is inverted */

    //css_selector_detail_value - union merged
    string:Option<uint>,
    a:i32,
    b:i32
}

/**< css_selector */
pub struct css_selector {
    combinator:Option<uint>,   /**< Combining selector */
    rule:Option<CSS_RULE_DATA_TYPE>,        /**< Owning rule */
    specificity:uint,                       /**< Specificity of selector */ 
    data:~[~css_selector_detail]        /* *< Selector data */
}


pub struct css_style {
    bytecode:~[u32],
    used:uint,                
    sheet:Option<@mut css_stylesheet>
}
pub struct hash_entry {
    selector:uint,
    next:Option<@mut hash_entry>
}

/**< Hashtable of selectors */
pub struct css_selector_hash {
    default_slots:u32,
    elements:~[Option<@mut hash_entry>],
    classes:~[Option<@mut hash_entry>],
    ids:~[Option<@mut hash_entry>],
    universal:~[Option<@mut hash_entry>],
}

pub struct css_stylesheet {
    selectors:@mut css_selector_hash,       /**< Hashtable of selectors */
    rule_count:uint,                        /**< Number of rules in sheet */
    rule_list:Option<CSS_RULE_DATA_TYPE>,   /**< List of rules in sheet */
    last_rule:Option<CSS_RULE_DATA_TYPE>,   /**< Last rule in list */
    disabled:bool,                          /**< Whether this sheet is  disabled */
    url:~str,                               /**< URL of this sheet */
    title:~str,                             /**< Title of this sheet */
    level:css_language_level,               /**< Language level of sheet */
    quirks_allowed:bool,                    /**< Quirks permitted */
    quirks_used:bool,                       /**< Quirks actually used */
    inline_style:bool,                      /**< Is an inline style */
    string_vector:~[uint],
    resolve : css_url_resolution_fn, // URL resolution function */
    import : Option<css_import_notification_fn>, // Import notification function */
    font : Option<css_font_resolution_fn>,   //Import font_resolution function
    color: Option<css_color_resolution_fn>,
    css_rule_list: ~[~css_rule],
    css_selectors_list:~[~css_selector]
}

pub struct css_rule {
    parent_rule:Option<CSS_RULE_DATA_TYPE> ,         /**< containing parent rule */ 
    parent_stylesheet:bool,   /**< parent stylesheet */              
    prev:Option<CSS_RULE_DATA_TYPE>,                 /**< prev in list */
    next:Option<CSS_RULE_DATA_TYPE>,                /**< next in list */
    //rule_type:css_rule_type,
    index:uint//,items:uint                         /**< index in sheet */
}

pub struct css_rule_selector {
    base:uint,
    selectors:~[uint],
    style:Option<~css_style>
} 

pub struct css_rule_media {
    base:uint,
    media:u64,
    first_child:Option<CSS_RULE_DATA_TYPE>,                
    last_child:Option<CSS_RULE_DATA_TYPE>                
} 

pub struct css_rule_font_face {
    base:uint,
    font_face:Option<@mut css_font_face>
} 

pub struct css_rule_page {
    base:uint,
    selector:Option<uint>,
    style:Option<~css_style>
} 

pub struct css_rule_import {
    base:uint,
    url:~str,
    media:u64,
    sheet:Option<@mut css_stylesheet>
} 
pub struct css_rule_charset {
    base:uint,
    encoding:~str   
} 


pub enum CSS_RULE_DATA_TYPE {
    RULE_UNKNOWN(uint),
    RULE_SELECTOR(@mut css_rule_selector),
    RULE_CHARSET(@mut css_rule_charset),
    RULE_IMPORT(@mut css_rule_import),
    RULE_MEDIA(@mut css_rule_media),
    RULE_FONT_FACE(@mut css_rule_font_face),
    RULE_PAGE(@mut css_rule_page)
}

pub enum CSS_RULE_PARENT_TYPE {
    CSS_RULE_PARENT_STYLESHEET,
    CSS_RULE_PARENT_RULE
}

pub enum css_rule_type {
    CSS_RULE_UNKNOWN,
    CSS_RULE_SELECTOR,
    CSS_RULE_CHARSET,
    CSS_RULE_IMPORT,
    CSS_RULE_MEDIA,
    CSS_RULE_FONT_FACE,
    CSS_RULE_PAGE
}


pub fn compare_css_rule_types(rule : Option<CSS_RULE_DATA_TYPE>, rule_type : css_rule_type) -> bool{
    
    match rule {
        None => false,
        Some(T) => {

            match T {
                RULE_UNKNOWN(_) =>{
                    match rule_type{
                        CSS_RULE_UNKNOWN => true,
                        _=> false
                    }
                } 
                RULE_SELECTOR(_) =>{
                    match rule_type {
                        CSS_RULE_SELECTOR => true,
                        _=> false
                    }
                } 
                RULE_CHARSET(_) =>{
                    match rule_type {
                        CSS_RULE_CHARSET => true,
                        _=> false
                    }
                } 
                RULE_IMPORT(_) =>{
                    match rule_type {
                        CSS_RULE_IMPORT => true,
                        _=> false
                    }
                } 
                RULE_MEDIA(_) =>{
                    match rule_type {
                        CSS_RULE_MEDIA => true,
                        _=> false
                    }
                } 
                RULE_FONT_FACE(_) =>{
                    match rule_type {
                        CSS_RULE_FONT_FACE => true,
                        _=> false
                    }
                } 
                RULE_PAGE(_) =>{
                    match rule_type {
                        CSS_RULE_PAGE => true,
                        _=> false
                    }
                } 
            }
        }
    }
}


pub fn compare_css_rdt(rule1: Option<CSS_RULE_DATA_TYPE>, rule2: Option<CSS_RULE_DATA_TYPE>) -> bool{
    
    match rule1{
        None => {
            match rule2 {
                None => true,
                Some(_) => false,
            }
        }

        Some(T1) => {
            match  T1 {
                RULE_UNKNOWN(x) => {
                    match rule2{
                        None => false,
                        Some(T2) =>{
                            match  T2 {
                                RULE_UNKNOWN(y) => if x == y { true } else { false },
                                _=> false,
                            }
                        }
                    }
                },
                RULE_SELECTOR(x) => {
                    match rule2{
                        None => false,
                        Some(T2) =>{
                            match  T2 {
                                RULE_SELECTOR(y) => mut_ptr_eq(x,y),
                                _=> false,
                            }
                        }
                    }
                },
                RULE_CHARSET(x) => {
                    match rule2{
                        None => false,
                        Some(T2) =>{
                            match  T2 {
                                RULE_CHARSET(y) => mut_ptr_eq(x,y),
                                _=> false,
                            }
                        }
                    }
                },
                RULE_IMPORT(x) => {
                    match rule2{
                        None => false,
                        Some(T2) =>{
                            match  T2 {
                                RULE_IMPORT(y) => mut_ptr_eq(x,y),
                                _=> false,
                            }
                        }
                    }
                },
                RULE_MEDIA(x) => {
                    match rule2{
                        None => false,
                        Some(T2) =>{
                            match  T2 {
                                RULE_MEDIA(y) => mut_ptr_eq(x,y),
                                _=> false,
                            }
                        }
                    }
                },
                RULE_FONT_FACE(x) => {
                    match rule2{
                        None => false,
                        Some(T2) =>{
                            match  T2 {
                                RULE_FONT_FACE(y) => mut_ptr_eq(x,y),
                                _=> false,
                            }
                        }
                    }
                },
                RULE_PAGE(x) => {
                    match rule2{
                        None => false,
                        Some(T2) =>{
                            match  T2 {
                                RULE_PAGE(y) => mut_ptr_eq(x,y),
                                _=> false,
                            }
                        }
                    }
                },
            }
        }
    }
}

impl css_stylesheet {

    pub fn get_css_rule_next(&mut self, rule: CSS_RULE_DATA_TYPE) -> Option<CSS_RULE_DATA_TYPE> {
        match rule {
            RULE_UNKNOWN(x) => self.css_rule_list[x].next,
            RULE_SELECTOR(x) => self.css_rule_list[x.base].next,
            RULE_CHARSET(x) => self.css_rule_list[x.base].next,
            RULE_IMPORT(x) => self.css_rule_list[x.base].next,
            RULE_MEDIA(x) => self.css_rule_list[x.base].next,
            RULE_FONT_FACE(x) => self.css_rule_list[x.base].next,
            RULE_PAGE(x) => self.css_rule_list[x.base].next,
        }
    }

    pub fn get_stylesheet_parent(&mut self, rule: CSS_RULE_DATA_TYPE) -> bool {
        
        match rule {
            RULE_UNKNOWN(x) => self.css_rule_list[x].parent_stylesheet,
            RULE_SELECTOR(x) => self.css_rule_list[x.base].parent_stylesheet,
            RULE_CHARSET(x) => self.css_rule_list[x.base].parent_stylesheet,
            RULE_IMPORT(x) => self.css_rule_list[x.base].parent_stylesheet,
            RULE_MEDIA(x) => self.css_rule_list[x.base].parent_stylesheet,
            RULE_FONT_FACE(x) => self.css_rule_list[x.base].parent_stylesheet,
            RULE_PAGE(x) => self.css_rule_list[x.base].parent_stylesheet,
        }
    }

    /**
    * #Description:
    *   Add a string to a stylesheet's string vector.
    
    * #Arguments:
    *  'strings' - The string to add.
    
    * #Return Value:
    *   'uint' - index next to the index of insertion is returned.
    */
    pub fn css__stylesheet_string_add(&mut self, string: uint) -> uint {
        //debug!("Entering: css__stylesheet_string_add");
        let mut i : uint = self.string_vector.len() ;
        while(i!=0) {
            i -= 1;
            if string == self.string_vector[i] {
                return (i+1) as uint ;
            }
        }
        self.string_vector.push(string);
        self.string_vector.len()
    }

    /**
    * #Description:
    *   Get a string from a stylesheet's string vector.
    
    * #Arguments:
    *  'num' - The index of string to retrive.
    
    * #Return Value:
    *   '(css_error,Option<lwc_string>)' - (CSS_BADPARM,None) if num param is not correct, 
    *                               else ( CSS_OK, option of the string. )
    */
    pub fn css__stylesheet_string_get(&self, num:uint) 
                                    -> (css_error,Option<uint>) {
        //debug!("Entering: css__stylesheet_string_get");

        if( (self.string_vector.len() < num) || (num == 0) ) {
            return (CSS_BADPARM,None) ;
        }
        ( CSS_OK, Some(self.string_vector[num-1]) )
    }

    #[inline]
    pub fn css__stylesheet_style_appendOPV(
                                        style: &mut ~css_style,
                                        opcode:css_properties_e,
                                        flags:u8,
                                        value:u16 ) {
        //debug!("Entering: css__stylesheet_style_appendOPV");
        css_stylesheet::css__stylesheet_style_append(
            style,
            buildOPV(opcode,flags,value)
        )
    }

    #[inline]
    pub fn css_stylesheet_style_inherit(
                                        style: &mut ~css_style,
                                        opcode:css_properties_e) {
        //debug!("Entering: css_stylesheet_style_inherit");

        css_stylesheet::css__stylesheet_style_append(
            style,
            buildOPV_flag(opcode,FLAG_INHERIT,0) 
        )
    }

    /**
    * Create a style, with sheet pointer set into the style
    
    * #Arguments:
    *  'self'  - css_stylesheet. 
    
    * #Return Value:
    *  'css_style' - css_style.
    */
    pub fn css__stylesheet_style_create(sheet : @mut css_stylesheet) -> ~css_style {
        ~css_style{ 
            bytecode:~[],
            used:0,
            sheet:Some(sheet)
        } 
    }

    /**
    * Merge a style to a CSS style
    * #Arguments:
    *  'target'  - The style to merge to. 
    *  'style'  - The style to merge. 
    */
    pub fn css__stylesheet_merge_style(target : &mut ~css_style, style: &mut ~css_style) {
        //debug!("Entering: css__stylesheet_merge_style");
        target.bytecode.push_all(style.bytecode);
    }

    /**
    * #Description:
    *   Append a style to a CSS style
    
    * #Arguments:
    *  'target'  - The style to add to. 
    *  'style'  - The style to add. 
    */
    pub fn css__stylesheet_style_append(target : &mut ~css_style, bytecode: u32) {
        //debug!("Entering: css__stylesheet_style_append");
        target.bytecode.push(bytecode);
    }
    
    /**
    * #Description:
    *   Append a style to a CSS style
    
    * #Arguments:
    *  'target'  - The style to add to. 
    *  'bytecodes'  - vector of style to add. 
    */
    pub fn css__stylesheet_style_vappend(target : &mut ~css_style, bytecodes: &[u32] ) {
        //debug!("Entering: css__stylesheet_style_vappend");
        target.bytecode.push_all(bytecodes);
    }

    /**
    * #Description:
    *   Create an element selector.
    
    * #Arguments:
    *  'qname' - Qualified name of selector.
    
    * #Return Value:
    *   'css_selector' - Pointer to box containing selector object.
    */
    pub fn css__stylesheet_selector_create(&mut self, lwc_ref:&mut ~lwc, qname : ~css_qname ) -> uint {
        //debug!("Entering: css__stylesheet_selector_create");
        //debug!(fmt!("css__stylesheet_selector_create:: qname == %?", qname));
        let mut sel = ~css_selector{  
            combinator:None, 
            rule:None, 
            specificity:{
                if self.inline_style {
                    CSS_SPECIFICITY_A
                }
                else if (lwc_ref.lwc_string_length(qname.name) != 1 || lwc_ref.lwc_string_data(qname.name).char_at(0) != '*') {
                    CSS_SPECIFICITY_D
                }
                else {
                    0u
                }
            },
            data:~[]
        };

        let sel_data = ~css_selector_detail{
            qname:qname,
            selector_type: CSS_SELECTOR_ELEMENT,
            combinator_type: CSS_COMBINATOR_NONE,
            value_type:CSS_SELECTOR_DETAIL_VALUE_STRING,
            negate:false,
            string: None,
            a:0,
            b:0
        };
        sel.data.push(sel_data);
        self.css_selectors_list.push(sel);
        self.css_selectors_list.len()-1
    }

    /**
    * #Description:
    *   Initialise a selector detail.
    
    * #Arguments:
    *  'sel_type' - The type of selector to create.
    *  'qname' - Qualified name of selector.
    *  'value_type' - type of the value.
    *  'string_value' - Option<@str>
    *  'ab_value' - Option<(i32,i32)>  css_selector_detail_value.
    *  'negate' - Whether the detail match should be negated.
    
    * #Return Value:
    *   '(css_error, Option<~css_selector_detail>)' - (CSS_OK,Some(css_selector_detail)).
    */
    pub fn css__stylesheet_selector_detail_init (
        sel_type: css_selector_type,
        qname : ~css_qname, 
        value_type : css_selector_detail_value_type,
        string_value : Option<uint> , 
        ab_value : Option<(i32,i32)>,
        negate:bool
    )  -> (css_error, Option<~css_selector_detail>) 
    {
        //debug!("Entering: css__stylesheet_selector_detail_init");
        let mut detail : ~css_selector_detail = ~css_selector_detail{
            qname:qname,
            selector_type:sel_type,
            combinator_type:CSS_COMBINATOR_NONE,  
            value_type:value_type,
            negate:negate,

            //css_selector_detail_value - union merged
            string:None,
            a:0,
            b:0
        };
        
        match value_type {
            CSS_SELECTOR_DETAIL_VALUE_STRING=>  {
                if string_value.is_some() {
                    detail.string=string_value ;
                }
            },
            CSS_SELECTOR_DETAIL_VALUE_NTH => 
                match ab_value { 
                    None=> {},
                    Some((x,y))=> { 
                                    detail.a=x ; 
                                    detail.b=y; 
                                  }
                }
        }
        (CSS_OK,Some(detail)) 
    }
    
    /**
    * #Description:
    *   Append a selector to the specifics chain of another selector.
    
    * #Arguments:
    *  'selector' - css_selector to which details get appended.
    *  'detail' - The css_selector_detail to be appended.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__stylesheet_selector_append_specific(&mut self, selector : uint, detail: ~css_selector_detail)  -> css_error  {
        //debug!("Entering: css__stylesheet_selector_append_specific");
        
        match detail.selector_type {
            CSS_SELECTOR_CLASS=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_C, 
            CSS_SELECTOR_PSEUDO_CLASS=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_C, 
            CSS_SELECTOR_ATTRIBUTE=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_C, 
            CSS_SELECTOR_ATTRIBUTE_EQUAL=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_C, 
            CSS_SELECTOR_ATTRIBUTE_DASHMATCH=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_C, 
            CSS_SELECTOR_ATTRIBUTE_INCLUDES=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_C, 
            CSS_SELECTOR_ATTRIBUTE_PREFIX=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_C, 
            CSS_SELECTOR_ATTRIBUTE_SUFFIX=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_C, 
            CSS_SELECTOR_ATTRIBUTE_SUBSTRING=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_C, 

            CSS_SELECTOR_ID=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_B ,
            
            CSS_SELECTOR_PSEUDO_ELEMENT=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_D ,
            CSS_SELECTOR_ELEMENT=> self.css_selectors_list[selector].specificity += CSS_SPECIFICITY_D 
        };

        self.css_selectors_list[selector].data.push(detail);
        CSS_OK
    }

    /**
    * #Description:
    *   Combine a pair of selectors.
    * For example, given A + B, the combinator field of B would point at A, 
    * with a combinator type of CSS_COMBINATOR_SIBLING. Thus, given B, we can
    * find its combinator. It is not possible to find B given A.
    
    * #Arguments:
    *  'combinator_type' - combinator types of selectors to be combined.
    *  'a' - css_selector.
    *  'b' - css_selector.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__stylesheet_selector_combine(&mut self, combinator_type : css_combinator, a : uint , 
                                            b : uint) -> css_error {
        
        //debug!("Entering: css__stylesheet_selector_combine");
        match self.css_selectors_list[b].combinator {
            Some(_)=> return CSS_INVALID,
            None=> {}
        };

        let mut counter_i = 0;
        while counter_i < self.css_selectors_list[a].data.len() {
            match self.css_selectors_list[a].data[counter_i].selector_type {
                CSS_SELECTOR_PSEUDO_ELEMENT => return CSS_INVALID ,
                _=> counter_i += 1
            };
        }

        self.css_selectors_list[b].combinator=Some(a);
        self.css_selectors_list[b].data[0].combinator_type=combinator_type;
        self.css_selectors_list[b].specificity += self.css_selectors_list[a].specificity;
        CSS_OK
    }

    /**
    * #Description:
    *   Create a CSS rule.
    
    * #Arguments:
    *  'rule_type' - The rule type.
    
    * #Return Value:
    *   'CSS_RULE_DATA_TYPE' - .
    */
    pub fn css_stylesheet_rule_create(&mut self, rule_type : css_rule_type ) -> CSS_RULE_DATA_TYPE  {

        //debug!("Entering: css_stylesheet_rule_create");
        
        let base_rule = ~css_rule{ 
            parent_rule:None,
            parent_stylesheet:false,
            next:None,
            prev:None,
            index:0
        };

        match rule_type {
            CSS_RULE_UNKNOWN=>  {   
                let ret_rule = ~css_rule{ 
                    parent_rule:None,
                    parent_stylesheet:false,
                    next:None,
                    prev:None,
                    index:0
                };
                self.css_rule_list.push(ret_rule);
                RULE_UNKNOWN(self.css_rule_list.len()-1) 
            },

            CSS_RULE_SELECTOR=> {   
                self.css_rule_list.push(base_rule);
                let ret_rule = @mut css_rule_selector{
                    base:self.css_rule_list.len()-1,
                    selectors:~[],
                    style:None
                };  
                RULE_SELECTOR(ret_rule)
            } ,


            CSS_RULE_CHARSET=>  {   
                self.css_rule_list.push(base_rule);
                let ret_rule = @mut css_rule_charset{
                    base:self.css_rule_list.len()-1,
                    encoding:~""
                };  
                RULE_CHARSET(ret_rule) 
            },

            CSS_RULE_IMPORT=>   {   
                self.css_rule_list.push(base_rule);
                let ret_rule = @mut css_rule_import{
                    base:self.css_rule_list.len()-1,
                    url:~"",
                    media:0,
                    sheet:None
                };  
                RULE_IMPORT(ret_rule) 
            },

            CSS_RULE_MEDIA=>    {   
                self.css_rule_list.push(base_rule);
                let ret_rule = @mut css_rule_media{ 
                    base:self.css_rule_list.len()-1,
                    media:0,
                    first_child:None,
                    last_child:None
                };  
                RULE_MEDIA(ret_rule) 
            },

            CSS_RULE_FONT_FACE=>{   
                self.css_rule_list.push(base_rule);
                let ret_rule = @mut css_rule_font_face{
                    base:self.css_rule_list.len()-1,
                    font_face:None
                };  
                RULE_FONT_FACE(ret_rule) 
            },

            CSS_RULE_PAGE=>     {   
                self.css_rule_list.push(base_rule);
                let ret_rule = @mut css_rule_page{
                    base:self.css_rule_list.len()-1,
                    selector:None,
                    style:None
                };  
                RULE_PAGE(ret_rule) 
            }

        }
    }

    /**
    * #Description:
    *   Add a selector to a CSS rule.
    
    * #Arguments:
    *  'css_rule' - The rule to which selector to be added.
    *  'selector' - The selector to be added.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__stylesheet_rule_add_selector(&mut self, css_rule : CSS_RULE_DATA_TYPE , selector : uint) -> css_error {
        //debug!("Entering: css__stylesheet_rule_add_selector");
        //debug!(fmt!("css__stylesheet_rule_add_selector:: selector == %?", selector));
        match css_rule {
            RULE_SELECTOR(x)=> {
                self.css_selectors_list[selector].rule = Some(css_rule);
                x.selectors.push(selector);
                CSS_OK
            },
            _=> CSS_BADPARM 
        }
    }
    
    /**
    * #Description:
    *   Append a style to a CSS rule.
    
    * #Arguments:
    *  'css_rule' - The rule to which style to be appended.
    *  'style' - The style to be appended.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__stylesheet_rule_append_style(&mut self, css_rule : CSS_RULE_DATA_TYPE , mut style :~css_style) -> css_error {
        //debug!("Entering: css__stylesheet_rule_append_style");
        match css_rule {
            RULE_PAGE(page)=> {
                if page.style.is_none() {
                    page.style = Some(style);
                }
                else {
                    css_stylesheet::css__stylesheet_merge_style(page.style.get_mut_ref(), &mut style);
                }
            },
            RULE_SELECTOR(selector)=> {
                if selector.style.is_none() {
                    selector.style = Some(style);
                }
                else {
                    css_stylesheet::css__stylesheet_merge_style(selector.style.get_mut_ref(), &mut style);
                }
            },
            _=> return CSS_BADPARM 
        };
        CSS_OK
    }

    /**
    * #Description:
    *   Set the encoding of a CSS rule.
    
    * #Arguments:
    *  'rule' - The rule whose encoding to be set to charset.
    *  'charset' - the charset to be set.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__stylesheet_rule_set_charset(css_rule : CSS_RULE_DATA_TYPE, charset: ~str) -> css_error {
        //debug!("Entering: css__stylesheet_rule_set_charset");
        
        if charset.len() <= 0 {
            return CSS_BADPARM;
        }

        match css_rule {
            RULE_CHARSET(x) => {
                x.encoding = charset;
                CSS_OK
            }
            _ => {
                CSS_BADPARM
            }
        }
    }

    /**
    * #Description:
    *   Set the necessary data to import a stylesheet associated with a rule.
    
    * #Arguments:
    *  'css_rule' - The rule whose data to be set.
    *  'url_str' - the url to be set.
    *  'media' - the media to be set.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__stylesheet_rule_set_nascent_import(
        css_rule : CSS_RULE_DATA_TYPE, url_str:&str, media:u64) -> css_error {
        //debug!("Entering: css__stylesheet_rule_set_nascent_import");
        match css_rule {
            RULE_IMPORT(x) => {
                x.url = url_str.to_owned();
                x.media=media;
                CSS_OK
            }
            _ => {
                CSS_BADPARM
            }
        }
    }

    /**
    * #Description:
    *   Set the media of an @media rule.
    
    * #Arguments:
    *  'css_rule' - The rule whose data to be set.
    *  'media' - the media to be set.
    
    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__stylesheet_rule_set_media(
        css_rule : CSS_RULE_DATA_TYPE, media:u64) -> css_error {
        //debug!("Entering: css__stylesheet_rule_set_media");
        match css_rule {
            RULE_MEDIA(x) => {
                x.media=media;
                CSS_OK
            }
            _ => {
                CSS_BADPARM
            }
        }
    }

    pub fn css__stylesheet_rule_set_page_selector(
        css_rule : CSS_RULE_DATA_TYPE, selector:uint) -> css_error {
        //debug!("Entering: css__stylesheet_rule_set_page_selector");
        match css_rule {
            RULE_PAGE(x) => {
                x.selector= Some(selector);
                CSS_OK
            }
            _ => {
                CSS_BADPARM
            }
        }
    }
    
    pub fn css__stylesheet_get_parent_type(&mut self, css_rule :  CSS_RULE_DATA_TYPE) -> CSS_RULE_PARENT_TYPE {
        
        let base_rule = css_stylesheet::css__stylesheet_get_base_rule(css_rule);

        if (self.css_rule_list[base_rule].parent_rule.is_some() && !self.css_rule_list[base_rule].parent_stylesheet) {
            return CSS_RULE_PARENT_RULE;
        }

        if (self.css_rule_list[base_rule].parent_rule.is_none() && self.css_rule_list[base_rule].parent_stylesheet) {
            return CSS_RULE_PARENT_STYLESHEET;
        }

        fail!(~"Parent type is ambiguous");
    }
    pub fn css__stylesheet_get_base_rule(css_rule : CSS_RULE_DATA_TYPE) -> uint {
        
        match css_rule {
            RULE_UNKNOWN(r) => {
                r
            },
            RULE_SELECTOR(r)=>{
                r.base
            },
            RULE_CHARSET(r)=>{
                r.base
            },
            RULE_IMPORT(r)=>{
                r.base
            },
            RULE_MEDIA(r)=>{
                r.base
            },
            RULE_FONT_FACE(r)=>{
                r.base
            },
            RULE_PAGE(r)=>{
                r.base
            },
        }
    }

    /**
    * #Description:
    *   Add a rule to a stylesheet.
   
   * #Arguments:
    *  'sheet' - The stylesheet to add to.
    *  'css_rule' - The rule to add.
    *  'parent_rule' - The parent rule, or None for a top-level rule.

    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__stylesheet_add_rule(sheet : @mut css_stylesheet,  lwc_ref:&mut ~lwc, css_rule : CSS_RULE_DATA_TYPE,
                                    parent_rule : Option<CSS_RULE_DATA_TYPE> ) -> css_error {
        
        //debug!("Entering: css__stylesheet_add_rule");
        let base_rule = css_stylesheet::css__stylesheet_get_base_rule(css_rule);

        sheet.css_rule_list[base_rule].index = sheet.rule_count;

        match sheet._add_selectors(lwc_ref, css_rule) {
            CSS_OK => {},
            x => return x
        }

        match parent_rule {
            Some(prule)=> {
                match prule {
                    RULE_MEDIA(media_rule)=>{
                        sheet.css_rule_list[base_rule].parent_rule = parent_rule;
                        sheet.rule_count += 1;
                        //let mut base_media_prule = css_stylesheet::css__stylesheet_get_base_rule(prule);


                        match media_rule.last_child {
                            None=>{
                                sheet.css_rule_list[base_rule].next = None;
                                sheet.css_rule_list[base_rule].prev = None;
                                media_rule.first_child = Some(css_rule);
                                media_rule.last_child = Some(css_rule);
                            },
                            Some(last_child)=>{
                                let last_child_base_rule = css_stylesheet::css__stylesheet_get_base_rule(last_child);
                                sheet.css_rule_list[last_child_base_rule].next = Some(css_rule);
                                sheet.css_rule_list[base_rule].prev = Some(last_child) ;
                                sheet.css_rule_list[base_rule].next = None;
                                media_rule.last_child = Some(css_rule);
                            }
                        }
                    },
                    _=> return CSS_INVALID
                }
            },
            None=>{
                sheet.css_rule_list[base_rule].parent_stylesheet = true;
                sheet.rule_count += 1 ;

                match sheet.last_rule {
                    None=>{
                        sheet.css_rule_list[base_rule].prev = None;
                        sheet.css_rule_list[base_rule].next = None;
                        sheet.rule_list = Some(css_rule);
                        sheet.last_rule = Some(css_rule);
                    },
                    Some(last_rule)=>{
                        let last_rule_base_rule = css_stylesheet::css__stylesheet_get_base_rule(last_rule);
                        sheet.css_rule_list[last_rule_base_rule].next = Some(css_rule);
                        sheet.css_rule_list[base_rule].prev = sheet.last_rule;
                        sheet.css_rule_list[base_rule].next = None;
                        sheet.last_rule = Some(css_rule);
                    }
                }
            }
        }
        CSS_OK
    }
    
    /**
    * #Description:
    *   Remove a rule from a stylesheet.

    * #Arguments:
    *  'sheet' - The sheet to remove from.
    *  'css_rule' - The rule to remove.

    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__stylesheet_remove_rule(sheet : @mut css_stylesheet,  lwc_ref:&mut ~lwc, css_rule : CSS_RULE_DATA_TYPE) 
                                        -> css_error {
        //debug!("Entering: css__stylesheet_remove_rule");
        match sheet._remove_selectors(lwc_ref, css_rule) {
            CSS_OK=>{},
            x =>return x 
        }

        let base_rule = css_stylesheet::css__stylesheet_get_base_rule(css_rule);
        match sheet.css_rule_list[base_rule].next {
            None=> {
                sheet.last_rule = sheet.css_rule_list[base_rule].prev;
            },
            Some(base_rule_next)=>{
                let next_rule = css_stylesheet::css__stylesheet_get_base_rule(base_rule_next);
                sheet.css_rule_list[next_rule].prev = sheet.css_rule_list[base_rule].prev;
            }
        }

        match sheet.css_rule_list[base_rule].prev {
            None=>{
                sheet.rule_list = sheet.css_rule_list[base_rule].next ;
            },
            Some(base_rule_prev)=>{
                let prev_rule = css_stylesheet::css__stylesheet_get_base_rule(base_rule_prev);
                sheet.css_rule_list[prev_rule].next = sheet.css_rule_list[base_rule].next ;
            }
        }
        sheet.css_rule_list[base_rule].parent_rule = None ;
        sheet.css_rule_list[base_rule].parent_stylesheet = false;
        sheet.css_rule_list[base_rule].next = None;
        sheet.css_rule_list[base_rule].prev = None ;
        CSS_OK
    }

    /**
    * #Description:
    *   Add selectors in a rule to the hash.

    * #Arguments:
    *  'css_rule' - Rule to consider.

    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn _add_selectors(&mut self, lwc_ref:&mut ~lwc, css_rule : CSS_RULE_DATA_TYPE) -> css_error {
        //debug!("Entering: _add_selectors");
        match css_rule {
            RULE_SELECTOR(x) => {
                if self.css_rule_list[x.base].parent_rule.is_some() || 
                        self.css_rule_list[x.base].parent_stylesheet {
                    return CSS_INVALID;
                }

                let mut i : int = 0 ;
                let length = x.selectors.len() as int;
                while (i< length ) {
                    match self.selectors.css__selector_hash_insert(self, lwc_ref, x.selectors[i]) {
                        CSS_OK=> { 
                            i += 1;
                            loop;
                        } ,
                        y => {
                            i -= 1;
                            while (i>=0){
                                // Ignore errors
                                self.selectors.css__selector_hash_remove(self, lwc_ref, x.selectors[i]);
                                i -= 1;
                            }
                            // Remove zeroth element
                            //self.selectors.css__selector_hash_remove(x.selectors[i]);
                            return y;
                        }
                    }
                }

                CSS_OK
            },
            RULE_MEDIA(x) => {
                if self.css_rule_list[x.base].parent_rule.is_some() || 
                        self.css_rule_list[x.base].parent_stylesheet {
                    return CSS_INVALID;
                }

                let mut ptr = x.first_child;
                loop {
                    match ptr {
                        None=> {
                            return CSS_OK
                        },
                        Some(current_rule) => {
                        
                            match self._add_selectors(lwc_ref, current_rule) {
                                CSS_OK=>{
                                    ptr = self.css_rule_list[css_stylesheet::css__stylesheet_get_base_rule(current_rule)].next;
                                    loop ;
                                },
                                x => {
                                    /* Failed, revert our changes */
                                    ptr = self.css_rule_list[css_stylesheet::css__stylesheet_get_base_rule(current_rule)].prev;
                                    loop {
                                        match ptr {
                                            None=>{
                                                return x ;
                                            }
                                            Some(prev_rule)=>{
                                                self._remove_selectors(lwc_ref, prev_rule) ;
                                                ptr = self.css_rule_list[css_stylesheet::css__stylesheet_get_base_rule(prev_rule)].prev;
                                                loop ;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            _ => {
                CSS_OK
            }
        }
    }

    /**
    * #Description:
    *   Remove selectors in a rule from the hash.

    * #Arguments:
    *  'css_rule' - Rule to consider.

    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn _remove_selectors(&mut self,  lwc_ref:&mut ~lwc, css_rule : CSS_RULE_DATA_TYPE) -> css_error {
        //debug!("Entering: _remove_selectors");
        match css_rule {
            RULE_SELECTOR(x) => {

                for &selector in x.selectors.mut_iter() {

                    match self.selectors.css__selector_hash_remove(self, lwc_ref, selector) {
                        CSS_OK=>{
                            loop;
                        },
                        x => { 
                            return x ;
                        } 
                    }
                }

                CSS_OK
            },

            RULE_MEDIA(x)=> {

                let mut ptr = x.first_child;
                loop {
                    match ptr {
                        None=> {
                            return CSS_OK ;
                        },
                        Some(base_rule)=>{
                            match self._remove_selectors(lwc_ref, base_rule) {
                                CSS_OK => {
                                    ptr = self.css_rule_list[css_stylesheet::css__stylesheet_get_base_rule(base_rule)].next;
                                },
                                x => { 
                                    return x ;
                                }
                            }
                        }
                    }
                }
            },
            _=>{CSS_OK}
        }
    }
}

/////////////////////////////////////////////////////
//          Implementation of css/src/select/hash.c
/////////////////////////////////////////////////////

pub enum css_hash_type {
    Element = 1,
    Class = 2,
    Ids = 3,
    Universal = 4
}

impl css_selector_hash {

    /**
    * #Description:
    *  Create a hash.

    * #Return Value:
    *  'css_selector_hash' - Hash table of selectors.
    */
    pub fn css__selector_hash_create() -> @mut css_selector_hash {
        //debug!("Entering: css__selector_hash_create");
        let hash = @mut css_selector_hash{ 
                        default_slots:(1<<6),
                        elements:~[], 
                        classes:~[], 
                        ids:~[],
                        universal:~[],
        };
        let size = hash.default_slots as uint;
        hash.elements.reserve(size);
		hash.classes.reserve(size);
        hash.ids.reserve(size);
        hash.universal.reserve(size);
		
		let mut i = 0;
		while i < size {
				hash.elements.push(None);
				hash.classes.push(None);
				hash.ids.push(None);
				hash.universal.push(None);
			        i = i + 1;
        }
        hash
    }
    
    /**
    * #Description:
    *  Retrieve the first class name in a selector, or empty if none.

    * #Arguments:
    *  'selector '  - Selector to consider. 

    * #Return Value:
    *  '@str' - class name.
    */

    #[inline]
    pub fn _class_name(&mut self , sheet: &mut css_stylesheet, lwc_ref:&mut ~lwc, selector : uint) 
                        -> uint {

        let mut counter_i = 0;
        while counter_i < sheet.css_selectors_list[selector].data.len() {
            match sheet.css_selectors_list[selector].data[counter_i].selector_type {
                CSS_SELECTOR_CLASS=>{
                    if (sheet.css_selectors_list[selector].data[counter_i].negate == false) {
                        return sheet.css_selectors_list[selector].data[counter_i].qname.name;
                    }
                    counter_i +=1
                },
                _=> counter_i +=1
            }
        }

        lwc_ref.lwc_intern_string("")
    }

    /**
    * #Description:
    *  Retrieve the first ID name in a selector, or empty if none.

    * #Arguments:
    *  'selector '  - Selector to consider. 

    * #Return Value:
    *  '@str' - ID name.
    */

    #[inline]
    pub fn _id_name(&mut self, sheet:&mut css_stylesheet, lwc_ref:&mut ~lwc, selector : uint) 
                        -> uint {

        let mut counter_i = 0;
        while counter_i <  sheet.css_selectors_list[selector].data.len() {
            match sheet.css_selectors_list[selector].data[counter_i].selector_type {
                CSS_SELECTOR_ID=>{
                    if (sheet.css_selectors_list[selector].data[counter_i].negate == false) {
                        return sheet.css_selectors_list[selector].data[counter_i].qname.name;
                    }
                    counter_i +=1
                },
                _=>counter_i +=1
            }
        }

       lwc_ref.lwc_intern_string("")
    }


    /**
    * #Description:
    *  Name hash function -- case-insensitive FNV.

    * #Arguments:
    *  'name '  - Name to hash. 

    * #Return Value:
    *  'uint' - hash value.
    */

    #[inline]
    pub fn _hash_name(_string: uint, lwc_ref:&mut ~lwc ) -> u32 {
        //debug!("Entering _hash_name");
        let mut z: u32 = 0x811c9dc5;
        let mut i: uint = 0;
        let string = lwc_ref.lwc_string_data(_string);
        let string_index = string.char_len();
        while i<string_index {
            z *= 0x01000193;
            z ^= string[i] as u32 & !0x20;
            i = i+1; 
        }
        //z = z%4091;
        z
    }
    
    /**
    * #Description:
    *  Insert an item into the hash table.

    * #Arguments:
    *  'selector'  - css selector. 

    * #Return Value:
    *  'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__selector_hash_insert(&mut self, sheet:&mut css_stylesheet, lwc_ref:&mut ~lwc, selector : uint) 
                                    -> css_error {
        //debug!("Entering: css__selector_hash_insert");

        let mut mask :u32 ;
        let mut index:u32=0;
        let mut name :uint ;
        if (sheet.css_selectors_list[selector].data.len() > 0) {
            let class_lwc_string = self._class_name(sheet, lwc_ref, selector);
            let id_lwc_string = self._id_name(sheet, lwc_ref, selector);
            // Named Element
            if ( lwc_ref.lwc_string_length(sheet.css_selectors_list[selector].data[0].qname.name) != 1) || 
                (lwc_ref.lwc_string_data(sheet.css_selectors_list[selector].data[0].qname.name).char_at(0) != '*' ) {
                    //debug!("Entering: css__selector_hash_insert:: Named Element");
                    mask = self.default_slots-1 ;
                    index = css_selector_hash::_hash_name(sheet.css_selectors_list[selector].data[0].qname.name, lwc_ref) & mask ;
                    return self._insert_into_chain(sheet,Element,index,selector);
            }

            // Named Class
            else if lwc_ref.lwc_string_length(class_lwc_string) != 0  {
                //debug!("Entering: css__selector_hash_insert:: Named Class");
                name = self._class_name(sheet, lwc_ref, selector);
                mask = self.default_slots-1 ;
                index = css_selector_hash::_hash_name(name, lwc_ref) & mask ;
                return self._insert_into_chain(sheet,Class,index,selector);
            }

            // Named Id
            else if lwc_ref.lwc_string_length(id_lwc_string) != 0 {
                //debug!("Entering: css__selector_hash_insert:: Named Id");
                name = self._id_name(sheet, lwc_ref, selector);
                mask = self.default_slots-1 ;
                index = css_selector_hash::_hash_name(name, lwc_ref) & mask ;
                return self._insert_into_chain(sheet,Ids,index,selector);
            }
            else {
                //debug!("Entering: css__selector_hash_insert:: else Universal");
                return self._insert_into_chain(sheet,Universal,index,selector);
            }
        }
        // Universal Chain
        //debug!("Entering: css__selector_hash_insert:: Universal Chain");
        return self._insert_into_chain(sheet, Universal,index,selector);
    }

    
    /**
    * #Description:
    *  Insert a selector into a hash chain.

    * #Arguments:
    *  'hash_type'  - hash type. 
    *  'index'  - index of insertion. 
    *  'selector'  - selector to be inserted. 

    * #Return Value:
    *  'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn _insert_into_chain(&mut self, sheet:&mut css_stylesheet,
                            hash_type : css_hash_type,
                            index:u32,
                            selector : uint) 
                            -> css_error {
        //debug!("Entering: _insert_into_chain");
        //debug!("_insert_into_chain:: hash_type == %?, index == %?", hash_type, index);
        let mut hash_entry_list : &mut ~[Option<@mut hash_entry>];

        match hash_type {
            Element => {hash_entry_list = &mut self.elements} ,
            Class => {hash_entry_list = &mut self.classes} ,
            Ids =>  {hash_entry_list = &mut self.ids} ,
            Universal => {hash_entry_list = &mut self.universal} ,
        };

        let entry = @mut hash_entry {
                selector:selector,
                next:None
        };
        //&~[Option<@mut hash_entry>] 

        match hash_entry_list[index] {
            None=> {
                //debug!("Entering: match (*hash_entry_list)[index] => None");
                hash_entry_list[index] = Some(entry);
                //debug!("(*hash_entry_list)[index] == %?", (*hash_entry_list)[index]);
            },
            Some(index_element)=> {
                //debug!("Entering: match (*hash_entry_list)[index] => Some(index_element)");
                let mut search = index_element;
                let mut prev = index_element ;
                let mut first_pos : bool = true ;
                loop {

                    if (selector == search.selector) {
                        // duplicate insert of same pointer css_selector should never occur,
                        // added , due to logical incompatibilty with "_remove_into_chain"
                        // in origical code , _remove_into_chain removes by comparing pointer values,
                        // and freeing the final result , by doing reallocation of 0 bytes ( line num : 650-671 , hash.c)
                        //debug!("_insert_into_chain : error: double insertion of same selector ") ;
                        return CSS_BADPARM;
                    }

                    if sheet.css_selectors_list[search.selector].specificity> sheet.css_selectors_list[selector].specificity {
                        break ;
                    }

                    if sheet.css_selectors_list[search.selector].specificity == sheet.css_selectors_list[selector].specificity {
                        if(sheet.css_selectors_list[search.selector].rule.is_none() || sheet.css_selectors_list[selector].rule.is_none() ){
                            //debug!("_insert_into_chain : error : rule is none  ") ;
                            return CSS_BADPARM ;
                        }

                        let base_search_rule = css_stylesheet::css__stylesheet_get_base_rule(sheet.css_selectors_list[search.selector].rule.expect(""));
                        let base_selector_rule = css_stylesheet::css__stylesheet_get_base_rule(sheet.css_selectors_list[selector].rule.expect(""));

                        if sheet.css_rule_list[base_search_rule].index > sheet.css_rule_list[base_selector_rule].index {
                            break ;
                        }
                    }

                    prev = search ;
                    first_pos = false ;
                    search = 
                        match search.next {
                            None=>{
                                break ;
                            },
                            Some(next_ptr)=>{
                                next_ptr
                            }
                    };
                }
                if(first_pos){
                    //debug!("Entering: _insert_into_chain:: if(first_pos)");
                    entry.next = Some(index_element);
                    hash_entry_list[index] = Some(entry);
                }
                else {
                    //debug!("Entering: _insert_into_chain:: if(first_pos)--else");
                    entry.next=prev.next;
                    prev.next= Some(entry);
                }
            }
        }
        //debug!("_insert_into_chain : after insertion list is hash_type=%?= index=%?=",hash_type,index) ;
        //css_selector_hash:://debug_print_hash_entry_list((*hash_entry_list)[index]) ;
        CSS_OK
    }

    /**
    * #Description:
    *  Remove an item from a hash.

    * #Arguments:
    *  'selector'  - css selector. 

    * #Return Value:
    *  'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__selector_hash_remove(&mut self, sheet:&mut css_stylesheet, lwc_ref:&mut ~lwc, selector : uint) 
                                    -> css_error {
        let mut mask :u32 ;
        let mut index:u32=0;
        let mut name : uint ;
        if (sheet.css_selectors_list[selector].data.len() > 0){
            let class_lwc_string = self._class_name(sheet, lwc_ref, selector);
            let id_lwc_string = self._id_name(sheet, lwc_ref, selector);
            // Named Element
            if ( lwc_ref.lwc_string_length(sheet.css_selectors_list[selector].data[0].qname.name) != 1) || 
                (lwc_ref.lwc_string_data(sheet.css_selectors_list[selector].data[0].qname.name).char_at(0) != '*' ) {
                    mask = self.default_slots-1 ;
                    index = css_selector_hash::_hash_name(sheet.css_selectors_list[selector].data[0].qname.name, lwc_ref) & mask ;
                    return self._remove_from_chain(Element,index,selector);
            }

            // Named Class
            else if lwc_ref.lwc_string_length(class_lwc_string) == 0  {
                name = self._class_name(sheet, lwc_ref, selector);
                mask = self.default_slots-1 ;
                index = css_selector_hash::_hash_name(name, lwc_ref) & mask ;
                return self._remove_from_chain(Class,index, selector);
            }

            // Named Id
            else if lwc_ref.lwc_string_length(id_lwc_string) == 0 {
                name = self._id_name(sheet, lwc_ref, selector);
                mask = self.default_slots-1 ;
                index = css_selector_hash::_hash_name(name, lwc_ref) & mask ;
                return self._remove_from_chain(Ids,index,selector);
            }
            else {
                return self._remove_from_chain(Universal,index,selector);
            }
        }
        // Universal Chain
        return self._remove_from_chain(Universal,index,selector);
    }

    /**
    * #Description:
    *  Remove a selector from a hash chain.

    * #Arguments:
    *  'hash_type'  - hash type. 
    *  'head'  - Head of chain to remove from. 
    *  'selector'  - selector to remove. 

    * #Return Value:
    *  'css_error' - CSS_OK on success, CSS_INVALID  if selector not found in chain.
    */
    pub fn _remove_from_chain(&mut self, 
                            hash_type : css_hash_type,
                            index:u32,
                            selector : uint) 
                            -> css_error {

        let mut hash_entry_list : &mut ~[Option<@mut hash_entry>];

        match hash_type {
            Element => {hash_entry_list = &mut self.elements} ,
            Class => {hash_entry_list = &mut self.classes} ,
            Ids =>  {hash_entry_list = &mut self.ids} ,
            Universal => {hash_entry_list = &mut self.universal} ,
        };
        //&~[Option<@mut hash_entry>] 

        match hash_entry_list[index] {
            None=>{
                return CSS_INVALID ;
            },
            Some(index_element)=>{

                let mut search = index_element;
                let mut prev = index_element ;
                let mut first_pos : bool = true ;

                loop {

                    if (selector == search.selector) {
                        break;
                    }

                    first_pos = false ;
                    search = 
                        match search.next {
                            None=>{
                                return CSS_INVALID ;
                            },
                            Some(next_ptr)=>{
                                prev = search ;
                                next_ptr
                            }
                    };
                }
                if(first_pos){
                    hash_entry_list[index] = search.next;
                }
                else {
                    prev.next= search.next;
                }
            }
        }
        CSS_OK
    }

    /**
    * #Description:
    *  Find the first selector that matches name.

    * #Arguments:
    *  'name'  - name to find. 

    * #Return Value:
    *  '(Option<@mut hash_entry>,css_error)' - (Some(hash_entry),CSS_OK) on success, otherwise (None, CSS_OK).
    */
    pub fn css__selector_hash_find(&mut self, sheet:@mut css_stylesheet, lwc_ref:&mut ~lwc, name : uint) -> (Option<@mut hash_entry>,css_error) {
        //debug!("Entering: css__selector_hash_find");
        let mask  = self.default_slots-1 ;
        let index = css_selector_hash::_hash_name(name, lwc_ref) & mask ; 
        let mut head = self.elements[index];

        //debug!(fmt!("css__selector_hash_find:: name=%?  mask=%?, index=%? ", name, mask, index ));
        loop {
            match head {
                None=>{
                    return (None,CSS_OK);
                },
                Some(node_element)=>{

                    if lwc_ref.lwc_string_caseless_isequal(
                                sheet.css_selectors_list[node_element.selector].data[0].qname.name,name)  {
                                //debug!("Exiting: css__selector_hash_find (1)");
                                return (head,CSS_OK);
                    }

                    match node_element.next {
                        None=> {
                            //debug!("Exiting: css__selector_hash_find (2)");
                            return (None,CSS_OK);
                        },
                        Some(_)=>{
                            head = node_element.next ;
                            loop ;
                        }
                    }
                }
            }
        }
    }
    

    /**
    * #Description:
    *  Find the first selector that has a class that matches name.

    * #Arguments:
    *  'name'  - name to find. 

    * #Return Value:
    *  '(Option<@mut hash_entry>,css_error)' - (Some(hash_entry),CSS_OK) on success, otherwise (None, CSS_OK).
    */
    pub fn css__selector_hash_find_by_class(&mut self, sheet:@mut css_stylesheet, lwc_ref:&mut ~lwc, name : uint) -> (Option<@mut hash_entry>,css_error) {

        let mask  = self.default_slots-1 ;
        let index = css_selector_hash::_hash_name(name, lwc_ref) & mask ; 
        let mut head = self.classes[index];

        //debug!(fmt!("name=%?  mask=%?, index=%? ", name, mask, index ));
        loop {
            match head {
                None=>{
                    return (None,CSS_OK);
                },
                Some(node_element)=>{

                    let n = self._class_name(sheet, lwc_ref, node_element.selector);

                    if lwc_ref.lwc_string_caseless_isequal(n, name) {
                        return (head,CSS_OK);
                    }

                    match node_element.next {
                        None=> {
                            return (None,CSS_OK);
                        },
                        Some(_)=>{
                            head = node_element.next ;
                            loop ;
                        }
                    }
                }
            }
        }
    }

    /**
    * #Description:
    *  Find the first selector that has an ID that matches name.

    * #Arguments:
    *  'name'  - name to find. 

    * #Return Value:
    *  '(Option<@mut hash_entry>,css_error)' - (Some(hash_entry),CSS_OK) on success, otherwise (None, CSS_OK).
    */
    pub fn css__selector_hash_find_by_id(&mut self, sheet:@mut css_stylesheet, lwc_ref:&mut ~lwc, name : uint) -> (Option<@mut hash_entry>,css_error) {

        let mask  = self.default_slots-1 ;
        let index = css_selector_hash::_hash_name(name, lwc_ref) & mask ; 
        let mut head = self.ids[index];

        loop {
            match head {
                None=>{
                    return (None,CSS_OK);
                },
                Some(node_element)=>{

                    let n = self._id_name(sheet, lwc_ref, node_element.selector);

                    if lwc_ref.lwc_string_caseless_isequal(n, name) {
                        return (head,CSS_OK);
                    }

                    match node_element.next {
                        None=> {
                            return (None,CSS_OK);
                        },
                        Some(_)=>{
                            head = node_element.next ;
                            loop ;
                        }
                    }
                }
            }
        }
    }


    /**
    * #Description:
    *  Find the first universal selector.

    * #Return Value:
    *  '(Option<@mut hash_entry>,css_error)' - (Some(hash_entry),CSS_OK) on success, otherwise (None, CSS_OK).
    */
    pub fn css__selector_hash_find_universal(&mut self) -> (Option<@mut hash_entry>,css_error) {

        let head = self.universal[0] ;
        match head {
            None=>{
                return (None,CSS_OK);
            },
            Some(_)=>{
                return (self.universal[0],CSS_OK);
            }
        }
    }

    /**
    * #Description:
    *  Find the next selector that matches.

    * #Arguments:
    *  'current'  - Current item. 

    * #Return Value:
    *  '(Option<@mut hash_entry>,css_error)' - (box to receive next item,CSS_OK) on success, otherwise (None, CSS_OK).
    */
    pub fn _iterate_elements(&mut self , sheet:@mut css_stylesheet, lwc_ref:&mut ~lwc, current : @mut hash_entry) -> (Option<@mut hash_entry>,css_error) {

        let mut head = current;

        loop {
            match head.next {
                None=>{
                    return (None,CSS_OK);
                },
                Some(next_entry)=>{
                    if sheet.css_selectors_list[head.selector].data.len()==0 || 
                        sheet.css_selectors_list[next_entry.selector].data.len()==0 {
                        return (None,CSS_INVALID);
                    }
                    if lwc_ref.lwc_string_caseless_isequal(
                        sheet.css_selectors_list[current.selector].data[0].qname.name,
                        sheet.css_selectors_list[next_entry.selector].data[0].qname.name) {

                        return (head.next,CSS_OK);
                    }
                    head = next_entry ;
                    loop ;
                }
            }
        }
    }

    /**
    * #Description:
    *  Find the next selector that matches.

    * #Arguments:
    *  'current'  - Current item. 

    * #Return Value:
    *  '(Option<@mut hash_entry>,css_error)' - (box to receive next item,CSS_OK) on success, otherwise (None, CSS_OK).
    */
    pub fn _iterate_classes(&mut self , sheet:@mut css_stylesheet, lwc_ref:&mut ~lwc, current : @mut hash_entry) -> (Option<@mut hash_entry>,css_error) {

        let mut head = current;

        let current_refer = self._class_name(sheet, lwc_ref, current.selector);

        loop {
            match head.next {
                None=>{
                    return (None,CSS_OK);
                },
                Some(next_entry)=>{
                    let name = self._class_name(sheet, lwc_ref, next_entry.selector);
                    if( lwc_ref.lwc_string_length(name)==0){
                        loop;
                    }
                    if  lwc_ref.lwc_string_caseless_isequal(name,current_refer) {
                        return (current.next,CSS_OK);
                    }
                    head = next_entry ;
                    loop ;
                }
            }
        }
    }

    /**
    * #Description:
    *  Find the next selector that matches.

    * #Arguments:
    *  'current'  - Current item. 

    * #Return Value:
    *  '(Option<@mut hash_entry>,css_error)' - (box to receive next item,CSS_OK) on success, otherwise (None, CSS_OK).
    */
    pub fn _iterate_ids(&mut self , sheet: @mut css_stylesheet, lwc_ref:&mut ~lwc, current : @mut hash_entry) -> (Option<@mut hash_entry>,css_error) {

        let mut head = current;

        let current_refer = self._id_name(sheet, lwc_ref, current.selector);

        loop {
            match head.next {
                None=>{
                    return (None,CSS_OK);
                },
                Some(next_entry)=>{
                    let name = self._id_name(sheet, lwc_ref, next_entry.selector);
                    if( lwc_ref.lwc_string_length(name)==0){
                        loop;
                    }
                    if lwc_ref.lwc_string_caseless_isequal(name,current_refer)  {
                        return (current.next,CSS_OK);
                    }
                    head = next_entry ;
                    loop ;
                }
            }
        }
    }

    /**
    * #Description:
    *  Find the next selector that matches.

    * #Arguments:
    *  'current'  - Current item. 

    * #Return Value:
    *  '(Option<@mut hash_entry>,css_error)' - (box to receive next item,CSS_OK) on success, otherwise (None, CSS_OK).
    */
    pub fn _iterate_universal(current : @mut hash_entry) -> (Option<@mut hash_entry>,css_error) {

        if current.next.is_some() {
            return (current.next,CSS_OK);
        }
        (None,CSS_OK)
    }

    pub fn debug_print_vector_of_hash_entry_list(hash_vec : &[Option<@mut hash_entry>]) {

        for &entry in hash_vec.iter() {
            css_selector_hash::debug_print_hash_entry_list(entry) ;
        }
    }

    pub fn debug_print_hash_entry_list(current : Option<@mut hash_entry>) {

        //debug!("Starting Printing hash_entry linked list ======");
        let mut ptr = current ;
        loop {
            match ptr {
                None=>{ 
                    //debug!("None Encountered");
                    //debug!("Ending Printing hash_entry linked list ======");
                    return ;
                },
                Some(x)=>{
                    //debug!("Selector:specificity=%?=,data=%?=",x.selector.specificity,x.selector.data);
                    ptr = x.next ;
                }
            }
        }
    }
}


///////////////////////////////////////////////////////////////////////////////////////
