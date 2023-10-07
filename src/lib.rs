use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Error {
    WrongCode,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongCode => write!(f, "WrongCode"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
pub enum Pauli {
    #[default]
    I = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

impl TryFrom<u8> for Pauli {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Pauli::*;
        match value {
            0 => Ok(I),
            1 => Ok(X),
            2 => Ok(Y),
            3 => Ok(Z),
            _ => Err(Self::Error::WrongCode),
        }
    }
}

impl From<Pauli> for u8 {
    fn from(value: Pauli) -> Self {
        match value {
            Pauli::I => 0,
            Pauli::X => 1,
            Pauli::Y => 2,
            Pauli::Z => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pauli_01() {
        assert_eq!(Pauli::try_from(0).unwrap(), Pauli::I);
        assert_eq!(Pauli::try_from(1).unwrap(), Pauli::X);
        assert_eq!(Pauli::try_from(2).unwrap(), Pauli::Y);
        assert_eq!(Pauli::try_from(3).unwrap(), Pauli::Z);
    }

    #[test]
    fn test_pauli_02() {
        let err = Pauli::try_from(4).unwrap_err();
        assert_eq!(err, Error::WrongCode);
    }

    #[test]
    fn test_pauli_03() {
        assert_eq!(u8::from(Pauli::I), 0);
        assert_eq!(u8::from(Pauli::X), 1);
        assert_eq!(u8::from(Pauli::Y), 2);
        assert_eq!(u8::from(Pauli::Z), 3);
    }
}
