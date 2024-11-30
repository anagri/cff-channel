use anyhow::{anyhow, bail};
use libc::{c_void, size_t};
use libloading::{Library, Symbol};
use std::{ffi::c_char, sync::RwLock};

pub struct DynamicLibrary {
  library: RwLock<Option<Library>>,
}

type Result<T> = std::result::Result<T, anyhow::Error>;
pub type Callback = extern "C" fn(*const c_char, size_t, *mut c_void) -> size_t;

impl DynamicLibrary {
  pub fn new(path: &str) -> Result<Self> {
    let library = unsafe { Library::new(path)? };
    Ok(DynamicLibrary {
      library: RwLock::new(Some(library)),
    })
  }

  pub fn is_loaded(&self) -> Result<bool> {
    let read_guard = self.library.read().map_err(|_| anyhow!("lock error"))?;
    match &*read_guard {
      Some(_) => Ok(true),
      None => Ok(false),
    }
  }

  pub fn clear_lib(&self) -> Result<()> {
    let mut write_guard = self.library.write().map_err(|_| anyhow!("lock error"))?;
    match write_guard.take() {
      Some(loaded_lib) => {
        loaded_lib.close()?;
      }
      None => return Err(anyhow!("library not loaded")),
    }
    Ok(())
  }

  pub fn with_lib<F, R>(&self, f: F) -> Result<R>
  where
    F: FnOnce(&Library) -> Result<R>,
  {
    let read_guard = self.library.read().map_err(|_| anyhow!("lock error"))?;
    match &*read_guard {
      Some(loaded_lib) => f(loaded_lib),
      None => bail!("library not loaded"),
    }
  }

  pub fn callback(
    &self,
    input: *const c_char,
    callback: Callback,
    userdata: *mut c_void,
  ) -> Result<()> {
    self.with_lib(|lib| unsafe {
      let func: Symbol<unsafe extern "C" fn(*const c_char, Callback, *mut c_void)> =
        lib.get(b"async_process")?;
      func(input, callback, userdata);
      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use std::ffi::CString;

  use super::*;

  fn get_lib_path() -> String {
    if cfg!(windows) {
      "libs/callback.dll".to_string()
    } else if cfg!(target_os = "linux") {
      "libs/libcallback.so".to_string()
    } else if cfg!(target_os = "macos") {
      "libs/libcallback.dylib".to_string()
    } else {
      panic!("Unsupported OS");
    }
  }

  #[test]
  fn test_load_library() -> anyhow::Result<()> {
    let library = DynamicLibrary::new(&get_lib_path())?;
    assert!(library.is_loaded()?);
    library.clear_lib()?;
    assert!(!library.is_loaded()?);
    Ok(())
  }

  #[allow(clippy::unnecessary_cast)]
  extern "C" fn test_sync_channel_callback(
    data: *const c_char,
    size: size_t,
    userdata: *mut c_void,
  ) -> size_t {
    let slice = unsafe { std::slice::from_raw_parts(data as *const u8, size) };
    let response = String::from_utf8_lossy(slice).into_owned();
    eprintln!("Received: {}", response);
    let tx = unsafe { &mut *(userdata as *mut std::sync::mpsc::SyncSender<String>) }.clone();
    if let Err(err) = tx.send(response.to_string()) {
      eprintln!("Error sending message to channel: {}", err);
      0
    } else {
      size
    }
  }

  #[test]
  fn test_callback() -> anyhow::Result<()> {
    let library = DynamicLibrary::new(&get_lib_path())?;
    let input = CString::new("hello")?;
    let (mut tx, rx) = std::sync::mpsc::sync_channel::<String>(100);
    library.callback(
      input.as_ptr(),
      test_sync_channel_callback,
      &mut tx as *mut _ as *mut c_void,
    )?;
    let response = rx.recv()?;
    assert_eq!(response, "this is a processed response");
    Ok(())
  }
}
