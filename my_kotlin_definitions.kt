@Serializable
data class Address (
	val city: String,
	val zipcode: String
)

@Serializable
data class Location (
	val position: List<Int>
)

@Serializable
data class Poi (
	val address: Address,
	val location: Location
)

@Serializable
data class User (
	val my_name: String,
	val my_age: UInt,
	val address: Address
)

@Serializable
sealed class MyEnum {
	@Serializable
	@SerialName("MyVariant")
	data class MyVariant(val content: Boolean): MyEnum()
	@Serializable
	@SerialName("MyOtherVariant")
	object MyOtherVariant: MyEnum()
	@Serializable
	@SerialName("MyNumber")
	data class MyNumber(val content: UInt): MyEnum()
}

