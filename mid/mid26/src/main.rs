#[derive(Clone, Copy)]
enum SizeClothes {
    Small,
    Medium,
    Large,
    ExtraLarge,
}
#[derive(Clone, Copy)]
enum TopsType {
    TShirt,
    Shirt,
    Polo,
}
#[derive(Clone, Copy)]
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
        println!("Price: {}", self.get_price());
        self.show_size();
    }
}
struct Tops {
    clothes: Clothes,
    types: TopsType,
}
impl Tops {
    fn new(price: f32, size: SizeClothes, types: TopsType) -> Tops {
        Tops {
            clothes: Clothes::new(price, size),
            types,
        }
    }
    fn get_types(&self) -> TopsType {
        self.types
    }
    fn set_types(&mut self, types: TopsType) {
        self.types = types;
    }
    fn show_types(&self) {
        match self.get_types() {
            TopsType::TShirt => println!("Type: T-Shirt"),
            TopsType::Shirt => println!("Type: Shirt"),
            TopsType::Polo => println!("Type: Polo"),
        }
    }
    fn show_info(&self) {
        println!("Tops Info: ");
        self.clothes.show_info();
        self.show_types();
        println!();
    }
}

struct Pants {
    clothes: Clothes,
    length: PantsLength,
}
impl Pants {
    fn new(price: f32, size: SizeClothes, length: PantsLength) -> Pants {
        Pants {
            clothes: Clothes::new(price, size),
            length,
        }
    }
    fn get_length(&self) -> PantsLength {
        self.length
    }
    fn set_length(&mut self, length: PantsLength) {
        self.length = length;
    }
    fn show_length(&self) {
        match self.get_length() {
            PantsLength::Short => println!("Length: Short"),
            PantsLength::Long => println!("Length: Long"),
        }
    }
    fn show_info(&self) {
        println!("Pants Info: ");
        self.clothes.show_info();
        self.show_length();
        println!();
    }
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

    let t1 = Tops::new(10.0, SizeClothes::Small, TopsType::TShirt);
    let mut t2 = Tops::new(0.0, SizeClothes::Small, TopsType::TShirt);
    t2.set_types(TopsType::Shirt);
    t1.show_info();
    t2.show_info();

    let p1 = Pants::new(20.0, SizeClothes::Medium, PantsLength::Long);
    let mut p2 = Pants::new(0.0, SizeClothes::Medium, PantsLength::Long);
    p2.set_length(PantsLength::Short);
    p1.show_info();
    p2.show_info();
}
