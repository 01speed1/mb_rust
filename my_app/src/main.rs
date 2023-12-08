mod goodbye;

use goodbye::ops::Ops;

// aqui llamamos la lib, si es lo primero que vez, ve a ./hello/src/lib.rs
//use hello;

fn main() {
    //println!("aqui una suma de 2 +2:{}", hello::mg_add(2, 2));

    let my_ops = Ops::new(
        String::from("my_ops"),
        vec![String::from("op1"), String::from("op2")],
    );

    println!("name: {}", my_ops.get_name());
    println!("operations: {:?}", my_ops.get_operations());
}
