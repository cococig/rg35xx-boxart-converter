extern crate image;

use image::{imageops, DynamicImage};
use std::fs;
use std::path::Path;

fn is_supported_extension(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        if let Some(extension_str) = extension.to_str() {
            return extension_str.eq_ignore_ascii_case("jpeg")
                || extension_str.eq_ignore_ascii_case("jpg")
                || extension_str.eq_ignore_ascii_case("png");
        }
    }
    false
}

fn process_image(input_path: &Path, output_path: &Path) {
    // 画像を開く
    let image = image::open(input_path).expect("Failed to open image");

    // 指定された幅に合わせてアスペクト比を保ちつつリサイズ
    let target_width = 340;
    let aspect_ratio = (image.height() as f64) / (image.width() as f64);
    let target_height = (target_width as f64 * aspect_ratio) as u32;

    // `resize`の返り値をDynamicImage::ImageRgba8にラップ
    let resized_image = DynamicImage::ImageRgba8(imageops::resize(
        &image,
        target_width,
        target_height,
        image::imageops::FilterType::Nearest,
    ));

    // アウトプット画像を作成
    let output_width = 640;
    let output_height = 480;
    let mut output_image = DynamicImage::new_rgba8(output_width, output_height);

    // 位置を高さの中心に揃えるためのオフセット計算
    let y_offset = if target_height > output_height {
        0 as i64
    } else {
        ((output_height - target_height) / 2) as i64
    };

    // リサイズされた画像をアウトプット画像に重ねる
    let (x_offset, y_offset_resized) = (0, y_offset);
    imageops::overlay(
        &mut output_image,
        &resized_image,
        x_offset,
        y_offset_resized,
    );

    // 出力先のパスを作成
    let output_file_name = output_path.join(input_path.file_name().expect("Invalid file name"));
    let output_file_name = output_file_name.with_extension("png");

    // 画像を保存
    output_image
        .save(output_file_name)
        .expect("Failed to save image");
}

fn process_directory(input_dir: &Path, output_dir: &Path) {
    // 入力ディレクトリ内のファイルを取得
    let entries = fs::read_dir(input_dir).expect("Failed to read directory");

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && is_supported_extension(&path) {
                // ファイルの場合は処理を行う
                process_image(&path, output_dir);
            } else if path.is_dir() {
                // ディレクトリの場合は再帰的に処理を行う
                let new_output_dir =
                    output_dir.join(path.file_name().expect("Invalid directory name"));
                fs::create_dir_all(&new_output_dir).expect("Failed to create output directory");
                process_directory(&path, &new_output_dir);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_directory> <output_directory>", args[0]);
        std::process::exit(1);
    }

    let input_dir = Path::new(&args[1]);
    let output_dir = Path::new(&args[2]);

    if !input_dir.exists() || !input_dir.is_dir() {
        eprintln!("Input directory does not exist or is not a directory.");
        std::process::exit(1);
    }

    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    process_directory(input_dir, output_dir);

    println!("Image processing complete.");
}
