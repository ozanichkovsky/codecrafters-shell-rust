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
    fn finalize(mut self) -> Vec<String> {
        if !self.current_token.is_empty() {
            self.tokens.push(self.current_token);
        }
        self.tokens
    }

    fn new_with_token(current_token: String, mut tokens: Vec<String>) -> StateMachine<Start> {
        tokens.push(current_token);
        StateMachine {
            current_token: String::new(),
            tokens,
            _marker: PhantomData,
        }
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

    fn process(self, c: char) -> Box<dyn ProcessState> {
        match c {
            ' ' | '\t' => Box::new(self), // Ignore leading whitespace
            '\'' => Box::new(self.enter_single_quoted()),
            '"' => Box::new(self.enter_double_quoted()),
            _ => {
                let mut machine = self;
                machine.current_token.push(c);
                Box::new(machine.enter_unquoted())
            }
        }
    }

    fn enter_single_quoted(mut self) -> StateMachine<SingleQuoted> {
        if self.tokens.len() > 0 && self.tokens.last().unwrap().is_empty() {
            self.tokens.pop();
        }
        StateMachine {
            current_token: self.current_token,
            tokens: self.tokens,
            _marker: PhantomData,
        }
    }

    fn enter_double_quoted(mut self) -> StateMachine<DoubleQuoted> {
        if self.tokens.len() > 0 && self.tokens.last().unwrap().is_empty() {
            self.tokens.pop();
        }
        StateMachine {
            current_token: self.current_token,
            tokens: self.tokens,
            _marker: PhantomData,
        }
    }

    fn enter_unquoted(mut self) -> StateMachine<Unquoted> {
        if self.tokens.len() > 0 && self.tokens.last().unwrap().is_empty() {
            self.tokens.pop();
        }
        StateMachine {
            current_token: self.current_token,
            tokens: self.tokens,
            _marker: PhantomData,
        }
    }
}

trait ProcessState {
    fn process(self: Box<Self>, c: char) -> Box<dyn ProcessState>;
    fn finalize(self: Box<Self>) -> Vec<String>;
}

impl ProcessState for StateMachine<Start> {
    fn process(self: Box<Self>, c: char) -> Box<dyn ProcessState> {
        (*self).process(c)
    }

    fn finalize(self: Box<Self>) -> Vec<String> {
        (*self).finalize()
    }
}

impl ProcessState for StateMachine<Unquoted> {
    fn process(mut self: Box<Self>, c: char) -> Box<dyn ProcessState> {
        match c {
            ' ' | '\t' => {
                if !self.current_token.is_empty() {
                    self.tokens.push(self.current_token.clone());
                }
                Box::new(StateMachine::<Start>::new_with_token("".into(), self.tokens))
            }
            _ => {
                self.current_token.push(c);
                self
            }
        }
    }

    fn finalize(self: Box<Self>) -> Vec<String> {
        (*self).finalize()
    }
}

impl ProcessState for StateMachine<SingleQuoted> {
    fn process(mut self: Box<Self>, c: char) -> Box<dyn ProcessState> {
        match c {
            '\'' => Box::new(StateMachine::<Start>::new_with_token(
                self.current_token.clone(),
                self.tokens.clone(),
            )),
            _ => {
                self.current_token.push(c);
                self
            }
        }
    }

    fn finalize(self: Box<Self>) -> Vec<String> {
        (*self).finalize()
    }
}

impl ProcessState for StateMachine<DoubleQuoted> {
    fn process(mut self: Box<Self>, c: char) -> Box<dyn ProcessState> {
        match c {
            '"' => Box::new(StateMachine::<Start>::new_with_token(
                self.current_token.clone(),
                self.tokens.clone(),
            )),
            _ => {
                self.current_token.push(c);
                self
            }
        }
    }

    fn finalize(self: Box<Self>) -> Vec<String> {
        (*self).finalize()
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut state: Box<dyn ProcessState> = Box::new(StateMachine::<Start>::new());

    for c in input.chars() {
        state = state.process(c);
    }

    state.finalize()
}


mod test {

}
