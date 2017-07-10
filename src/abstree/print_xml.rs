use xml::{ProteusXmlCreator,XMLable};
use abstree::AbsTree;
use abstree::abs_decl::*;
use abstree::abs_expr::*;
//use abstree::abs_position::AbsPosition;
use abstree::abs_stmt::*;
use abstree::abs_type::*;
use abstree::visitor::Visitor;
use abstree::positioner::Positioner;
use lexanal::position::Position;
use std::io::Write;

pub  struct AbsTreeXmlPrinter 
{
    xml: ProteusXmlCreator
}

impl AbsTreeXmlPrinter
{
    pub fn new(xml_creator : ProteusXmlCreator) -> AbsTreeXmlPrinter
    {
        AbsTreeXmlPrinter{xml : xml_creator}
    }
}

impl Visitor for AbsTreeXmlPrinter 
{
    fn visit_abs_arr_type(&mut self,  acceptor : &AbsArrType)
    {
        writeln!(self.xml, "<absnode node=\"ArrType\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.arr_type.accept(self);
        acceptor.size.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_assign_stmt(&mut self, acceptor : &AbsAssignStmt)
    {
        writeln!(self.xml, "<absnode node=\"AssignStmt\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.left_sub_expr.accept(self);
        acceptor.right_sub_expr.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_atom_expr(&mut self, acceptor : &AbsAtomExpr)
    {
        writeln!(self.xml, "<absnode node=\"AtomExpr\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.expr.as_ref().map(|expr|  expr.to_xml(&mut self.xml));
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_atom_type(&mut self, acceptor : &AbsAtomType)
    {
        let type_name = match acceptor.atom_type
        {
            AtomType::BOOL => "BOOL",
            AtomType::INT => "INT",
            AtomType::REAL => "REAL",
            AtomType::STRING => "STRING",
            AtomType::VOID => "VOID",
        };
        writeln!(self.xml, "<absnode node=\"AtomType\" value=\"{}\">",type_name).unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_bin_expr(&mut self, acceptor : &AbsBinExpr)
    {
        let oper_name = match acceptor.operation 
        {
            AbsBinOper::OR => "OR",
            AbsBinOper::AND => "AND",
            AbsBinOper::EQU => "EQU",
            AbsBinOper::NEQ => "NEQ",
            AbsBinOper::LTH => "LTH",
            AbsBinOper::GTH => "GTH",
            AbsBinOper::LEQ => "LEQ",
            AbsBinOper::GEQ => "GEQ",
            AbsBinOper::ADD => "ADD",
            AbsBinOper::SUB => "SUB",
            AbsBinOper::MUL => "MUL",
            AbsBinOper::DIV => "DIV",
            AbsBinOper::MOD => "MOD",
            AbsBinOper::ARR => "ARR",
            AbsBinOper::REC => "REC",
        };
        writeln!(self.xml, "<absnode node=\"BinExpr\" value=\"{}\">", oper_name).unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.left_sub_expr.accept(self);
        acceptor.right_sub_expr.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_decls(&mut self, acceptor : &AbsDecls)
    {
        writeln!(self.xml, "<absnode node=\"Decls\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        for decl in &acceptor.decls { decl.accept(self); }
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_expr_name(&mut self, acceptor : &AbsExprName)
    {
        writeln!(self.xml, "<absnode node=\"ExprName\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.identifier.to_xml(&mut self.xml);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_exprs(&mut self, acceptor : &AbsExprs)
    {
        writeln!(self.xml, "<absnode node=\"AbsExprs\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        for expr in &acceptor.exprs { expr.accept(self); } 
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_for_stmt(&mut self, acceptor : &AbsForStmt)
    {
        writeln!(self.xml, "<absnode node=\"ForStmt\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.var_name.accept(self);
        acceptor.lower_bound.accept(self);
        acceptor.higher_bound.accept(self);
        acceptor.loop_exprs.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_fun_call(&mut self, acceptor : &AbsFunCall)
    {
        writeln!(self.xml, "<absnode node=\"FunCall\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.name.accept(self);
        acceptor.args.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_fun_decl(&mut self, acceptor : &AbsFunDecl)
    {
        writeln!(self.xml, "<absnode node=\"FunDecl\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.name.accept(self);
        acceptor.params.accept(self);
        acceptor.return_type.accept(self);
        acceptor.exprs.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_if_stmt(&mut self, acceptor : &AbsIfStmt)
    {
        writeln!(self.xml, "<absnode node=\"IfStmt\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.cond_expr.accept(self);
        acceptor.then_expr.accept(self);
        acceptor.else_expr.as_ref().map(|expr| expr.accept(self));
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_pointer_type(&mut self, acceptor : &AbsPointerType)
    {
        writeln!(self.xml, "<absnode node=\"PointerType\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.ptype.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_rec_type(&mut self, acceptor : &AbsRecType)
    {
        writeln!(self.xml, "<absnode node=\"RecType\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.compoments.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_type_decl(&mut self, acceptor : &AbsTypeDecl)
    {
        writeln!(self.xml, "<absnode node=\"TypeDecl\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.type_name.accept(self);
        acceptor.source_type.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_type_name(&mut self, acceptor : &AbsTypeName)
    {
        writeln!(self.xml, "<absnode node=\"TypeName\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.identifier.to_xml(&mut self.xml);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_abs_un_expr(&mut self, acceptor : &AbsUnExpr)
    {
        let unrar_operation = match acceptor.operation {
            AbsUnOper::ADD => "ADD",
            AbsUnOper::SUB => "SUB",
            AbsUnOper::MUL => "MUL",
            AbsUnOper::AND => "AND",
            AbsUnOper::NOT => "NOT",
        };
        writeln!(self.xml, "<absnode node=\"UnExpr\" value=\"{}\">",unrar_operation).unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.sub_expr.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_var_decl(&mut self, acceptor : &AbsVarDecl)
    {
        writeln!(self.xml, "<absnode node=\"VarDecl\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.var_name.accept(self);
        acceptor.var_type.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_where_expr(&mut self, acceptor : &AbsWhereExpr)
    {
        writeln!(self.xml, "<absnode node=\"WhereExpr\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.sub_expr.accept(self);
        acceptor.decls.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
    fn visit_abs_while_stmt(&mut self, acceptor : &AbsWhileStmt)
    {
        writeln!(self.xml, "<absnode node=\"WhileStmt\">").unwrap();
        acceptor.get_position_ref().map(|position| position.to_xml(&mut self.xml));
        acceptor.cond_expr.accept(self);
        acceptor.loop_expr.accept(self);
        writeln!(self.xml, "</absnode>").unwrap();
    }
}
