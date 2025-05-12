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
        self.size.clone()
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
#[derive(Clone, Copy)]
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
impl Wardrobe{
    fn new(size: Size, amount_of_tops: i32, amount_of_pants: i32) -> Wardrobe {
        Wardrobe {
            size,
            tops: Vec::new(),
            pants: Vec::new(),
            amount_of_tops,
            amount_of_pants,
        }
    }
    fn add_top(&mut self, top: Tops) {
        if self.tops.len() < self.amount_of_tops as usize {
            self.tops.push(top);
        } else {
            println!("Cannot add more tops to the wardrobe.");
        }
    }
    fn add_pant(&mut self, pant: Pants) {
        if self.pants.len() < self.amount_of_pants as usize {
            self.pants.push(pant);
        } else {
            println!("Cannot add more pants to the wardrobe.");
        }
    }
    fn show_info(&self) {
        println!("Wardrobe Size: {}x{}x{}", self.size.width, self.size.length, self.size.height);
        println!("Amount of Tops: {}", self.amount_of_tops);
        println!("Amount of Pants: {}", self.amount_of_pants);
        println!("Tops in Wardrobe: ");
        for top in &self.tops {
            top.show_info();
        }
        println!("Pants in Wardrobe: ");
        for pant in &self.pants {
            pant.show_info();
        }
    }
    fn get_size(&self) -> Size {
        self.size
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn get_amount_of_tops(&self) -> i32 {
        self.amount_of_tops
    }
    fn set_amount_of_tops(&mut self, amount_of_tops: i32) {
        self.amount_of_tops = amount_of_tops;
    }
    fn get_amount_of_pants(&self) -> i32 {
        self.amount_of_pants
    }
    fn set_amount_of_pants(&mut self, amount_of_pants: i32) {
        self.amount_of_pants = amount_of_pants;
    }
    fn get_tops(&self) -> &Vec<Tops> {
        &self.tops
    }
    fn get_pants(&self) -> &Vec<Pants> {
        &self.pants
    }
    fn set_tops(&mut self, tops: Vec<Tops>) {
        self.tops = tops;
    }
    fn set_pants(&mut self, pants: Vec<Pants>) {
        self.pants = pants;
    }
    fn show_tops(&self) {
        for top in &self.tops {
            top.show_info();
        }
    }
    fn show_pants(&self) {
        for pant in &self.pants {
            pant.show_info();
        }
    }
    fn show_size(&self) {
        println!("Wardrobe Size: {}x{}x{}", self.size.width, self.size.length, self.size.height);
    }
    fn show_amount_of_tops(&self) {
        println!("Amount of Tops: {}", self.amount_of_tops);
    }
    fn show_amount_of_pants(&self) {
        println!("Amount of Pants: {}", self.amount_of_pants);
    }
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

    let size = Size {
        width: 100,
        length: 200,
        height: 300,
    };
    let mut wardrobe = Wardrobe::new(size, 5, 5);
    wardrobe.add_top(t1);
    wardrobe.add_top(t2);
    wardrobe.add_pant(p1);
    wardrobe.add_pant(p2);
    wardrobe.show_info();
    wardrobe.set_size(Size {
        width: 200,
        length: 300,
        height: 400,
    });
    wardrobe.show_size();
    wardrobe.set_amount_of_tops(10);
    wardrobe.show_amount_of_tops();
    wardrobe.set_amount_of_pants(10);
    wardrobe.show_amount_of_pants();
    wardrobe.show_tops();
    wardrobe.show_pants();

    wardrobe.set_tops(
        vec![
            Tops::new(15.0, SizeClothes::Large, TopsType::Polo),
            Tops::new(25.0, SizeClothes::ExtraLarge, TopsType::TShirt),
        ],
    );
    wardrobe.set_pants(
        vec![
            Pants::new(30.0, SizeClothes::Large, PantsLength::Short),
            Pants::new(40.0, SizeClothes::ExtraLarge, PantsLength::Long),
        ],
    );
    wardrobe.get_amount_of_tops();
    wardrobe.get_amount_of_pants();
    wardrobe.get_size();
    wardrobe.get_tops();
    wardrobe.get_pants();

}
