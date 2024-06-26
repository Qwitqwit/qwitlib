use crate::from_excel::operators::count_columns;
use crate::{
    core::{CsvError, CsvRow, CsvRowOperator, CsvValue},
    epoch_conversion::ExcelEpoch,
};
use calamine::{Data, Range};

pub mod operators;

pub fn write_range(
    range: &Range<Data>,
    mut operator: impl CsvRowOperator,
    line_length: i32,
    sep: String,
) -> Result<(), CsvError> {
    let all_rows = range.rows().map(CsvRow::iterator);
    operator.operate(sep, line_length, all_rows)
}

pub fn get_max_line_length_of_first_x_lines(
    range: &Range<Data>,
    lines_to_consider: i32,
) -> Result<i32, CsvError> {
    let all_rows = range.rows().map(CsvRow::iterator);
    count_columns(lines_to_consider, all_rows)
}

#[allow(clippy::cast_possible_truncation)]
impl From<Data> for CsvValue {
    fn from(value: Data) -> Self {
        match value {
            // we do nothing on empty
            Data::Empty => CsvValue(Err("Empty Value".to_owned())),
            // we write for those types
            Data::String(ref s) | Data::DateTimeIso(ref s) | Data::DurationIso(ref s) => {
                // surround strings with '"' to handle separator inside
                let escaped = format!(r#""{}""#, s);
                CsvValue(Ok(escaped))
            }

            Data::DateTime(ref f) => {
                let as_int = f.as_f64().round() as i64;
                let epoch = ExcelEpoch(as_int);
                let as_string = epoch.to_string();
                CsvValue(Ok(as_string))
            }

            // we also just write for those
            Data::Float(ref f) => CsvValue(Ok(f.to_string())),
            Data::Int(ref i) => CsvValue(Ok(i.to_string())),
            Data::Bool(ref b) => CsvValue(Ok(b.to_string())),
            Data::Error(ref e) => CsvValue(Err(format!(
                "error in sheet, fix or remove cell error: {e}"
            ))),
        }
    }
}
