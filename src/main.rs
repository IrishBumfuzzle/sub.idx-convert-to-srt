use vobsub::Index;
use image::RgbaImage;
use image::imageops::colorops::invert;
use std::process::Command;
use std::fs::File;
use std::fs;
use std::fs::remove_file;
use std::io::prelude::*;
use toml::Value;

fn convert_file(mut srt: &File, idx: Index, tesseract: &str) {
    let mut sub_number = 1; //Serial number of subtitle as required by srt format
    let mut sub_time: f64 = 0.0;

    for sub in idx.subtitles() {
        let sub = sub.unwrap();

        //Stops when reached the end of first language of subtitles, removing this will put every language's subtitles in one file
        if sub.start_time() < sub_time {
            break;
        }
        else {
            sub_time = sub.start_time();
        }

        let mut img: RgbaImage = sub.to_image(idx.palette());
        invert(&mut img); //Inverts image as required by tesseract (dark text on light background)
        img.save("DONT_DELETE.png").unwrap();

        Command::new(tesseract).arg("DONT_DELETE.png").arg("DONT_DELETE").output().expect("Failed to execute tesseract, ensure path is correct");
        let mut file = File::open("DONT_DELETE.txt").unwrap();
        let mut subs = String::new();
        file.read_to_string(&mut subs).expect("Can't open srt file for some reason");

        let start_time = time_parse(sub.start_time());
        let end_time = time_parse(sub.end_time());
        let time_stamp = format!("{:02}:{:02}:{:02},{:03} --> {:02}:{:02}:{:02},{:03}", start_time.0, start_time.1, start_time.2, start_time.3, end_time.0, end_time.1, end_time.2, end_time.3);

        subs = subs.replace("|", "I"); //Bug with tesseract, recognizes capital I as | symbol

        srt.write((&format!("{}\n{}\n{}\n", sub_number, time_stamp, subs)).as_bytes()).expect("Can't write to srt file for some reason");
        sub_number += 1;
    }

    remove_file("DONT_DELETE.png").expect("couldn't delete file, now you can delete the png and txt yourself");
    remove_file("DONT_DELETE.txt").expect("couldn't delete file, now you can delete the png and txt yourself");
    
}



fn main() {

    let mut conf_file = File::open("conf.toml").expect("Cannot find conf.toml");
    let mut conf = String::new();
    conf_file.read_to_string(&mut conf).expect("Cannot read conf.toml");

    let conf = conf.parse::<Value>().unwrap();

    if conf["mode"].as_str().unwrap() == "all" {
        let entries = fs::read_dir(".").unwrap()
        .map(|res| res.map(|e| e.path()));
        // .collect::<Vec<_>>();
        
        for fi in entries {
            let mut file_name = fi.unwrap().to_str().unwrap().to_string();
            if !file_name.contains(".idx") {
                continue;
            }
            file_name = file_name.replace(".idx", "");

            let idx = Index::open(format!("{}.idx", file_name)).expect("Check if both .sub and .idx files exist and have the same name"); //opens idx file
            let mut srt = File::create(format!("{}.srt", file_name)).unwrap(); //opens srt file to write to

            println!("Converting {}", file_name);
            convert_file(&mut srt, idx, conf["tesseract"].as_str().unwrap());
            println!("Converted {}", file_name);

        }
    }


    else {
        let file_name = conf["file"].as_str().unwrap();
        let idx = Index::open(format!("{}.idx", file_name)).expect("Check if both .sub and .idx files exist and have the same name"); //opens idx file
        let mut srt = File::create(format!("{}.srt", file_name)).unwrap(); //opens srt file to write to

        println!("Converting {}", file_name);
        convert_file(&mut srt, idx, conf["tesseract"].as_str().unwrap());
        println!("Converted {}", file_name);
    }

    
}


// Converts second into (h, m, s, ms) required by srt format
fn time_parse(x: f64) -> (u32, u32, u32, u32) {
    let mut x = (x * 1000.0) as u32;
    let h = x / (60*60*1000);
    x = x - h*(60*60*1000);
    let m = x / (60*1000);
    x = x - m*(60*1000);
    let s = x / 1000;
    x = x - s*1000;
    (h,m,s,x)
}