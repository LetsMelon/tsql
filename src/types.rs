use subenum::subenum;

#[derive(Debug)]
pub struct Table {
    pub raw_name: String,
    pub name: String,

    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub raw_name: String,
    pub name: String,

    pub raw_datatype: RawDataType,
    pub datatype: DataType,
}

#[subenum(RawDataType)]
#[derive(Debug)]
pub enum DataType {
    #[subenum(RawDataType)]
    Unknwon,
    #[subenum(RawDataType)]
    Int,
    Inferred(RawDataType),
}

fn parse_data_type(input: &str) -> Option<DataType> {
    match input {
        "int" => Some(DataType::Int),
        "_" => Some(DataType::Unknwon),
        _ => None,
    }
}

impl DataType {
    pub fn parse(input: &str) -> Option<Self> {
        parse_data_type(input)
    }
}

impl RawDataType {
    pub fn parse(input: &str) -> Option<Self> {
        let dt = DataType::parse(input);

        match dt {
            Some(dt) => match dt {
                DataType::Unknwon => Some(RawDataType::Unknwon),
                DataType::Int => Some(RawDataType::Int),
                DataType::Inferred(_) => None,
            },
            None => None,
        }
    }
}
