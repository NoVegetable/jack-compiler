//! Utilities for writing XML output

use crate::ast;
use std::io::Write;
use xml::writer::{EmitterConfig, Error as XmlError, EventWriter, XmlEvent};

pub type WriteResult = Result<(), XmlError>;

pub fn init_writer<W: Write>(writer: W) -> EventWriter<W> {
    EmitterConfig::new()
        .write_document_declaration(false)
        .perform_indent(true)
        .normalize_empty_elements(false)
        .create_writer(writer)
}

fn write_start<'a, W: Write>(writer: &mut EventWriter<W>, element: &'a str) -> WriteResult {
    writer.write(XmlEvent::start_element(element))
}

fn write_end<'a, W: Write>(writer: &mut EventWriter<W>) -> WriteResult {
    writer.write(XmlEvent::end_element())
}

fn write_element<'a, W: Write>(
    writer: &mut EventWriter<W>,
    element: &'a str,
    chars: &'a str,
) -> WriteResult {
    writer.write(XmlEvent::start_element(element))?;
    writer.write(XmlEvent::characters(chars))?;
    writer.write(XmlEvent::end_element())
}

pub trait XmlWrite {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult;
}

impl XmlWrite for String {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        let chars = format!(" {} ", self);
        write_element(writer, "identifier", &chars)
    }
}

impl XmlWrite for ast::Class {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "class")?;
        write_element(writer, "keyword", " class ")?;
        self.name.write_xml(writer)?;
        write_element(writer, "symbol", " { ")?;
        for variable in &self.variables {
            variable.write_xml(writer)?;
        }
        for subroutine in &self.subroutines {
            subroutine.write_xml(writer)?;
        }
        write_element(writer, "symbol", " } ")?;
        write_end(writer)
    }
}

impl XmlWrite for ast::ClassVarDec {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "classVarDec")?;
        self.kind.write_xml(writer)?;
        self.ty.write_xml(writer)?;
        self.names[0].write_xml(writer)?;
        for name in &self.names[1..] {
            write_element(writer, "symbol", " , ")?;
            name.write_xml(writer)?;
        }
        write_element(writer, "symbol", " ; ")?;
        write_end(writer)
    }
}

impl XmlWrite for ast::VarKind {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        use ast::VarKind::*;
        match self {
            Static => write_element(writer, "keyword", " static "),
            Field => write_element(writer, "keyword", " field "),
        }
    }
}

impl XmlWrite for ast::Ty {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        use ast::Ty::*;
        match self {
            Int => write_element(writer, "keyword", " int "),
            Char => write_element(writer, "keyword", " char "),
            Boolean => write_element(writer, "keyword", " boolean "),
            Class(class_name) => class_name.write_xml(writer),
        }
    }
}

impl XmlWrite for ast::SubroutineDec {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "subroutineDec")?;
        self.kind.write_xml(writer)?;
        self.return_ty.write_xml(writer)?;
        self.name.write_xml(writer)?;
        write_element(writer, "symbol", " ( ")?;
        self.params.write_xml(writer)?;
        write_element(writer, "symbol", " ) ")?;
        self.body.write_xml(writer)?;
        write_end(writer)
    }
}

impl XmlWrite for ast::SubroutineKind {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        use ast::SubroutineKind::*;
        match self {
            Constructor => write_element(writer, "keyword", " constructor "),
            Function => write_element(writer, "keyword", " function "),
            Method => write_element(writer, "keyword", " method "),
        }
    }
}

impl XmlWrite for ast::SubroutineReturnTy {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        use ast::SubroutineReturnTy::*;
        match self {
            Void => write_element(writer, "keyword", " void "),
            Type(ty) => ty.write_xml(writer),
        }
    }
}

impl XmlWrite for ast::ParameterList {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "parameterList")?;
        if !self.0.is_empty() {
            self.0[0].write_xml(writer)?;
            for param in &self.0[1..] {
                write_element(writer, "symbol", " , ")?;
                param.write_xml(writer)?;
            }
        }
        write_end(writer)
    }
}

impl XmlWrite for ast::Parameter {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        self.ty.write_xml(writer)?;
        self.name.write_xml(writer)
    }
}

impl XmlWrite for ast::SubroutineBody {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "subroutineBody")?;
        write_element(writer, "symbol", " { ")?;
        for var_dec in &self.variables {
            var_dec.write_xml(writer)?;
        }
        self.stmts.write_xml(writer)?;
        write_element(writer, "symbol", " } ")?;
        write_end(writer)
    }
}

impl XmlWrite for ast::VarDec {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "varDec")?;
        write_element(writer, "keyword", " var ")?;
        self.ty.write_xml(writer)?;
        self.names[0].write_xml(writer)?;
        for name in &self.names[1..] {
            write_element(writer, "symbol", " , ")?;
            name.write_xml(writer)?;
        }
        write_element(writer, "symbol", " ; ")?;
        write_end(writer)
    }
}

impl XmlWrite for ast::Stmts {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "statements")?;
        for stmt in &self.0 {
            stmt.write_xml(writer)?;
        }
        write_end(writer)
    }
}

impl XmlWrite for ast::Stmt {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        use ast::Stmt::*;
        match self {
            Let(let_stmt) => let_stmt.write_xml(writer),
            If(if_stmt) => if_stmt.write_xml(writer),
            While(while_stmt) => while_stmt.write_xml(writer),
            Do(do_stmt) => do_stmt.write_xml(writer),
            Return(return_stmt) => return_stmt.write_xml(writer),
        }
    }
}

impl XmlWrite for ast::LetStmt {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "letStatement")?;
        write_element(writer, "keyword", " let ")?;
        self.var_name.write_xml(writer)?;
        if let Some(ref idx) = self.idx_expr {
            write_element(writer, "symbol", " [ ")?;
            idx.write_xml(writer)?;
            write_element(writer, "symbol", " ] ")?;
        }
        write_element(writer, "symbol", " = ")?;
        self.assign_expr.write_xml(writer)?;
        write_element(writer, "symbol", " ; ")?;
        write_end(writer)
    }
}

impl XmlWrite for ast::IfStmt {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "ifStatement")?;
        write_element(writer, "keyword", " if ")?;
        write_element(writer, "symbol", " ( ")?;
        self.condition.write_xml(writer)?;
        write_element(writer, "symbol", " ) ")?;
        write_element(writer, "symbol", " { ")?;
        self.stmts.write_xml(writer)?;
        write_element(writer, "symbol", " } ")?;
        if let Some(ref stmts) = self.else_stmts {
            write_element(writer, "keyword", " else ")?;
            write_element(writer, "symbol", " { ")?;
            stmts.write_xml(writer)?;
            write_element(writer, "symbol", " } ")?;
        }
        write_end(writer)
    }
}

impl XmlWrite for ast::WhileStmt {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "whileStatement")?;
        write_element(writer, "keyword", " while ")?;
        write_element(writer, "symbol", " ( ")?;
        self.condition.write_xml(writer)?;
        write_element(writer, "symbol", " ) ")?;
        write_element(writer, "symbol", " { ")?;
        self.stmts.write_xml(writer)?;
        write_element(writer, "symbol", " } ")?;
        write_end(writer)
    }
}

impl XmlWrite for ast::DoStmt {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "doStatement")?;
        write_element(writer, "keyword", " do ")?;
        self.call.write_xml(writer)?;
        write_element(writer, "symbol", " ; ")?;
        write_end(writer)
    }
}

impl XmlWrite for ast::ReturnStmt {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "returnStatement")?;
        write_element(writer, "keyword", " return ")?;
        if let Some(ref expr) = self.return_val {
            expr.write_xml(writer)?;
        }
        write_element(writer, "symbol", " ; ")?;
        write_end(writer)
    }
}

impl XmlWrite for ast::Expression {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "expression")?;
        self.leading_term.write_xml(writer)?;
        for (op, term) in &self.following_terms {
            op.write_xml(writer)?;
            term.write_xml(writer)?;
        }
        write_end(writer)
    }
}

impl XmlWrite for ast::Term {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        use ast::Term::*;
        write_start(writer, "term")?;
        match self {
            IntegerConst(n) => {
                write_element(writer, "integerConstant", &format!(" {} ", n.to_string()))?;
            }
            StringConst(s) => write_element(
                writer,
                "stringConstant",
                &format!(" {} ", &s[1..s.len() - 1]),
            )?,
            KeywordConst(kw) => kw.write_xml(writer)?,
            VarRef(var_name) => var_name.write_xml(writer)?,
            VarRefWithIdx(var_name, expr) => {
                var_name.write_xml(writer)?;
                write_element(writer, "symbol", " [ ")?;
                expr.write_xml(writer)?;
                write_element(writer, "symbol", " ] ")?;
            }
            SubroutineCall(call) => call.write_xml(writer)?,
            Expr(expr) => {
                write_element(writer, "symbol", " ( ")?;
                expr.write_xml(writer)?;
                write_element(writer, "symbol", " ) ")?;
            }
            UnaryOperation(op, term) => {
                op.write_xml(writer)?;
                term.write_xml(writer)?;
            }
        }
        write_end(writer)
    }
}

impl XmlWrite for ast::SubroutineCall {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        if let Some(ref p) = self.prefix {
            p.write_xml(writer)?;
            write_element(writer, "symbol", " . ")?;
        }
        self.name.write_xml(writer)?;
        write_element(writer, "symbol", " ( ")?;
        self.args.write_xml(writer)?;
        write_element(writer, "symbol", " ) ")
    }
}

impl XmlWrite for ast::ExpressionList {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        write_start(writer, "expressionList")?;
        if !self.0.is_empty() {
            self.0[0].write_xml(writer)?;
            for expr in &self.0[1..] {
                write_element(writer, "symbol", " , ")?;
                expr.write_xml(writer)?;
            }
        }
        write_end(writer)
    }
}

impl XmlWrite for ast::Op {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        use ast::Op::*;
        let chars = match *self {
            Add => " + ",
            Sub => " - ",
            Mul => " * ",
            Div => " / ",
            And => " & ",
            Or => " | ",
            Lt => " < ",
            Gt => " > ",
            Eq => " = ",
        };
        write_element(writer, "symbol", chars)
    }
}

impl XmlWrite for ast::UnaryOp {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        use ast::UnaryOp::*;
        let chars = match *self {
            Negative => " - ",
            Neg => " ~ ",
        };
        write_element(writer, "symbol", chars)
    }
}

impl XmlWrite for ast::KeywordConst {
    fn write_xml<W: Write>(&self, writer: &mut EventWriter<W>) -> WriteResult {
        use ast::KeywordConst::*;
        let chars = match *self {
            True => " true ",
            False => " false ",
            Null => " null ",
            This => " this ",
        };
        write_element(writer, "keyword", chars)
    }
}
