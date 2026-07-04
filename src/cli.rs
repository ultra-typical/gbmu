use crate::gui::GbType;
use clap::{Arg, ArgAction, command, value_parser};
use std::fs::metadata;
use std::io::ErrorKind;

#[derive(Debug, Clone)]
pub struct EmulatorArguments {
    pub rom_path: Option<String>,
    pub boot_rom: bool,
    pub gb_type: Option<GbType>,
}

impl EmulatorArguments {
    pub fn get() -> Result<Self, String> {
        let matches = command!()
            .arg(Arg::new("rom_path").help("The path of the rom you want to launch."))
            .arg(
                Arg::new("boot_rom")
                    .short('b')
                    .long("boot_rom")
                    .action(ArgAction::SetTrue)
                    .help("If set, nintendo basic boot rom will boot first."),
            )
            .arg(
                Arg::new("type")
                    .short('t')
                    .long("type")
                    .value_parser(value_parser!(GbType))
                    .help("Set the type of gameboy to emulate. (Cgb or Dmg)"),
            )
            .get_matches();

        // path is specified in cli command
        let rom_path = matches.get_one::<String>("rom_path").map(String::from);

        // boot_with_nintendo_room
        let boot_rom = matches.get_flag("boot_rom");
        let gb_type = matches.get_one::<GbType>("type");

        let unchecked = Self {
            rom_path,
            boot_rom,
            gb_type: gb_type.cloned(),
        };

        unchecked.check_fields()
    }

    pub fn check_fields(self) -> Result<Self, String> {
        let mut errors: Vec<String> = vec![];

        // Put checks here
        if let Err(error) = self.check_rom_path() {
            errors.push(String::from("rom_path : ") + error.as_str());
        }

        if errors.is_empty() {
            Ok(self)
        } else {
            errors.push(String::from(""));
            Err(errors.join("\n"))
        }
    }

    pub fn check_rom_path(&self) -> Result<(), String> {
        let Some(path) = &self.rom_path else {
            return Ok(());
        };
        use std::os::unix::fs::PermissionsExt;
        match metadata(path) {
            Ok(meta) => {
                if meta.permissions().mode() & 0o400 == 0 {
                    Err(String::from(path) + " : Not allowed to read file")
                } else if meta.is_file() {
                    Ok(())
                } else {
                    Err(String::from(path) + " : Path doesn't refer to a file")
                }
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Err(String::from(path) + " : File not found"),
                ErrorKind::PermissionDenied => Err(String::from(path) + " : Not allowed"),
                ErrorKind::NotADirectory | ErrorKind::InvalidInput => {
                    Err(String::from(path) + " : Invalid path")
                }
                ErrorKind::Unsupported => Err(String::from(path) + " : Unsuported stats read"),
                other => Err(format!("Unexpected error: {:?}, {}", other, e)),
            },
        }
    }
}
