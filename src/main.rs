#![recursion_limit="117"]

#[macro_use]
extern crate mashup;

macro_rules! foo {
    ( $s:ident => $($x: ident : $t: ty),+ ) => {
        mashup! { $(
            m["fname" $x] = with_ $x;
        )* }

        m! {
        impl $s { $(
            pub fn "fname" $x (mut self, val: $t) -> Self { self.$x = val; self }
        )* }}
    };
}

#[derive(Default,Debug)]
struct A {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
}

foo!(A => 
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32
);

fn main() {
    let a = A::default();
    let v = a
        .with_a(42)
        .with_b(43)
        .with_c(44)
        .with_d(45)
        .with_e(46);
    println!("{:?}", v);
}
