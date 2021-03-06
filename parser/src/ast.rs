
// virtual node for zlang
use crate::location::SourceLocation;

// 所有的Node枚举
#[derive(Clone, Debug)]
pub enum Kind {
  Program,
  Function,
  EmptyStatement,
  BlockStatement,
  ExpressionStatement,
  IfStatement,
  ReturnStatement,
  WhileStatement,
  ForInStatement,
  ArrayExpression,
  AssignmentExpression,
  LogicalExpression,
  CallExpression,
  BinaryExpression,
  AssignmentOperator,
  Identifier,
  Literal,
  LogicalOperator,
  Property,
  UnaryOperator,
  BinaryOperator
}

#[derive(Clone, Debug)]
pub struct Node {
  kind: Kind,
  loc: SourceLocation
}


#[derive(Clone, Debug)]
pub struct Program {
  kind: Kind,
  body: Vec<Box<Statement>>
}

impl Program {
  pub fn new () -> Self {
    Program {
      kind: Kind::Program,
      body: vec![]
    }
  }
}


#[derive(Clone, Debug)]
pub struct Function {
  kind: Kind,
  id: Option<Identifier>,
  params: Vec<Property>,
  body: BlockStatement
}


#[derive(Clone, Debug)]
pub struct Identifier {
  kind: Kind,
  name: String
}


#[derive(Clone, Debug)]
pub struct Property {
  kind: Kind,
  id: Identifier
}


#[derive(Clone, Debug)]
pub enum Statement {
  EmptyStatement(EmptyStatement),
  BlockStatment(BlockStatement),
  ExpressionStatement(ExpressionStatement),
  IfStatement(IfStatement),
  ReturnStatement(ReturnStatement),
  WhileStatement(WhileStatement),
  ForInStatement(ForInStatement)
}


#[derive(Clone, Debug)]
pub struct EmptyStatement {
  kind: Kind,
}


impl EmptyStatement {
  fn new () -> Self {
    EmptyStatement {
      kind: Kind::EmptyStatement
    }
  }
}

#[derive(Clone, Debug)]
pub struct BlockStatement {
  kind: Kind,
  body: Option<Vec<Box<Statement>>>
}

impl BlockStatement {
  fn new () -> Self {
    BlockStatement {
      kind: Kind::BlockStatement,
      body: None
    }
  }
}

#[derive(Clone, Debug)]
pub struct ExpressionStatement {
  kind: Kind,
  expression: Option<Expression>
}

impl ExpressionStatement {
  fn new () -> Self {
    ExpressionStatement {
      kind: Kind::ExpressionStatement,
      expression: None
    }
  }
}

#[derive(Clone, Debug)]
pub struct IfStatement {
  kind: Kind,
  test: Option<Box<Expression>>,
  consequent: Option<Box<Statement>>,
  alternate: Option<Box<Statement>>
}

impl IfStatement {
  fn new () -> Self {
    IfStatement {
      kind: Kind::IfStatement,
      test: None,
      consequent: None,
      alternate: None
    }
  }
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
  kind: Kind,
  argument: Option<Expression>
}

impl ReturnStatement {
  fn new () -> Self {
    ReturnStatement {
      kind: Kind::ReturnStatement,
      argument: None
    }
  }
}

#[derive(Clone, Debug)]
pub struct WhileStatement {
  kind: Kind,
  test: Option<Expression>,
  body: Option<BlockStatement>
}

impl WhileStatement {
  fn new () -> Self {
    WhileStatement {
      kind: Kind::WhileStatement,
      test: None,
      body: None
    }
  }
}

#[derive(Clone, Debug)]
pub struct ForInStatement {
  kind: Kind,
  left: Option<Expression>,
  right: Option<Expression>,
  body: Option<BlockStatement>,
  each: bool
}

impl ForInStatement {
  fn new () -> Self {
    ForInStatement {
      kind: Kind::ForInStatement,
      left: None,
      right: None,
      body: None,
      each: true
    }
  }
}

#[derive(Clone, Debug)]
pub enum Expression {
  ArrayExpression(Box<ArrayExpression>),
  AssignmentExpression(Box<AssignmentExpression>),
  LogicalExpression(Box<LogicalExpression>),
  CallExpression(Box<CallExpression>),
  BinaryExpression(Box<BinaryExpression>)
}

#[derive(Clone, Debug)]
pub struct ArrayExpression {
  kind: Kind,
  elements: Option<Vec<Expression>>
}

impl ArrayExpression {
  fn new () -> Self {
    ArrayExpression {
      kind: Kind::ArrayExpression,
      elements: None
    }
  }
}


#[derive(Clone, Debug)]
pub struct AssignmentExpression {
  kind: Kind,
  operator: Option<AssignmentOperator>,
  left: Option<Expression>,
  right: Option<Expression>
}

impl AssignmentExpression {
  fn new () -> Self {
    AssignmentExpression {
      kind: Kind::AssignmentExpression,
      operator: None,
      left: None,
      right: None
    }
  }
}


#[derive(Clone, Debug)]
pub struct LogicalExpression {
  kind: Kind,
  operator: Option<LogicalOperator>,
  left: Option<Expression>,
  right: Option<Expression>
}

impl LogicalExpression {
  fn new () -> Self {
    LogicalExpression {
      kind: Kind::LogicalExpression,
      operator: None,
      left: None,
      right: None
    }
  }
}

#[derive(Clone, Debug)]
pub struct CallExpression {
  kind: Kind,
  callee: Option<Box<Expression>>,
  arguments: Option<Vec<Option<Box<Expression>>>>
}

impl CallExpression {
  fn new () -> Self {
    CallExpression {
      kind: Kind::CallExpression,
      callee: None,
      arguments: None
    }
  }
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
  kind: Kind,
  operator: Option<BinaryOperator>,
  left: Option<Expression>,
  right: Option<Expression>
}

impl BinaryExpression {
  fn new () -> Self {
    BinaryExpression {
      kind: Kind::BinaryExpression,
      operator: None,
      left: None,
      right: None
    }
  }
}

#[derive(Clone, Debug)]
pub enum EnumAssignmentOperators {
  Assign,
  PlusAssign,
  MinusAssign,
  TimeAssign,
  DivAssign,
  ModeAssign,
  LShiftAssign,
  RShiftAssign,
  OrAssisn,
  XorAssign,
  AndAssign
}

#[derive(Clone, Debug)]
pub struct AssignmentOperator {
  kind: Kind,
  value: EnumAssignmentOperators
}

impl AssignmentOperator {
  fn new (operator: EnumAssignmentOperators) -> Self {
    AssignmentOperator {
      kind: Kind::AssignmentOperator,
      value: operator
    }
  }
}

#[derive(Clone, Debug)]
enum EnumLogicalOperators {
  LogicalAnd,
  LogicalOr
}

#[derive(Clone, Debug)]
pub struct LogicalOperator {
  kind: Kind,
  value: EnumLogicalOperators
}

impl LogicalOperator {
  fn new (operator: EnumLogicalOperators) -> Self {
    LogicalOperator {
      kind: Kind::LogicalOperator,
      value: operator
    }
  }
}


#[derive(Clone, Debug)]
enum EnumLiteral {
  String(String),
  Boolean(bool),
  Number(f64),
  None
}

#[derive(Clone, Debug)]
pub struct Literal {
  kind: Kind,
  value: EnumLiteral
}

impl Literal {
  fn string (s: String) -> Self {
    Literal {
      kind: Kind::Literal,
      value: EnumLiteral::String(s)
    }
  }

  fn boolean (b: bool) -> Self {
    Literal {
      kind: Kind::Literal,
      value: EnumLiteral::Boolean(b)
    }
  }

  fn number (n: f64) -> Self {
    Literal {
      kind: Kind::Literal,
      value: EnumLiteral::Number(n)
    }
  }

  fn none () -> Self {
    Literal {
      kind: Kind::Literal,
      value: EnumLiteral::None
    }
  }
}

// 一元运算符
#[derive(Clone, Debug)]
enum EnumUnaryOperators {
  Not,
  Xor,
  Typeof
}

#[derive(Clone, Debug)]
pub struct UnaryOperator {
  kind: Kind,
  value: EnumUnaryOperators
}

impl UnaryOperator {
  fn new (operator: EnumUnaryOperators) -> Self {
    UnaryOperator {
      kind: Kind::UnaryOperator,
      value: operator
    }
  }
}

// 二元运算符
#[derive(Clone, Debug)]
enum EnumBinaryOperators {
  Add,
  Minus,
  Time,
  Div,
  Mode,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  Equal,
  NotEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  BitwiseLShift,
  BitwiseRShift
}

#[derive(Clone, Debug)]
struct BinaryOperator {
  kind: Kind,
  value: EnumBinaryOperators
}




