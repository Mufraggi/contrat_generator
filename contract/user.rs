#[typeshare]
struct User {
    my_name: String,
    my_age: u32,
    address: Address,
}

#[typeshare]
struct Address {
    city: String,
    zipcode: String,
}

#[typeshare]
#[serde(tag = "type", content = "content")]
enum MyEnum {
    MyVariant(bool),
    MyOtherVariant,
    MyNumber(u32),
}