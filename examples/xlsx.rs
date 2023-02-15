use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};

fn main() {
    let path = format!("{}/fixtures/test.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
    let sheets = workbook.sheet_names().to_owned();
    let sheet_one = sheets.first().unwrap();

    println!("{:?}", sheets);
    let range = workbook
        .worksheet_range(&sheet_one)
        .ok_or(Error::Msg("Cannot find 'Sheet1'"))
        .unwrap()
        .unwrap();

    let mut iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)
        .unwrap();

    if let Some(result) = iter.next() {
        let val: Vec<String> = result.unwrap();
        println!("{val:?}")
    } else {
        println!("expected at least one record but got none");
    }
}
