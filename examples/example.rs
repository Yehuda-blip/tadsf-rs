use tadsf_rs::{add, Add};


fn main() {
    type v1 = add!(5, Add<7, 8>);
    type v2 =  add!(5, Add<7, 8>);
    type v3 = add!(v1, v2);
    println!("{:?}", v3::sym())
}
