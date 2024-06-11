use std::cell::RefCell;
use std::iter;

use log::Level;
use notcurses::{Alpha, Channel};
use crate::notrogue::gui::util::wrap_text;

use super::GuiTrait;

pub struct LogItem {
    level: Level,
    log_str: String
}

thread_local! {
    static LOGS: RefCell<Vec<LogItem>> = RefCell::new(Vec::new());
}

pub struct LogMemory;

impl log::Log for LogMemory {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            LOGS.with_borrow_mut(|v| {
                v.push(LogItem { level: record.level(), log_str: record.args().to_string() })
            });
        }
    }
    
    fn flush(&self) {}
}

pub struct Logger;

impl GuiTrait for Logger {
    fn on_render(&self, _nc: &mut notcurses::Notcurses, cli: &mut notcurses::Plane) -> notcurses::NotcursesResult<Vec<notcurses::Plane>> {
        let height = cli.size().h() as u32;
        LOGS.with_borrow(|v| -> notcurses::NotcursesResult<()> {
            if v.is_empty() { return Ok(()) }
            let mut i = 0u32;
            'logs: for current_log in v.iter().rev() {
                for s in wrap_text(&current_log.log_str, cli.size().w() as u32).iter().rev() {
                    if height < i + 1 {
                        break 'logs;
                    }
                    cli.putstr_at_xy(Some(0), Some(height - 1 - i), s)?;
                    i += 1;
                }
            }
            Ok(())
        })?;
        cli.set_fg((255, 0, 0));
        cli.set_bg(Channel::from_rgb_alpha((0, 0, 0), Alpha::Blend));
        cli.putstr_at_xy(Some(0), Some(0), &LOGS.with_borrow(|v| { v.len() }).to_string())?;
        Ok(Vec::new())
    }
}