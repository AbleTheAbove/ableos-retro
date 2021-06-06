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
    name: [byte; 100],
    // 8 File mode
    mode: u8,
    // 8 	Owner's numeric user ID
    owner_id: u8,
    // 8 	Group's numeric user ID
    group_id: u8,
    // 12 	File size in bytes (octal base)
    size: [byte; 12],
    // 12 	Last modification time in numeric Unix time format (octal)
    last_mod: [byte; 12],
    // 8 	Checksum for header record
    checksum: u8,
    // 1 	Type flag
    type_flag: u8,
    // 100 Name of linked file
    name_linked_file: [byte; 100],
    // 6 	UStar indicator "ustar" then NUL
    ustar: [byte; 6],
    // 2 	UStar version "00"
    ustar_version: [byte; 2],
    // 32 	Owner user name
    owner_name: [byte; 32],
    // 32 	Owner group name
    group_name: [byte; 32],
    // 8 	Device major number
    dev_ver_major: u8,
    // 8 	Device minor number
    dev_ver_minor: u8,
    // 155 Filename prefix
    filename: [byte; 155],
}
