use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    Insert(u8),
    Contains(u8),
    Remove(u8),
    Get,
}

#[derive(PartialEq, Eq, Debug)]
enum Response {
    Yes,
    No,
    Ok,
    Values(Vec<u8>),
    Error(String),
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        let operation = *tokens
            .first()
            .ok_or("expected operation as first argument")?;

        match operation {
            "INSERT" => {
                let operand: u8 = tokens
                    .get(1)
                    .ok_or("expected number as second argument")?
                    .parse()
                    .map_err(|_| "failed to parse number")?;

                Ok(Operation::Insert(operand))
            }
            "CONTAINS" => {
                let operand: u8 = tokens
                    .get(1)
                    .ok_or("expected number as second argument")?
                    .parse()
                    .map_err(|_| "failed to parse number")?;

                Ok(Operation::Contains(operand))
            }
            "REMOVE" => {
                let operand: u8 = tokens
                    .get(1)
                    .ok_or("expected number as second argument")?
                    .parse()
                    .map_err(|_| "failed to parse number")?;

                Ok(Operation::Remove(operand))
            }
            "GET" => Ok(Operation::Get),
            _ => Err("unknown operation"),
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Yes => write!(f, "YES"),
            Response::No => write!(f, "NO"),
            Response::Ok => write!(f, "OK"),
            Response::Values(values) => {
                write!(
                    f,
                    "VALUES {}",
                    values
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            Response::Error(reason) => write!(f, "ERROR \"{}\"", reason),
        }
    }
}

fn main() {
    let _ = Operation::from_str("INSERT 10").unwrap();
    let _ = Response::Ok.to_string();
    let _ = Response::Yes.to_string();
    let _ = Response::No.to_string();
    let _ = Response::Values(vec![1, 2, 3]).to_string();
    let _ = Response::Error(String::from("failure")).to_string();
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{Operation, Response};

    #[test]
    fn parse_operation() {
        let cases = [
            ("INSERT 10", Operation::Insert(10)),
            ("CONTAINS 10", Operation::Contains(10)),
            ("REMOVE 10", Operation::Remove(10)),
            ("GET", Operation::Get),
        ];

        for (operation_string, expected_operation) in cases {
            let operation = Operation::from_str(operation_string).unwrap();
            assert_eq!(operation, expected_operation)
        }
    }

    #[test]
    fn print_response() {
        let cases = [
            (Response::Ok, "OK"),
            (Response::Yes, "YES"),
            (Response::No, "NO"),
            (Response::Values(vec![1, 2, 3]), "VALUES 1 2 3"),
            (
                Response::Error(String::from("failure")),
                "ERROR \"failure\"",
            ),
        ];

        for (response, expected_response_string) in cases {
            let response_string = response.to_string();
            assert_eq!(response_string, expected_response_string)
        }
    }
}
