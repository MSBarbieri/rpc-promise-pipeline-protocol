use rpc_pp_core::Server;
use rpc_pp_shared::MyStruct;

fn main() {
    let foo = MyStruct::new(1, 2);
    println!("{:?}", Server);
    println!("{:?}", foo);
}
