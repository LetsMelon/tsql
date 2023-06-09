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
    BigInt,
    Date,
    DateTime,
    Time,
    Double,
    Float,
    Uuid,
}

#[derive(Debug)]
pub enum DataType {
    Raw(RawDataType),
    Inferred(RawDataType),
}

impl DataType {
    pub fn parse(input: &str) -> Option<Self> {
        match input {
            "int" => Some(DataType::Raw(RawDataType::Int)),
            "bool" => Some(DataType::Raw(RawDataType::Bool)),
            "bigint" => Some(DataType::Raw(RawDataType::BigInt)),
            "date" => Some(DataType::Raw(RawDataType::Date)),
            "datetime" => Some(DataType::Raw(RawDataType::DateTime)),
            "time" => Some(DataType::Raw(RawDataType::Time)),
            "double" => Some(DataType::Raw(RawDataType::Double)),
            "float" => Some(DataType::Raw(RawDataType::Float)),
            "uuid" => Some(DataType::Raw(RawDataType::Uuid)),
            "_" => Some(DataType::Raw(RawDataType::Unknown)),
            _ => None,
        }
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
