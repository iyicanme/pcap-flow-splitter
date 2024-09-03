use std::cmp::Ordering;
use std::ffi::OsString;
use std::fs::FileType;
use std::path::Path;

use ratatui::prelude::Style;
use ratatui::widgets::Row;

use crate::error::Error;
use crate::ui::style;

pub struct DirectoryContent {
    content: Vec<DirectoryEntry>,
}

impl DirectoryContent {
    pub fn read(directory: impl AsRef<Path>) -> Result<Self, Error> {
        let mut content = std::fs::read_dir(directory)
            .map_err(Error::ReadDirContent)?
            .filter_map(Result::ok)
            .filter_map(|e| e.file_type().map(|t| (t, e.file_name())).ok())
            .filter_map(|(os_file_type, file_name)| DirectoryEntryType::try_from(os_file_type).map(|file_type| (file_type, file_name)).ok())
            .map(DirectoryEntry::from)
            .collect::<Vec<_>>();

        content.sort();

        Ok(Self { content })
    }

    pub fn iter(&self) -> DirectoryContentIterator {
        DirectoryContentIterator { inner: self, index: 0 }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, i: usize) -> Option<&DirectoryEntry> {
        self.content.get(i)
    }
}

pub struct DirectoryContentIterator<'a> {
    inner: &'a DirectoryContent,
    index: usize,
}

impl<'a> Iterator for DirectoryContentIterator<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let directory_entry = self.inner.content.get(self.index)?;

        let file_type_style = directory_entry.entry_type.style();
        let order_style = style::table::get_row_style_by_index(self.index, false);
        let style = order_style.patch(file_type_style);

        let row = Row::new([directory_entry.display_name.clone()]).style(style);

        self.index += 1;

        Some(row)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct DirectoryEntry {
    entry_type: DirectoryEntryType,
    file_name: OsString,
    sort_name: OsString,
    display_name: String,
}

impl DirectoryEntry {
    pub fn entry_type(&self) -> DirectoryEntryType {
        self.entry_type
    }

    pub fn file_name(&self) -> OsString {
        self.file_name.clone()
    }
}

impl PartialOrd for DirectoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = match (self.entry_type.cmp(&other.entry_type), self.sort_name.cmp(&other.sort_name)) {
            (Ordering::Greater, _) => Ordering::Less,
            (Ordering::Equal, ordering) => ordering,
            (Ordering::Less, _) => Ordering::Greater,
        };

        Some(ordering)
    }
}

impl Ord for DirectoryEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<(DirectoryEntryType, OsString)> for DirectoryEntry {
    fn from(value: (DirectoryEntryType, OsString)) -> Self {
        let sort_name = value.1.to_ascii_lowercase();
        let display_name = value.1.to_string_lossy().to_string();
        Self { entry_type: value.0, file_name: value.1, sort_name, display_name }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub enum DirectoryEntryType {
    File = 1,
    Directory = 2,
    SymbolicLink = 0,
}

impl DirectoryEntryType {
    fn style(self) -> Style {
        match self {
            DirectoryEntryType::File => style::file(),
            DirectoryEntryType::Directory => style::directory(),
            DirectoryEntryType::SymbolicLink => style::symbolic_link(),
        }
    }
}

impl TryFrom<FileType> for DirectoryEntryType {
    type Error = Error;

    fn try_from(value: FileType) -> Result<Self, Self::Error> {
        let file_type = match (value.is_file(), value.is_dir(), value.is_symlink()) {
            (true, false, false) => Self::File,
            (false, true, false) => Self::Directory,
            (false, false, true) => Self::SymbolicLink,
            _ => return Err(Error::FileTypeConversion)
        };

        Ok(file_type)
    }
}