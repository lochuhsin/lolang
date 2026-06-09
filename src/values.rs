use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::vm::RuntimeError;

#[derive(Clone, Debug)]
pub enum Value {
    Str(String),
}
#[derive(Clone, Debug)]
pub struct DynamicSizeObject {
    // looks like a node or something, for future gc purposes
    value: Value,
    prev: Option<Box<DynamicSizeObject>>, // Pure heap alloc, find a way free
    next: Option<Box<DynamicSizeObject>>,
}

impl DynamicSizeObject {
    pub fn from_string(s: String) -> DynamicSizeObject {
        DynamicSizeObject {
            value: Value::Str(s),
            prev: None,
            next: None,
        }
    }
}

impl Add for DynamicSizeObject {
    type Output = Result<DynamicSizeObject, RuntimeError>; // Should be using Result, and define an error for compiler error to handler
    fn add(self, rhs: Self) -> Self::Output {
        match (&self.value, &rhs.value) {
            (Value::Str(s1), Value::Str(s2)) => {
                Ok(DynamicSizeObject::from_string(s1.to_owned() + s2))
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                String::from("Generic object type1 To be implemented"),
                String::from("Generic object type2 To be implemented"),
            )),
        }
    }
}

impl PartialEq for DynamicSizeObject {
    fn eq(&self, other: &Self) -> bool {
        match (&self.value, &other.value) {
            (Value::Str(s1), Value::Str(s2)) => s1 == s2,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum GenericValueType {
    Bool(bool),
    Number(f64),
    Object(DynamicSizeObject),
    Nil,
}

impl GenericValueType {
    fn get_type_as_str(&self) -> String {
        match self {
            GenericValueType::Bool(_) => String::from("bool"),
            GenericValueType::Number(_) => String::from("number"),
            GenericValueType::Nil => String::from("nil"),
            GenericValueType::Object(obj) => match obj.value.clone() {
                Value::Str(s) => s,
            },
        }
    }
}

pub type GenericValue = GenericValueType;

impl GenericValue {
    pub fn from_bool(value: bool) -> GenericValue {
        GenericValue::Bool(value)
    }
    pub fn from_f64(value: f64) -> GenericValue {
        GenericValue::Number(value)
    }
    pub fn from_none() -> GenericValue {
        GenericValue::Nil
    }
    pub fn from_string(value: String) -> GenericValue {
        GenericValue::Object(DynamicSizeObject::from_string(value))
    }
    pub fn from_object(value: DynamicSizeObject) -> GenericValue {
        GenericValue::Object(value)
    }
}

impl Default for GenericValue {
    fn default() -> Self {
        GenericValue::Nil
    }
}

impl Display for GenericValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericValueType::Bool(v) => write!(f, "{}", v),
            GenericValueType::Number(v) => write!(f, "{}", v),
            GenericValueType::Nil => write!(f, "nil"),
            GenericValueType::Object(v) => match v.value.clone() {
                Value::Str(s) => write!(f, "String<Object>: {}", s),
            },
        }
    }
}

impl GenericValue {
    pub fn as_bool(&self) -> Option<bool> {
        if let GenericValueType::Bool(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        if let GenericValueType::Number(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let GenericValueType::Object(o) = self {
            let Value::Str(s) = &o.value;
            Some(s.clone())
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<DynamicSizeObject> {
        if let GenericValueType::Object(o) = self {
            Some(o.clone())
        } else {
            None
        }
    }
}

impl Add for GenericValue {
    type Output = Result<GenericValueType, RuntimeError>; // Should be using Result, and define an error for compiler error to handler
    fn add(self, other: GenericValue) -> Result<Self, RuntimeError> {
        match (&self, &other) {
            (GenericValueType::Number(lhs), GenericValueType::Number(rhs)) => {
                Ok(GenericValue::from_f64(lhs + rhs))
            }
            (GenericValueType::Object(o1), GenericValueType::Object(o2)) => {
                let new_o = (o1.clone() + o2.clone())?;
                Ok(GenericValue::from_object(new_o))
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str(),
                other.get_type_as_str(),
            )),
        }
    }
}

impl Sub for GenericValue {
    type Output = Result<GenericValueType, RuntimeError>;

    fn sub(self, other: GenericValueType) -> Result<Self, RuntimeError> {
        match (&self, &other) {
            (GenericValueType::Number(lhs), GenericValueType::Number(rhs)) => {
                Ok(GenericValue::from_f64(lhs - rhs))
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str(),
                other.get_type_as_str(),
            )),
        }
    }
}

impl Mul for GenericValue {
    type Output = Result<GenericValueType, RuntimeError>;

    fn mul(self, other: GenericValueType) -> Result<Self, RuntimeError> {
        match (&self, &other) {
            (GenericValueType::Number(lhs), GenericValueType::Number(rhs)) => {
                Ok(GenericValue::from_f64(lhs * rhs))
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str(),
                other.get_type_as_str(),
            )),
        }
    }
}

impl Div for GenericValue {
    type Output = Result<GenericValueType, RuntimeError>;

    fn div(self, other: GenericValueType) -> Result<Self, RuntimeError> {
        match (&self, &other) {
            (GenericValueType::Number(lhs), GenericValueType::Number(rhs)) => {
                if *rhs == 0.0 {
                    Err(RuntimeError::InvalidOperation(
                        "could not divide by zero".to_string(),
                    ))
                } else {
                    Ok(GenericValue::from_f64(lhs / rhs))
                }
            }
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str(),
                other.get_type_as_str(),
            )),
        }
    }
}

impl Neg for GenericValue {
    type Output = Result<Self, RuntimeError>;

    fn neg(self) -> Result<Self, RuntimeError> {
        match self {
            GenericValue::Number(value) => Ok(GenericValue::Number(-value)),
            _ => Err(RuntimeError::UnsupportedOperation(
                self.get_type_as_str(),
                self.get_type_as_str(),
            )),
        }
    }
}

impl PartialEq for GenericValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GenericValueType::Number(v1), GenericValueType::Number(v2)) => v1 == v2,
            (GenericValueType::Bool(b1), GenericValueType::Bool(b2)) => b1 == b2,
            (GenericValueType::Object(o1), GenericValueType::Object(o2)) => o1 == o2,
            _ => false,
        }
    }
}

impl Eq for GenericValue {}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueArray {
    pub values: Vec<GenericValue>,
    pub count: usize,
}

impl ValueArray {
    pub fn new(values: Vec<GenericValue>) -> ValueArray {
        ValueArray {
            count: values.len(),
            values,
        }
    }
    pub fn write_value_array(&mut self, value: GenericValue) {
        self.values.push(value);
        self.count += 1;
    }
}
