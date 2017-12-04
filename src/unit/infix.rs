trait CharExt{
    fn is_op(self) -> bool;
    fn lower_precedence(self, other: Self) -> bool;
}

impl CharExt for char {
    fn is_op(self) -> bool {
        match self{
            '+'|'-'|'*'|'/'|'^'|'('|')' => true,
            '⁻'|'⁰'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹' => true,
            _ => false,
        }
    }
    fn lower_precedence(self, o: Self) -> bool {
        match self {
            '=' => false,
            '+'|'-' => o!='+'&&o!='-',
            '*'|'/' => o=='^' || o=='(',
            '⁻'|'⁰'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹'|'^' => o=='(',
            '(' => true,
            _ => false
        }
    }
}

use tokeniser::Tokeniser;

#[allow(dead_code)]
pub fn infix_to_postfix(infix: &str) -> Option<String> {
    let mut operator_stack = Vec::<char>::new();
    let mut postfix = String::with_capacity(infix.len());

    for token in Tokeniser::new(infix) {
        let c = token.chars().next().unwrap();

        if token.len() == 1 && c.is_op() {
            while let Some(op) = operator_stack.pop() {
                if op.lower_precedence(c) {
                    operator_stack.push(op);
                    break;
                }
                postfix.push(' ');
                postfix.push(op);
            }
            if c == ')' {
                let mut operator = operator_stack.pop()?;
                while operator != '(' {
                    postfix.push(' ');
                    postfix.push(operator);
                    operator = operator_stack.pop()?;
                }
            } else{
                operator_stack.push(c);
            }
        } else if token.len() == 1 && c == ' ' {
        } else { // it's an operand
            postfix.push(' ');
            postfix.push_str(&token);
        }
    }

    for o in operator_stack.into_iter() {
        postfix.push(' ');
        postfix.push(o);
    }

    Some(postfix.trim().to_owned())
}
