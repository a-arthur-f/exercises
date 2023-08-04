pub fn calc(expr: &str) -> Result<f64, String> {
    let tokens = tokenizer(expr)?;
    let postfix = into_postfix(tokens);

    match parse_tokens(postfix) {
        Some(val) => Ok(val),
        None => Err(format!("Parse failed")),
    }
}

fn tokenizer(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];

    for e in expr.split(' ') {
        if let Some(op) = Token::op_from(e.chars().next().unwrap()) {
            tokens.push(op);
        } else if let Some(val) = Token::val_from(e) {
            tokens.push(val);
        } else {
            return Err(format!("Tokenizer failed"));
        }
    }

    Ok(tokens)
}

fn into_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut operators: Vec<Op> = vec![];
    let mut output: Vec<Token> = vec![];

    for token in tokens {
        match token {
            Token::Val(_) => output.push(token),
            Token::Operator(op) => {
                while let Some(stack_op) = operators.last() {
                    if stack_op.precedence() >= op.precedence() {
                        let stack_op = operators.pop().unwrap();
                        output.push(Token::Operator(stack_op));
                    } else {
                        break;
                    }
                }

                operators.push(op);
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(Token::Operator(op));
    }

    output
}

fn parse_tokens(tokens: Vec<Token>) -> Option<f64> {
    let mut result: Vec<f64> = vec![];

    for token in tokens {
        match token {
            Token::Val(val) => result.push(val),
            Token::Operator(op) => {
                let operand_2 = result.pop()?;
                let operand_1 = result.pop()?;

                result.push(op.calc(operand_1, operand_2));
            }
        }
    }

    result.first().copied()
}

#[derive(Debug)]
enum Token {
    Val(f64),
    Operator(Op),
}

#[derive(Debug)]
enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Token {
    fn op_from(ch: char) -> Option<Token> {
        match ch {
            '+' => Some(Token::Operator(Op::Plus)),
            '-' => Some(Token::Operator(Op::Minus)),
            '*' => Some(Token::Operator(Op::Multiply)),
            '/' => Some(Token::Operator(Op::Divide)),
            _ => None,
        }
    }

    fn val_from(str: &str) -> Option<Token> {
        match str.parse() {
            Ok(val) => Some(Token::Val(val)),
            Err(_) => None,
        }
    }
}

impl Op {
    fn precedence(&self) -> u8 {
        match self {
            Self::Plus | Self::Minus => 1,
            Self::Multiply | Self::Divide => 2,
        }
    }

    fn calc(&self, operand_1: f64, operand_2: f64) -> f64 {
        match self {
            Self::Plus => operand_1 + operand_2,
            Self::Minus => operand_1 - operand_2,
            Self::Multiply => operand_1 * operand_2,
            Self::Divide => operand_1 / operand_2,
        }
    }
}
