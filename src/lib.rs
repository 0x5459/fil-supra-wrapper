use std::{
    ffi::{c_char, CString},
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
};

mod libsupraseal {
    use std::ffi::c_char;

    extern "C" {
        pub fn get_max_block_offset(sector_size: usize) -> usize;

        pub fn get_slot_size(num_sectors: usize, sector_size: usize) -> usize;

        pub fn pc1(
            block_offset: usize,
            num_sectors: usize,
            replica_ids: *const u8,
            parents_filename: *const c_char,
            sector_size: usize,
        ) -> u32;

        pub fn pc2(
            block_offset: usize,
            num_sectors: usize,
            output_dir: *const c_char,
            data_filenames: *const *const c_char,
            sector_size: usize,
        ) -> u32;

        pub fn pc2_cleanup(
            num_sectors: usize,
            output_dir: *const c_char,
            sector_size: usize,
        ) -> u32;

        pub fn c1(
            block_offset: usize,
            num_sectors: usize,
            sector_slot: usize,
            replica_ids: *const u8,
            seed: *const u8,
            ticket: *const u8,
            cache_path: *const c_char,
            parents_filename: *const c_char,
            replica_path: *const c_char,
            sector_size: usize,
        ) -> u32;
    }
}

pub fn get_max_block_offset(sector_size: usize) -> usize {
    unsafe { libsupraseal::get_max_block_offset(sector_size) }
}

pub fn get_slot_size(num_sectors: usize, sector_size: usize) -> usize {
    unsafe { libsupraseal::get_slot_size(num_sectors, sector_size) }
}

pub fn pc1<P: AsRef<Path>>(
    block_offset: usize,
    num_sectors: usize,
    replica_ids: Vec<[u8; 32]>,
    parents_filename: P,
    sector_size: usize,
) -> u32 {
    let replica_ids = replica_ids.into_iter().flatten().collect::<Vec<u8>>();
    let parents_filename_c =
        CString::new(parents_filename.as_ref().as_os_str().as_bytes()).unwrap();
    unsafe {
        libsupraseal::pc1(
            block_offset,
            num_sectors,
            replica_ids.as_ptr(),
            parents_filename_c.as_ptr(),
            sector_size,
        )
    }
}

pub fn pc2<P: AsRef<Path>>(
    block_offset: usize,
    num_sectors: usize,
    output_dir: P,
    data_filenames: &[PathBuf],
    sector_size: usize,
) -> u32 {
    let output_dir_c = CString::new(output_dir.as_ref().as_os_str().as_bytes()).unwrap();
    let data_filenames_c = data_filenames
        .iter()
        .map(|data_filename| {
            CString::new(data_filename.as_os_str().as_bytes())
                .unwrap()
                .into_bytes()
        })
        .flatten()
        .collect::<Vec<u8>>();
    let pc2_status = unsafe {
        libsupraseal::pc2(
            block_offset,
            num_sectors,
            output_dir_c.as_ptr(),
            data_filenames_c.as_ptr() as *const *const c_char,
            sector_size,
        )
    };
    println!("PC2 returned {}", pc2_status);
    return pc2_status;
}

pub fn pc2_cleanup<P: AsRef<Path>>(num_sectors: usize, output_dir: P, sector_size: usize) -> u32 {
    let output_dir_c = CString::new(output_dir.as_ref().as_os_str().as_bytes()).unwrap();
    unsafe { libsupraseal::pc2_cleanup(num_sectors, output_dir_c.as_ptr(), sector_size) }
}

pub fn c1<P: AsRef<Path>>(
    block_offset: usize,
    num_sectors: usize,
    sector_slot: usize,
    replica_ids: Vec<[u8; 32]>,
    seed: [u8; 32],
    ticket: [u8; 32],
    cache_path: P,
    parents_filename: P,
    replica_path: P,
    sector_size: usize,
) -> u32 {
    let replica_ids = replica_ids.into_iter().flatten().collect::<Vec<u8>>();

    let cache_path_c: CString = CString::new(cache_path.as_ref().as_os_str().as_bytes()).unwrap();
    let parents_filename_c =
        CString::new(parents_filename.as_ref().as_os_str().as_bytes()).unwrap();
    let replica_path_c: CString =
        CString::new(replica_path.as_ref().as_os_str().as_bytes()).unwrap();

    unsafe {
        libsupraseal::c1(
            block_offset,
            num_sectors,
            sector_slot,
            replica_ids.as_ptr(),
            seed.as_ptr(),
            ticket.as_ptr(),
            cache_path_c.as_ptr(),
            parents_filename_c.as_ptr(),
            replica_path_c.as_ptr(),
            sector_size,
        )
    }
}
