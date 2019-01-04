use crate::line_interner::{LineId, LineInterner};
use crate::util::split_lines_with_endings;
const NO_NEW_LINE_TAG: &[u8] = b"\\ No newline at end of file\n";
    static ref MINUS_FILENAME: Regex = Regex::new(r"^--- ([^\t]+)\n$").unwrap();
    static ref PLUS_FILENAME: Regex = Regex::new(r"^\+\+\+ ([^\t]+)\n$").unwrap();
    static ref CHUNK: Regex = Regex::new(r"^@@ -(?P<remove_line>[\d]+)(?:,(?P<remove_count>[\d]+))? \+(?P<add_line>[\d]+)(?:,(?P<add_count>[\d]+))? @@?(?P<place_name>.*)\n$").unwrap();

    static ref DIFF_GIT: Regex = Regex::new(r"^diff --git +(?P<oldfilename>[^ ]+) +(?P<newfilename>[^ ]+)\n$").unwrap();
        r"^git --diff ", // this is exactly what patch uses to recognize end of hunk-less filepatch
        r"^old mode +(?P<permissions>[0-9]+)\n$",
        r"^new mode +(?P<permissions>[0-9]+)\n$",
        r"^deleted file mode +(?P<permissions>[0-9]+)\n$",
        r"^new file mode +(?P<permissions>[0-9]+)\n$",
#[allow(unused)]
    GitDiffSeparator = 0,
    Index,
struct FilePatchMetadata<'a> {
    old_filename: Option<&'a [u8]>,
    new_filename: Option<&'a [u8]>,
impl<'a> Default for FilePatchMetadata<'a> {
            old_filename: None,
            new_filename: None,
fn new_filepatch<'a>(filepatch_metadata: &FilePatchMetadata, strip: usize) -> Result<Option<TextFilePatch<'a>>, Error> {
    if let (Some(old_filename), Some(new_filename)) = (filepatch_metadata.old_filename, filepatch_metadata.new_filename) {
        let (kind, filename, other_filename) = if old_filename == NULL_FILENAME {
            (FilePatchKind::Create, new_filename, None)
        } else if new_filename == NULL_FILENAME {
            (FilePatchKind::Delete, old_filename, None)
        } else {
            // TODO: What to do if new_filename and old_filename differ after stripping the beginning?

            (FilePatchKind::Modify, new_filename, Some(old_filename))
        };

        fn strip_filename(filename: &[u8], strip: usize) -> Result<PathBuf, Error> {
            let filename = PathBuf::from(OsStr::from_bytes(filename));
            if !filename.is_relative() {
                return Err(format_err!("Path in patch is not relative: \"{:?}\"", filename));
            }

            let mut components = filename.components();
            for _ in 0..strip { components.next(); }
            Ok(components.as_path().to_path_buf())
        }

        let filename = strip_filename(filename, strip)?;

        if filepatch_metadata.rename_from && filepatch_metadata.rename_to && other_filename.is_some() {
            let other_filename = strip_filename(other_filename.unwrap(), strip)?;
            Ok(Some(FilePatch::new_renamed(kind, other_filename, filename)))
        } else {
            Ok(Some(FilePatch::new(kind, filename)))
        }
    } else {
        Ok(None)
    }
}

    let mut lines = split_lines_with_endings(bytes).peekable();
    while let Some(line) = lines.peek() {
            lines.next();
        if let Some(capture) = MINUS_FILENAME.captures(line) {
            filepatch_metadata.old_filename = Some(capture.get(1).unwrap().as_bytes());
            lines.next();
            continue;
        }
        if let Some(capture) = PLUS_FILENAME.captures(line) {
            filepatch_metadata.new_filename = Some(capture.get(1).unwrap().as_bytes());
            lines.next();
            continue;
        }
        if let Some(capture) = DIFF_GIT.captures(line) {
            // patch uses "diff --git " as a separator that can mean a filepatch ended even if it had no hunks
            {
                if let Some(file_patch) = new_filepatch(&filepatch_metadata, strip)? {
                    file_patches.push(file_patch);
                filepatch_metadata = FilePatchMetadata::default();
            filepatch_metadata.old_filename = Some(capture.name("oldfilename").unwrap().as_bytes());
            filepatch_metadata.new_filename = Some(capture.name("newfilename").unwrap().as_bytes());
            lines.next();
            continue;
        }

        if !CHUNK.is_match(line) {
            lines.next();
            continue;
        }

        let mut file_patch = match new_filepatch(&filepatch_metadata, strip)? {
            Some(file_patch) => file_patch,
            None => {
                // TODO: Better error reporting...
                return Err(format_err!("Badly formated patch!"));
        filepatch_metadata = FilePatchMetadata::default();
                file_patch.change_kind(FilePatchKind::Create);
                file_patch.change_kind(FilePatchKind::Delete);
            if file_patch.kind() == FilePatchKind::Create {
            if file_patch.kind() == FilePatchKind::Delete {
                let line = if line == b"\n" {
                    b" \n"
                // Check for the "No newline..." tag
                if lines.peek() == Some(&NO_NEW_LINE_TAG) && line_content.last() == Some(&b'\n') {
                    // Cut away the '\n' from the end of the line. It does not belong to the content,
                    // it is just there for patch formating.
                    line_content = &line_content[..(line_content.len()-1)];

                    // Skip the line with the tag
                    lines.next();
                }

            file_patch.hunks.push(hunk);
        file_patches.push(file_patch);
        let add_count = self.add.content.len();
        let remove_count = self.remove.content.len();
        let add_line = if add_count == 0 {
        let remove_line = if remove_count == 0 {
        let mut write_line = |c: u8, line_id: LineId| -> Result<(), Error> {
            let line = interner.get(line_id).unwrap(); // NOTE(unwrap): Must succeed, we are printing patch that was already interned. If it is not there, it is a bug.
            writer.write(&[c])?;
            writer.write(line)?;
            if line.last() != Some(&b'\n') {
                // If the line doesn't end with newline character, we have to write it ourselves
                // (otherwise it would not be valid patch file), but we also print the "No newline..."
                // tag which informs that the newline is not part of the file.
                writer.write(NO_NEW_LINE_TAG)?;
                write_line(b'-', remove[remove_i])?;
                write_line(b'+', add[add_i])?;
                write_line(b' ', remove[remove_i])?;