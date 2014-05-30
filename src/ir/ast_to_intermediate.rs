use session::Interner;

use ast::*;
use ir::*;
use collections::TreeSet;

pub struct ASTToIntermediate<'a> {
    var_count: uint,
    label_count: uint,
    interner: &'a mut Interner,
}

impl<'a> ASTToIntermediate<'a> {
    pub fn new(interner: &'a mut Interner) -> ASTToIntermediate<'a> {
        ASTToIntermediate { var_count: 0,
                            label_count: 0,
                            interner: interner }
    }

    fn gen_temp(&mut self) -> Var {
        let res = Var {
            name: self.interner.intern(format!("TEMP{}", self.var_count)),
            generation: None,
        };
        self.var_count += 1;
        res
    }

    fn gen_label(&mut self) -> uint {
        let res = self.label_count;
        self.label_count += 1;
        res
    }

    pub fn convert_stmt(&mut self, stmt: &Stmt) -> (Vec<Op>, Var) {
        match stmt.val {
            ExprStmt(ref e) => self.convert_expr(e),
            SemiStmt(ref e) => self.convert_expr(e),
            _ => fail!(), // TODO: let statements.
        }
    }

    pub fn convert_block(&mut self, block: &Block) -> (Vec<Op>, Var) {
        let mut ops = vec!();
        for stmt in block.stmts.iter() {
            let (new_ops, _) = self.convert_stmt(stmt);
            ops.push_all_move(new_ops);
        }
        match block.expr {
            Some(ref e) => {
                let (new_ops, new_var) = self.convert_expr(e);
                ops.push_all_move(new_ops);
                (ops, new_var)
            },
            None => {
                (ops, self.gen_temp())
            }
        }
    }

    pub fn convert_expr(&mut self, expr: &Expr) -> (Vec<Op>, Var) {
        match expr.val {
            LitExpr(ref lit) => {
                let res_var = self.gen_temp();
                let insts = vec!(
                    Assign(VarLValue(res_var.clone()),
                           DirectRValue(Constant(lit.val.clone())))
                    );
                (insts, res_var)
            }
            BinOpExpr(ref op, ref e1, ref e2) => {
                let (mut insts1, var1) = self.convert_expr(*e1);
                let (insts2, var2) = self.convert_expr(*e2);
                insts1.push_all_move(insts2);
                let new_res = self.gen_temp();
                insts1.push(
                    Assign(VarLValue(new_res.clone()),
                           BinOpRValue(op.val.clone(),
                                       Variable(var1),
                                       Variable(var2))));
                (insts1, new_res)
            },
            PathExpr(ref path) => {
                //fail!("Need to do paths properly")
                (vec!(), Var { name: path.val.elems.last().unwrap().val.name,
                               generation: None })
            },
            AssignExpr(ref e1, ref e2) => {
                let mut res;
                let (insts2, var2) = self.convert_expr(*e2);
                let (lhs, res_v) = match e1.val {
                    PathExpr(ref path) => {
                        //fail!("Need to do paths properly")
                        res = vec!();
                        (VarLValue(Var { name: path.val.elems.last().unwrap().val.name,
                                         generation: None }),
                         Var { name: path.val.elems.last().unwrap().val.name,
                               generation: None })
                    },
                    UnOpExpr(ref op, ref e) => {
                        let (insts, var) = self.convert_expr(*e);
                        res = insts;
                        match op.val {
                            Deref => {
                                (PtrLValue(var.clone()),
                                 var2.clone())
                            },
                            _ => fail!(),
                        }
                    },
                    _ => fail!(),
                };
                res.push_all_move(insts2);
                res.push(Assign(lhs, DirectRValue(Variable(var2))));
                (res, res_v)
            }
            BlockExpr(ref b) => self.convert_block(*b),
            IfExpr(ref e, ref b1, ref b2) => {
                let (mut insts, if_var) = self.convert_expr(*e);
                let (b1_insts, b1_var) = self.convert_block(*b1);
                let (b2_insts, b2_var) = self.convert_block(*b2);
                let b1_label = self.gen_label();
                let end_var = self.gen_temp();
                insts.push(CondGoto(Variable(if_var), b1_label, TreeSet::new()));
                insts.push_all_move(b2_insts);
                insts.push(Assign(VarLValue(end_var),
                                  DirectRValue(Variable(b2_var))));
                insts.push(Label(b1_label, TreeSet::new()));
                insts.push_all_move(b1_insts);
                insts.push(Assign(VarLValue(end_var),
                                  DirectRValue(Variable(b1_var))));
                (insts, end_var)
            },
            WhileExpr(ref e, ref b) => {
                let begin_label = self.gen_label();
                let middle_label = self.gen_label();
                let end_label = self.gen_label();
                let mut res = vec!(Label(begin_label, TreeSet::new()));
                let (cond_insts, cond_var) = self.convert_expr(*e);
                res.push_all_move(cond_insts);
                res.push(CondGoto(Variable(cond_var),
                                  middle_label, TreeSet::new()));
                res.push(Goto(end_label, TreeSet::new()));
                res.push(Label(middle_label, TreeSet::new()));
                let (block_insts, _) = self.convert_block(*b);
                res.push_all_move(block_insts);
                res.push(Goto(begin_label, TreeSet::new()));
                res.push(Label(end_label, TreeSet::new()));
                (res, self.gen_temp())
            },
            //ForExpr(ref init, ref cond, ref iter, ref body) => {
            //    let (mut insts, _) = self.convert_expr(*init);
            //    
            //}
            GroupExpr(ref e) => self.convert_expr(*e),
            _ => (vec!(), self.gen_temp()),
        }
    }
}
