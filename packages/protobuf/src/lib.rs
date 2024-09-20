include!(concat!(env!("OUT_DIR"), "/pb.rs"));

use prost::Message;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        // let a2 = AnimalV2::default();
        // let a1 = Animal::default();

        // let bytes = a2.encode_to_vec();
        let mut foo_V2 = FooV2::default();
        foo_V2.set_animal(AnimalV2::Cat);

        let bytes = foo_V2.encode_to_vec();

        let foo = Foo::decode(bytes.as_ref()).unwrap();
        println!("{:?}", foo.animal())
    }
}
