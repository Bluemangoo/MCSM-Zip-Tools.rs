use std::fs::File;
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

fn add_to_zip(
    writer: &mut ZipWriter<File>,
    path: &Path,
    file_name: &str,
) -> zip::result::ZipResult<()> {
    if path.is_file() {
        let mut file = File::open(path)?;
        writer.start_file(file_name, SimpleFileOptions::default())?;
        std::io::copy(&mut file, writer)?;
    } else if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_name = entry.file_name();
            let entry_name_str = entry_name.to_str().unwrap();

            add_to_zip(
                writer,
                &entry_path,
                &format!("{}/{}", file_name, entry_name_str),
            )?;
        }
    }
    Ok(())
}

pub fn do_zip(zip_path: &str, files: &Vec<String>) -> anyhow::Result<()> {
    println!("ZIP: {:?} --> {}", files, zip_path);
    let zip_file = File::create(zip_path)?;
    let mut writer = ZipWriter::new(zip_file);
    for file in files {
        let path = Path::new(file);
        add_to_zip(
            &mut writer,
            path,
            path.file_name().unwrap().to_str().unwrap(),
        )?;
    }
    Ok(())
}

pub fn do_unzip(zip_path: &str, dest_dir_path: &str) -> anyhow::Result<()> {
    println!("UNZIP: {} --> {}", zip_path, dest_dir_path);
    let zip_file = File::open(zip_path)?;
    let mut f = zip::ZipArchive::new(zip_file)?;
    f.extract(dest_dir_path)?;
    Ok(())
}
