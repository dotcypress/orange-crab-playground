use btoi::btou_radix;
use core::fmt::Write;
use orange_crab_hal::led::*;
use orange_crab_hal::{serial, uart1};
use ushell::{autocomplete::*, history::*, *};

const SHELL_PROMPT: &str = "» ";
const CR: &str = "\r\n";
const HELP: &str = "\r\n\
\x1b[33mOrangeCrab\x1b[0m Shell\r\n\r\n\
USAGE:\r\n\
\tcommand [arg]\r\n\r\n\
COMMANDS:\r\n\
\ton        Switch led on\r\n\
\tcolor     Set led color\r\n\
\toff       Switch led off\r\n\
\tstatus    Print led status\r\n\
\tclear     Clear screen\r\n\
\thelp      Print this message\r\n
";

const MAX_COMMAND_LEN: usize = 16;
const HISTORY_SIZE: usize = 4;
const AUTOCOMPLETE_SIZE: usize = 6;

pub type Serial = serial::Serial<uart1::Instance>;
pub type Autocomplete = StaticAutocomplete<{ AUTOCOMPLETE_SIZE }>;
pub type History = LRUHistory<{ MAX_COMMAND_LEN }, { HISTORY_SIZE }>;
pub type Shell = UShell<Serial, Autocomplete, History, { MAX_COMMAND_LEN }>;
pub type EnvResult = SpinResult<Serial, ()>;

pub struct Env {
    led: Led,
}

impl Env {
    pub fn new(led: Led) -> Self {
        Self { led }
    }

    fn help_cmd(&mut self, shell: &mut Shell, _args: &str) -> EnvResult {
        shell.write_str(HELP)?;
        Ok(())
    }

    fn set_color_cmd(&mut self, shell: &mut Shell, args: &str) -> EnvResult {
        match btou_radix::<u32>(args.as_bytes(), 16) {
            Ok(color) if color <= 0xFFF => {
                self.led.set_color(color.into());
                shell.write_str(CR)?;
            }
            _ => write!(shell, "{0:}unsupported color{0:}", CR)?,
        }
        Ok(())
    }

    fn status_cmd(&mut self, shell: &mut Shell, _args: &str) -> EnvResult {
        let status = if self.led.enabled() { "On" } else { "Off" };
        let color: u32 = self.led.color().into();
        write!(
            shell,
            "{0:}Led status: {1:}; Color: 0x{2:x} {0:}",
            CR, status, color
        )?;
        Ok(())
    }
}

impl Environment<Serial, Autocomplete, History, (), { MAX_COMMAND_LEN }> for Env {
    fn command(&mut self, shell: &mut Shell, cmd: &str, args: &str) -> EnvResult {
        match cmd {
            "clear" => shell.clear()?,
            "help" => self.help_cmd(shell, args)?,
            "status" => self.status_cmd(shell, args)?,
            "color" => self.set_color_cmd(shell, args)?,
            "on" => {
                self.led.enable();
                shell.write_str(CR)?;
            }
            "off" => {
                self.led.disable();
                shell.write_str(CR)?;
            }
            "" => shell.write_str(CR)?,
            _ => write!(shell, "{0:}unsupported command{0:}", CR)?,
        }

        shell.write_str(SHELL_PROMPT)?;
        Ok(())
    }

    fn control(&mut self, shell: &mut Shell, code: u8) -> EnvResult {
        if code == control::CTRL_B {
            shell.bell()?;
        }
        Ok(())
    }
}
