use std::collections::HashMap;

pub type Value = i32;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub struct Forth {
    stack: Vec<Value>,
    method_table: MethodTable,
}

type MethodTable = HashMap<String, VersionedMethod>;
type VersionedMethod = Vec<MethodBody>;
type MethodBody = Vec<Expression>;
#[derive(Clone)]
enum Expression {
    Load(Value),
    Dispatch(String, usize), // (method_name, version)
    Plus,
    Minus,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
}

#[derive(Debug)]
enum ParserResult<'input> {
    Word(LoadOrDispatch<'input>),
    Declaration(&'input str, Vec<LoadOrDispatch<'input>>),
}

#[derive(Debug)]
enum LoadOrDispatch<'input> {
    Load(i32),
    Dispatch(&'input str),
}

struct Parser<'input> {
    words: Vec<&'input str>,
    pointer: usize,
}

impl<'input> Parser<'input> {
    fn new(input: &'input str) -> Self {
        Self {
            words: input.split(" ").collect(),
            pointer: 0,
        }
    }

    fn pop_one(&mut self) -> Option<&'input str> {
        if self.pointer >= self.words.len() {
            return None;
        }
        self.pointer += 1;
        Some(self.words[self.pointer - 1])
    }
}

impl<'input> Iterator for Parser<'input> {
    type Item = Result<ParserResult<'input>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let current;
        if let Some(str) = self.pop_one() {
            current = str;
        } else {
            return None;
        }
        let is_word = current != ":";
        if is_word {
            return match current.parse::<i32>() {
                Ok(val) => Some(Ok(ParserResult::Word(LoadOrDispatch::Load(val)))),
                Err(_) => Some(Ok(ParserResult::Word(LoadOrDispatch::Dispatch(current)))),
            };
        } else {
            let id;
            if let Some(str) = self.pop_one() {
                if str.starts_with(|c: char| c.is_numeric()) {
                    return Some(Err(Error::InvalidWord));
                }
                id = str;
            } else {
                return Some(Err(Error::InvalidWord));
            }
            let mut body = vec![];
            loop {
                match self.pop_one() {
                    Some(";") => break,
                    None | Some(":") => {
                        return Some(Err(Error::InvalidWord));
                    }
                    Some(word) => match word.parse::<i32>() {
                        Ok(val) => body.push(LoadOrDispatch::Load(val)),
                        Err(_) => body.push(LoadOrDispatch::Dispatch(word)),
                    },
                }
            }
            return Some(Ok(ParserResult::Declaration(id, body)));
        }
    }
}

impl Forth {
    pub fn new() -> Self {
        let mut method_table: MethodTable = HashMap::new();
        // install default methods
        method_table.insert("+".to_string(), vec![vec![Expression::Plus]]);
        method_table.insert("-".to_string(), vec![vec![Expression::Minus]]);
        method_table.insert("*".to_string(), vec![vec![Expression::Mul]]);
        method_table.insert("/".to_string(), vec![vec![Expression::Div]]);
        method_table.insert("dup".to_string(), vec![vec![Expression::Dup]]);
        method_table.insert("drop".to_string(), vec![vec![Expression::Drop]]);
        method_table.insert("swap".to_string(), vec![vec![Expression::Swap]]);
        method_table.insert("over".to_string(), vec![vec![Expression::Over]]);
        Self {
            stack: vec![],
            method_table,
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval_dispatch(&mut self, id: &str, version: usize) -> Result<(), Error> {
        // id is guaranteed to in the method table
        let len = self.method_table.get(id).unwrap()[version].len();
        for i in 0..len {
            match self.method_table.get(id).unwrap()[version][i].clone() {
                Expression::Load(val) => self.stack.push(val),
                Expression::Dispatch(id, version) => self.eval_dispatch(&id, version)?,
                Expression::Plus => {
                    let v2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let v1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(v1 + v2);
                }
                Expression::Minus => {
                    let v2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let v1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(v1 - v2);
                }
                Expression::Mul => {
                    let v2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let v1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(v1 * v2);
                }
                Expression::Div => {
                    let v2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let v1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    if v2 == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    self.stack.push(v1 / v2);
                }
                Expression::Dup => {
                    let v = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(v);
                    self.stack.push(v);
                }
                Expression::Drop => {
                    self.stack.pop().ok_or(Error::StackUnderflow)?;
                }
                Expression::Swap => {
                    let v2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let v1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(v2);
                    self.stack.push(v1);
                }
                Expression::Over => {
                    let v2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let v1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(v1);
                    self.stack.push(v2);
                    self.stack.push(v1);
                }
            }
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        let parser = Parser::new(input);
        for parser_result in parser.into_iter() {
            let parser_result = parser_result?;
            match parser_result {
                ParserResult::Word(LoadOrDispatch::Load(val)) => self.stack.push(val),
                ParserResult::Word(LoadOrDispatch::Dispatch(id)) => {
                    let id = id.to_lowercase();
                    if let Some(versioned_method) = self.method_table.get(&id) {
                        // dispatch to the latest version
                        // should be evaluated recursively and depth first
                        self.eval_dispatch(&id, versioned_method.len() - 1)?;
                    } else {
                        return Err(Error::UnknownWord);
                    }
                }
                ParserResult::Declaration(id, body) => {
                    let id = id.to_lowercase();
                    let mut method_body = vec![];
                    for e in body.iter() {
                        match e {
                            LoadOrDispatch::Load(i) => method_body.push(Expression::Load(*i)),
                            LoadOrDispatch::Dispatch(id) => {
                                let id = id.to_lowercase();
                                if let Some(versioned_method) = self.method_table.get(&id) {
                                    method_body.push(Expression::Dispatch(
                                        id.to_lowercase(),
                                        versioned_method.len() - 1, // dispatch to the latest version
                                    ))
                                } else {
                                    return Err(Error::UnknownWord);
                                }
                            }
                        }
                    }
                    self.method_table
                        .entry(id)
                        .or_insert(vec![])
                        .push(method_body);
                }
            }
        }
        Ok(())
    }
}
