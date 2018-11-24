extern crate inflections;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use json;
use json::JsonValue;
use inflections::case::to_sentence_case;

fn json_to_anim_str(j: JsonValue, name: &str) -> String {
    const state_bp = "state: AnimationState::";
    const fd_bp = "Some(FrameData(";
    const fs_bp = "FrameState::";

    let mut out = String::new();
    out += state_bp;
    out += to_sentence_case(name);
    out += ",";
    
    if !j["hurt"].is_null() {
        if 
    }
    
}

fn parse_boxes(j: JsonValue) -> String {

}

fn main() {
    //get output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{:?}", out_dir);

    // get vector of fighter paths
    let mut fighters: Vec<PathBuf> = vec!();
    let fighter_dir: PathBuf = ["src", "fighters"].iter().collect();
    for f in fighter_dir.read_dir().expect("read_dir call failed") {
        if let Ok(f) = f {
            if let Ok(fm) = f.metadata() {
                if fm.is_dir() {
                    fighters.push(f.path());
                    println!("{:?}", f.path());
                }
            }
        }
    }

    const header = "
        use common::fighter::{Fighter, AnimationArray};
        use common::animation::{AnimationState, FrameData, FrameState, Animation};

        pub fn new<'a>() -> Fighter<'a> {
            let mut aa: AnimationArray = Default::default();
    ";
    const anim_bp = "aa += Animation {";

    for fdir in fighters.iter() {
        let dest_path = Path::new(&out_dir).join(fdir.file_name()).set_extension("rs");
        let mut fout = File::create(&dest_path).unwrap();
        fout.write(&header);

        for fnanim in fdir.read_dir().expect("read_dir call failed") {
            if let Ok(fna) = fnanim {
                let fna = fna.path();
                if let Some(ext) = fna.extenstion() {
                    if ext == "anim" {
                        if Ok(fa) = File::open(&fna).expect(write!("faled to open {}", fna)) {
                            let mut contents = String::new();
                            fa.read_to_string(&mut contents);
                            let pc = json::parse(contents);
                            fout.write(&anim_bp);
                            fout.write(buf: &[u8])
                        }
                    }
                }
            }
        }
    }

    // let dest_path = Path::new(&out_dir).join("hello.rs");
    // let mut f = File::create(&dest_path).unwrap();

    // f.write_all(b"
    //     pub fn message() -> &'static str {
    //         \"Hello, World!\"
    //     }
    // ").unwrap();
}