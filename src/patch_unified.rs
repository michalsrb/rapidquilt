
    // Same order as MatchedMetadata enum
    static ref METADATA: RegexSet = RegexSet::new(&[
        r"^index", // TODO: ?
        r"^old mode +(?P<permissions>[0-9]+)$",
        r"^new mode +(?P<permissions>[0-9]+)$",
        r"^deleted file mode +(?P<permissions>[0-9]+)$",
        r"^new file mode +(?P<permissions>[0-9]+)$",
        r"^rename from ",  // patch ignores the filename behind this
        r"^rename to ",    // patch ignores the filename behind this
        r"^copy from ",    // patch ignores the filename behind this
        r"^copy to ",      // patch ignores the filename behind this
        r"^GIT binary patch", // TODO: ???
    ]).unwrap();
}

// Same order as METADATA RegexSet!
#[repr(usize)]
enum MatchedMetadata {
    Index = 0,
    OldMode,
    NewMode,
    DeletedFileMode,
    NewFileMode,
    RenameFrom,
    RenameTo,
    CopyFrom,
    CopyTo,
    GitBinaryPatch
}

struct FilePatchMetadata {
    rename_from: bool,
    rename_to: bool,
}

impl Default for FilePatchMetadata {
    fn default() -> Self {
        FilePatchMetadata {
            rename_from: false,
            rename_to: false,
        }
    }
    let mut filepatch_metadata = FilePatchMetadata::default();

        if let Some(metadata) = METADATA.matches(line).iter().next() { // There will be at most one match because the METADATA regexes are mutually exclusive
            // TODO: Use TryFrom instead of transmute when stable.
            match unsafe { std::mem::transmute::<usize, MatchedMetadata>(metadata) } {
                MatchedMetadata::RenameFrom => {
                    // We do not actually care about the filename written next to the "rename from" line.
                    // patch doesn't care either
                    filepatch_metadata.rename_from = true;
                },
                MatchedMetadata::RenameTo => {
                    // We do not actually care about the filename written next to the "rename to" line.
                    // patch doesn't care either
                    filepatch_metadata.rename_to = true;
                },
                MatchedMetadata::GitBinaryPatch => {
                    return Err(format_err!("GIT binary patch is not supported!"));
                }
                _ => {
                    // TODO: Handle the other metadata...
                }
            }
            continue;
        }

            let (kind, filename, other_filename) = if minus_filename == NULL_FILENAME {
                (FilePatchKind::Create, plus_filename, None)
                (FilePatchKind::Delete, minus_filename, None)
                (FilePatchKind::Modify, plus_filename, Some(minus_filename))
            fn strip_filename(filename: &[u8], strip: usize) -> Result<PathBuf, Error> {
                let filename = PathBuf::from(OsStr::from_bytes(filename));
                if !filename.is_relative() {
                    return Err(format_err!("Path in patch is not relative: \"{:?}\"", filename));
                }

                Ok(components.as_path().to_path_buf())
            }

            let filename = strip_filename(filename, strip)?;
            if filepatch_metadata.rename_from && filepatch_metadata.rename_to && other_filename.is_some() {
                let original_filename = strip_filename(other_filename.unwrap(), strip)?;
                FilePatch::new_renamed(kind, filename, original_filename)
            } else {
                FilePatch::new(kind, filename)
            }