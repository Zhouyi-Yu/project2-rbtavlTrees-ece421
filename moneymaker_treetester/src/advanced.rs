// lang.rs — prefix language parser and executor for tree test harness
//
// GRAMMAR:
//   program  ::= stmt*
//   stmt     ::= 'r' stmt+ 't' expr      -- repeat body N times; max depth 4
//              | cmd
//   cmd      ::= LETTER genspec?         -- 't' is reserved, not a valid cmd
//   genspec  ::= 'rand' expr expr expr   -- rand MIN MAX COUNT
//              | 'rang' expr expr expr?  -- rang MIN MAX [STEP]
//              | expr+                   -- literal list
//   expr     ::= term (('+' | '-') term)*
//   term     ::= atom ('*' atom)*
//   atom     ::= NUMBER | 'n1' | 'n2' | 'n3' | 'n4'
//
// TOKEN RULES:
//   Single letter (not r, t)  → command
//   'r'                        → repeat opener
//   't'                        → repeat terminator (reserved, not a command)
//   'rand' / 'rang'            → generator keywords
//   'n1'..'n4'                 → loop-depth index variables
//   integer                    → number literal / repeat count
//   +  -  *                    → expression operators
//   anything else              → parse error
//
// COMMANDS (same letters as the original driver):
//   i  insert      d  delete     s  search
//   l  count leaves              h  height
//   e  is empty    o  in-order   p  print tree
//   b  back        q  quit
//
// EXAMPLES:
//   i 1 2 3 h o
//   i rang 1 10 h p
//   i rang 1 20 2 s rang 1 20 2
//   r i rand 1 100 5 h t 3
//   r r i rand n1 n1*10 n2 t 4 h t 3

use std::time::{SystemTime, UNIX_EPOCH};

// ── Public interface ──────────────────────────────────────────────────────────

/// Implement this trait on your tree type, then pass it to `run_line`.
pub trait TreeOps {
    fn insert(&mut self, v: u32);
    fn delete(&mut self, v: u32) -> bool;
    fn search(&self, v: u32) -> bool;
    fn count_leaves(&self) -> usize;
    fn height(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn print_inorder(&self);
    fn print_tree(&self);
}

/// Result of executing one input line.
#[derive(Debug, PartialEq)]
pub enum ExecResult {
    Continue, // keep the REPL going
    Back,     // 'b' — return to previous menu
    Quit,     // 'q' — exit the program
}

/// Parse and execute one line of input against `tree`.
/// Returns `Err(msg)` on any parse or runtime error.
pub fn run_line(input: &str, tree: &mut dyn TreeOps) -> Result<ExecResult, String> {
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Ok(ExecResult::Continue);
    }
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse_program()?;
    let mut exec = Executor::new(tree);
    exec.run_stmts(&stmts)
}

pub fn print_lang_help() {
    println!(
        "
┌─── Lang Parser Reference ────────────────────────────────────────────────────┐
│  COMMANDS                                                                     │
│    i  Insert      d  Delete      s  Search                                   │
│    l  Leaves      h  Height      e  Empty?                                   │
│    o  In-order    p  Print       b  Back    q  Quit                          │
│                                                                               │
│  VALUE SPECS  (follow i / d / s)                                             │
│    i 1 2 3              literals — insert 1, then 2, then 3                  │
│    i rang 1 10          range 1..=10, step defaults to 1                     │
│    i rang 1 10 2        range 1..=10, step 2  →  1 3 5 7 9                  │
│    i rand 1 100 5       5 pseudorandom values in [1, 100]                    │
│                                                                               │
│  EXPRESSIONS  (inside rang / rand args)                                      │
│    + - *                e.g.  rand n1 n1*10 3                                │
│    n1 n2 n3 n4          current loop index, outermost → innermost            │
│                                                                               │
│  REPEAT                                                                       │
│    r <stmts> t <n>      repeat the body n times  (max 4 levels deep)        │
│    r i rand 1 50 5 t 3  insert 5 random values, repeated 3 times            │
│                                                                               │
│  CHAINING  (no separator needed)                                              │
│    i rang 1 20 h p      insert 1..20, print height, print tree               │
│    r i rand n1 n1*10 3 t 4 h p                                               │
│      → 4 iters: insert 3 randoms in [n1, n1*10], then height, print         │
│                                                                               │
│  ERRORS                                                                       │
│    Stray number         forgot to close an 'r' block with 't'                │
│    nX out of depth      used n3 while only 2 loops deep                      │
│    Nesting > 4          parse error — simplify your repeat blocks            │
│                                                                               │
│  ?                      print this guide                                      │
└───────────────────────────────────────────────────────────────────────────────┘"
    );
}

// ── Tokens ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Cmd(char),  // single command letter (a–z, not r or t)
    R,          // 'r' — repeat opener
    T,          // 't' — repeat terminator (reserved)
    Rand,       // 'rand'
    Rang,       // 'rang'
    Idx(usize), // n1 .. n4
    Num(i64),   // integer literal
    Plus,
    Minus,
    Star,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '0'..='9' => {
                let mut s = String::new();
                while chars.peek().map_or(false, |d| d.is_ascii_digit()) {
                    s.push(chars.next().unwrap());
                }
                let n: i64 = s.parse().map_err(|_| format!("Integer overflow: {}", s))?;
                tokens.push(Token::Num(n));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut s = String::new();
                while chars.peek().map_or(false, |a| a.is_ascii_alphanumeric()) {
                    s.push(chars.next().unwrap());
                }
                let tok = match s.as_str() {
                    "rand" => Token::Rand,
                    "rang" => Token::Rang,
                    "n1" => Token::Idx(1),
                    "n2" => Token::Idx(2),
                    "n3" => Token::Idx(3),
                    "n4" => Token::Idx(4),
                    "r" => Token::R,
                    "t" => Token::T,
                    other if other.len() == 1 => Token::Cmd(other.chars().next().unwrap()),
                    other => return Err(format!("Unknown keyword '{}'", other)),
                };
                tokens.push(tok);
            }
            _ => return Err(format!("Unexpected character '{}'", c)),
        }
    }
    Ok(tokens)
}

// ── AST ───────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Expr {
    Num(i64),
    Idx(usize), // n1..n4 — resolved at runtime to current loop counter
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
enum GenSpec {
    None,
    Rand {
        min: Expr,
        max: Expr,
        count: Expr,
    },
    Rang {
        min: Expr,
        max: Expr,
        step: Option<Expr>,
    },
    Literals(Vec<Expr>),
}

#[derive(Debug, Clone)]
enum Stmt {
    Repeat { body: Vec<Stmt>, times: Expr },
    Cmd { op: char, spec: GenSpec },
}

// ── Parser ────────────────────────────────────────────────────────────────────

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    depth: usize, // current nesting depth; max 4
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            depth: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek().cloned() {
            // ── repeat block: r <body...> t <count> ──────────────────────────
            Some(Token::R) => {
                self.advance();
                self.depth += 1;
                if self.depth > 4 {
                    return Err(format!(
                        "Maximum nesting depth 4 exceeded (depth {}). \
                         Simplify your repeat blocks.",
                        self.depth
                    ));
                }
                let mut body = Vec::new();
                loop {
                    match self.peek() {
                        Some(Token::T) => {
                            self.advance(); // consume 't'
                            break;
                        }
                        None => return Err("Unclosed 'r' block — expected 't <count>'".to_string()),
                        _ => body.push(self.parse_stmt()?),
                    }
                }
                if body.is_empty() {
                    return Err("Empty 'r' block — no statements between 'r' and 't'".to_string());
                }
                let times = self.parse_expr()?;
                self.depth -= 1;
                Ok(Stmt::Repeat { body, times })
            }

            // ── command ──────────────────────────────────────────────────────
            Some(Token::Cmd(op)) => {
                self.advance();
                self.parse_cmd(op)
            }

            // ── error cases ───────────────────────────────────────────────────
            Some(Token::T) => Err("Unexpected 't' — no matching 'r' to close".to_string()),
            Some(Token::Num(n)) => Err(format!("Stray number {} — did you forget 'r ... t'?", n)),
            Some(t) => Err(format!("Expected a command or 'r', got {:?}", t)),
            None => Err("Expected a statement, got end of input".to_string()),
        }
    }

    fn parse_cmd(&mut self, op: char) -> Result<Stmt, String> {
        // zero-argument commands — consume no further tokens
        if matches!(op, 'l' | 'h' | 'e' | 'o' | 'p' | 'b' | 'q') {
            return Ok(Stmt::Cmd {
                op,
                spec: GenSpec::None,
            });
        }

        // value-taking commands: i, d, s
        let spec = match self.peek() {
            Some(Token::Rand) => {
                self.advance();
                let min = self.parse_expr()?;
                let max = self.parse_expr()?;
                let count = self.parse_expr()?;
                GenSpec::Rand { min, max, count }
            }
            Some(Token::Rang) => {
                self.advance();
                let min = self.parse_expr()?;
                let max = self.parse_expr()?;
                // step is optional: consume only if the next token starts an expr
                let step = if self.peek_starts_expr() {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                GenSpec::Rang { min, max, step }
            }
            _ => {
                // literal list: consume as many exprs as are available
                let mut exprs = Vec::new();
                while self.peek_starts_expr() {
                    exprs.push(self.parse_expr()?);
                }
                if exprs.is_empty() {
                    GenSpec::None
                } else {
                    GenSpec::Literals(exprs)
                }
            }
        };
        Ok(Stmt::Cmd { op, spec })
    }

    /// An expression can only START with a number literal or an nX index.
    fn peek_starts_expr(&self) -> bool {
        matches!(self.peek(), Some(Token::Num(_)) | Some(Token::Idx(_)))
    }

    // ── expression parser (precedence: + - < * ) ─────────────────────────────

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_term()?;
        while matches!(self.peek(), Some(Token::Plus) | Some(Token::Minus)) {
            let op = self.advance().unwrap();
            let rhs = self.parse_term()?;
            lhs = match op {
                Token::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
                Token::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            };
        }
        Ok(lhs)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_atom()?;
        while matches!(self.peek(), Some(Token::Star)) {
            self.advance();
            let rhs = self.parse_atom()?;
            lhs = Expr::Mul(Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Token::Num(n)) => Ok(Expr::Num(n)),
            Some(Token::Idx(d)) => Ok(Expr::Idx(d)),
            Some(t) => Err(format!(
                "Expected a number or loop index (n1..n4), got {:?}",
                t
            )),
            None => Err("Expected a number or loop index, got end of input".to_string()),
        }
    }
}

// ── Executor ──────────────────────────────────────────────────────────────────

struct Executor<'a> {
    tree: &'a mut dyn TreeOps,
    /// loop_indices[0] = n1 (outermost), loop_indices[3] = n4 (innermost)
    loop_indices: [i64; 4],
    /// current active nesting depth (0 = not inside any repeat)
    depth: usize,
}

impl<'a> Executor<'a> {
    fn new(tree: &'a mut dyn TreeOps) -> Self {
        Executor {
            tree,
            loop_indices: [0; 4],
            depth: 0,
        }
    }

    fn eval(&self, expr: &Expr) -> Result<i64, String> {
        match expr {
            Expr::Num(n) => Ok(*n),
            Expr::Idx(d) => {
                if *d > self.depth {
                    Err(format!(
                        "n{} used outside sufficient nesting (current depth: {})",
                        d, self.depth
                    ))
                } else {
                    Ok(self.loop_indices[d - 1])
                }
            }
            Expr::Add(a, b) => Ok(self.eval(a)? + self.eval(b)?),
            Expr::Sub(a, b) => Ok(self.eval(a)? - self.eval(b)?),
            Expr::Mul(a, b) => Ok(self.eval(a)? * self.eval(b)?),
        }
    }

    fn resolve(&self, spec: &GenSpec) -> Result<Vec<u32>, String> {
        match spec {
            GenSpec::None => Ok(vec![]),

            GenSpec::Literals(exprs) => exprs
                .iter()
                .map(|e| {
                    let v = self.eval(e)?;
                    to_u32(v)
                })
                .collect(),

            GenSpec::Rand { min, max, count } => {
                let min = to_u32(self.eval(min)?)?;
                let max = to_u32(self.eval(max)?)?;
                let count = self.eval(count)? as usize;
                if min > max {
                    return Err(format!("rand: min {} > max {}", min, max));
                }
                Ok(lcg_rand(min, max, count))
            }

            GenSpec::Rang { min, max, step } => {
                let min = self.eval(min)?;
                let max = self.eval(max)?;
                let step = step
                    .as_ref()
                    .map(|s| self.eval(s))
                    .transpose()?
                    .unwrap_or(1);
                if step == 0 {
                    return Err("rang: step cannot be zero".to_string());
                }
                let mut vals = Vec::new();
                let mut v = min;
                if step > 0 {
                    while v <= max {
                        vals.push(to_u32(v)?);
                        v += step;
                    }
                } else {
                    while v >= max {
                        vals.push(to_u32(v)?);
                        v += step;
                    }
                }
                Ok(vals)
            }
        }
    }

    fn run_stmts(&mut self, stmts: &[Stmt]) -> Result<ExecResult, String> {
        for stmt in stmts {
            match self.run_stmt(stmt)? {
                ExecResult::Continue => {}
                other => return Ok(other), // propagate Back / Quit immediately
            }
        }
        Ok(ExecResult::Continue)
    }

    fn run_stmt(&mut self, stmt: &Stmt) -> Result<ExecResult, String> {
        match stmt {
            // ── repeat ───────────────────────────────────────────────────────
            Stmt::Repeat { body, times } => {
                let n = self.eval(times)?;
                if n <= 0 {
                    println!("  [repeat x{} — skipped]", n);
                    return Ok(ExecResult::Continue);
                }
                self.depth += 1;
                let d = self.depth;
                println!("  ┌─ repeat x{} (n{} = 1..{}) ─────────", n, d, n);
                for i in 1..=n {
                    self.loop_indices[d - 1] = i;
                    println!("  │ iter {}/{}", i, n);
                    match self.run_stmts(body)? {
                        ExecResult::Continue => {}
                        other => {
                            self.loop_indices[d - 1] = 0;
                            self.depth -= 1;
                            println!("  └─ repeat interrupted ───────────────");
                            return Ok(other);
                        }
                    }
                }
                self.loop_indices[d - 1] = 0;
                self.depth -= 1;
                println!("  └─ repeat done ──────────────────────");
            }

            // ── command ───────────────────────────────────────────────────────
            Stmt::Cmd { op, spec } => {
                return self.run_cmd(*op, spec);
            }
        }
        Ok(ExecResult::Continue)
    }

    fn run_cmd(&mut self, op: char, spec: &GenSpec) -> Result<ExecResult, String> {
        match op {
            'i' => {
                let vals = self.resolve(spec)?;
                println!("  [insert {} value(s): {:?}]", vals.len(), vals);
                for v in vals {
                    self.tree.insert(v);
                    println!("    ✓ inserted {}", v);
                }
            }
            'd' => {
                let vals = self.resolve(spec)?;
                println!("  [delete {} value(s): {:?}]", vals.len(), vals);
                for v in vals {
                    if self.tree.delete(v) {
                        println!("    ✓ deleted {}", v);
                    } else {
                        println!("    ✗ {} not found", v);
                    }
                }
            }
            's' => {
                let vals = self.resolve(spec)?;
                println!("  [search {} value(s)]", vals.len());
                for v in vals {
                    println!(
                        "    {} — {}",
                        v,
                        if self.tree.search(v) {
                            "FOUND"
                        } else {
                            "NOT found"
                        }
                    );
                }
            }
            'l' => println!("  [leaves]  {}", self.tree.count_leaves()),
            'h' => println!("  [height]  {}", self.tree.height()),
            'e' => println!("  [empty]   {}", self.tree.is_empty()),
            'o' => {
                println!("  [inorder]");
                self.tree.print_inorder();
            }
            'p' => {
                println!("  [print]");
                self.tree.print_tree();
            }
            'b' => {
                println!("  [back]");
                return Ok(ExecResult::Back);
            }
            'q' => {
                println!("  [quit]");
                return Ok(ExecResult::Quit);
            }
            _ => return Err(format!("Unknown command '{}'", op)),
        }
        Ok(ExecResult::Continue)
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn to_u32(v: i64) -> Result<u32, String> {
    if v < 0 || v > u32::MAX as i64 {
        Err(format!("Value {} is out of u32 range", v))
    } else {
        Ok(v as u32)
    }
}

fn lcg_rand(min: u32, max: u32, n: usize) -> Vec<u32> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(42);
    let range = (max - min + 1) as u64;
    let mut state = seed;
    (0..n)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            min + ((state >> 33) % range) as u32
        })
        .collect()
}
