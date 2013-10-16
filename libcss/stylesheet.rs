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
static  ELEMENT : uint = 0 ;
static  CLASSES : uint = 1 ;
static  IDS : uint = 2 ;
static  UNIVERSAL : uint = 3 ;

pub type css_rule_data_index = uint;

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
    rule:Option<css_rule_data_index>,        /**< Owning rule */
    specificity:uint,                       /**< Specificity of selector */ 
    data:~[~css_selector_detail]        /* *< Selector data */
}


pub struct css_style {
    bytecode:~[u32],
    used:uint,                
    sheet:Option<uint>
}

type hash_entry_index = uint;

pub struct hash_entry {
    selector:uint,
    next:Option<hash_entry_index>
}

/**< Hashtable of selectors */
pub struct css_selector_hash {
    default_slots:u32,
    ele_class_ids_univ:~[~[Option<hash_entry_index>]],
    hash_entry_list:~[~hash_entry]
}

pub struct css_stylesheet {
    selectors:~css_selector_hash,       /**< Hashtable of selectors */
    rule_count:uint,                        /**< Number of rules in sheet */
    rule_list:Option<css_rule_data_index>,   /**< List of rules in sheet */
    last_rule:Option<css_rule_data_index>,   /**< Last rule in list */
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
    parent_rule:Option<css_rule_data_index> ,         /**< containing parent rule */ 
    parent_stylesheet:bool,   /**< parent stylesheet */              
    prev:Option<css_rule_data_index>,                 /**< prev in list */
    next:Option<css_rule_data_index>,                /**< next in list */
    //rule_type:CSS_RULE_TYPE,
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
    first_child:Option<css_rule_data_index>,                
    last_child:Option<css_rule_data_index>                
} 

pub struct css_rule_font_face {
    base:uint,
    font_face:Option<~css_font_face>
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
    sheet:Option<uint>
} 
pub struct css_rule_charset {
    base:uint,
    encoding:~str   
} 


pub struct css_rule_data_type {
    rule_type:CSS_RULE_TYPE,
    rule_unknown: uint,
    rule_selector: Option<~css_rule_selector>,
    rule_charset: Option<~css_rule_charset>,
    rule_import: Option<~css_rule_import>,
    rule_media: Option<~css_rule_media>,
    rule_font_face: Option<~css_rule_font_face>,
    rule_page: Option<~css_rule_page>
}

pub enum CSS_RULE_PARENT_TYPE {
    CSS_RULE_PARENT_STYLESHEET,
    CSS_RULE_PARENT_RULE
}

pub enum CSS_RULE_TYPE {
    CSS_RULE_UNKNOWN,
    CSS_RULE_SELECTOR,
    CSS_RULE_CHARSET,
    CSS_RULE_IMPORT,
    CSS_RULE_MEDIA,
    CSS_RULE_FONT_FACE,
    CSS_RULE_PAGE
}


pub fn compare_CSS_RULE_TYPEs(rule : Option<& ~css_rule_data_type>, rule_type : CSS_RULE_TYPE) -> bool{
    
    match rule {
        None => false,
        Some(T) => {
            T.rule_type as uint == rule_type as uint
        }
    }
}


pub fn compare_css_rdt(rule1: Option<css_rule_data_index>, rule2: Option<css_rule_data_index>) -> bool{
    
    match rule1 {
        None => {
            match rule2 {
                None => true,
                Some(_) => false,
            }
        },

        Some(T1) => {
            match rule2{
                None => false,
                Some(T2) =>{
                    T1 == T2
                }
            }
        }
    }
}

impl css_stylesheet {

    pub fn get_css_rule_next(&mut self, css_rule_data_list:&mut ~[~css_rule_data_type], rule: css_rule_data_index) -> Option<css_rule_data_index> {
        match css_rule_data_list[rule].rule_type {
            CSS_RULE_UNKNOWN => self.css_rule_list[css_rule_data_list[rule].rule_unknown].next,
            CSS_RULE_SELECTOR => self.css_rule_list[css_rule_data_list[rule].rule_selector.get_ref().base].next,
            CSS_RULE_CHARSET => self.css_rule_list[css_rule_data_list[rule].rule_charset.get_ref().base].next,
            CSS_RULE_IMPORT => self.css_rule_list[css_rule_data_list[rule].rule_import.get_ref().base].next,
            CSS_RULE_MEDIA => self.css_rule_list[css_rule_data_list[rule].rule_media.get_ref().base].next,
            CSS_RULE_FONT_FACE => self.css_rule_list[css_rule_data_list[rule].rule_font_face.get_ref().base].next,
            CSS_RULE_PAGE => self.css_rule_list[css_rule_data_list[rule].rule_page.get_ref().base].next,
        }
    }

    pub fn get_stylesheet_parent(&mut self, css_rule_data_list:&mut ~[~css_rule_data_type], rule: css_rule_data_index) -> bool {
        
        match css_rule_data_list[rule].rule_type {
            CSS_RULE_UNKNOWN => self.css_rule_list[css_rule_data_list[rule].rule_unknown].parent_stylesheet,
            CSS_RULE_SELECTOR => self.css_rule_list[css_rule_data_list[rule].rule_selector.get_mut_ref().base].parent_stylesheet,
            CSS_RULE_CHARSET => self.css_rule_list[css_rule_data_list[rule].rule_charset.get_mut_ref().base].parent_stylesheet,
            CSS_RULE_IMPORT => self.css_rule_list[css_rule_data_list[rule].rule_import.get_mut_ref().base].parent_stylesheet,
            CSS_RULE_MEDIA => self.css_rule_list[css_rule_data_list[rule].rule_media.get_mut_ref().base].parent_stylesheet,
            CSS_RULE_FONT_FACE => self.css_rule_list[css_rule_data_list[rule].rule_font_face.get_mut_ref().base].parent_stylesheet,
            CSS_RULE_PAGE => self.css_rule_list[css_rule_data_list[rule].rule_page.get_mut_ref().base].parent_stylesheet,
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
    pub fn css__stylesheet_style_create(sheet : uint) -> ~css_style {
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
    *   '~css_rule_data_type' - .
    */
    pub fn css_stylesheet_rule_create(&mut self, css_rule_data_list:&mut ~[~css_rule_data_type], rule_type : CSS_RULE_TYPE ) -> uint  {

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
                // let ret_rule = ~css_rule{ 
                //     parent_rule:None,
                //     parent_stylesheet:false,
                //     next:None,
                //     prev:None,
                //     index:0
                // };
                self.css_rule_list.push(base_rule);
                
                let rule = ~css_rule_data_type{
                    rule_type:CSS_RULE_UNKNOWN,
                    rule_unknown:self.css_rule_list.len()-1,
                    rule_selector: None,
                    rule_charset: None,
                    rule_import: None,
                    rule_media: None,
                    rule_font_face: None,
                    rule_page: None
                };    

                css_rule_data_list.push(rule);
                css_rule_data_list.len()-1

            },

            CSS_RULE_SELECTOR=> {   
                self.css_rule_list.push(base_rule);
                let ret_rule = ~css_rule_selector{
                    base:self.css_rule_list.len()-1,
                    selectors:~[],
                    style:None
                };  
                
                let rule = ~css_rule_data_type{
                    rule_type:CSS_RULE_SELECTOR,
                    rule_unknown:0,
                    rule_selector: Some(ret_rule),
                    rule_charset: None,
                    rule_import: None,
                    rule_media: None,
                    rule_font_face: None,
                    rule_page: None
                };

                css_rule_data_list.push(rule);
                css_rule_data_list.len()-1
                
            },


            CSS_RULE_CHARSET=>  {   
                self.css_rule_list.push(base_rule);
                let ret_rule = ~css_rule_charset{
                    base:self.css_rule_list.len()-1,
                    encoding:~""
                };  
                
                let rule = ~css_rule_data_type{
                    rule_type:CSS_RULE_CHARSET,
                    rule_unknown:0,
                    rule_selector: None,
                    rule_charset: Some(ret_rule),
                    rule_import: None,
                    rule_media: None,
                    rule_font_face: None,
                    rule_page: None
                };

                css_rule_data_list.push(rule);
                css_rule_data_list.len()-1

                
            },

            CSS_RULE_IMPORT=>   {   
                self.css_rule_list.push(base_rule);
                let ret_rule = ~css_rule_import{
                    base:self.css_rule_list.len()-1,
                    url:~"",
                    media:0,
                    sheet:None
                };  
                
                let rule = ~css_rule_data_type{
                    rule_type:CSS_RULE_IMPORT,
                    rule_unknown:0,
                    rule_selector: None,
                    rule_charset: None,
                    rule_import: Some(ret_rule),
                    rule_media: None,
                    rule_font_face: None,
                    rule_page: None
                };

                css_rule_data_list.push(rule);
                css_rule_data_list.len()-1
                 
            },

            CSS_RULE_MEDIA=>    {   
                self.css_rule_list.push(base_rule);
                let ret_rule = ~css_rule_media{ 
                    base:self.css_rule_list.len()-1,
                    media:0,
                    first_child:None,
                    last_child:None
                };  
                
                let rule = ~css_rule_data_type{
                    rule_type:CSS_RULE_MEDIA,
                    rule_unknown:0,
                    rule_selector: None,
                    rule_charset: None,
                    rule_import: None,
                    rule_media: Some(ret_rule),
                    rule_font_face: None,
                    rule_page: None
                }; 

                css_rule_data_list.push(rule);
                css_rule_data_list.len()-1

            },

            CSS_RULE_FONT_FACE=>{   
                self.css_rule_list.push(base_rule);
                let ret_rule = ~css_rule_font_face{
                    base:self.css_rule_list.len()-1,
                    font_face:None
                };  
                
                let rule = ~css_rule_data_type{
                    rule_type:CSS_RULE_FONT_FACE,
                    rule_unknown:0,
                    rule_selector: None,
                    rule_charset: None,
                    rule_import: None,
                    rule_media: None,
                    rule_font_face: Some(ret_rule),
                    rule_page: None
                };

                css_rule_data_list.push(rule);
                css_rule_data_list.len()-1
                
            },

            CSS_RULE_PAGE=>     {   
                self.css_rule_list.push(base_rule);
                let ret_rule = ~css_rule_page{
                    base:self.css_rule_list.len()-1,
                    selector:None,
                    style:None
                };  
                
                let rule = ~css_rule_data_type{
                    rule_type:CSS_RULE_PAGE,
                    rule_unknown:0,
                    rule_selector: None,
                    rule_charset: None,
                    rule_import: None,
                    rule_media: None,
                    rule_font_face: None,
                    rule_page: Some(ret_rule)
                }; 

                css_rule_data_list.push(rule);
                css_rule_data_list.len()-1

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
    pub fn css__stylesheet_rule_add_selector(&mut self, css_rule_data_list:&mut ~[~css_rule_data_type], css_rule : css_rule_data_index , selector : uint) -> css_error {
        //debug!("Entering: css__stylesheet_rule_add_selector");
        //debug!(fmt!("css__stylesheet_rule_add_selector:: selector == %?", selector));
        match css_rule_data_list[css_rule].rule_type {
            CSS_RULE_SELECTOR=> {
                self.css_selectors_list[selector].rule = Some(css_rule);
                css_rule_data_list[css_rule].rule_selector.get_mut_ref().selectors.push(selector);
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
    pub fn css__stylesheet_rule_append_style(&mut self, css_rule :&mut  ~css_rule_data_type , mut style :~css_style) -> css_error {
        //debug!("Entering: css__stylesheet_rule_append_style");
        match css_rule.rule_type {
            CSS_RULE_PAGE=> {
                if css_rule.rule_page.get_mut_ref().style.is_none() {
                    css_rule.rule_page.get_mut_ref().style = Some(style);
                }
                else {
                    css_stylesheet::css__stylesheet_merge_style(css_rule.rule_page.get_mut_ref().style.get_mut_ref(), &mut style);
                }
            },
            CSS_RULE_SELECTOR=> {
                if css_rule.rule_selector.get_mut_ref().style.is_none() {
                    css_rule.rule_selector.get_mut_ref().style = Some(style);
                }
                else {
                    css_stylesheet::css__stylesheet_merge_style(css_rule.rule_selector.get_mut_ref().style.get_mut_ref(), &mut style);
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
    pub fn css__stylesheet_rule_set_charset(css_rule : &mut ~css_rule_data_type, charset: ~str) -> css_error {
        //debug!("Entering: css__stylesheet_rule_set_charset");
        
        if charset.len() <= 0 {
            return CSS_BADPARM;
        }

        match css_rule.rule_type {
            CSS_RULE_CHARSET => {
                css_rule.rule_charset.get_mut_ref().encoding = charset;
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
        css_rule : &mut ~css_rule_data_type, url_str:&str, media:u64) -> css_error {
        //debug!("Entering: css__stylesheet_rule_set_nascent_import");
        match css_rule.rule_type {
            CSS_RULE_IMPORT => {
                css_rule.rule_import.get_mut_ref().url = url_str.to_owned();
                css_rule.rule_import.get_mut_ref().media=media;
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
        css_rule : &mut ~css_rule_data_type, media:u64) -> css_error {
        //debug!("Entering: css__stylesheet_rule_set_media");
        match css_rule.rule_type {
            CSS_RULE_MEDIA => {
                css_rule.rule_media.get_mut_ref().media=media;
                CSS_OK
            }
            _ => {
                CSS_BADPARM
            }
        }
    }

    pub fn css__stylesheet_rule_set_page_selector(
        css_rule : &mut ~css_rule_data_type, selector:uint) -> css_error {
        //debug!("Entering: css__stylesheet_rule_set_page_selector");
        match css_rule.rule_type {
            CSS_RULE_PAGE => {
                css_rule.rule_page.get_mut_ref().selector= Some(selector);
                CSS_OK
            }
            _ => {
                CSS_BADPARM
            }
        }
    }
    
    pub fn css__stylesheet_get_parent_type(&mut self, css_rule_data_list:&mut ~[~css_rule_data_type], css_rule : css_rule_data_index) -> CSS_RULE_PARENT_TYPE {
        
        let base_rule = self.css__stylesheet_get_base_rule(css_rule_data_list, css_rule);

        if (self.css_rule_list[base_rule].parent_rule.is_some() && !self.css_rule_list[base_rule].parent_stylesheet) {
            return CSS_RULE_PARENT_RULE;
        }

        if (self.css_rule_list[base_rule].parent_rule.is_none() && self.css_rule_list[base_rule].parent_stylesheet) {
            return CSS_RULE_PARENT_STYLESHEET;
        }

        fail!(~"Parent type is ambiguous");
    }
    pub fn css__stylesheet_get_base_rule(&mut self, css_rule_data_list:&mut ~[~css_rule_data_type], css_rule : css_rule_data_index) -> uint {
        
        match css_rule_data_list[css_rule].rule_type {
            CSS_RULE_UNKNOWN => {
                css_rule_data_list[css_rule].rule_unknown
            },
            CSS_RULE_SELECTOR =>{
                css_rule_data_list[css_rule].rule_selector.get_ref().base
            },
            CSS_RULE_CHARSET=>{
                css_rule_data_list[css_rule].rule_charset.get_ref().base
            },
            CSS_RULE_IMPORT=>{
                css_rule_data_list[css_rule].rule_import.get_ref().base
            },
            CSS_RULE_MEDIA=>{
                css_rule_data_list[css_rule].rule_media.get_ref().base
            },
            CSS_RULE_FONT_FACE=>{
                css_rule_data_list[css_rule].rule_font_face.get_ref().base
            },
            CSS_RULE_PAGE=>{
                css_rule_data_list[css_rule].rule_page.get_ref().base
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
    pub fn css__stylesheet_add_rule(stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], sheet : uint,  lwc_ref:&mut ~lwc, css_rule : css_rule_data_index,
                                    parent_rule : Option<css_rule_data_index> ) -> css_error {
        
        //debug!("Entering: css__stylesheet_add_rule");
        let base_rule = stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, css_rule);

        stylesheet_vector[sheet].css_rule_list[base_rule].index = stylesheet_vector[sheet].rule_count;

        match stylesheet_vector[sheet]._add_selectors(css_rule_data_list, lwc_ref, css_rule) {
            CSS_OK => {},
            x => return x
        }

        match parent_rule {
            Some(prule)=> {
                match css_rule_data_list[prule].rule_type {
                    CSS_RULE_MEDIA =>{
                        stylesheet_vector[sheet].css_rule_list[base_rule].parent_rule = parent_rule;
                        stylesheet_vector[sheet].rule_count += 1;
                        //let mut base_media_prule = self.css__stylesheet_get_base_rule(prule);


                        match css_rule_data_list[prule].rule_media.get_mut_ref().last_child {
                            None=>{
                                stylesheet_vector[sheet].css_rule_list[base_rule].next = None;
                                stylesheet_vector[sheet].css_rule_list[base_rule].prev = None;
                                css_rule_data_list[prule].rule_media.get_mut_ref().first_child = Some(css_rule);
                                css_rule_data_list[prule].rule_media.get_mut_ref().last_child = Some(css_rule);
                            },
                            Some(last_child)=>{
                                let last_child_base_rule = stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, last_child);
                                stylesheet_vector[sheet].css_rule_list[last_child_base_rule].next = Some(css_rule);
                                stylesheet_vector[sheet].css_rule_list[base_rule].prev = Some(last_child) ;
                                stylesheet_vector[sheet].css_rule_list[base_rule].next = None;
                                css_rule_data_list[prule].rule_media.get_mut_ref().last_child = Some(css_rule);
                            }
                        }
                    },
                    _=> return CSS_INVALID
                }
            },
            None=>{
                stylesheet_vector[sheet].css_rule_list[base_rule].parent_stylesheet = true;
                stylesheet_vector[sheet].rule_count += 1 ;

                match stylesheet_vector[sheet].last_rule {
                    None=>{
                        stylesheet_vector[sheet].css_rule_list[base_rule].prev = None;
                        stylesheet_vector[sheet].css_rule_list[base_rule].next = None;
                        stylesheet_vector[sheet].rule_list = Some(css_rule);
                        stylesheet_vector[sheet].last_rule = Some(css_rule);
                    },
                    Some(last_rule)=>{
                        let last_rule_base_rule = stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, last_rule);
                        stylesheet_vector[sheet].css_rule_list[last_rule_base_rule].next = Some(css_rule);
                        stylesheet_vector[sheet].css_rule_list[base_rule].prev = stylesheet_vector[sheet].last_rule;
                        stylesheet_vector[sheet].css_rule_list[base_rule].next = None;
                        stylesheet_vector[sheet].last_rule = Some(css_rule);
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
    pub fn css__stylesheet_remove_rule(stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], sheet : uint,  lwc_ref:&mut ~lwc, css_rule : css_rule_data_index) 
                                        -> css_error {
        //debug!("Entering: css__stylesheet_remove_rule");
        match stylesheet_vector[sheet]._remove_selectors(css_rule_data_list, lwc_ref, css_rule) {
            CSS_OK=>{},
            x =>return x 
        }

        let base_rule = stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, css_rule);
        match stylesheet_vector[sheet].css_rule_list[base_rule].next {
            None=> {
                stylesheet_vector[sheet].last_rule = stylesheet_vector[sheet].css_rule_list[base_rule].prev;
            },
            Some(base_rule_next)=>{
                let next_rule = stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, base_rule_next);
                stylesheet_vector[sheet].css_rule_list[next_rule].prev = stylesheet_vector[sheet].css_rule_list[base_rule].prev;
            }
        }

        match stylesheet_vector[sheet].css_rule_list[base_rule].prev {
            None=>{
                stylesheet_vector[sheet].rule_list = stylesheet_vector[sheet].css_rule_list[base_rule].next ;
            },
            Some(base_rule_prev)=>{
                let prev_rule = stylesheet_vector[sheet].css__stylesheet_get_base_rule(css_rule_data_list, base_rule_prev);
                stylesheet_vector[sheet].css_rule_list[prev_rule].next = stylesheet_vector[sheet].css_rule_list[base_rule].next ;
            }
        }
        stylesheet_vector[sheet].css_rule_list[base_rule].parent_rule = None ;
        stylesheet_vector[sheet].css_rule_list[base_rule].parent_stylesheet = false;
        stylesheet_vector[sheet].css_rule_list[base_rule].next = None;
        stylesheet_vector[sheet].css_rule_list[base_rule].prev = None ;
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
    pub fn _add_selectors(&mut self, css_rule_data_list:&mut ~[~css_rule_data_type], lwc_ref:&mut ~lwc, css_rule : css_rule_data_index) -> css_error {
        //debug!("Entering: _add_selectors");
        match css_rule_data_list[css_rule].rule_type {
            CSS_RULE_SELECTOR => {
                if self.css_rule_list[css_rule_data_list[css_rule].rule_selector.get_mut_ref().base].parent_rule.is_some() || 
                        self.css_rule_list[css_rule_data_list[css_rule].rule_selector.get_mut_ref().base].parent_stylesheet {
                    return CSS_INVALID;
                }

                let mut i : int = 0 ;
                let length = css_rule_data_list[css_rule].rule_selector.get_mut_ref().selectors.len() as int;
                while (i< length ) {
                    let selector:uint = css_rule_data_list[css_rule].rule_selector.get_mut_ref().selectors[i];
                    match self.css__selector_hash_insert(css_rule_data_list, lwc_ref, selector) {
                        CSS_OK=> { 
                            i += 1;
                            loop;
                        } ,
                        y => {
                            i -= 1;
                            while (i>=0){
                                // Ignore errors
                                let inner_selector = css_rule_data_list[css_rule].rule_selector.get_mut_ref().selectors[i];
                                self.css__selector_hash_remove(lwc_ref, inner_selector);
                                i -= 1;
                            }
                            // Remove zeroth elementcss_rule_data_list[css_rule]
                            //self.selectors.css__selector_hash_remove(x.selectors[i]);
                            return y;
                        }
                    }
                }

                CSS_OK
            },
            CSS_RULE_MEDIA => {
                if self.css_rule_list[css_rule_data_list[css_rule].rule_media.get_mut_ref().base].parent_rule.is_some() || 
                        self.css_rule_list[css_rule_data_list[css_rule].rule_media.get_mut_ref().base].parent_stylesheet {
                    return CSS_INVALID;
                }

                let mut ptr = css_rule_data_list[css_rule].rule_media.get_mut_ref().first_child;
                loop {
                    match ptr {
                        None=> {
                            return CSS_OK
                        },
                        Some(current_rule) => {
                        
                            match self._add_selectors(css_rule_data_list, lwc_ref, current_rule) {
                                CSS_OK=>{
                                    ptr = self.css_rule_list[self.css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].next;
                                    loop ;
                                },
                                x => {
                                    /* Failed, revert our changes */
                                    ptr = self.css_rule_list[self.css__stylesheet_get_base_rule(css_rule_data_list, current_rule)].prev;
                                    loop {
                                        match ptr {
                                            None=>{
                                                return x ;
                                            }
                                            Some(prev_rule)=>{
                                                self._remove_selectors(css_rule_data_list, lwc_ref, prev_rule) ;
                                                ptr = self.css_rule_list[self.css__stylesheet_get_base_rule(css_rule_data_list, prev_rule)].prev;
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
    pub fn _remove_selectors(&mut self, css_rule_data_list:&mut ~[~css_rule_data_type],  lwc_ref:&mut ~lwc, css_rule : css_rule_data_index) -> css_error {
        //debug!("Entering: _remove_selectors");
        match css_rule_data_list[css_rule].rule_type {
            CSS_RULE_SELECTOR => {
                let len = css_rule_data_list[css_rule].rule_selector.get_mut_ref().selectors.len();
                let mut i = 0;
                while i < len {
                    let selector= css_rule_data_list[css_rule].rule_selector.get_mut_ref().selectors[i];
                    match self.css__selector_hash_remove(lwc_ref, selector) {
                        CSS_OK=>{
                            i+=1;
                            loop;
                        },
                        x => { 
                            return x ;
                        } 
                    }
                }

                CSS_OK
            },

            CSS_RULE_MEDIA=> {

                let mut ptr = css_rule_data_list[css_rule].rule_media.get_mut_ref().first_child;
                loop {
                    match ptr {
                        None=> {
                            return CSS_OK ;
                        },
                        Some(base_rule)=>{
                            match self._remove_selectors(css_rule_data_list, lwc_ref, base_rule) {
                                CSS_OK => {
                                    ptr = self.css_rule_list[self.css__stylesheet_get_base_rule(css_rule_data_list, base_rule)].next;
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

        /**
    * #Description:
    *  Retrieve the first class name in a selector, or empty if none.

    * #Arguments:
    *  'selector '  - Selector to consider. 

    * #Return Value:
    *  '@str' - class name.
    */

    #[inline]
    pub fn _class_name(&mut self, lwc_ref:&mut ~lwc, selector : uint) 
                        -> uint {

        let mut counter_i = 0;
        while counter_i < self.css_selectors_list[selector].data.len() {
            match self.css_selectors_list[selector].data[counter_i].selector_type {
                CSS_SELECTOR_CLASS=>{
                    if (self.css_selectors_list[selector].data[counter_i].negate == false) {
                        return self.css_selectors_list[selector].data[counter_i].qname.name;
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
    pub fn _id_name(&mut self, lwc_ref:&mut ~lwc, selector : uint) 
         -> uint {

        let mut counter_i = 0;
        while counter_i <  self.css_selectors_list[selector].data.len() {
            match self.css_selectors_list[selector].data[counter_i].selector_type {
                CSS_SELECTOR_ID=>{
                    if (self.css_selectors_list[selector].data[counter_i].negate == false) {
                        return self.css_selectors_list[selector].data[counter_i].qname.name;
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
    *  Insert an item into the hash table.

    * #Arguments:
    *  'selector'  - css selector. 

    * #Return Value:
    *  'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__selector_hash_insert(&mut self, css_rule_data_list:&mut ~[~css_rule_data_type], lwc_ref:&mut ~lwc, selector : uint) 
                                    -> css_error {
        //debug!("Entering: css__selector_hash_insert");

        let mut mask :u32 ;
        let mut index:u32=0;
        let mut name :uint ;
        if (self.css_selectors_list[selector].data.len() > 0) {
            let class_lwc_string = self._class_name(lwc_ref, selector);
            let id_lwc_string = self._id_name(lwc_ref, selector);
            // Named Element
            if ( lwc_ref.lwc_string_length(self.css_selectors_list[selector].data[0].qname.name) != 1) || 
                (lwc_ref.lwc_string_data(self.css_selectors_list[selector].data[0].qname.name).char_at(0) != '*' ) {
                    //debug!("Entering: css__selector_hash_insert:: Named Element");
                    mask = self.selectors.default_slots-1 ;
                    index = css_selector_hash::_hash_name(self.css_selectors_list[selector].data[0].qname.name, lwc_ref) & mask ;
                    return self._insert_into_chain(css_rule_data_list, Element,index,selector);
            }

            // Named Class
            else if lwc_ref.lwc_string_length(class_lwc_string) != 0  {
                //debug!("Entering: css__selector_hash_insert:: Named Class");
                name = self._class_name(lwc_ref, selector);
                mask = self.selectors.default_slots-1 ;
                index = css_selector_hash::_hash_name(name, lwc_ref) & mask ;
                return self._insert_into_chain(css_rule_data_list, Class,index,selector);
            }

            // Named Id
            else if lwc_ref.lwc_string_length(id_lwc_string) != 0 {
                //debug!("Entering: css__selector_hash_insert:: Named Id");
                name = self._id_name( lwc_ref, selector);
                mask = self.selectors.default_slots-1 ;
                index = css_selector_hash::_hash_name(name, lwc_ref) & mask ;
                return self._insert_into_chain(css_rule_data_list, Ids,index,selector);
            }
            else {
                //debug!("Entering: css__selector_hash_insert:: else Universal");
                return self._insert_into_chain(css_rule_data_list, Universal,index,selector);
            }
        }
        // Universal Chain
        //debug!("Entering: css__selector_hash_insert:: Universal Chain");
        return self._insert_into_chain(css_rule_data_list, Universal,index,selector);
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
    pub fn _insert_into_chain(&mut self,
                            css_rule_data_list:&mut ~[~css_rule_data_type], 
                            hash_type : css_hash_type,
                            index:u32,
                            selector : uint) 
                            -> css_error {
        //debug!("Entering: _insert_into_chain");
        //debug!("_insert_into_chain:: hash_type == %?, index == %?", hash_type, index);
        //let mut hash_entry_list : &mut ~[Option<hash_entry_index>];
        let mut hash_entry_list_type : uint;
        
        match hash_type {
            Element => {hash_entry_list_type = ELEMENT} ,
            Class => {hash_entry_list_type = CLASSES} ,
            Ids =>  {hash_entry_list_type = IDS} ,
            Universal => {hash_entry_list_type = UNIVERSAL} ,
        };

        let entry = ~hash_entry {
                selector:selector,
                next:None
        };
        self.selectors.hash_entry_list.push(entry);
        let entry_index = self.selectors.hash_entry_list.len() -1;

        match self.selectors.ele_class_ids_univ[hash_entry_list_type][index] {
            None=> {
                //debug!("Entering: match (*hash_entry_list)[index] => None");
                self.selectors.ele_class_ids_univ[hash_entry_list_type][index] = Some(entry_index);
                //debug!("(*hash_entry_list)[index] == %?", (*hash_entry_list)[index]);
            },
            Some(index_element)=> {
                //debug!("Entering: match (*hash_entry_list)[index] => Some(index_element)");
                let mut search = index_element;
                let mut prev = index_element ;
                let mut first_pos : bool = true ;
                loop {

                    if (selector == self.selectors.hash_entry_list[search].selector) {
                        // duplicate insert of same pointer css_selector should never occur,
                        // added , due to logical incompatibilty with "_remove_into_chain"
                        // in origical code , _remove_into_chain removes by comparing pointer values,
                        // and freeing the final result , by doing reallocation of 0 bytes ( line num : 650-671 , hash.c)
                        //debug!("_insert_into_chain : error: double insertion of same selector ") ;
                        return CSS_BADPARM;
                    }

                    if self.css_selectors_list[self.selectors.hash_entry_list[search].selector].specificity> self.css_selectors_list[selector].specificity {
                        break ;
                    }

                    if self.css_selectors_list[self.selectors.hash_entry_list[search].selector].specificity == self.css_selectors_list[selector].specificity {
                        if(self.css_selectors_list[self.selectors.hash_entry_list[search].selector].rule.is_none() || self.css_selectors_list[selector].rule.is_none() ){
                            //debug!("_insert_into_chain : error : rule is none  ") ;
                            return CSS_BADPARM ;
                        }

                        let base_search_rule = self.css__stylesheet_get_base_rule(css_rule_data_list, self.css_selectors_list[self.selectors.hash_entry_list[search].selector].rule.expect(""));
                        let base_selector_rule = self.css__stylesheet_get_base_rule(css_rule_data_list, self.css_selectors_list[selector].rule.expect(""));
                        
                        if self.css_rule_list[base_search_rule].index > self.css_rule_list[base_selector_rule].index {
                            break ;
                        }
                    }

                    prev = search ;
                    first_pos = false ;
                    search = 
                        match self.selectors.hash_entry_list[search].next {
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
                    self.selectors.hash_entry_list[entry_index].next = Some(index_element);
                    self.selectors.ele_class_ids_univ[hash_entry_list_type][index] = Some(entry_index);
                }
                else {
                    //debug!("Entering: _insert_into_chain:: if(first_pos)--else");
                    self.selectors.hash_entry_list[entry_index].next=self.selectors.hash_entry_list[prev].next;
                    self.selectors.hash_entry_list[prev].next= Some(entry_index);
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
    pub fn css__selector_hash_remove(&mut self, lwc_ref:&mut ~lwc, selector : uint) 
                                    -> css_error {
        let mut mask :u32 ;
        let mut index:u32=0;
        let mut name : uint ;
        if (self.css_selectors_list[selector].data.len() > 0){
            let class_lwc_string = self._class_name(lwc_ref, selector);
            let id_lwc_string = self._id_name(lwc_ref, selector);
            // Named Element
            if ( lwc_ref.lwc_string_length(self.css_selectors_list[selector].data[0].qname.name) != 1) || 
                (lwc_ref.lwc_string_data(self.css_selectors_list[selector].data[0].qname.name).char_at(0) != '*' ) {
                    mask = self.selectors.default_slots-1 ;
                    index = css_selector_hash::_hash_name(self.css_selectors_list[selector].data[0].qname.name, lwc_ref) & mask ;
                    return self.selectors._remove_from_chain(Element,index,selector);
            }

            // Named Class
            else if lwc_ref.lwc_string_length(class_lwc_string) == 0  {
                name = self._class_name(lwc_ref, selector);
                mask = self.selectors.default_slots-1 ;
                index = css_selector_hash::_hash_name(name, lwc_ref) & mask ;
                return self.selectors._remove_from_chain(Class,index, selector);
            }

            // Named Id
            else if lwc_ref.lwc_string_length(id_lwc_string) == 0 {
                name = self._id_name(lwc_ref, selector);
                mask = self.selectors.default_slots-1 ;
                index = css_selector_hash::_hash_name(name, lwc_ref) & mask ;
                return self.selectors._remove_from_chain(Ids,index,selector);
            }
            else {
                return self.selectors._remove_from_chain(Universal,index,selector);
            }
        }
        // Universal Chain
        return self.selectors._remove_from_chain(Universal,index,selector);
    }

    /**
    * #Description:
    Find the first selector that matches name.

    * #Arguments:
    *  'name'  - name to find. 

    * #Return Value:
    *  '(Option<@mut hash_entry>,css_error)' - (Some(hash_entry),CSS_OK) on success, otherwise (None, CSS_OK).
    */
    pub fn css__selector_hash_find(&mut self, lwc_ref:&mut ~lwc, name : uint) -> (Option<uint>,css_error) {
        //debug!("Entering: css__selector_hash_find");
        let mask  = self.selectors.default_slots-1 ;
        let index = css_selector_hash::_hash_name(name, lwc_ref) & mask ; 
        let mut head = self.selectors.ele_class_ids_univ[ELEMENT][index];

        //debug!(fmt!("css__selector_hash_find:: name=%?  mask=%?, index=%? ", name, mask, index ));
        loop {
            match head {
                None=>{
                    return (None,CSS_OK);
                },
                Some(node_element)=>{

                    if lwc_ref.lwc_string_caseless_isequal(
                                self.css_selectors_list[self.selectors.hash_entry_list[node_element].selector].data[0].qname.name,name)  {
                                //debug!("Exiting: css__selector_hash_find (1)");
                                return (head,CSS_OK);
                    }

                    match self.selectors.hash_entry_list[node_element].next {
                        None=> {
                            //debug!("Exiting: css__selector_hash_find (2)");
                            return (None,CSS_OK);
                        },
                        Some(_)=>{
                            head = self.selectors.hash_entry_list[node_element].next ;
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
    pub fn css__selector_hash_find_by_class(&mut self, lwc_ref:&mut ~lwc, name : uint) -> (Option<uint>,css_error) {

        let mask  = self.selectors.default_slots-1 ;
        let index = css_selector_hash::_hash_name(name, lwc_ref) & mask ; 
        let mut head = self.selectors.ele_class_ids_univ[CLASSES][index];

        //debug!(fmt!("name=%?  mask=%?, index=%? ", name, mask, index ));
        loop {
            match head {
                None=>{
                    return (None,CSS_OK);
                },
                Some(node_element)=>{

                    let n = self._class_name(lwc_ref, self.selectors.hash_entry_list[node_element].selector);

                    if lwc_ref.lwc_string_caseless_isequal(n, name) {
                        return (head,CSS_OK);
                    }

                    match self.selectors.hash_entry_list[node_element].next {
                        None=> {
                            return (None,CSS_OK);
                        },
                        Some(_)=>{
                            head = self.selectors.hash_entry_list[node_element].next ;
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
    pub fn css__selector_hash_find_by_id(&mut self, lwc_ref:&mut ~lwc, name : uint) -> (Option<uint>,css_error) {

        let mask  = self.selectors.default_slots-1 ;
        let index = css_selector_hash::_hash_name(name, lwc_ref) & mask ; 
        let mut head = self.selectors.ele_class_ids_univ[IDS][index];

        loop {
            match head {
                None=>{
                    return (None,CSS_OK);
                },
                Some(node_element)=>{

                    let n = self._id_name(lwc_ref, self.selectors.hash_entry_list[node_element].selector);

                    if lwc_ref.lwc_string_caseless_isequal(n, name) {
                        return (head,CSS_OK);
                    }

                    match self.selectors.hash_entry_list[node_element].next {
                        None=> {
                            return (None,CSS_OK);
                        },
                        Some(_)=>{
                            head = self.selectors.hash_entry_list[node_element].next ;
                            loop ;
                        }
                    }
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
    pub fn _iterate_ids(&mut self, lwc_ref:&mut ~lwc, current : uint) -> (Option<uint>,css_error) {

        let mut head = current;

        let current_refer = self._id_name(lwc_ref, self.selectors.hash_entry_list[current].selector);

        loop {
            match self.selectors.hash_entry_list[head].next {
                None=>{
                    return (None,CSS_OK);
                },
                Some(next_entry)=>{
                    let name = self._id_name(lwc_ref, self.selectors.hash_entry_list[next_entry].selector);
                    if( lwc_ref.lwc_string_length(name)==0){
                        loop;
                    }
                    if lwc_ref.lwc_string_caseless_isequal(name,current_refer)  {
                        return (self.selectors.hash_entry_list[current].next,CSS_OK);
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
    pub fn _iterate_elements(&mut self, lwc_ref:&mut ~lwc, current : uint) -> (Option<uint>,css_error) {

        let mut head = current;

        loop {
            match self.selectors.hash_entry_list[head].next {
                None=>{
                    return (None,CSS_OK);
                },
                Some(next_entry)=>{
                    if self.css_selectors_list[self.selectors.hash_entry_list[head].selector].data.len()==0 || 
                        self.css_selectors_list[self.selectors.hash_entry_list[next_entry].selector].data.len()==0 {
                        return (None,CSS_INVALID);
                    }
                    if lwc_ref.lwc_string_caseless_isequal(
                        self.css_selectors_list[self.selectors.hash_entry_list[current].selector].data[0].qname.name,
                        self.css_selectors_list[self.selectors.hash_entry_list[next_entry].selector].data[0].qname.name) {

                        return (self.selectors.hash_entry_list[head].next,CSS_OK);
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
    pub fn _iterate_classes(&mut self, lwc_ref:&mut ~lwc, current : uint) -> (Option<uint>,css_error) {

        let mut head = current;

        let current_refer = self._class_name(lwc_ref, self.selectors.hash_entry_list[current].selector);

        loop {
            match self.selectors.hash_entry_list[head].next {
                None=>{
                    return (None,CSS_OK);
                },
                Some(next_entry)=>{
                    let name = self._class_name(lwc_ref, self.selectors.hash_entry_list[next_entry].selector);
                    if( lwc_ref.lwc_string_length(name)==0){
                        loop;
                    }
                    if  lwc_ref.lwc_string_caseless_isequal(name,current_refer) {
                        return (self.selectors.hash_entry_list[current].next,CSS_OK);
                    }
                    head = next_entry ;
                    loop ;
                }
            }
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
    pub fn css__selector_hash_create() -> ~css_selector_hash {
        //debug!("Entering: css__selector_hash_create");
        let mut hash = ~css_selector_hash{ 
                        default_slots:(1<<6),
                        ele_class_ids_univ:~[~[],~[],~[],~[]],
                        hash_entry_list:~[]
        };
        let size = hash.default_slots as uint;
        hash.ele_class_ids_univ[ELEMENT].reserve(size);
		hash.ele_class_ids_univ[CLASSES].reserve(size);
        hash.ele_class_ids_univ[IDS].reserve(size);
        hash.ele_class_ids_univ[UNIVERSAL].reserve(size);
		
		let mut i = 0;
		while i < size {
				hash.ele_class_ids_univ[ELEMENT].push(None);
				hash.ele_class_ids_univ[CLASSES].push(None);
				hash.ele_class_ids_univ[IDS].push(None);
				hash.ele_class_ids_univ[UNIVERSAL].push(None);
			        i = i + 1;
        }
        hash
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

        let mut hash_entry_list : &mut ~[Option<uint>];

        match hash_type {
            Element => {hash_entry_list = &mut self.ele_class_ids_univ[ELEMENT]} ,
            Class => {hash_entry_list = &mut self.ele_class_ids_univ[CLASSES]} ,
            Ids =>  {hash_entry_list = &mut self.ele_class_ids_univ[IDS]} ,
            Universal => {hash_entry_list = &mut self.ele_class_ids_univ[UNIVERSAL]} ,
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

                    if (selector == self.hash_entry_list[search].selector) {
                        break;
                    }

                    first_pos = false ;
                    search = 
                        match self.hash_entry_list[search].next {
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
                    hash_entry_list[index] = self.hash_entry_list[search].next;
                }
                else {
                    self.hash_entry_list[prev].next= self.hash_entry_list[search].next;
                }
            }
        }
        CSS_OK
    }

    

    /**
    * #Description:
    *  Find the first universal selector.

    * #Return Value:
    *  '(Option<@mut hash_entry>,css_error)' - (Some(hash_entry),CSS_OK) on success, otherwise (None, CSS_OK).
    */
    pub fn css__selector_hash_find_universal(&mut self) -> (Option<uint>,css_error) {

        let head = self.ele_class_ids_univ[UNIVERSAL][0] ;
        match head {
            None=>{
                return (None,CSS_OK);
            },
            Some(_)=>{
                return (self.ele_class_ids_univ[UNIVERSAL][0],CSS_OK);
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
    pub fn _iterate_universal(&mut self, current : uint) -> (Option<uint>,css_error) {

        if self.hash_entry_list[current].next.is_some() {
            return (self.hash_entry_list[current].next,CSS_OK);
        }
        (None,CSS_OK)
    }

    pub fn debug_print_vector_of_hash_entry_list(&mut self, hash_vec : &[Option<uint>]) {

        for &entry in hash_vec.iter() {
            self.debug_print_hash_entry_list(entry) ;
        }
    }

    pub fn debug_print_hash_entry_list(&mut self, current : Option<uint>) {

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
                    ptr = self.hash_entry_list[x].next ;
                }
            }
        }
    }
}


///////////////////////////////////////////////////////////////////////////////////////
