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

#[derive(Debug)]
pub enum RawDataType {
    Unknown,
    Int,
    Bool,
}

#[derive(Debug)]
pub enum DataType {
    Raw(RawDataType),
    Inferred(RawDataType),
}

fn parse_data_type(input: &str) -> Option<DataType> {
    match input {
        "int" => Some(DataType::Raw(RawDataType::Int)),
        "bool" => Some(DataType::Raw(RawDataType::Bool)),
        "_" => Some(DataType::Raw(RawDataType::Unknown)),
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
                DataType::Raw(rdt) => Some(rdt),
                DataType::Inferred(_) => None,
            },
            None => None,
        }
    }
}
