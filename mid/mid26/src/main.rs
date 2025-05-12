
enum SizeClothes{
    Small,
    Medium,
    Large,
    ExtraLarge,
}
enum TopsType{
    TShirt,
    Shirt,
    Polo,
}
enum PantsLength{
    Short,
    Long,
}
struct Clothes{
    price: f32,
    size: SizeClothes,
}
struct Tops{
    clothes: Clothes,
    types: TopsType,
}
struct Pants{
    clothes: Clothes,
    length: PantsLength,
}
fn main() {
}
