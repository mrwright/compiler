use ast::*;

pub trait Visitor {
    fn visit_item(&self, item: &Item) { walk_item(self, item) }
    fn visit_type(&self, t: &Type) { walk_type(self, t) }
    fn visit_func_arg(&self, arg: &FuncArg) { walk_func_arg(self, arg) }
    fn visit_block(&self, block: &Block) { walk_block(self, block) }
    fn visit_stmt(&self, stmt: &Stmt) { walk_stmt(self, stmt) }
    fn visit_expr(&self, expr: &Expr) { walk_expr(self, expr) }
    fn visit_lit(&self, lit: &Lit) { walk_lit(self, lit) }
    fn visit_ident(&self, ident: &Ident) { walk_ident(self, ident) }
}

pub fn walk_item<T: Visitor>(visitor: &T, item: &Item) {
    match item.val {
        FuncItem(ref id, ref args, ref t, ref def, ref tps) => {
            visitor.visit_ident(id);
            args.iter().map(|arg| visitor.visit_func_arg(arg));
            visitor.visit_type(t);
            visitor.visit_block(def);
            tps.iter().map(|id| visitor.visit_ident(id));
        }
    }
}

pub fn walk_type<T: Visitor>(visitor: &T, t: &Type) {
    match t.val {
        PtrType(ref p) => {
            visitor.visit_type(*p);
        }
        NamedType(ref id) => {
            visitor.visit_ident(id);
        }
        FuncType(ref d, ref r) => {
            visitor.visit_type(*d);
            visitor.visit_type(*r);
        }
        ArrayType(ref a, _) => {
            visitor.visit_type(*a);
        }
        TupleType(ref ts) => {
            ts.iter().map(|t| visitor.visit_type(t));
        }
        BoolType | UnitType | IntType(..) => {}
    }
}

pub fn walk_func_arg<T: Visitor>(visitor: &T, arg: &FuncArg) {
    visitor.visit_ident(&arg.ident);
    visitor.visit_type(&arg.argtype);
}

pub fn walk_block<T: Visitor>(visitor: &T, block: &Block) {
    block.items.iter().map(|item| visitor.visit_item(item));
    block.stmts.iter().map(|stmt| visitor.visit_stmt(stmt));
    block.expr.iter().map(|expr| visitor.visit_expr(expr));
}

pub fn walk_stmt<T: Visitor>(visitor: &T, stmt: &Stmt) {
    match stmt.val {
        LetStmt(ref id, ref t, ref e) => {
            visitor.visit_ident(id);
            t.iter().map(|t| visitor.visit_type(t));
            e.iter().map(|e| visitor.visit_expr(e));
        }
        ExprStmt(ref e) => {
            visitor.visit_expr(e);
        }
        SemiStmt(ref e) => {
            visitor.visit_expr(e);
        }
    }
}

pub fn walk_expr<T: Visitor>(visitor: &T, expr: &Expr) {
    match expr.val {
        LitExpr(ref l) => {
            visitor.visit_lit(l);
        }
        TupleExpr(ref es) => {
            es.iter().map(|e| visitor.visit_expr(e));
        }
        IdentExpr(ref id) => {
            visitor.visit_ident(id);
        }
        BinOpExpr(_, ref l, ref r) => {
            visitor.visit_expr(*l);
            visitor.visit_expr(*r);
        }
        UnOpExpr(_, ref e) => {
            visitor.visit_expr(*e);
        }
        IndexExpr(ref a, ref i) => {
            visitor.visit_expr(*a);
            visitor.visit_expr(*i);
        }
        DotExpr(ref e, ref id) => {
            visitor.visit_expr(*e);
            visitor.visit_ident(id);
        }
        ArrowExpr(ref e, ref id) => {
            visitor.visit_expr(*e);
            visitor.visit_ident(id);
        }
        AssignExpr(ref lv, ref rv) => {
            visitor.visit_expr(*lv);
            visitor.visit_expr(*rv);
        }
        CallExpr(ref f, ref args) => {
            visitor.visit_expr(*f);
            args.iter().map(|arg| visitor.visit_expr(arg));
        }
        CastExpr(ref e, ref t) => {
            visitor.visit_expr(*e);
            visitor.visit_type(t);
        }
        IfExpr(ref c, ref tb, ref fb) => {
            visitor.visit_expr(*c);
            visitor.visit_block(*tb);
            visitor.visit_block(*fb);
        }
        BlockExpr(ref b) => {
            visitor.visit_block(*b);
        }
        ReturnExpr(ref e) => {
            visitor.visit_expr(*e);
        }
        UnitExpr => {}
    }
}

pub fn walk_lit<T: Visitor>(_: &T, lit: &Lit) {
    match lit.val {
        NumLit(..) | StringLit(..) | BoolLit(..) => {}
    }
}

pub fn walk_ident<T: Visitor>(visitor: &T, ident: &Ident) {
    for tps in ident.tps.iter() {
        tps.iter().map(|tp| visitor.visit_type(tp));
    }
}