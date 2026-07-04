use never_test::MyContainer;

fn main() {
    println!(
        "The type weighs {} bytes",
        std::mem::size_of::<Option<MyContainer>>()
    );
    let container = MyContainer::new("container");
    println!("Container status: {:?}", container.is_some());
    if let Some(container) = container {
        container.do_stuff();
    }
}
