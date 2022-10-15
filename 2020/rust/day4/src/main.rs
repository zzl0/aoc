#[derive(Clone, Copy, PartialEq, Debug)]
struct Year(u64);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Length {
    Cm(u64),
    In(u64),
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Color<'a>(&'a str);

#[derive(Clone, Copy, PartialEq, Debug)]
struct ID<'a>(&'a str);

#[derive(PartialEq, Debug)]
struct Passport<'a> {
    birth_year: Year,
    issue_year: Year,
    expiration_year: Year,
    height: Length,
    hair_color: Color<'a>,
    eye_color: Color<'a>,
    passport_id: ID<'a>,
    country_id: Option<ID<'a>>,
}

#[derive(PartialEq, Debug, Default)]
struct PassportBuilder<'a> {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiration_year: Option<Year>,
    height: Option<Length>,
    hair_color: Option<Color<'a>>,
    eye_color: Option<Color<'a>>,
    passport_id: Option<ID<'a>>,
    country_id: Option<ID<'a>>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("missing field: {0}")]
    MissingField(&'static str),
}

impl<'a> PassportBuilder<'a> {
    fn build(self) -> Result<Passport<'a>, Error> {
        macro_rules! build {
            (
                required => {
                    $($req: ident),* $(,)*
                }$(,)*
                optional => {
                    $($opt: ident),* $(,)*
                }$(,)*
            ) => {
                Ok(Passport {
                    $($req: self.$req.ok_or(Error::MissingField(stringify!($req)))?),*,
                    $($opt: self.$opt),*
                })
            }
        }

        build! {
            required => {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
            },
            optional => {
                country_id,
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_builder() {
    assert!(PassportBuilder {
        ..Default::default()
    }
    .build()
    .is_err());
    assert!(PassportBuilder {
        birth_year: Some(Year(2014)),
        issue_year: Some(Year(2017)),
        expiration_year: Some(Year(2023)),
        height: Some(Length::Cm(195)),
        hair_color: Some(Color("#ffffff")),
        eye_color: Some(Color("#ee7812")),
        passport_id: Some(ID("00023437")),
        country_id: None,
    }
    .build()
    .is_ok());
}