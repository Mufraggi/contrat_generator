#[typeshare]
struct Poi {
    address: Address,
    location: Location

}
#[typeshare]
struct Location { position: Vec<i32> }