use std::error::Error;
use std::io;

use clap::Parser;
use regex::Regex;

use crate::args::argument::{Args, Commands, SetStruct};
use crate::config::config::{ConfigRoot, WebsiteBuilder};
use crate::context::{const_context, context_func::invoke_browser};
use crate::lifecycle::Lifecycle;

pub struct BrowsercherApp {
    args: Args,
    running: bool,
}

impl BrowsercherApp {
    pub fn new() -> Self {
        let args = Args::parse();
        Self {
            args,
            running: false,
        }
    }

    fn handle_set_command(&self, set: &SetStruct) -> Result<(), Box<dyn Error>> {
        let website = WebsiteBuilder::default()
            .name(set.name.clone())
            .url(set.url.clone())
            .aliases(set.alias.clone())
            .rules(set.rules.clone())
            .build()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        ConfigRoot::add_website(website)?;
        Ok(())
    }

    fn handle_list_command(&self) -> Result<(), Box<dyn Error>> {
        ConfigRoot::list();
        Ok(())
    }

    fn handle_browse_command(&self) -> Result<(), Box<dyn Error>> {
        let url = match self.args.url.as_deref() {
            Some(url) => url,
            None => {
                eprintln!("❌ 请输入网址或别名！");
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "missing url or alias",
                )));
            }
        };

        let config = &const_context::CFG_STRUCT.config;

        if let Some(found_url) = config.find_url_by_alias(url) {
            invoke_browser(
                found_url,
                self.args.text.as_deref(),
                config.find_rule_by_url(found_url),
            )?;
            return Ok(());
        }

        let url_pattern = Regex::new(r"^https?://.*$")?;
        if url_pattern.is_match(url) {
            invoke_browser(url, None, None)?;
        } else {
            eprintln!("❌ 请输入正确的网址！");
        }

        Ok(())
    }
}

impl Lifecycle for BrowsercherApp {
    fn on_start(&mut self) -> Result<(), Box<dyn Error>> {
        self.running = true;
        Ok(())
    }

    fn on_run(&mut self) -> Result<(), Box<dyn Error>> {
        match &self.args.command {
            Some(Commands::Set(set)) => self.handle_set_command(set)?,
            Some(Commands::List) => self.handle_list_command()?,
            None => self.handle_browse_command()?,
        }

        Ok(())
    }

    fn on_stop(&mut self) -> Result<(), Box<dyn Error>> {
        if self.running {
            self.running = false;
        }
        Ok(())
    }

    fn on_cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
