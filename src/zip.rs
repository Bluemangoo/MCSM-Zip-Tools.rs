use anyhow::anyhow;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, Write};
use std::path::Path;
use zifu_core::filename_decoder::IDecoder;
use zifu_core::InputZIPArchive;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

fn add_to_zip(
    writer: &mut ZipWriter<File>,
    path: &Path,
    file_name: &str,
) -> zip::result::ZipResult<()> {
    if path.is_file() {
        let mut file = File::open(path)?;
        writer.start_file(
            file_name,
            SimpleFileOptions::default().compression_method(CompressionMethod::Deflated),
        )?;
        std::io::copy(&mut file, writer)?;
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
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

pub fn do_unzip(
    zip_path: &str,
    dest_dir_path: &str,
    encode: Option<&String>,
) -> anyhow::Result<()> {
    println!("UNZIP: {} --> {}", zip_path, dest_dir_path);
    let zip = zifu(zip_path, encode).or_else(|_| fs::read(zip_path))?;
    unzip_zip(zip, dest_dir_path)
}

fn zifu(zip_path: &str, encode: Option<&String>) -> anyhow::Result<Vec<u8>> {
    let mut zip = InputZIPArchive::new(BufReader::new(File::open(zip_path)?))?;
    zip.check_unsupported_zip_type()?;
    let legacy_decoder = if let Some(encoding_name) = encode {
        <dyn IDecoder>::from_encoding_name(encoding_name)
            .unwrap_or(<dyn IDecoder>::native_oem_encoding())
    } else {
        <dyn IDecoder>::native_oem_encoding()
    };
    let utf8_decoder = <dyn IDecoder>::utf8();
    let ascii_decoder = <dyn IDecoder>::ascii();
    let decoders_list = vec![&*ascii_decoder, &*legacy_decoder, &*utf8_decoder];
    // Detect encoding by trying decoding all of file names and comments
    let best_fit_decoder_index_ = zip.get_filename_decoder_index(&decoders_list);
    best_fit_decoder_index_.ok_or(anyhow!(
        "file names & comments are not encoded in UTF-8 or {}.  Try with -e <another encoding> option.",
        legacy_decoder.encoding_name()
    ))?;
    let guessed_encoder = decoders_list[best_fit_decoder_index_.unwrap()];
    zip.convert_central_directory_file_names(guessed_encoder);
    let mut buffer = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buffer);
        zip.output_archive_with_central_directory_file_names(&mut writer)?;
        writer.flush()?;
    }
    Ok(buffer)
}

fn unzip_zip(buffer: Vec<u8>, dest_dir_path: &str) -> anyhow::Result<()> {
    let mut f = ZipArchive::new(BufReader::new(Cursor::new(&buffer)))?;
    f.extract(dest_dir_path)?;
    Ok(())
}
