use rosbag2_rs::reader::Reader;
use std::env;
use std::path::PathBuf;
// use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello");

    // ホームディレクトリを取得
    let home_dir = env::var("HOME")?;

    // ファイルパスを組み立てる
    let mut path = PathBuf::from(home_dir);
    path.push("rosbags/20240808_tlr_toyama/5a12ba83-d00a-4d19-88bc-ced99e1d508c");

    // ROS 2のbagファイルを開く
    println!("Path to rosbag: {:?}", path);
    // let metapath = path.join("metadata.yaml");
    // let metadata_contents = fs::read_to_string(metapath).unwrap();
    // println!("{}", metadata_contents);
    if let Some(path_str) = path.to_str() {
        let reader_result = Reader::new(path_str);
        match reader_result {
            Ok(_) => {
                println!("Successfully opened the rosbag.");
                // ここで他の処理を続ける
            }
            Err(e) => {
                eprintln!("Failed to open the rosbag: {}", e);
                // 必要に応じて他のエラーハンドリングを行う
            }
        }
    } else {
        eprintln!("Invalid path: not a valid UTF-8 string.");
    }

    Ok(())
}
