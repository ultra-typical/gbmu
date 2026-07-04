use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::{fs, fs::File};

use crate::gui::keymapping::KeyMapping;

use crate::gameboy::GameBoy;
use crate::mmu::HardwareKind;
use crate::mmu::MemoryMapper;
use crate::mmu::mbc::MbcType;

pub(crate) const SAVE_STATE_FILE: &str = "save_state.json";
pub(crate) const SAVE_STATE_TYPES_FILE: &str = "save_state_types.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayedRom {
    pub last_launched: DateTime<Utc>,
    pub rom_name: String,
    pub rom_path: PathBuf,
    pub launch_count: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub keymapping: KeyMapping,
    pub volume: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            keymapping: KeyMapping::default(),
            volume: 100.0,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct GbmuFile {
    pub history: Vec<PlayedRom>,
    pub settings: Settings,

    #[serde(skip)]
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct SaveStateTypes {
    pub hardware: HardwareKind,
    pub cart: MbcType,
}

impl GbmuFile {
    pub fn get_existing_or_new() -> Self {
        let path_main_file = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".gbmu/gbmu.json");
        // let path_save_state_dir = dirs::home_dir()
        //     .expect("Could not find home directory")
        //     .join(".gbmu/save_states/");
        // let path_save_dir = dirs::home_dir()
        //     .expect("Could not find home directory")
        //     .join(".gbmu/saves/");
        let gbmu_file = Self::open_gbmu_file(path_main_file).unwrap();
        // let _save_state_entries = Self::get_entries_or_create_directory(&path_save_state_dir)
        // .expect("Error while getting entries in save state directory");
        // let _save_entries = Self::get_entries_or_create_directory(&path_save_dir)
        //     .expect("Error while getting entries in save directory");
        gbmu_file
    }

    #[allow(clippy::field_reassign_with_default)]
    fn open_gbmu_file(path: PathBuf) -> Result<GbmuFile, std::io::Error> {
        match File::open(&path) {
            Ok(file) => {
                println!("Reading existing file!");
                let mut gbmu: GbmuFile = serde_json::from_reader(file).unwrap_or_else(|e| {
                    eprintln!("Warning: Could not parse config, starting fresh: {e}");
                    GbmuFile::default()
                });
                gbmu.path = path;
                Ok(gbmu)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {
                println!("Creating new file!");
                let dir = path.parent().expect("Path has no parent directory");
                fs::create_dir_all(dir).expect("Could not create ~/.gbmu/");
                let mut gbmu = GbmuFile::default();
                gbmu.path = path;
                gbmu.persist(); // write empty JSON so the file exists
                Ok(gbmu)
            }
            Err(e) => panic!(
                "Something went wrong opening ~/.gbmu/gbmu.json -> {e:?}.\n\
                 If you think this is an error, delete it and restart to create a fresh config."
            ),
        }
    }

    fn get_entries_or_create_directory(path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
        let mut entries: Vec<String> = Vec::new();
        println!("{}", fs::exists(path)?);
        if fs::exists(path)? {
            println!("pass in exist");
            let paths = fs::read_dir(path)?;
            for path in paths {
                entries.push(path?.path().display().to_string());
            }
            Ok(entries)
        } else {
            fs::create_dir(path)?;
            Ok(entries)
        }
    }

    pub fn create_save_state<T: MemoryMapper + Serialize>(
        path: &PathBuf,
        game_boy: &mut GameBoy<T>,
    ) {
        let _now = chrono::offset::Local::now();
        let new_save_state_dir_path = path;
        fs::create_dir_all(new_save_state_dir_path)
            .unwrap_or_else(|_| panic!("Could not create {}", new_save_state_dir_path.display()));

        // The generic types (M in GameBoy<M>) aren't stored in the JSON itself,
        // so we persist them alongside: the loader needs to know which concrete
        // GameBoy<DmgMmu<..>>/GameBoy<CgbMmu<..>> to deserialize back into.
        let types = SaveStateTypes {
            hardware: game_boy.bus.hardware_kind(),
            cart: game_boy.bus.get_cart().kind(),
        };

        match serde_json::to_string_pretty(&*game_boy) {
            Ok(json) => {
                let file = File::create(new_save_state_dir_path.join(SAVE_STATE_FILE))
                    .expect("failed to create save_state.json");
                if let Err(e) = Self::write_save_to_file(file, json) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => eprintln!("Could not serialize save state: {e}"),
        }

        match serde_json::to_string_pretty(&types) {
            Ok(json) => {
                let file = File::create(new_save_state_dir_path.join(SAVE_STATE_TYPES_FILE))
                    .expect("failed to create save_state_types.json");
                if let Err(e) = Self::write_save_to_file(file, json) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => eprintln!("Could not serialize save state types: {e}"),
        }

        let path_file_name = new_save_state_dir_path.join("name.txt");
        let file: File = File::create(path_file_name).expect("Could not create file 'name.txt'");
        if let Err(e) =
            Self::write_save_to_file(file, String::from(path.to_str().unwrap()) + "snapshot.json")
        {
            println!("{}", e);
        }
    }

    pub fn write_save_to_file(mut file: File, json: String) -> Result<(), String> {
        let ret = file.write_all(json.as_bytes());
        let _: () = ret.unwrap();
        Ok(())
    }

    pub fn record_launch(&mut self, rom_name: String, rom_path: PathBuf) {
        if let Some(entry) = self.history.iter_mut().find(|r| r.rom_path == rom_path) {
            entry.last_launched = Utc::now();
            entry.launch_count += 1;
        } else {
            self.history.push(PlayedRom {
                last_launched: Utc::now(),
                rom_name,
                rom_path,
                launch_count: 1,
            });
        }
        self.history
            .sort_by_key(|b| std::cmp::Reverse(b.last_launched));
        self.persist();
    }

    pub fn persist(&self) {
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = fs::write(&self.path, json) {
                    eprintln!("Warning: Could not write to {:?}: {e}", self.path);
                }
            }
            Err(e) => eprintln!("Warning: Could not serialize config: {e}"),
        }
    }
}
