use crate::error::BResult;
use base64::Engine;
use std::{
    fs::File,
    io::{BufWriter, Write},
};
byond_fn!(fn file_write(data, path, ...rest) {
    let mut should_decode_b64 = false;
    if rest.first().map(|x| &**x) == Some("true") {
        should_decode_b64 = true;
    }
    write(data, path, should_decode_b64).err()
});

fn write(data: &str, path: &str, base64decode: bool) -> BResult<usize> {
    let path: &std::path::Path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = BufWriter::new(File::create(path)?);

    let written = if base64decode {
        file.write(
            base64::prelude::BASE64_STANDARD
                .decode(data)
                .unwrap()
                .as_ref(),
        )?
    } else {
        file.write(data.as_bytes())?
    };

    file.flush()?;
    file.into_inner()
        .map_err(|e| std::io::Error::new(e.error().kind(), e.error().to_string()))? // This is god-awful, but the compiler REFUSES to let me get an owned copy of `e`
        .sync_all()?;

    Ok(written)
}
