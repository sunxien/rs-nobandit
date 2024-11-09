
///
#[derive(Debug)]
#[allow(dead_code, unused)]
pub struct History {

    // primary key
    pub id: u64,

    // license plate of car
    pub license_plate: String,

    //
    pub enter_time: String,

    //
    pub exit_time: String,

    //
    pub status: u8,
}