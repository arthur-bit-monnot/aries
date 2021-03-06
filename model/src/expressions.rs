use crate::lang::{BExpr, Expr};
use aries_collections::ref_store::RefVec;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Expressions {
    interned: HashMap<Expr, ExprHandle>,
    expressions: RefVec<ExprHandle, Expr>,
}
#[derive(Eq, PartialEq)]
pub enum NExpr<'a> {
    Pos(&'a Expr),
    Neg(&'a Expr),
}

// Identifier of an expression which can be retrieved with [Expressions::get]
aries_collections::create_ref_type!(ExprHandle);

impl Expressions {
    pub fn is_interned(&self, expr: &Expr) -> bool {
        self.interned.contains_key(expr)
    }

    pub fn handle_of(&self, expr: &Expr) -> Option<ExprHandle> {
        self.interned.get(expr).copied()
    }

    pub fn get(&self, expr_id: ExprHandle) -> &Expr {
        &self.expressions[expr_id]
    }

    /// Interns the given expression and returns the corresponding handle.
    /// If the expression was already interned, the handle to the previously inserted
    /// instance will be returned.
    pub fn intern(&mut self, expr: Expr) -> ExprHandle {
        if let Some(handle) = self.interned.get(&expr) {
            *handle
        } else {
            let handle = self.expressions.push(expr.clone());
            self.interned.insert(expr, handle);
            handle
        }
    }

    pub fn expr_of(&self, atom: impl Into<BExpr>) -> NExpr {
        let atom = atom.into();
        let e = self.get(atom.expr);
        if atom.negated {
            NExpr::Neg(e)
        } else {
            NExpr::Pos(e)
        }
    }
}
