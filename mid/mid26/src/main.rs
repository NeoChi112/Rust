#[derive(Clone, Copy)]
enum SizeClothes {
    Small,
    Medium,
    Large,
    ExtraLarge,
}
enum TopsType {
    TShirt,
    Shirt,
    Polo,
}
enum PantsLength {
    Short,
    Long,
}
struct Clothes {
    price: f32,
    size: SizeClothes,
}
impl Clothes {
    fn new(price: f32, size: SizeClothes) -> Clothes {
        Clothes { price, size }
    }
    fn get_price(&self) -> f32 {
        self.price
    }
    fn get_size(&self) -> SizeClothes {
        self.size
    }
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }
    fn set_size(&mut self, size: SizeClothes) {
        self.size = size;
    }
    fn show_size(&self) {
        match self.get_size() {
            SizeClothes::Small => println!("Size: Small"),
            SizeClothes::Medium => println!("Size: Medium"),
            SizeClothes::Large => println!("Size: Large"),
            SizeClothes::ExtraLarge => println!("Size: Extra Large"),
        }
    }
    fn show_info(&self) {
        println!("Clothes Info: ");
        println!("Price: {}", self.get_price());
        self.show_size();
        println!();
    }
}
struct Tops {
    clothes: Clothes,
    types: TopsType,
}
struct Pants {
    clothes: Clothes,
    length: PantsLength,
}
struct Size {
    width: i32,
    length: i32,
    height: i32,
}
struct Wardrobe {
    size: Size,
    tops: Vec<Tops>,
    pants: Vec<Pants>,
    amount_of_tops: i32,
    amount_of_pants: i32,
}
fn main() {
    let c1 = Clothes::new(10.0, SizeClothes::Small);

    let mut c2 = Clothes::new(0.0, SizeClothes::Small);
    c2.set_price(20.0);
    c2.set_size(SizeClothes::Medium);
    c1.show_info();
    c2.show_info();
}
