extern crate gluster;
extern crate tempdir;

use backend::Backend;

use std::io::Result;
use std::path::Path;

use self::tempdir::TempDir;

pub struct GlusterBackend;
/*
    Add a disk
    0. Make sure the brick pid isn't running
    1. Need to know which brick we're replacing.
      a. Pull from sqlite3
    2. Create a dir on the gluster mount that doesn't exist
    Using Temp
         mkdir /mnt/r2/<name-of-nonexistent-dir>
         rmdir /mnt/r2/<name-of-nonexistent-dir>
         setfattr -n trusted.non-existent-key -v abc /mnt/r2
         setfattr -x trusted.non-existent-key  /mnt/r2

    Remove a disk
    1. Kill the brick pid
    2. wipe it
    3. Record in sqlite where it was mounted so we can replace it
    later
*/

impl Backend for GlusterBackend {
    fn add_disk(
        &self,
        device: &Path,
        id: Option<u64>,
        journal: Option<&str>,
        journal_partition: Option<u32>,
        simulate: bool,
    ) -> Result<()> {
        Ok(())
    }

    /// Remove a disk from a cluster
    /// If simulate is passed no action should be taken
    fn remove_disk(&self, device: &Path, simulate: bool) -> Result<()> {
        Ok(())
    }

    /// Check if it's safe to remove a disk from a cluster
    /// If simulate is passed then this always returns true
    /// Take any actions needed with this call to figure out if a disk is safe
    /// to remove from the cluster.
    fn safe_to_remove(&self, device: &Path, simulate: bool) -> Result<bool> {
        Ok(true)
    }
}
