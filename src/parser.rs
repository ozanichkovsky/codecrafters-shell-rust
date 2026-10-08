use std::marker::PhantomData;

trait State {}

struct Start;
struct Unquoted;
struct SingleQuoted;
struct DoubleQuoted;
struct Escape;

impl State for Start {}
impl State for Unquoted {}
impl State for SingleQuoted {}
impl State for DoubleQuoted {}
impl State for Escape {}

struct StateMachine<StateType> {
    current_token: String,
    tokens: Vec<String>,
    _marker: PhantomData<StateType>,
}

impl<StateType> StateMachine<StateType> {
    fn transition<NewState>(self) -> StateMachine<NewState> {
        StateMachine {
            current_token: self.current_token,
            tokens: self.tokens,
            _marker: PhantomData,
        }
    }

    // Ends the current word; quotes never end a word, only unquoted whitespace does
    fn end_token(mut self) -> StateMachine<Start> {
        self.tokens.push(std::mem::take(&mut self.current_token));
        self.transition()
    }
}

impl StateMachine<Start> {
    fn new() -> Self {
        Self {
            current_token: String::new(),
            tokens: Vec::new(),
            _marker: PhantomData,
        }
    }
}

trait ProcessState {
    fn process(self: Box<Self>, c: char) -> Box<dyn ProcessState>;
    fn finalize(self: Box<Self>) -> Vec<String>;
}

impl ProcessState for StateMachine<Start> {
    fn process(mut self: Box<Self>, c: char) -> Box<dyn ProcessState> {
        match c {
            ' ' | '\t' => self, // Skip whitespace between words
            '\'' => Box::new(self.transition::<SingleQuoted>()),
            '"' => Box::new(self.transition::<DoubleQuoted>()),
            _ => {
                self.current_token.push(c);
                Box::new(self.transition::<Unquoted>())
            }
        }
    }

    fn finalize(self: Box<Self>) -> Vec<String> {
        self.tokens
    }
}

impl ProcessState for StateMachine<Unquoted> {
    fn process(mut self: Box<Self>, c: char) -> Box<dyn ProcessState> {
        match c {
            ' ' | '\t' => Box::new(self.end_token()),
            '\'' => Box::new(self.transition::<SingleQuoted>()),
            '"' => Box::new(self.transition::<DoubleQuoted>()),
            _ => {
                self.current_token.push(c);
                self
            }
        }
    }

    fn finalize(self: Box<Self>) -> Vec<String> {
        self.end_token().tokens
    }
}

impl ProcessState for StateMachine<SingleQuoted> {
    fn process(mut self: Box<Self>, c: char) -> Box<dyn ProcessState> {
        match c {
            // Closing quote returns to the word, so adjacent parts are concatenated
            '\'' => Box::new(self.transition::<Unquoted>()),
            _ => {
                self.current_token.push(c);
                self
            }
        }
    }

    fn finalize(self: Box<Self>) -> Vec<String> {
        self.end_token().tokens
    }
}

impl ProcessState for StateMachine<DoubleQuoted> {
    fn process(mut self: Box<Self>, c: char) -> Box<dyn ProcessState> {
        match c {
            '"' => Box::new(self.transition::<Unquoted>()),
            _ => {
                self.current_token.push(c);
                self
            }
        }
    }

    fn finalize(self: Box<Self>) -> Vec<String> {
        self.end_token().tokens
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut state: Box<dyn ProcessState> = Box::new(StateMachine::<Start>::new());

    for c in input.chars() {
        state = state.process(c);
    }

    state.finalize()
}


#[cfg(test)]
mod test {
    use super::tokenize;

    #[test]
    fn single_quotes() {
        assert_eq!(tokenize("echo 'hello    world'"), vec!["echo", "hello    world"]);
        assert_eq!(tokenize("echo hello    world"), vec!["echo", "hello", "world"]);
        assert_eq!(tokenize("echo 'hello''world'"), vec!["echo", "helloworld"]);
        assert_eq!(tokenize("echo hello''world"), vec!["echo", "helloworld"]);
        assert_eq!(tokenize("cat '/tmp/a b' 'c'"), vec!["cat", "/tmp/a b", "c"]);
    }
}
