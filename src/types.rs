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

    VarChar(usize),
    Char(usize),
    Text(usize),
}

#[derive(Debug)]
pub enum DataType {
    Raw(RawDataType),
    Inferred(RawDataType),
}

impl DataType {
    pub fn parse(input: &str, argument: Option<&str>) -> Option<Self> {
        match (input, argument) {
            ("int", None) => Some(DataType::Raw(RawDataType::Int)),
            ("bool", None) => Some(DataType::Raw(RawDataType::Bool)),
            ("bigint", None) => Some(DataType::Raw(RawDataType::BigInt)),
            ("date", None) => Some(DataType::Raw(RawDataType::Date)),
            ("datetime", None) => Some(DataType::Raw(RawDataType::DateTime)),
            ("time", None) => Some(DataType::Raw(RawDataType::Time)),
            ("double", None) => Some(DataType::Raw(RawDataType::Double)),
            ("float", None) => Some(DataType::Raw(RawDataType::Float)),
            ("uuid", None) => Some(DataType::Raw(RawDataType::Uuid)),
            ("_", None) => Some(DataType::Raw(RawDataType::Unknown)),

            ("varchar", Some(length)) => match length.parse() {
                Ok(l) => Some(DataType::Raw(RawDataType::VarChar(l))),
                _ => None,
            },
            ("char", Some(length)) => match length.parse() {
                Ok(l) => Some(DataType::Raw(RawDataType::Char(l))),
                _ => None,
            },
            ("text", Some(length)) => match length.parse() {
                Ok(l) => Some(DataType::Raw(RawDataType::Text(l))),
                _ => None,
            },

            _ => None,
        }
    }
}

impl RawDataType {
    pub fn parse(input: &str, argument: Option<&str>) -> Option<Self> {
        let dt = DataType::parse(input, argument);

        match dt {
            Some(dt) => match dt {
                DataType::Raw(rdt) => Some(rdt),
                DataType::Inferred(_) => None,
            },
            None => None,
        }
    }
}
