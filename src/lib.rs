use std::{ops::Deref, sync::Arc};

#[cfg(all(feature = "infallible", feature = "never"))]
compiler_error! {"Only use 1 feature"}

#[cfg(feature = "never")]
type Appendix = never_say_never::Never;
#[cfg(feature = "infallible")]
type Appendix = std::convert::Infallible;
#[cfg(all(not(feature = "never"), not(feature = "infallible")))]
type Appendix = ();

pub struct MyContainer(Arc<str>, Appendix);
//pub struct MyContainer(Arc<str>);

impl MyContainer {
    pub fn do_stuff(&self) {
        println!("MyContainer::do_stuff: My container contains {}", self.0)
    }

    pub fn to_inner(self) -> Arc<str> {
        eprintln!("MyContainer::to_inner: Doing to-inner on MyContainer");
        self.0
    }

    pub fn replace(&mut self, n: Arc<str>) {
        eprintln!("MyContainer::replace: Doing replace on MyContainer");
        self.0 = n
    }

    #[inline(never)]
    pub fn new() -> Option<Self> {
        eprintln!("Creating container");
        #[cfg(all(not(feature = "never"), not(feature = "infallible")))]
        return Some(MyContainer("The contents of my container".into(), ()));
        #[allow(unreachable_code)]
        None
    }
}

impl Deref for MyContainer {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
