mod tokenize {

    use crate::tokens;

    #[test]
    fn test_with_padding() {
        let s = String::from("  1  ");
        let ts = tokens::tokenize(&s).unwrap();
        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0], tokens::Token::Num(String::from("1")));
    }

    #[test]
    fn test_single_integer() {
        let s = String::from("1");
        let ts = tokens::tokenize(&s).unwrap();
        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0], tokens::Token::Num(s));
    }

    #[test]
    fn test_adding_integers() {
        let s = String::from("1 + 2");
        let ts = tokens::tokenize(&s).unwrap();
        assert_eq!(ts.len(), 3);
        assert_eq!(ts[0], tokens::Token::Num(String::from("1")));
        assert_eq!(ts[1], tokens::Token::Plus);
        assert_eq!(ts[2], tokens::Token::Num(String::from("2")));
    }

    #[test]
    fn test_str_literal() {
        let s = String::from("\"my string 1\"");
        let ts = tokens::tokenize(&s).unwrap();
        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0], tokens::Token::Str(s));
    }

    #[test]
    fn test_float_literal() {
        let s = String::from("1.72");
        let ts = tokens::tokenize(&s).unwrap();
        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0], tokens::Token::Num(s));
    }
}

mod parse {

    use crate::parser::{self, Expr, Prog};
    use crate::tokens::Token;
    use parser::Stmt;

    #[test]
    fn test_single_integer() {
        let ts = [Token::Num(String::from("1")), Token::NewLine];
        let res = parser::parse(&ts);

        match res {
            Ok(Prog::Stmt(box Stmt::Expr(Expr::Num(n)), box Prog::End)) => assert_eq!(n, 1.0),
            _ => unreachable!(),
        }
    }
}

mod interpret {
    use crate::{
        interpreter,
        parser::{Expr, Prog, Stmt},
    };

    #[test]
    fn test_single_integer() {
        let prog = Prog::Stmt(box Stmt::Expr(Expr::Num(1.0)), box Prog::End);
        match interpreter::interpret(prog) {
            Ok(out) => assert_eq!(out, "1\n"),
            _ => unreachable!(),
        }
    }
}

mod eval {

    use crate::eval;

    fn assert_eval(input: &str, expected: &str) {
        match eval(input) {
            Ok(s) => assert_eq!(s, expected),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_with_padding() {
        assert_eval("  1  \n ", "1\n")
    }

    #[test]
    fn test_single_integer() {
        assert_eval("1\n", "1\n")
    }

    #[test]
    fn test_adding_integers() {
        assert_eval("1 + 3\n", "4\n")
    }

    #[test]
    fn test_multiplying_integers() {
        assert_eval("2 * 3\n", "6\n")
    }

    #[test]
    fn test_dividing_integers() {
        assert_eval("1 / 4\n", "0.25\n")
    }

    #[test]
    fn test_parens() {
        assert_eval("(1 + 2)\n", "3\n")
    }

    #[test]
    fn test_parens_after_integer() {
        assert_eval("1 + (2 + 3) + 4\n", "10\n")
    }

    #[test]
    fn test_associative() {
        assert_eval("1.0 / 2\n", "0.5\n");
        assert_eval("1.0 / 2 / 2\n", "0.25\n");
        assert_eval("1.0 / 2 / 2 / 2\n", "0.125\n");
    }

    #[test]
    fn test_addition_with_parens() {
        assert_eval("1 - (2 + 7) + 4\n", "-4\n")
    }

    #[test]
    fn test_math_expr() {
        // assert_eval("1 + 7*(9 - 2) % 5 / 10\n", "1.4\n")
        assert_eval("1 + 7 * (9 - 2) % 5 / 10\n", "1.4\n")
    }

    #[test]
    fn test_multiline() {
        assert_eval("1 + 1\n2 + 2\n3 + 3\n", "2\n4\n6\n")
    }

    #[test]
    fn test_assign_is_void() {
        assert_eval("$a = 1 + 2\n", "")
    }

    #[test]
    fn test_vars() {
        assert_eval("$a = 1 + 1\n$a = $a + 1\n$a\n", "3\n")
    }
}
