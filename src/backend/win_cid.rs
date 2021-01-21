//! Windows Common Item Dialog
//! Win32 Vista
use crate::FileDialog;
use crate::FileHandle;

use std::path::PathBuf;

use winapi::shared::winerror::HRESULT;

mod utils;
use utils::init_com;

mod win_dialog;
use win_dialog::IDialog;

mod async_dialog;
pub use async_dialog::{AsyncDialog, DialogFuture};

use super::DialogFutureType;

//
// File Picker
//

use crate::backend::FilePickerDialogImpl;
impl FilePickerDialogImpl for FileDialog {
    fn pick_file(self) -> Option<PathBuf> {
        fn run(opt: FileDialog) -> Result<PathBuf, HRESULT> {
            init_com(|| {
                let dialog = IDialog::build_pick_file(&opt)?;
                dialog.show()?;
                dialog.get_result()
            })?
        }
        run(self).ok()
    }

    fn pick_files(self) -> Option<Vec<PathBuf>> {
        fn run(opt: FileDialog) -> Result<Vec<PathBuf>, HRESULT> {
            init_com(|| {
                let dialog = IDialog::build_pick_files(&opt)?;
                dialog.show()?;
                dialog.get_results()
            })?
        }
        run(self).ok()
    }
}

use crate::backend::AsyncFilePickerDialogImpl;
impl AsyncFilePickerDialogImpl for FileDialog {
    fn pick_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        let ret: DialogFuture<_> =
            AsyncDialog::new(move || IDialog::build_pick_file(&self).ok()).into();
        Box::pin(ret)
    }

    fn pick_files_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        let ret: DialogFuture<_> =
            AsyncDialog::new(move || IDialog::build_pick_files(&self).ok()).into();
        Box::pin(ret)
    }
}

//
// Folder Picker
//

use crate::backend::FolderPickerDialogImpl;
impl FolderPickerDialogImpl for FileDialog {
    fn pick_folder(self) -> Option<PathBuf> {
        fn run(opt: FileDialog) -> Result<PathBuf, HRESULT> {
            init_com(|| {
                let dialog = IDialog::build_pick_folder(&opt)?;
                dialog.show()?;
                dialog.get_result()
            })?
        }

        run(self).ok()
    }
}

use crate::backend::AsyncFolderPickerDialogImpl;
impl AsyncFolderPickerDialogImpl for FileDialog {
    fn pick_folder_async(self) -> DialogFutureType<Option<FileHandle>> {
        let ret: DialogFuture<_> =
            AsyncDialog::new(move || IDialog::build_pick_folder(&self).ok()).into();
        Box::pin(ret)
    }
}

//
// File Save
//

use crate::backend::FileSaveDialogImpl;
impl FileSaveDialogImpl for FileDialog {
    fn save_file(self) -> Option<PathBuf> {
        fn run(opt: FileDialog) -> Result<PathBuf, HRESULT> {
            init_com(|| {
                let dialog = IDialog::build_save_file(&opt)?;
                dialog.show()?;
                dialog.get_result()
            })?
        }

        run(self).ok()
    }
}

use crate::backend::AsyncFileSaveDialogImpl;
impl AsyncFileSaveDialogImpl for FileDialog {
    fn save_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        let ret: DialogFuture<_> =
            AsyncDialog::new(move || IDialog::build_save_file(&self).ok()).into();
        Box::pin(ret)
    }
}

//
// Message Dialog
//

use crate::backend::MessageDialogImpl;
use crate::MessageDialog;

impl MessageDialogImpl for MessageDialog {
    fn show(self) {
        unimplemented!("");
    }
}
