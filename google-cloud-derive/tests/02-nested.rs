use google_cloud::datastore::{FromValue, IntoValue};
use google_cloud::error::ConvertError;

#[derive(Debug, FromValue, IntoValue)]
pub struct Foo {
    bar: Bar,
    qux: bool,
}

#[derive(Debug, FromValue, IntoValue)]
pub struct Bar {
    baz: String,
}

fn main() {
    let foo = Foo {
        bar: Bar {
            baz: String::from("test"),
        },
        qux: true,
    };
    println!("original: {:?}", foo);

    let converted = foo.into_value();
    println!("converted: {:?}", converted);

    let recovered: Result<Foo, ConvertError> = Foo::from_value(converted);
    println!("recovered: {:?}", recovered);
}
