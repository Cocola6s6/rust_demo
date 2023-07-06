use xlsxwriter::{
    prelude::{FormatAlignment, FormatColor, FormatUnderline},
    Format, Workbook,
};

fn main() {
    println!("Hello, world!");
    do_export();
}

fn do_export() {
    let workbook = Workbook::new("target/simple1.xlsx").unwrap();

    // let format1 = workbook.add_format().set_font_color(FormatColor::Red);
    let mut format1 = Format::new();
    format1.set_font_color(FormatColor::Blue);

    // let format2 = workbook
    //     .add_format()
    //     .set_font_color(FormatColor::Blue)
    //     .set_underline(FormatUnderline::Single);

    let mut format2 = Format::new();
    format2
        .set_font_color(FormatColor::Blue)
        .set_underline(FormatUnderline::Single);

    // let format3 = workbook
    //     .add_format()
    //     .set_font_color(FormatColor::Green)
    //     .set_align(FormatAlignment::CenterAcross)
    //     .set_align(FormatAlignment::Center);

    let mut format3 = Format::new();
    format3
        .set_font_color(FormatColor::Green)
        .set_align(FormatAlignment::CenterAcross)
        .set_align(FormatAlignment::Center);

    let mut sheet1 = workbook.add_worksheet(None).unwrap();
    sheet1
        .write_string(0, 0, "Red text", Some(&format1))
        .unwrap();
    sheet1.write_number(0, 1, 20., None).unwrap();
    sheet1.write_formula_num(1, 0, "=10+B1", None, 30.).unwrap();
    sheet1
        .write_url(
            1,
            1,
            "https://github.com/informationsea/xlsxwriter-rs",
            Some(&format2),
        )
        .unwrap();
    sheet1
        .merge_range(2, 0, 3, 2, "Hello, world", Some(&format3))
        .unwrap();

    sheet1.set_selection(1, 0, 1, 2);
    sheet1.set_tab_color(FormatColor::Cyan);
    workbook.close().unwrap();
}
