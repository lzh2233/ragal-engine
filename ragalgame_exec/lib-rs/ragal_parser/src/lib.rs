use std::any;
use std::io;
use std::path::Path;
use std::collections::HashMap;

use serde::Serialize;

mod optionsbuild_wrapper;

use optionsbuild_wrapper::*;



pub use analyzer::*;


#[derive(Serialize)]
pub struct Value<S: Serialize>{
    value: S,
    type_name: &'static str
}


impl<S: Serialize> Value<S>{
    pub fn new(value: S) -> Self{
        let type_name  = any::type_name::<S>().split("::").last().unwrap(); 
        Self{
            value,
            type_name
        }
    }
}



#[derive(Debug, Serialize)]
pub enum CustomScript {
    Js(String),
    Html(String),
    MarkDown(String),
    Rhai(String),
}



#[derive(Debug, Serialize)]
pub enum Linker {
    Link(Box<Link>),
    Image(Box<Image>)
}


impl From<Link> for Linker{
    fn from(value: Link) -> Self {
        Self::Link(Box::new(value))
    }
}

impl From<Image> for Linker {
    fn from(value: Image) -> Self {
        Self::Image(Box::new(value))
    }
}




#[derive(Debug, Clone, Serialize)]
pub struct Link{
    txt: String,
    inner: Option<String>
}


impl Link {
    fn new(url: impl ToString, txt: impl Into<String>) -> Self {
        Self { 
            inner: {let inner = url.to_string(); if inner.is_empty(){None}else{Some(inner)}},
            txt: txt.into()
        }
    }
}



#[derive(Debug,Serialize)]
pub struct Image{
    link : Link,
    style: HashMap<String, String>
}


impl From<Link> for Image{
    fn from(value: Link) -> Self {
        Self { link: value, style: Default::default() }
    }
}

impl std::ops::Deref for Image {
    type Target = Link;
    fn deref(&self) -> &Self::Target {
        return &self.link
    }
}



#[derive(Debug, Serialize)]
pub struct Role{
    name: String,
    state: Option<String>
}

impl Role {
    fn new(name: String, state: Option<String>) -> Self{
        Self { name, state }
    }
}




#[derive(Debug, Serialize)]
pub struct Dialog{
    speaker: Option<Role>,
    speech: String
}

impl Dialog {
    fn new(name: String, state: Option<String>, speech: String) -> Self{
        let speaker = if name == ""{None}else{Some(Role::new(name, state))};
        Self { speaker, speech}
    }
}



#[derive(Debug, Serialize)]
pub struct RalCommand{
    cmd: String
}



#[derive(Debug, Serialize)]
pub struct OptionCondition{
    inner: String,
    ret_val: Option<String>
}

impl OptionCondition{
    fn new(inner : String, ret_val: Option<String>) -> Self{
        Self{
            inner,
            ret_val
        }
    }
}



#[derive(Debug, Serialize)]
pub struct OptionItem{
    val: Linker,
    conditions: Vec<OptionCondition>
}

impl OptionItem{
    fn new(val: Linker, conditions: Vec<OptionCondition>) -> Self{
        Self{
            val, conditions
        }
    }
}

impl std::ops::Deref for OptionItem{
    type Target = Linker;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

type OptionList = Vec<OptionItem>;

mod analyzer{
    use super::*;

    use std::cell::RefMut;
    use std::io::{BufReader, BufRead, Seek};
    use std::path::PathBuf;
    use std::borrow::Cow;
    use std::{mem, fs};

    use comrak::{Arena, Options};
    use comrak::nodes::{AstNode, NodeValue, Ast};
    use comrak::nodes::NodeCodeBlock;
    use comrak::parse_document;
    use comrak::format_html;

    use serde_json::json;
    use serde_json::Value as JsonValue;


    pub fn dialog<'a>(mut header: RefMut<'a, Ast>, ext: &'a AstNode<'a>) -> Vec<Value<Dialog>>{
        let name: String;
        {
            let _header = header.value.text_mut().unwrap();
            let (nm, speech) = _header.split_once(|c|{
                c == ':' || c == '：'
            }).unwrap_or(("",_header));
            name = nm.trim().to_string();
            *_header = speech.trim_start().to_string();
            mem::drop(header);
        };
        let (name, state) = match name.split_once(|c|{ c == '(' || c == '（'}){
            None => (name, None),
            Some((name, state)) => {
                let mut flg = ' ';
                if state.ends_with(|c|{flg.clone_from(&c);c == ')' || c == '）'}){
                    let state = state.trim_start().trim_end_matches(|c|{
                        if flg != ' '{
                            let _flg = flg;
                            flg = ' ';
                            c == _flg
                        }else{
                            c == ' ' || c == '\t' || c == '\n'
                        }
                    });
                    let state = if state.is_empty(){
                        None
                    }else{
                        Some(state.to_string())
                    };
                    (name.trim_end().to_owned(), state)
                }else{
                    (name.trim_end().to_owned(), None)
                }
            }
        };

        let mut content = vec![];
        let mut res = Vec::new();
        let mut opt = Options::default();
        opt.render.hardbreaks = true;
        opt.render.unsafe_ = true;
        for child in ext.children(){
            if let NodeValue::SoftBreak = child.data.borrow().value{
                res.push(Value::new(Dialog::new(
                    name.clone(),
                    state.clone(),
                    unsafe{String::from_utf8_unchecked(mem::replace(&mut content, vec![]))}
                )));
                continue;
            }
            format_html(child, &opt, &mut content).unwrap();
        }
        if !content.is_empty(){
            res.push(Value::new(Dialog::new(name, state, unsafe{String::from_utf8_unchecked(content)})))
        }
        res
    }


    pub fn linker<'a>(nlink: &'a AstNode<'a>) -> Value<Linker>{
        debug_assert!(matches!(nlink.data.borrow().value, NodeValue::Link(_)|NodeValue::Image(_)));
        let _bind = &nlink.data.borrow().value;
        let mut txt = String::new();
        for e in nlink.children(){
            format_html(e, &Default::default(), unsafe { txt.as_mut_vec() }).unwrap()
        }
        match _bind {
            NodeValue::Link(lk) => Value::new(Linker::Link(Box::new(Link::new(&lk.url, if txt.trim().is_empty(){String::new()}else{txt})))),
            NodeValue::Image(lk) => {
                let link = Link::new(&lk.url, if txt.trim().is_empty(){String::new()}else{txt});
                Value::new(Linker::Image(Box::new(Image::from(link))))
            }
            _ => unsafe{std::hint::unreachable_unchecked()}
        }
    }


    fn option_item<'a>(nitem: &'a AstNode<'a>) -> OptionItem{
        let mut children = nitem.children();
        let para = children.next().unwrap();
        let link = if para.data.borrow().value == NodeValue::Paragraph{
            let first_child = para.first_child().unwrap();
            if let NodeValue::Link(_) = first_child.data.borrow().value{
                linker(first_child).value
            }else{
                let mut txtbuf = Vec::<u8>::new();
                for child in para.children(){
                    format_html(&child, &Options::default(), &mut txtbuf).unwrap();
                }
                Link::new("", unsafe { String::from_utf8_unchecked(txtbuf) }).into()      
            }
        }else{
            panic!("not Paragraph!")
        };
        let mut condition = Vec::new();
        if let Some(list) = children.next(){
            for item in list.children(){
                let p = item.first_child().unwrap();
                if p.data.borrow().value != NodeValue::Paragraph{
                    continue;
                }
                let first_child = p.first_child().unwrap();
                if let NodeValue::Link(link) = &first_child.data.borrow().value{
                    let mut cond = String::new();
                    for txt in first_child.children(){
                        cond += txt.data.borrow().value.text().unwrap_or(&String::new());
                    }
                    let cond = cond;
                    condition.push(OptionCondition::new(cond, Some(link.url.to_string())));
                }else{
                    let mut cond = String::new();
                    for txt in p.children(){
                        cond += txt.data.borrow().value.text().unwrap_or(&String::new());
                    }
                    condition.push(OptionCondition::new(cond, None));
                }
            }
        }

        OptionItem::new(link, condition)
    }


    pub fn options_list<'a>(nlist: &'a AstNode<'a>) -> Value<OptionList>{
        let mut optionlist = OptionList::new();
        for child in nlist.children(){
            if let NodeValue::Item(_) = &child.data.borrow().value{
                let item = option_item(child);
                optionlist.push(item);
            }
        }
        Value{
            value: optionlist,
            type_name: "OptionList"
        }
    }


    pub fn custom_script(nscript: &NodeCodeBlock) -> Option<Value<CustomScript>>{
        let info = nscript.info.to_lowercase();
        let literal = nscript.literal.to_owned();
        let res = match info.as_str() {
            "js" => CustomScript::Js(literal),
            "html" => CustomScript::Html(literal),
            "markDown" => CustomScript::MarkDown(literal),
            "rhai" => CustomScript::Rhai(literal),
            _ => None?
        };
        Some(Value::new(res))
    }


    pub fn ral_command<'a>(ncmd: &'a AstNode<'a>) -> Vec<Value<RalCommand>>{
        debug_assert!(matches!(ncmd.data.borrow().value, NodeValue::BlockQuote));
        let mut res = Vec::new();
        let mut cmd = String::new();
        for child in ncmd.children(){
            let value = &child.data.borrow().value;
            if &NodeValue::Paragraph != value{
                continue;
            }
            for gchild in child.children(){
                if gchild.data.borrow().value == NodeValue::SoftBreak{
                    res.push(Value::new(RalCommand{
                        cmd: mem::replace(&mut cmd, String::new())
                    }));
                    continue;
                }
                comrak::format_commonmark(gchild, &Options::default(), unsafe { cmd.as_mut_vec() }).unwrap();
                let _ = cmd.pop();
            }
            if !cmd.is_empty(){
                res.push(Value::new(RalCommand{
                    cmd: mem::replace(&mut cmd, String::new())
                }));
            }
        }
        res
    }


    fn parse_galmd<'a>(root: &'a AstNode<'a>) -> Vec<JsonValue>{
        let mut res = Vec::<JsonValue>::new();
        for child in root.children(){
            match &child.data.borrow().value {
                NodeValue::Paragraph => {
                    let g_child = child.first_child().unwrap();
                    let gchild_data =g_child.data.borrow_mut();
                    match &gchild_data.value {
                        NodeValue::Text(_) => {
                            let dialogs = dialog(gchild_data, child);
                            for dialog in dialogs{
                                res.push(json!(dialog))
                            }
                        },
                        NodeValue::Link(_)|NodeValue::Image(_)=> {
                            mem::drop(gchild_data);
                            let link = linker(g_child);
                            res.push(json!(link))
                        },
                        _ => ()
                    }
                },
                NodeValue::List(_) => {
                    let opt_lists = options_list(child);
                    res.push(json!(opt_lists))
                },
                NodeValue::CodeBlock(code) => {
                    let script = custom_script(code);
                    if let Some(script) = script{
                        res.push(json!(script))
                    }
                },
                NodeValue::BlockQuote => {
                    let cmds = ral_command(child);
                    for cmd in cmds{
                        res.push(json!(cmd))
                    }
                },
                _ => ()
            }
        }
        res
    }

    #[derive(Serialize)]
    pub struct RalGalScript{
        pub file_name : String,
        pub dir : std::path::PathBuf,
        pub content: Vec<JsonValue>,
        pub custom_state: Option<serde_yaml::Mapping>,
        pub len : u64
    }

    pub fn open_galmd(pth: impl AsRef<Path>) ->  io::Result<RalGalScript>{
        use io::Read;
        use std::fs::File;
        let path = pth.as_ref();
        let name = path.file_name()
            .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Not File!"))?
            .to_string_lossy();
       let file_name =  match name {
            Cow::Borrowed(name) => name.to_string(),
            Cow::Owned(name) => name
        };
        let dir = path.parent().unwrap_or("".as_ref()).to_path_buf();
        let mut file = File::open(pth)?;
        let len = file.metadata()?.len();
        let mut buf = String::new();

        file.read_to_string(&mut buf)?;
        let arena = Arena::new();
        let opt = OptionsBuilder::default_of_ragal()
            .build()
            .unwrap();
        let root = parse_document(&arena, &buf, &opt);
        
        let mut rt_children = root.children().peekable();

        let custom_state = if let Some(first_node) = rt_children.peek(){
            if let NodeValue::FrontMatter(pre_yml) = &first_node.data.borrow().value{
                let _ = rt_children.next();
                serde_yaml::from_str(&pre_yml.trim().trim_matches('-')).unwrap_or(None)
            }else{
                None
            }
        }else{
            None
        };

        let mut content = Vec::new();
        while let Some(node) = rt_children.next(){
            content.extend(parse_galmd(node))
        }

        Ok(RalGalScript {
            dir,
            file_name, 
            content, 
            custom_state,
            len 
        })
    }

    
    pub struct GalmdReader{
        buff: BufReader<fs::File>,
        file_path: PathBuf
    }

    impl GalmdReader{
        pub fn open(pth: impl Into<PathBuf>) -> io::Result<Self>{
            let file_path = pth.into();
            let f_handle = fs::File::open(&file_path)?;
            Ok(Self{
                buff: BufReader::new(f_handle),
                file_path
            })
        }

        pub fn get_info(&mut self) -> io::Result<RalGalScript>{
            let buff = &mut self.buff;
            let mut pre_yml = String::new();
            buff.rewind()?;
            let mut flag = false;
            let pre_yml = loop{
                let remain = buff.read_line(&mut pre_yml)?;
                if remain == 0{
                    break None;
                }
                if flag{
                    if pre_yml.trim_end().ends_with("\n---"){
                        break Some(pre_yml);
                    }
                    continue;
                }

                if pre_yml.trim().len() == 0{
                    pre_yml.clear();
                }else if pre_yml.trim_end() == "---"{
                    flag = true;
                }else{
                    break None;
                }
            };


            let custom_state = if let Some(ref pre_yml) = pre_yml{
                serde_yaml::from_str(pre_yml.trim_end().trim_matches('-')).unwrap_or(None)
            }else{
                buff.rewind()?;
                None
            };
            let file_name =self.file_path.file_name()
                .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Not File!"))?
                .to_string_lossy();
            let file_name = match file_name {
                Cow::Borrowed(name) => name.to_string(),
                Cow::Owned(name) => name
            };
            let dir = self.file_path.parent().unwrap_or("".as_ref()).to_path_buf();
            Ok(RalGalScript{
                dir,
                file_name,
                content: Vec::new(),
                custom_state,
                len : self.buff.get_ref().metadata()?.len()
            })
        }

        pub fn read_exp(&mut self, buff: &mut Vec<JsonValue>,exp_lines: u64) -> io::Result<u64>{
            let mut txt = String::new();
            let mut temp = String::new();
            let mut cnt = 0;
            let mut flag = true;
            loop{
                let remain = self.buff.read_line(&mut temp)?;
                if remain == 0{
                    break;
                }
                if temp.starts_with("```"){
                    flag = !flag;
                }
                if cnt > exp_lines && flag && temp.trim() == ""{
                    break;
                }
                txt.push_str(&temp);
                temp.clear();
                cnt += 1;
            }
            let arena = Arena::new();
            let opt = OptionsBuilder::default_of_ragal().build().unwrap();
            let root = parse_document(&arena, &txt, &opt);
            buff.extend(parse_galmd(root));
            self.buff.stream_position()
        }

        pub fn reopen(&mut self, pth: impl Into<PathBuf>) -> io::Result<()>{
            self.file_path = pth.into();
            let handle = fs::File::open(&self.file_path)?;
            self.buff = BufReader::new(handle);
            Ok(())
        }
    }

    impl io::Seek for GalmdReader{
        fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
            self.buff.seek(pos)
        }
    }
}