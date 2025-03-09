use thiserror::Error;
use tracing::info;
pub fn proto_run() {
    info!("proto run");

    let person: Person = parse_message(&[
        0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a, 0x16, 0x0a, 0x0e,
        0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35, 0x2d, 0x31, 0x32, 0x31, 0x32, 0x12,
        0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a, 0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38, 0x30, 0x30, 0x2d,
        0x38, 0x36, 0x37, 0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d, 0x6f, 0x62, 0x69, 0x6c,
        0x65,
    ])
    .unwrap();

    info!("person {:#?}", person);
}

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid varint")]
    InvalidVarint,
    #[error("Invalid wire-type")]
    InvalidWireType,
    #[error("EOF")]
    UnexpectedEOF,
    #[error("Invalid size")]
    InvalidSize(#[from] std::num::TryFromIntError),
    #[error("Unexpected wire-type")]
    UnexpectedWireType,
    #[error("Invalid string (not utf08)")]
    InvalidString,
}

enum WireType {
    Varint,
    Len,
    I32,
}

#[derive(Debug)]
enum FieldValue<'a> {
    Varint(u64),
    Len(&'a [u8]),
    I32(i32),
}

#[derive(Debug)]
struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

trait ProtoMessage<'a>: Default + 'a {
    fn add_field(&mut self, field: Field<'a>) -> Result<(), Error>;
}

impl TryFrom<u64> for WireType {
    type Error = Error;
    fn try_from(value: u64) -> Result<WireType, Error> {
        match value {
            0 => Ok(WireType::Varint),
            2 => Ok(WireType::Len),
            5 => Ok(WireType::I32),
            _ => Err(Error::InvalidWireType),
        }
    }
}

impl<'a> FieldValue<'a> {
    fn as_string(&self) -> Result<&'a str, Error> {
        let FieldValue::Len(data) = self else {
            return Err(Error::UnexpectedWireType);
        };
        std::str::from_utf8(data).map_err(|_| Error::InvalidString)
    }

    fn as_bytes(&self) -> Result<&'a [u8], Error> {
        let FieldValue::Len(data) = self else {
            return Err(Error::UnexpectedWireType);
        };
        Ok(data)
    }

    fn as_u64(&self) -> Result<u64, Error> {
        let FieldValue::Varint(data) = self else {
            return Err(Error::UnexpectedWireType);
        };
        Ok(*data)
    }
}

fn parse_varint(data: &[u8]) -> Result<(u64, &[u8]), Error> {
    for i in 0..7 {
        let Some(b) = data.get(i) else {
            return Err(Error::InvalidVarint);
        };
        if dbg!(b) & 0x80 == 0 {
            let mut value = 0u64;

            for b in data[..=i].iter().rev() {
                value = (value << 7) | (b & 0x7f) as u64;
            }
            return Ok((dbg!(value), &data[i + 1..]));
        }
    }
    Err(Error::InvalidVarint)
}

fn unpack_tag(tag: u64) -> Result<(u64, WireType), Error> {
    let field_num = tag >> 3;
    let wire_type = WireType::try_from(tag & 0b111)?;
    Ok((field_num, wire_type))
}

fn parse_field(data: &[u8]) -> Result<(Field, &[u8]), Error> {
    let (tag, remainder) = parse_varint(data)?;
    let (field_num, wire_type) = unpack_tag(tag)?;
    let (fieldvalue, remainder) = match wire_type {
        WireType::Varint => {
            let (value, remainder) = parse_varint(remainder)?;
            (FieldValue::Varint(value), remainder)
        }
        WireType::Len => {
            let (length, remainder) = parse_varint(remainder)?;
            let len = length.try_into()?;
            if remainder.len() < len {
                return Err(Error::UnexpectedEOF);
            }
            let (value, remainder) = remainder.split_at(len);
            (FieldValue::Len(value), remainder)
        }
        WireType::I32 => {
            if remainder.len() < 4 {
                return Err(Error::UnexpectedEOF);
            }
            let (value, remainder) = remainder.split_at(4);
            let value = i32::from_be_bytes(value.try_into().unwrap());
            (FieldValue::I32(value), remainder)
        }
    };
    Ok((
        Field {
            field_num,
            value: fieldvalue,
        },
        remainder,
    ))
}

fn parse_message<'a, T: ProtoMessage<'a>>(mut data: &'a [u8]) -> Result<T, Error> {
    let mut result = T::default();
    while !data.is_empty() {
        let parsed: (Field<'_>, &[u8]) = parse_field(data)?;
        result.add_field(parsed.0)?;
        data = parsed.1;
    }
    Ok(result)
}

// TODO: implement protoMessage for person and phoneNumber

#[derive(Debug, Default)]
struct PhoneNumber<'a> {
    number: &'a str,
    type_: &'a str,
}

#[derive(Debug, Default)]
struct Person<'a> {
    name: &'a str,
    id: u64,
    phone: Vec<PhoneNumber<'a>>,
}

impl<'a> ProtoMessage<'a> for Person<'a> {
    fn add_field(&mut self, field: Field<'a>) -> Result<(), Error> {
        match field.field_num {
            1 => {
                self.name = field.value.as_string()?;
            }
            2 => {
                self.id = field.value.as_u64()?;
            }
            3 => {
                self.phone.push(parse_message(field.value.as_bytes()?)?);
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'a> ProtoMessage<'a> for PhoneNumber<'a> {
    fn add_field(&mut self, field: Field<'a>) -> Result<(), Error> {
        match field.field_num {
            1 => {
                self.number = field.value.as_string()?;
            }
            2 => {
                self.type_ = field.value.as_string()?;
            }
            _ => {}
        }
        Ok(())
    }
}
