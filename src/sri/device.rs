enum DeviceType {
    Display,
    Harddrive,
    Ram,
}

/// `device://{dtype}/{name}?{query}``
struct Device {
    dtype: DeviceType,
    name: Option<str>,
    query: str,
}

