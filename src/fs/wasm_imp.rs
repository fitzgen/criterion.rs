use failure::{bail, format_err};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::path::Path;
use web_sys;

use error::Result;
use report::BenchmarkId;

fn window() -> Result<web_sys::Window> {
    web_sys::window().ok_or_else(|| format_err!("No window -- are we in a worker or node.js?"))
}

fn get_local_storage() -> Result<web_sys::Storage> {
    let win = window()?;
    Ok(win.local_storage().unwrap().unwrap())
}

pub fn load<A, P: ?Sized>(path: &P) -> Result<A>
where
    A: DeserializeOwned,
    P: AsRef<Path>,
{
    let storage = get_local_storage()?;
    let key = path.as_ref().display().to_string();
    match storage
        .get_item(&key)
        .map_err(|_| format_err!("Storage#getItem failed"))?
    {
        None => bail!("no item with key = {}", key),
        Some(item) => {
            let result: A = serde_json::from_str(item.as_str())?;
            Ok(result)
        }
    }
}

pub fn is_dir<P>(_path: &P) -> bool
where
    P: AsRef<Path>,
{
    true
}

pub fn mkdirp<P>(_path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    Ok(())
}

pub fn cp(from: &Path, to: &Path) -> Result<()> {
    let storage = get_local_storage()?;
    let from = from.display().to_string();
    match storage
        .get_item(&from)
        .map_err(|_| format_err!("Storage#getItem failed"))?
    {
        None => bail!("no item with key = {}", from),
        Some(item) => {
            let to = to.display().to_string();
            storage
                .set_item(&to, &item)
                .map_err(|_| format_err!("Storage#setItem failed"))
        }
    }
}

pub fn save<D, P>(data: &D, path: &P) -> Result<()>
where
    D: Serialize,
    P: AsRef<Path>,
{
    let buf = serde_json::to_string(&data)?;
    save_string(&buf, path)
}

pub fn save_string<P>(data: &str, path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    let storage = get_local_storage()?;
    let key = path.as_ref().display().to_string();
    storage
        .set_item(&key, data)
        .map_err(|_| format_err!("Storage#setItem failed"))
}

pub fn list_existing_benchmarks<P>(directory: &P) -> Result<Vec<BenchmarkId>>
where
    P: AsRef<Path>,
{
    let storage = get_local_storage()?;
    let len = storage
        .length()
        .map_err(|_| format_err!("Storage#length"))?;

    let dir = directory.as_ref().display().to_string();

    let mut ids = vec![];

    for i in 0..len {
        let key = storage.key(i).unwrap().unwrap();
        if key.starts_with(&dir) && key.ends_with("new/benchmark.json") {
            let id: BenchmarkId = load(&key)?;
            ids.push(id);
        }
    }

    Ok(ids)
}
