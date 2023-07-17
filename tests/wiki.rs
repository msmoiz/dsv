//! Sourced from: https://en.wikipedia.org/wiki/Comma-separated_values.

use dsv::{Dsv, Options};

#[test]
fn wiki_1() {
    let text = "1997,Ford,E350";
    let dsv = Dsv::from_str(text).unwrap();
    assert_eq!(dsv[0][0], "1997");
    assert_eq!(dsv[0][1], "Ford");
    assert_eq!(dsv[0][2], "E350");
}

#[test]
fn wiki_2() {
    let text = r#""1997","Ford","E350""#;
    let dsv = Dsv::from_str(text).unwrap();
    assert_eq!(dsv[0][0], "1997");
    assert_eq!(dsv[0][1], "Ford");
    assert_eq!(dsv[0][2], "E350");
}

#[test]
fn wiki_3() {
    let text = r#"1997,Ford,E350,"Super, luxurious truck""#;
    let dsv = Dsv::from_str(text).unwrap();
    assert_eq!(dsv[0][0], "1997");
    assert_eq!(dsv[0][1], "Ford");
    assert_eq!(dsv[0][2], "E350");
    assert_eq!(dsv[0][3], "Super, luxurious truck");
}

#[test]
fn wiki_4() {
    let text = r#"1997,Ford,E350,"Super, ""luxurious"" truck""#;
    let dsv = Dsv::from_str(text).unwrap();
    assert_eq!(dsv[0][0], "1997");
    assert_eq!(dsv[0][1], "Ford");
    assert_eq!(dsv[0][2], "E350");
    assert_eq!(dsv[0][3], r#"Super, "luxurious" truck"#);
}

#[test]
fn wiki_5() {
    let text = r#"1997,Ford,E350,"Go get one now
they are going fast""#;
    let dsv = Dsv::from_str(text).unwrap();
    assert_eq!(dsv[0][0], "1997");
    assert_eq!(dsv[0][1], "Ford");
    assert_eq!(dsv[0][2], "E350");
    assert_eq!(dsv[0][3], "Go get one now\nthey are going fast");
}

#[test]
fn wiki_6() {
    let text = r#"1997,Ford,E350," Super luxurious truck ""#;
    let dsv = Dsv::from_str(text).unwrap();
    assert_eq!(dsv[0][0], "1997");
    assert_eq!(dsv[0][1], "Ford");
    assert_eq!(dsv[0][2], "E350");
    assert_eq!(dsv[0][3], " Super luxurious truck ");
}

#[test]
fn wiki_7() {
    let text = r#"Los Angeles,34°03′N,118°15′W
New York City,40°42′46″N,74°00′21″W
Paris,48°51′24″N,2°21′03″E"#;

    let dsv = Dsv::from_str(text).unwrap();

    assert_eq!(dsv[0][0], "Los Angeles");
    assert_eq!(dsv[0][1], "34°03′N");
    assert_eq!(dsv[0][2], "118°15′W");

    assert_eq!(dsv[1][0], "New York City");
    assert_eq!(dsv[1][1], "40°42′46″N");
    assert_eq!(dsv[1][2], "74°00′21″W");

    assert_eq!(dsv[2][0], "Paris");
    assert_eq!(dsv[2][1], "48°51′24″N");
    assert_eq!(dsv[2][2], "2°21′03″E");
}

#[test]
fn wiki_8() {
    let text = r#"Year,Make,Model
1997,Ford,E350
2000,Mercury,Cougar"#;

    let dsv = Dsv::from_str(text).unwrap();

    assert_eq!(dsv[0][0], "Year");
    assert_eq!(dsv[0][1], "Make");
    assert_eq!(dsv[0][2], "Model");

    assert_eq!(dsv[1][0], "1997");
    assert_eq!(dsv[1][1], "Ford");
    assert_eq!(dsv[1][2], "E350");

    assert_eq!(dsv[2][0], "2000");
    assert_eq!(dsv[2][1], "Mercury");
    assert_eq!(dsv[2][2], "Cougar");
}

#[test]
fn wiki_9() {
    let text = r#"Year,Make,Model,Description,Price
1997,Ford,E350,"ac, abs, moon",3000.00
1999,Chevy,"Venture ""Extended Edition""","",4900.00
1999,Chevy,"Venture ""Extended Edition, Very Large""","",5000.00
1996,Jeep,Grand Cherokee,"MUST SELL!
air, moon roof, loaded",4799.00
"#;

    let dsv = Dsv::from_str(text).unwrap();

    assert_eq!(dsv[0][0], "Year");
    assert_eq!(dsv[0][1], "Make");
    assert_eq!(dsv[0][2], "Model");
    assert_eq!(dsv[0][3], "Description");
    assert_eq!(dsv[0][4], "Price");

    assert_eq!(dsv[1][0], "1997");
    assert_eq!(dsv[1][1], "Ford");
    assert_eq!(dsv[1][2], "E350");
    assert_eq!(dsv[1][3], "ac, abs, moon");
    assert_eq!(dsv[1][4], "3000.00");

    assert_eq!(dsv[2][0], "1999");
    assert_eq!(dsv[2][1], "Chevy");
    assert_eq!(dsv[2][2], "Venture \"Extended Edition\"");
    assert_eq!(dsv[2][3], "");
    assert_eq!(dsv[2][4], "4900.00");

    assert_eq!(dsv[3][0], "1999");
    assert_eq!(dsv[3][1], "Chevy");
    assert_eq!(dsv[3][2], "Venture \"Extended Edition, Very Large\"");
    assert_eq!(dsv[3][3], "");
    assert_eq!(dsv[3][4], "5000.00");

    assert_eq!(dsv[4][0], "1996");
    assert_eq!(dsv[4][1], "Jeep");
    assert_eq!(dsv[4][2], "Grand Cherokee");
    assert_eq!(dsv[4][3], "MUST SELL!\nair, moon roof, loaded");
    assert_eq!(dsv[4][4], "4799.00");
}

#[test]
fn wiki_10() {
    let text = "Year,Make,Model,Length
1997,Ford,E350,2.35
2000,Mercury,Cougar,2.38";

    let dsv = Dsv::from_str(text).unwrap();

    assert_eq!(dsv[0][0], "Year");
    assert_eq!(dsv[0][1], "Make");
    assert_eq!(dsv[0][2], "Model");
    assert_eq!(dsv[0][3], "Length");

    assert_eq!(dsv[1][0], "1997");
    assert_eq!(dsv[1][1], "Ford");
    assert_eq!(dsv[1][2], "E350");
    assert_eq!(dsv[1][3], "2.35");

    assert_eq!(dsv[2][0], "2000");
    assert_eq!(dsv[2][1], "Mercury");
    assert_eq!(dsv[2][2], "Cougar");
    assert_eq!(dsv[2][3], "2.38");
}

#[test]
fn wiki_11() {
    let text = "Year;Make;Model;Length
1997;Ford;E350;2,35
2000;Mercury;Cougar;2,38";

    let dsv = Dsv::from_str_with_options(
        text,
        Options {
            delimiter: b';',
            ..Default::default()
        },
    )
    .unwrap();

    assert_eq!(dsv[0][0], "Year");
    assert_eq!(dsv[0][1], "Make");
    assert_eq!(dsv[0][2], "Model");
    assert_eq!(dsv[0][3], "Length");

    assert_eq!(dsv[1][0], "1997");
    assert_eq!(dsv[1][1], "Ford");
    assert_eq!(dsv[1][2], "E350");
    assert_eq!(dsv[1][3], "2,35");

    assert_eq!(dsv[2][0], "2000");
    assert_eq!(dsv[2][1], "Mercury");
    assert_eq!(dsv[2][2], "Cougar");
    assert_eq!(dsv[2][3], "2,38");
}
