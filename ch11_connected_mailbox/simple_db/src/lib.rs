#[derive(Debug, PartialEq)]
pub enum Command {
    RETRIEVE,
    PUBLISH(String),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteMessage,
    UnexpectedNewline,
    EmptyMessage,
    UnknownCommand,
    UnexpectedPayload,
    MissingPayload,
}

pub fn parse(input: &str) -> Result<Command, Error> {
    let s = input.strip_suffix('\n').ok_or(Error::IncompleteMessage)?;
    if s.is_empty() {
        return Err(Error::EmptyMessage);
    }
    if s.contains('\n') {
        return Err(Error::UnexpectedNewline);
    }

    if let Some(rest) = s.strip_prefix("RETRIEVE") {
        if rest.is_empty() {
            return Ok(Command::RETRIEVE);
        } else {
            return Err(Error::UnexpectedPayload);
        }
    }

    if let Some(rest) = s.strip_prefix("PUBLISH") {
        let payload = rest.strip_prefix(' ').ok_or(Error::MissingPayload)?;
        return Ok(Command::PUBLISH(payload.into()));
    }

    return Err(Error::UnknownCommand);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests placement of \n
    #[test]
    fn test_missing_nl() {
        let line = "RETRIEVE";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_trailing_data() {
        let line = "PUBLISH The message\n is wrong \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedNewline);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_string() {
        let line = "";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::IncompleteMessage);
        assert_eq!(result, expected);
    }

    // Tests for empty messages and unknown commands
    #[test]
    fn test_only_nl() {
        let line = "\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::EmptyMessage);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unknown_command() {
        let line = "SERVE\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnknownCommand);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of RETRIEVE command
    #[test]
    fn test_retrieve_w_whitespace() {
        let line = "RETRIEVE \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve_payload() {
        let line = "RETRIEVE this has a payload\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::UnexpectedPayload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_retrieve() {
        let line = "RETRIEVE\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::RETRIEVE);
        assert_eq!(result, expected);
    }

    // Tests correct formatting of PUBLISH command
    #[test]
    fn test_publish() {
        let line = "PUBLISH TestMessage\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::PUBLISH("TestMessage".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_publish() {
        let line = "PUBLISH \n";
        let result: Result<Command, Error> = parse(line);
        let expected = Ok(Command::PUBLISH("".into()));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_missing_publish() {
        let line = "PUBLISH\n";
        let result: Result<Command, Error> = parse(line);
        let expected = Err(Error::MissingPayload);
        assert_eq!(result, expected);
    }
}
