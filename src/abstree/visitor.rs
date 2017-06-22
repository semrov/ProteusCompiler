use abstree::abs_decl::{AbsDecls,AbsFunDecl,AbsTypeDecl,AbsVarDecl};
use abstree::abs_expr::{AbsAtomExpr,AbsBinExpr,AbsExprName,AbsExprs,AbsFunCall,AbsUnExpr,AbsWhereExpr};
use abstree::abs_stmt::{AbsAssignStmt,AbsForStmt,AbsIfStmt,AbsWhileStmt};
use abstree::abs_type::{AbsArrType,AbsAtomType,AbsPointerType,AbsRecType,AbsTypeName};

pub trait Visitor
{
    fn visit_abs_arr_type(&mut self,  acceptor : &AbsArrType);
    fn visit_abs_assign_stmt(&mut self, acceptor : &AbsAssignStmt);
    fn visit_abs_atom_expr(&mut self, acceptor : &AbsAtomExpr);
    fn visit_abs_atom_type(&mut self, acceptor : &AbsAtomType);
    fn visit_abs_bin_expr(&mut self, acceptor : &AbsBinExpr);
    fn visit_abs_decls(&mut self, acceptor : &AbsDecls);
    fn visit_abs_expr_name(&mut self, acceptor : &AbsExprName);
    fn visit_abs_exprs(&mut self, acceptor : &AbsExprs);
    fn visit_abs_for_stmt(&mut self, acceptor : &AbsForStmt);
    fn visit_abs_fun_call(&mut self, acceptor : &AbsFunCall);
    fn visit_abs_fun_decl(&mut self, acceptor : &AbsFunDecl);
    fn visit_abs_if_stmt(&mut self, acceptor : &AbsIfStmt);
    fn visit_abs_pointer_type(&mut self, acceptor : &AbsPointerType);
    fn visit_abs_rec_type(&mut self, acceptor : &AbsRecType);
    fn visit_abs_type_decl(&mut self, acceptor : &AbsTypeDecl);
    fn visit_abs_type_name(&mut self, acceptor : &AbsTypeName);
    fn visit_abs_abs_un_expr(&mut self, acceptor : &AbsUnExpr);
    fn visit_abs_var_decl(&mut self, acceptor : &AbsVarDecl);
    fn visit_abs_where_expr(&mut self, acceptor : &AbsWhereExpr);
    fn visit_abs_while_stmt(&mut self, acceptor : &AbsWhileStmt);
}