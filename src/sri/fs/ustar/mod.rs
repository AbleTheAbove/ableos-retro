pub enum TypeFlag {
    Normal = 0,   // '0' or (ASCII NUL) 	Normal file
    HardLink = 1, // '1' 	Hard link
    SymLink = 2,  // '2' 	Symbolic link
    CharDev = 3,  // '3' 	Character device
    BlockDev = 4, // '4' 	Block device
    Dir = 5,      // '5' 	Directory
    Pipe = 6,     // '6' 	Named pipe (FIFO)
}

pub struct File {
    // 100 File name
    name: [u8; 100],
    // 8 File mode
    mode: u8,
    // 8 	Owner's numeric user ID
    owner_id: u8,
    // 8 	Group's numeric user ID
    group_id: u8,
    // 12 	File size in u8s (octal base)
    size: [u8; 12],
    // 12 	Last modification time in numeric Unix time format (octal)
    last_mod: [u8; 12],
    // 8 	Checksum for header record
    checksum: u8,
    // 1 	Type flag
    type_flag: u8,
    // 100 Name of linked file
    name_linked_file: [u8; 100],
    // 6 	UStar indicator "ustar" then NUL
    ustar: [u8; 6],
    // 2 	UStar version "00"
    ustar_version: [u8; 2],
    // 32 	Owner user name
    owner_name: [u8; 32],
    // 32 	Owner group name
    group_name: [u8; 32],
    // 8 	Device major number
    dev_ver_major: u8,
    // 8 	Device minor number
    dev_ver_minor: u8,
    // 155 Filename prefix
    filename: [u8; 155],
}

#[test_case]
pub fn ustar_file() {
    let x = File {
        name: [0; 100],
        mode: 0,
        owner_id: 0,
        group_id: 0,
        size: [0; 12],
        last_mod: [0; 12],
        checksum: 0,
        type_flag: 0,
        name_linked_file: [0; 100],
        ustar: [0; 6],
        ustar_version: [0; 2],
        owner_name: [0; 32],
        group_name: [0; 32],
        dev_ver_major: 0,
        dev_ver_minor: 0,
        filename: [0; 155],
    };
}
