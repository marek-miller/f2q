use std::{
    collections::HashMap,
    fmt::Display,
};

use num::Float;

#[derive(Debug, PartialEq)]
pub enum Error {
    NoCode,
}

impl Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::NoCode => write!(f, "WrongCode"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
pub enum Pauli {
    #[default]
    I,
    X,
    Y,
    Z,
}

impl Pauli {
    fn from_u128(value: u128) -> Self {
        Self::try_from(value)
            .expect("value should be an integer between 0 and 3")
    }
}

macro_rules! impl_pauli_int {
    ( $($Typ:ty)* ) => {
        $(

            impl TryFrom<$Typ> for Pauli {
                type Error = Error;

                fn try_from(value: $Typ) -> Result<Self, Self::Error> {
                    use Pauli::*;
                    match value {
                        0 => Ok(I),
                        1 => Ok(X),
                        2 => Ok(Y),
                        3 => Ok(Z),
                        _ => Err(Self::Error::NoCode),
                    }
                }
            }

            impl From<Pauli> for $Typ {
                fn from(value: Pauli) -> Self {
                    match value {
                        Pauli::I => 0,
                        Pauli::X => 1,
                        Pauli::Y => 2,
                        Pauli::Z => 3,
                    }
                }
            }

        )*
    };
}

impl_pauli_int!(u8 u16 u32 u64 u128);
impl_pauli_int!(i8 i16 i32 i64 i128);

/// Pauli string of up to 64 qubits.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PauliCode {
    pack: u128,
}

impl PauliCode {
    pub fn new(pack: u128) -> Self {
        Self {
            pack,
        }
    }

    pub fn as_u128(&self) -> &u128 {
        &self.pack
    }

    pub fn iter(&self) -> Codes<'_> {
        Codes::new(self)
    }

    pub fn from_paulis<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Pauli>,
    {
        let pack = (0..32)
            .zip(iter.into_iter())
            .fold(0, |acc, (i, pauli)| acc + (u128::from(pauli) << (i * 2)));
        Self::new(pack)
    }
}

// Iterate over Paulis in PauliCode
#[derive(Debug)]
pub struct Codes<'a> {
    code:  &'a PauliCode,
    index: u8,
}

impl<'a> Codes<'a> {
    fn new(code: &'a PauliCode) -> Self {
        Self {
            code,
            index: 0,
        }
    }
}

impl<'a> Iterator for Codes<'a> {
    type Item = Pauli;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 64 {
            return None;
        }

        let pauli_int = (self.code.pack >> self.index * 2) & 0b11;
        self.index += 1;

        Some(Pauli::from_u128(pauli_int))
    }
}

#[derive(Debug)]
pub struct PauliHamil<T> {
    map: HashMap<PauliCode, T>,
}

impl<T> PauliHamil<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn as_map(&self) -> &HashMap<PauliCode, T> {
        &self.map
    }

    pub fn as_map_mut(&mut self) -> &mut HashMap<PauliCode, T> {
        &mut self.map
    }
}

impl<T> PauliHamil<T>
where
    T: Float,
{
    pub fn coeff(
        &self,
        code: &PauliCode,
    ) -> T {
        match self.map.get(code) {
            Some(coeff) => *coeff,
            None => T::zero(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pauli_01() {
        assert_eq!(Pauli::try_from(0u128).unwrap(), Pauli::I);
        assert_eq!(Pauli::try_from(1u128).unwrap(), Pauli::X);
        assert_eq!(Pauli::try_from(2u128).unwrap(), Pauli::Y);
        assert_eq!(Pauli::try_from(3u128).unwrap(), Pauli::Z);
    }

    #[test]
    fn test_pauli_02() {
        let err = Pauli::try_from(4u128).unwrap_err();
        assert_eq!(err, Error::NoCode);
    }

    #[test]
    fn test_pauli_03() {
        assert_eq!(u8::from(Pauli::I), 0);
        assert_eq!(u8::from(Pauli::X), 1);
        assert_eq!(u8::from(Pauli::Y), 2);
        assert_eq!(u8::from(Pauli::Z), 3);
    }

    #[test]
    fn test_paulicode_init() {
        let pauli = PauliCode::new(0b01);
        assert_eq!(*pauli.as_u128(), 0b01);
    }

    #[test]
    fn test_paulicode_codes_iter_01() {
        use Pauli::*;
        let result = PauliCode::new(0b01).iter().take(3).collect::<Vec<_>>();

        assert_eq!(result, &[X, I, I])
    }

    #[test]
    fn test_paulicode_codes_iter_02() {
        use Pauli::*;
        let result =
            PauliCode::new(0b111001).iter().take(5).collect::<Vec<_>>();

        assert_eq!(result, &[X, Y, Z, I, I])
    }

    #[test]
    fn test_paulicode_from_paulis_01() {
        use Pauli::*;

        assert_eq!(
            PauliCode::from_paulis([I, X, Y, Z]),
            PauliCode::new(0b11100100)
        )
    }

    #[test]
    fn test_paulihamil_init_01() {
        let code = PauliCode::new(1234);
        let mut hamil = PauliHamil::new();

        hamil.as_map_mut().insert(code, 4321.);
        assert_eq!(hamil.coeff(&code), 4321.)
    }
}
