#![allow(unused)]
use comrak::{ParseOptionsBuilder, ExtensionOptionsBuilder, RenderOptionsBuilder, Options};

#[derive(Debug)]
pub enum OptionsBuilderError {
    ParseError(String),
    ExtensionError(String),
    RenderError(String)
}

#[derive(Clone)]
pub struct OptionsBuilder{
    parser: ParseOptionsBuilder,
    extension: ExtensionOptionsBuilder,
    render: RenderOptionsBuilder
}

impl Default for OptionsBuilder{
    fn default() -> Self {
        Self { parser: Default::default(), extension: Default::default(), render: Default::default() }
    }
}

impl OptionsBuilder{
    pub fn all(&mut self, value: bool) -> &mut Self{
        self.relaxed_autolinks(value)
            .relaxed_tasklist_matching(value)
            .smart(value)
            .autolink(value)
            .description_lists(value)
            .footnotes(value)
            .strikethrough(value)
            .superscript(value)
            .table(value)
            .tagfilter(value)
            .tasklist(value)
            .escape(value)
            .full_info_string(value)
            .github_pre_lang(value)
            .hardbreaks(value)
            .sourcepos(value)
            .unsafe_(value);
        self
    }

    pub fn default_of_ragal() -> Self{
        let mut opt = OptionsBuilder::default();
        opt.all(true)
            .default_info_string(Some("js".to_string()))
            .front_matter_delimiter(Some("---".to_string()))
            .sourcepos(false)
            .escape(false)
            .github_pre_lang(false);
        opt
    }

    pub fn relaxed_autolinks(&mut self, value: bool) -> &mut Self{
        self.parser.relaxed_autolinks(value);
        self
    }
    pub fn relaxed_tasklist_matching(&mut self, value: bool) -> &mut Self{
        self.parser.relaxed_tasklist_matching(value);
        self
    }
    pub fn default_info_string(&mut self, value: Option<String>) -> &mut Self {
       self.parser.default_info_string(value);
       self
    }
    pub fn smart(&mut self, value: bool) -> &mut Self{
        self.parser.smart(value);
        self
    }

    pub fn autolink(&mut self, value: bool) -> &mut Self{
        self.extension.autolink(value);
        self
    }
    pub fn description_lists(&mut self, value: bool) -> &mut Self{
        self.extension.description_lists(value);
        self
    }
    pub fn footnotes(&mut self, value: bool) -> &mut Self{
        self.extension.footnotes(value);
        self
    }
    pub fn strikethrough(&mut self, value: bool) -> &mut Self{
        self.extension.footnotes(value);
        self
    }
    pub fn superscript(&mut self, value: bool) -> &mut Self{
        self.extension.superscript(value);
        self
    }
    pub fn table(&mut self, value: bool) -> &mut Self{
        self.extension.table(value);
        self
    }
    pub fn tagfilter(&mut self, value: bool) -> &mut Self{
        self.extension.tagfilter(value);
        self
    }
    pub fn tasklist(&mut self, value: bool) -> &mut Self{
        self.extension.tasklist(value);
        self
    }
    pub fn front_matter_delimiter(&mut self, value: Option<String>) -> &mut Self{
        self.extension.front_matter_delimiter(value);
        self
    }
    pub fn header_ids(&mut self, value: Option<String>) -> &mut Self{
        self.extension.header_ids(value);
        self
    }

    pub fn escape(&mut self, value: bool) -> &mut Self{
        self.render.escape(value);
        self
    }
    pub fn full_info_string(&mut self, value: bool) -> &mut Self{
        self.render.full_info_string(value);
        self
    }
    pub fn github_pre_lang(&mut self, value: bool) -> &mut Self{
        self.render.github_pre_lang(value);
        self
    }
    pub fn hardbreaks(&mut self, value: bool) -> &mut Self{
        self.render.hardbreaks(value);
        self
    }
    pub fn sourcepos(&mut self, value: bool) -> &mut Self{
        self.render.sourcepos(value);
        self
    }
    pub fn unsafe_(&mut self, value: bool) -> &mut Self{
        self.render.unsafe_(value);
        self
    }
    pub fn width(&mut self, value: usize) -> &mut Self{
        self.render.width(value);
        self
    }
    pub fn list_style(&mut self, value: comrak::ListStyleType) -> &mut Self{
        self.render.list_style(value);
        self
    }

    pub fn build(&self) -> Result<Options, OptionsBuilderError>{
        Ok(Options{
            parse: self.parser.build().map_err(|err|{
                OptionsBuilderError::ParseError(err.to_string())
            })?,
            extension: self.extension.build().map_err(|err|{
                OptionsBuilderError::ExtensionError(err.to_string())
            })?,
            render: self.render.build().map_err(|err|{
                OptionsBuilderError::RenderError(err.to_string())
            })?,
        })
    }
}