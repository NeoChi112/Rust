enum StoveType{
    Electric,
    Gas,
    Induction,
}
struct Stove{
    types:StoveType
}
impl Stove{
    fn new(types:StoveType)->Stove{
        Stove{types}
    }
    
    fn get_types(&self)->i32{
        match self.types{
            StoveType::Electric => 1,
            StoveType::Gas => 2,
            StoveType::Induction => 3,
        }
    }
    fn show_types(&self){
        match self.get_types(){
            1 => println!("Electric"),
            2 => println!("Gas"),
            3 => println!("Induction"),
            _ => println!("Unknown"),
        }
    }
}
struct Pot{
    price: f64,
    width:i32,
    height:i32,
    Stove:*mut Stove,
}
fn main() {
    let s1 = Stove::new(StoveType::Electric);
    let s2 = Stove::new(StoveType::Gas);
    s1.show_types();
    s2.show_types();
}
