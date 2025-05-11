use std::rc::Rc;
enum StoveType {
    Electric,
    Gas,
    Induction,
}
struct Stove {
    types: StoveType,
}
impl Stove {
    fn new(types: StoveType) -> Stove {
        Stove { types }
    }

    fn get_types(&self) -> i32 {
        match self.types {
            StoveType::Electric => 1,
            StoveType::Gas => 2,
            StoveType::Induction => 3,
        }
    }
    fn show_types(&self) {
        match self.get_types() {
            1 => println!("Electric"),
            2 => println!("Gas"),
            3 => println!("Induction"),
            _ => println!("Unknown"),
        }
    }
}
struct Pot {
    price: f64,
    width: i32,
    height: i32,
    amount_of_stove: i32,
    stove: Rc<Stove>,
}

impl Pot {
    fn new(
        price: f64,
        width: i32,
        height: i32,
        amount_of_stove: i32,
        stove: Rc<Stove>,
    ) -> Pot {
        Pot {
            price,
            width,
            height,
            amount_of_stove,
            stove,
        }
    }
    fn get_price(&self) -> f64 {
        self.price
    }
    fn get_width(&self) -> i32 {
        self.width
    }
    fn get_height(&self) -> i32 {
        self.height
    }
    fn get_amount_of_stove(&self) -> i32 {
        self.amount_of_stove
    }
    fn get_stove(&self) -> Rc<Stove> {
        Rc::clone(&self.stove)
    }
    fn show_price(&self) {
        println!("Price: {}", self.get_price());
    }
    fn show_width(&self) {
        println!("Width: {}", self.get_width());
    }
    fn show_height(&self) {
        println!("Height: {}", self.get_height());
    }
    fn show_amount_of_stove(&self) {
        println!("Amount of stove: {}", self.get_amount_of_stove());
    }
    fn show_stove(&self) {
        println!("Stove: ");
        self.stove.show_types();
    }
    fn show_all(&self) {
        self.show_price();
        self.show_width();
        self.show_height();
        self.show_amount_of_stove();
        self.show_stove();
    }
    
}
struct Streaming_Pot{
    pot: Pot,
    layer: i32,
}
impl Streaming_Pot{
    fn new(pot: Pot, layer: i32) -> Streaming_Pot {
        Streaming_Pot { pot, layer }
    }
    fn get_layer(&self) -> i32 {
        self.layer
    }
    fn show_layer(&self) {
        println!("Layer: {}", self.get_layer());
    }
    fn show_all(&self) {
        self.pot.show_all();
        self.show_layer();
    }
}
struct Noodle_Pot{
    pot: Pot,
    compartment: i32,
}
impl Noodle_Pot{
    fn new(pot: Pot, compartment: i32) -> Noodle_Pot {
        Noodle_Pot { pot, compartment }
    }
    fn get_compartment(&self) -> i32 {
        self.compartment
    }
    fn show_compartment(&self) {
        println!("Compartment: {}", self.get_compartment());
    }
    fn show_all(&self) {
        self.pot.show_all();
        self.show_compartment();
    }
    
}
fn main() {

    let s0= Stove::new(StoveType::Electric);
    s0.show_types();
    let s1 = Rc::new(Stove{
        types: StoveType::Electric,
    });
    let s2 = Rc::new(Stove{
        types: StoveType::Gas,
    });
    let s3 = Rc::new(Stove{
        types: StoveType::Induction,
    });
    s1.show_types();
    s2.show_types();
    s3.show_types();

    println!();

    let p1 = Pot::new(100.0, 10, 20, 2, Rc::clone(&s1));
    let p2 = Pot::new(200.0, 15, 25, 3, Rc::clone(&s2));
    let p3 = Pot::new(300.0, 20, 30, 4, Rc::clone(&s3));
    p1.show_all();
    p2.show_all();
    p3.show_all();
    p1.show_stove();
    p1.get_stove();

    println!();

    let sp1 = Streaming_Pot::new(p1, 2);
    let sp2 = Streaming_Pot::new(p2, 3);

    sp1.show_all();
    sp2.show_all();

    println!();

    let np1 = Noodle_Pot::new(
        Pot::new(150.0, 12, 22, 2, Rc::clone(&s1)),
        2);
    
    let np2 = Noodle_Pot::new(
        Pot::new(250.0, 17, 27, 3, Rc::clone(&s2)),
        3);
    np1.show_all();
    np2.show_all();
}
