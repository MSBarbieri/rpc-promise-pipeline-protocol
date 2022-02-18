use rpc_pp_core::Client;
use rpc_pp_shared::MyStruct;

fn main() {
    let foo = MyStruct::new(1, 2);
    println!("{:?}", foo);
    println!("{:?}", Client);
}
