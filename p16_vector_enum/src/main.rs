enum SheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let v = vec![SheetCell::Int(10),SheetCell::Float(3.0),SheetCell::Text("rust".to_string())];
}
