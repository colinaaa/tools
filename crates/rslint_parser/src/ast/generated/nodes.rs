//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::redundant_clone)]
use crate::{
	ast::*,
	SyntaxKind::{self, *},
	SyntaxNode, SyntaxResult, SyntaxToken, T,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownStatement {
	pub fn syntax_element(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownExpression {
	pub fn syntax_element(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownPattern {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownPattern {
	pub fn syntax_element(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownMember {
	pub fn syntax_element(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownBinding {
	pub fn syntax_element(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownAssignmentTarget {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownAssignmentTarget {
	pub fn syntax_element(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
	pub(crate) syntax: SyntaxNode,
}
impl Ident {
	pub fn ident_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![ident])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Script {
	pub(crate) syntax: SyntaxNode,
}
impl Script {
	pub fn shebang_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![shebang])
	}
	pub fn items(&self) -> AstNodeList<Stmt> { support::node_list(&self.syntax, 0usize) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
	pub(crate) syntax: SyntaxNode,
}
impl Module {
	pub fn shebang_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![shebang])
	}
	pub fn items(&self) -> AstNodeList<ModuleItem> { support::node_list(&self.syntax, 0usize) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockStmt {
	pub(crate) syntax: SyntaxNode,
}
impl BlockStmt {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn stmts(&self) -> AstNodeList<Stmt> { support::node_list(&self.syntax, 0usize) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmptyStmt {
	pub(crate) syntax: SyntaxNode,
}
impl EmptyStmt {
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ExprStmt {
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfStmt {
	pub(crate) syntax: SyntaxNode,
}
impl IfStmt {
	pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![if])
	}
	pub fn condition(&self) -> SyntaxResult<Condition> { support::as_mandatory_node(&self.syntax) }
	pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![else])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoWhileStmt {
	pub(crate) syntax: SyntaxNode,
}
impl DoWhileStmt {
	pub fn do_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![do])
	}
	pub fn cons(&self) -> SyntaxResult<Stmt> { support::as_mandatory_node(&self.syntax) }
	pub fn while_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![while])
	}
	pub fn condition(&self) -> SyntaxResult<Condition> { support::as_mandatory_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileStmt {
	pub(crate) syntax: SyntaxNode,
}
impl WhileStmt {
	pub fn while_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![while])
	}
	pub fn condition(&self) -> SyntaxResult<Condition> { support::as_mandatory_node(&self.syntax) }
	pub fn cons(&self) -> SyntaxResult<Stmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmt {
	pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![for])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn init(&self) -> Option<ForStmtInit> { support::as_optional_node(&self.syntax) }
	pub fn test(&self) -> Option<ForStmtTest> { support::as_optional_node(&self.syntax) }
	pub fn update(&self) -> Option<ForStmtUpdate> { support::as_optional_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
	pub fn cons(&self) -> SyntaxResult<Stmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForInStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ForInStmt {
	pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![for])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn left(&self) -> SyntaxResult<ForStmtInit> { support::as_mandatory_node(&self.syntax) }
	pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![in])
	}
	pub fn right(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
	pub fn cons(&self) -> SyntaxResult<Stmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForOfStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ForOfStmt {
	pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![for])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn left(&self) -> SyntaxResult<ForStmtInit> { support::as_mandatory_node(&self.syntax) }
	pub fn of_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![of])
	}
	pub fn right(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
	pub fn cons(&self) -> SyntaxResult<Stmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContinueStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ContinueStmt {
	pub fn continue_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![continue])
	}
	pub fn name_ref(&self) -> SyntaxResult<NameRef> { support::as_mandatory_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakStmt {
	pub(crate) syntax: SyntaxNode,
}
impl BreakStmt {
	pub fn break_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![break])
	}
	pub fn ident_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![ident])
	}
	pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ReturnStmt {
	pub fn return_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![return])
	}
	pub fn value(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithStmt {
	pub(crate) syntax: SyntaxNode,
}
impl WithStmt {
	pub fn with_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![with])
	}
	pub fn condition(&self) -> SyntaxResult<Condition> { support::as_mandatory_node(&self.syntax) }
	pub fn cons(&self) -> SyntaxResult<Stmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabelledStmt {
	pub(crate) syntax: SyntaxNode,
}
impl LabelledStmt {
	pub fn label(&self) -> SyntaxResult<Name> { support::as_mandatory_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn stmt(&self) -> SyntaxResult<Stmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchStmt {
	pub(crate) syntax: SyntaxNode,
}
impl SwitchStmt {
	pub fn switch_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![switch])
	}
	pub fn test(&self) -> SyntaxResult<Condition> { support::as_mandatory_node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn cases(&self) -> AstNodeList<SwitchCase> { support::node_list(&self.syntax, 0usize) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThrowStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ThrowStmt {
	pub fn throw_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![throw])
	}
	pub fn exception(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TryStmt {
	pub(crate) syntax: SyntaxNode,
}
impl TryStmt {
	pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![try])
	}
	pub fn test(&self) -> SyntaxResult<BlockStmt> { support::as_mandatory_node(&self.syntax) }
	pub fn handler(&self) -> Option<CatchClause> { support::as_optional_node(&self.syntax) }
	pub fn finalizer(&self) -> Option<Finalizer> { support::as_optional_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DebuggerStmt {
	pub(crate) syntax: SyntaxNode,
}
impl DebuggerStmt {
	pub fn debugger_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![debugger])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Condition {
	pub(crate) syntax: SyntaxNode,
}
impl Condition {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn condition(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStmtInit {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmtInit {
	pub fn inner(&self) -> SyntaxResult<ForHead> { support::as_mandatory_node(&self.syntax) }
	pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStmtTest {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmtTest {
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStmtUpdate {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmtUpdate {
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VarDecl {
	pub(crate) syntax: SyntaxNode,
}
impl VarDecl {
	pub fn var_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![var])
	}
	pub fn const_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![const])
	}
	pub fn declared(&self) -> AstSeparatedList<Declarator> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef {
	pub(crate) syntax: SyntaxNode,
}
impl NameRef {
	pub fn ident_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![ident])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
	pub(crate) syntax: SyntaxNode,
}
impl Name {
	pub fn ident_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![ident])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaseClause {
	pub(crate) syntax: SyntaxNode,
}
impl CaseClause {
	pub fn case_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![case])
	}
	pub fn test(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn cons(&self) -> AstNodeList<Stmt> { support::node_list(&self.syntax, 0usize) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefaultClause {
	pub(crate) syntax: SyntaxNode,
}
impl DefaultClause {
	pub fn default_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![default])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn cons(&self) -> AstNodeList<Stmt> { support::node_list(&self.syntax, 0usize) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatchClause {
	pub(crate) syntax: SyntaxNode,
}
impl CatchClause {
	pub fn catch_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![catch])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn error(&self) -> SyntaxResult<Pattern> { support::as_mandatory_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
	pub fn cons(&self) -> SyntaxResult<BlockStmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Finalizer {
	pub(crate) syntax: SyntaxNode,
}
impl Finalizer {
	pub fn finally_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![finally])
	}
	pub fn cons(&self) -> SyntaxResult<BlockStmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrowExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ArrowExpr {
	pub fn async_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![async])
	}
	pub fn type_params(&self) -> Option<TsTypeParams> { support::as_optional_node(&self.syntax) }
	pub fn params(&self) -> Option<ArrowExprParams> { support::as_optional_node(&self.syntax) }
	pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [=>])
	}
	pub fn colon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> Option<TsType> { support::as_optional_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
	pub(crate) syntax: SyntaxNode,
}
impl Literal {
	pub fn true_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![true])
	}
	pub fn false_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![false])
	}
	pub fn number_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![number])
	}
	pub fn regex_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![regex])
	}
	pub fn float_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![float])
	}
	pub fn big_int_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![big_int])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Template {
	pub(crate) syntax: SyntaxNode,
}
impl Template {
	pub fn backtick_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['`'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThisExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ThisExpr {
	pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![this])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ArrayExpr {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> AstNodeList<ExprOrSpread> { support::node_list(&self.syntax, 0usize) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![']'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ObjectExpr {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn props(&self) -> AstSeparatedList<ObjectProp> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupingExpr {
	pub(crate) syntax: SyntaxNode,
}
impl GroupingExpr {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn inner(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BracketExpr {
	pub(crate) syntax: SyntaxNode,
}
impl BracketExpr {
	pub fn super_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![super])
	}
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['['])
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![']'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DotExpr {
	pub(crate) syntax: SyntaxNode,
}
impl DotExpr {
	pub fn super_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![super])
	}
	pub fn object(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [.])
	}
	pub fn prop(&self) -> SyntaxResult<Name> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewExpr {
	pub(crate) syntax: SyntaxNode,
}
impl NewExpr {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![new])
	}
	pub fn type_args(&self) -> Option<TsTypeArgs> { support::as_optional_node(&self.syntax) }
	pub fn object(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn arguments(&self) -> SyntaxResult<ArgList> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
	pub(crate) syntax: SyntaxNode,
}
impl CallExpr {
	pub fn type_args(&self) -> Option<TsTypeArgs> { support::as_optional_node(&self.syntax) }
	pub fn callee(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn arguments(&self) -> SyntaxResult<ArgList> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnaryExpr {
	pub(crate) syntax: SyntaxNode,
}
impl UnaryExpr {
	pub fn operator(&self) -> Option<SyntaxToken> {
		support::find_token(
			&self.syntax,
			&[
				T![delete],
				T![void],
				T![typeof],
				T ! [+],
				T ! [-],
				T ! [~],
				T![!],
				T![await],
			],
		)
	}
	pub fn argument(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreUpdateExpression {
	pub(crate) syntax: SyntaxNode,
}
impl PreUpdateExpression {
	pub fn operator(&self) -> Option<SyntaxToken> {
		support::find_token(&self.syntax, &[T ! [++], T ! [--]])
	}
	pub fn operand(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PostUpdateExpression {
	pub(crate) syntax: SyntaxNode,
}
impl PostUpdateExpression {
	pub fn operand(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn operator(&self) -> Option<SyntaxToken> {
		support::find_token(&self.syntax, &[T ! [++], T ! [--]])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinExpr {
	pub(crate) syntax: SyntaxNode,
}
impl BinExpr {
	pub fn operator(&self) -> Option<SyntaxToken> {
		support::find_token(
			&self.syntax,
			&[
				T ! [<],
				T ! [>],
				T ! [<=],
				T ! [>=],
				T ! [==],
				T ! [===],
				T ! [!=],
				T ! [!==],
				T ! [+],
				T ! [-],
				T ! [*],
				T ! [/],
				T ! [%],
				T ! [**],
				T ! [<<],
				T ! [>>],
				T ! [>>>],
				T ! [&],
				T ! [|],
				T ! [^],
				T ! [??],
				T ! [||],
				T ! [&&],
				T![in],
				T![instanceof],
			],
		)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CondExpr {
	pub(crate) syntax: SyntaxNode,
}
impl CondExpr {
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignExpr {
	pub(crate) syntax: SyntaxNode,
}
impl AssignExpr {
	pub fn operator(&self) -> Option<SyntaxToken> {
		support::find_token(
			&self.syntax,
			&[
				T ! [=],
				T ! [+=],
				T ! [-=],
				T ! [*=],
				T ! [%=],
				T ! [**=],
				T ! [>>=],
				T ! [<<=],
				T ! [>>>=],
				T ! [&=],
				T ! [|=],
				T ! [^=],
				T ! [&&=],
				T ! [||=],
				T ! [??=],
			],
		)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SequenceExpr {
	pub(crate) syntax: SyntaxNode,
}
impl SequenceExpr {
	pub fn exprs(&self) -> AstSeparatedList<Expr> { support::separated_list(&self.syntax, 0usize) }
	pub fn bin_expr(&self) -> SyntaxResult<BinExpr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnExpr {
	pub(crate) syntax: SyntaxNode,
}
impl FnExpr {
	pub fn async_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![async])
	}
	pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![function])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [*])
	}
	pub fn name(&self) -> SyntaxResult<Name> { support::as_mandatory_node(&self.syntax) }
	pub fn type_params(&self) -> Option<TsTypeParams> { support::as_optional_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<ArgList> { support::as_mandatory_node(&self.syntax) }
	pub fn colon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> Option<TsType> { support::as_optional_node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<BlockStmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ClassExpr {
	pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![class])
	}
	pub fn name(&self) -> SyntaxResult<Name> { support::as_mandatory_node(&self.syntax) }
	pub fn type_params(&self) -> Option<TsTypeParams> { support::as_optional_node(&self.syntax) }
	pub fn parent(&self) -> Option<Expr> { support::as_optional_node(&self.syntax) }
	pub fn implements_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![implements])
	}
	pub fn implements(&self) -> Option<TsExprWithTypeArgs> {
		support::as_optional_node(&self.syntax)
	}
	pub fn body(&self) -> SyntaxResult<ClassBody> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewTarget {
	pub(crate) syntax: SyntaxNode,
}
impl NewTarget {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![new])
	}
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [.])
	}
	pub fn target_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![target])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportMeta {
	pub(crate) syntax: SyntaxNode,
}
impl ImportMeta {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![import])
	}
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [.])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuperCall {
	pub(crate) syntax: SyntaxNode,
}
impl SuperCall {
	pub fn super_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![super])
	}
	pub fn arguments(&self) -> SyntaxResult<ArgList> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportCall {
	pub(crate) syntax: SyntaxNode,
}
impl ImportCall {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![import])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn argument(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct YieldExpr {
	pub(crate) syntax: SyntaxNode,
}
impl YieldExpr {
	pub fn yield_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![yield])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [*])
	}
	pub fn value(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwaitExpr {
	pub(crate) syntax: SyntaxNode,
}
impl AwaitExpr {
	pub fn await_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![await])
	}
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrivatePropAccess {
	pub(crate) syntax: SyntaxNode,
}
impl PrivatePropAccess {
	pub fn lhs(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [.])
	}
	pub fn rhs(&self) -> SyntaxResult<PrivateName> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsNonNull {
	pub(crate) syntax: SyntaxNode,
}
impl TsNonNull {
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![!])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsAssertion {
	pub(crate) syntax: SyntaxNode,
}
impl TsAssertion {
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
	pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [<])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [>])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsConstAssertion {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstAssertion {
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
	pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [<])
	}
	pub fn const_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![const])
	}
	pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [>])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTypeArgs {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeArgs {
	pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [<])
	}
	pub fn args(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [>])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgList {
	pub(crate) syntax: SyntaxNode,
}
impl ArgList {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn args(&self) -> AstSeparatedList<Expr> { support::separated_list(&self.syntax, 0usize) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTypeParams {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeParams {
	pub fn l_angle_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [<])
	}
	pub fn params(&self) -> SyntaxResult<TsTypeParam> { support::as_mandatory_node(&self.syntax) }
	pub fn r_angle_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [>])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParameterList {
	pub(crate) syntax: SyntaxNode,
}
impl ParameterList {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn parameters(&self) -> AstSeparatedList<Pattern> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsExprWithTypeArgs {
	pub(crate) syntax: SyntaxNode,
}
impl TsExprWithTypeArgs {
	pub fn item(&self) -> SyntaxResult<TsEntityName> { support::as_mandatory_node(&self.syntax) }
	pub fn type_params(&self) -> SyntaxResult<TsTypeArgs> {
		support::as_mandatory_node(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassBody {
	pub(crate) syntax: SyntaxNode,
}
impl ClassBody {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn elements(&self) -> AstNodeList<ClassElement> { support::node_list(&self.syntax, 0usize) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Method {
	pub(crate) syntax: SyntaxNode,
}
impl Method {
	pub fn static_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![static])
	}
	pub fn async_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![async])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [*])
	}
	pub fn name(&self) -> SyntaxResult<PropName> { support::as_mandatory_node(&self.syntax) }
	pub fn type_params(&self) -> Option<TsTypeParams> { support::as_optional_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<ParameterList> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn colon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> Option<TsType> { support::as_optional_node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<BlockStmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrivateProp {
	pub(crate) syntax: SyntaxNode,
}
impl PrivateProp {
	pub fn class_prop(&self) -> SyntaxResult<ClassProp> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassProp {
	pub(crate) syntax: SyntaxNode,
}
impl ClassProp {
	pub fn declare_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![declare])
	}
	pub fn abstract_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![abstract])
	}
	pub fn static_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![static])
	}
	pub fn accessibility(&self) -> Option<TsAccessibility> {
		support::as_optional_node(&self.syntax)
	}
	pub fn hash_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [#])
	}
	pub fn key(&self) -> SyntaxResult<PropName> { support::as_mandatory_node(&self.syntax) }
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [?])
	}
	pub fn excl_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![!])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn eq_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [=])
	}
	pub fn value(&self) -> Option<Expr> { support::as_optional_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constructor {
	pub(crate) syntax: SyntaxNode,
}
impl Constructor {
	pub fn accessibility(&self) -> Option<TsAccessibility> {
		support::as_optional_node(&self.syntax)
	}
	pub fn name(&self) -> SyntaxResult<PropName> { support::as_mandatory_node(&self.syntax) }
	pub fn type_params(&self) -> Option<TsTypeParams> { support::as_optional_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<ParameterList> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn body(&self) -> SyntaxResult<BlockStmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsIndexSignature {
	pub(crate) syntax: SyntaxNode,
}
impl TsIndexSignature {
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![readonly])
	}
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['['])
	}
	pub fn pat(&self) -> SyntaxResult<SinglePattern> { support::as_mandatory_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![']'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Getter {
	pub(crate) syntax: SyntaxNode,
}
impl Getter {
	pub fn get_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![get])
	}
	pub fn key(&self) -> SyntaxResult<PropName> { support::as_mandatory_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<ParameterList> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn body(&self) -> SyntaxResult<BlockStmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Setter {
	pub(crate) syntax: SyntaxNode,
}
impl Setter {
	pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![set])
	}
	pub fn key(&self) -> SyntaxResult<PropName> { support::as_mandatory_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<ParameterList> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn body(&self) -> SyntaxResult<BlockStmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsAccessibility {
	pub(crate) syntax: SyntaxNode,
}
impl TsAccessibility {
	pub fn private_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![private])
	}
	pub fn readonly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![readonly])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstructorParameters {
	pub(crate) syntax: SyntaxNode,
}
impl ConstructorParameters {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn parameters(&self) -> AstSeparatedList<ConstructorParamOrPat> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsConstructorParam {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstructorParam {
	pub fn readonly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![readonly])
	}
	pub fn pat(&self) -> SyntaxResult<Pattern> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpreadElement {
	pub(crate) syntax: SyntaxNode,
}
impl SpreadElement {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [...])
	}
	pub fn element(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Null {
	pub(crate) syntax: SyntaxNode,
}
impl Null {
	pub fn null_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![null])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Undefined {
	pub(crate) syntax: SyntaxNode,
}
impl Undefined {
	pub fn undefined_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![undefined])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SinglePattern {
	pub(crate) syntax: SyntaxNode,
}
impl SinglePattern {
	pub fn name(&self) -> SyntaxResult<Name> { support::as_mandatory_node(&self.syntax) }
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [?])
	}
	pub fn excl_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![!])
	}
	pub fn colon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RestPattern {
	pub(crate) syntax: SyntaxNode,
}
impl RestPattern {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [...])
	}
	pub fn pat(&self) -> SyntaxResult<Pattern> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignPattern {
	pub(crate) syntax: SyntaxNode,
}
impl AssignPattern {
	pub fn key(&self) -> SyntaxResult<Pattern> { support::as_mandatory_node(&self.syntax) }
	pub fn colon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [=])
	}
	pub fn value(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectPattern {
	pub(crate) syntax: SyntaxNode,
}
impl ObjectPattern {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn elements(&self) -> AstSeparatedList<ObjectPatternProp> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayPattern {
	pub(crate) syntax: SyntaxNode,
}
impl ArrayPattern {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> AstNodeList<Pattern> { support::node_list(&self.syntax, 0usize) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![']'])
	}
	pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![!])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprPattern {
	pub(crate) syntax: SyntaxNode,
}
impl ExprPattern {
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyValuePattern {
	pub(crate) syntax: SyntaxNode,
}
impl KeyValuePattern {
	pub fn key(&self) -> SyntaxResult<PropName> { support::as_mandatory_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralProp {
	pub(crate) syntax: SyntaxNode,
}
impl LiteralProp {
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpreadProp {
	pub(crate) syntax: SyntaxNode,
}
impl SpreadProp {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [...])
	}
	pub fn value(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitializedProp {
	pub(crate) syntax: SyntaxNode,
}
impl InitializedProp {
	pub fn key(&self) -> SyntaxResult<Name> { support::as_mandatory_node(&self.syntax) }
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [=])
	}
	pub fn value(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentProp {
	pub(crate) syntax: SyntaxNode,
}
impl IdentProp {
	pub fn name(&self) -> SyntaxResult<Name> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComputedPropertyName {
	pub(crate) syntax: SyntaxNode,
}
impl ComputedPropertyName {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['['])
	}
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![']'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrivateName {
	pub(crate) syntax: SyntaxNode,
}
impl PrivateName {
	pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [#])
	}
	pub fn name(&self) -> SyntaxResult<Name> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnDecl {
	pub(crate) syntax: SyntaxNode,
}
impl FnDecl {
	pub fn async_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![async])
	}
	pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![function])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [*])
	}
	pub fn name(&self) -> SyntaxResult<Name> { support::as_mandatory_node(&self.syntax) }
	pub fn type_parameters(&self) -> Option<TsTypeParams> {
		support::as_optional_node(&self.syntax)
	}
	pub fn parameters(&self) -> SyntaxResult<ParameterList> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn colon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> Option<TsType> { support::as_optional_node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<BlockStmt> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassDecl {
	pub(crate) syntax: SyntaxNode,
}
impl ClassDecl {
	pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![class])
	}
	pub fn name(&self) -> SyntaxResult<Name> { support::as_mandatory_node(&self.syntax) }
	pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![extends])
	}
	pub fn parent(&self) -> SyntaxResult<NameRef> { support::as_mandatory_node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<ClassBody> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsEnum {
	pub(crate) syntax: SyntaxNode,
}
impl TsEnum {
	pub fn const_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![const])
	}
	pub fn enum_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![enum])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> AstNodeList<TsEnumMember> { support::node_list(&self.syntax, 0usize) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTypeAliasDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeAliasDecl {
	pub fn type_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![type])
	}
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [=])
	}
	pub fn ts_type(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsNamespaceDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsNamespaceDecl {
	pub fn declare_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![declare])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
	pub fn dot_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [.])
	}
	pub fn ts_namespace_body(&self) -> SyntaxResult<TsNamespaceBody> {
		support::as_mandatory_node(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsModuleDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsModuleDecl {
	pub fn declare_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![declare])
	}
	pub fn global_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![global])
	}
	pub fn module_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![module])
	}
	pub fn dot_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [.])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<TsNamespaceBody> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsInterfaceDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsInterfaceDecl {
	pub fn declare_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![declare])
	}
	pub fn interface_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![interface])
	}
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn extends_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![extends])
	}
	pub fn extends(&self) -> Option<TsExprWithTypeArgs> { support::as_optional_node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> SyntaxResult<TsTypeElement> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Declarator {
	pub(crate) syntax: SyntaxNode,
}
impl Declarator {
	pub fn pattern(&self) -> SyntaxResult<Pattern> { support::as_mandatory_node(&self.syntax) }
	pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![!])
	}
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [=])
	}
	pub fn value(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportDecl {
	pub(crate) syntax: SyntaxNode,
}
impl ImportDecl {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![import])
	}
	pub fn imports(&self) -> AstNodeList<ImportClause> { support::node_list(&self.syntax, 0usize) }
	pub fn type_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![type])
	}
	pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![from])
	}
	pub fn asserted_object(&self) -> SyntaxResult<ObjectExpr> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn assert_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![assert])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportNamed {
	pub(crate) syntax: SyntaxNode,
}
impl ExportNamed {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![type])
	}
	pub fn from_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![from])
	}
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn specifiers(&self) -> AstSeparatedList<Specifier> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportDefaultDecl {
	pub(crate) syntax: SyntaxNode,
}
impl ExportDefaultDecl {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![export])
	}
	pub fn default_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![default])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![type])
	}
	pub fn decl(&self) -> SyntaxResult<DefaultDecl> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportDefaultExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ExportDefaultExpr {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![type])
	}
	pub fn default_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![default])
	}
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportWildcard {
	pub(crate) syntax: SyntaxNode,
}
impl ExportWildcard {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![type])
	}
	pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [*])
	}
	pub fn as_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![as])
	}
	pub fn ident(&self) -> Option<Ident> { support::as_optional_node(&self.syntax) }
	pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![from])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportDecl {
	pub(crate) syntax: SyntaxNode,
}
impl ExportDecl {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![type])
	}
	pub fn decl(&self) -> SyntaxResult<Decl> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsImportEqualsDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsImportEqualsDecl {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![import])
	}
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![export])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [=])
	}
	pub fn module(&self) -> SyntaxResult<TsModuleRef> { support::as_mandatory_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsExportAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl TsExportAssignment {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![export])
	}
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [=])
	}
	pub fn expr(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsNamespaceExportDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsNamespaceExportDecl {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![export])
	}
	pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![as])
	}
	pub fn namespace_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![namespace])
	}
	pub fn ident(&self) -> Option<Ident> { support::as_optional_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WildcardImport {
	pub(crate) syntax: SyntaxNode,
}
impl WildcardImport {
	pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [*])
	}
	pub fn as_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![as])
	}
	pub fn ident(&self) -> Option<Ident> { support::as_optional_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedImports {
	pub(crate) syntax: SyntaxNode,
}
impl NamedImports {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn specifiers(&self) -> AstSeparatedList<Specifier> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportStringSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl ImportStringSpecifier {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Specifier {
	pub(crate) syntax: SyntaxNode,
}
impl Specifier {
	pub fn name(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsExternalModuleRef {
	pub(crate) syntax: SyntaxNode,
}
impl TsExternalModuleRef {
	pub fn require_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![require])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsAny {
	pub(crate) syntax: SyntaxNode,
}
impl TsAny {
	pub fn any_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![any])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsUnknown {
	pub(crate) syntax: SyntaxNode,
}
impl TsUnknown {
	pub fn unknown_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![unknown])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsNumber {
	pub(crate) syntax: SyntaxNode,
}
impl TsNumber {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsObject {
	pub(crate) syntax: SyntaxNode,
}
impl TsObject {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsBoolean {
	pub(crate) syntax: SyntaxNode,
}
impl TsBoolean {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsBigint {
	pub(crate) syntax: SyntaxNode,
}
impl TsBigint {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsString {
	pub(crate) syntax: SyntaxNode,
}
impl TsString {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsSymbol {
	pub(crate) syntax: SyntaxNode,
}
impl TsSymbol {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsVoid {
	pub(crate) syntax: SyntaxNode,
}
impl TsVoid {
	pub fn void_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![void])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsUndefined {
	pub(crate) syntax: SyntaxNode,
}
impl TsUndefined {
	pub fn undefined_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![undefined])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsNull {
	pub(crate) syntax: SyntaxNode,
}
impl TsNull {
	pub fn null_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![null])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsNever {
	pub(crate) syntax: SyntaxNode,
}
impl TsNever {
	pub fn never_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![never])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsThis {
	pub(crate) syntax: SyntaxNode,
}
impl TsThis {
	pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![this])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsLiteral {
	pub(crate) syntax: SyntaxNode,
}
impl TsLiteral {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsPredicate {
	pub(crate) syntax: SyntaxNode,
}
impl TsPredicate {
	pub fn lhs(&self) -> SyntaxResult<TsThisOrMore> { support::as_mandatory_node(&self.syntax) }
	pub fn rhs(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTuple {
	pub(crate) syntax: SyntaxNode,
}
impl TsTuple {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> SyntaxResult<TsTupleElement> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![']'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsParen {
	pub(crate) syntax: SyntaxNode,
}
impl TsParen {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTypeRef {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeRef {
	pub fn name(&self) -> SyntaxResult<TsEntityName> { support::as_mandatory_node(&self.syntax) }
	pub fn type_args(&self) -> SyntaxResult<TsTypeArgs> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTemplate {
	pub(crate) syntax: SyntaxNode,
}
impl TsTemplate {
	pub fn elements(&self) -> SyntaxResult<TsTemplateElement> {
		support::as_mandatory_node(&self.syntax)
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsMappedType {
	pub(crate) syntax: SyntaxNode,
}
impl TsMappedType {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn readonly_modifier(&self) -> Option<TsMappedTypeReadonly> {
		support::as_optional_node(&self.syntax)
	}
	pub fn minus_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [-])
	}
	pub fn plus_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [+])
	}
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [?])
	}
	pub fn param(&self) -> SyntaxResult<TsMappedTypeParam> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [;])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsImport {
	pub(crate) syntax: SyntaxNode,
}
impl TsImport {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![import])
	}
	pub fn type_args(&self) -> SyntaxResult<TsTypeArgs> { support::as_mandatory_node(&self.syntax) }
	pub fn dot_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [.])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['('])
	}
	pub fn qualifier(&self) -> SyntaxResult<TsEntityName> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![')'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsArray {
	pub(crate) syntax: SyntaxNode,
}
impl TsArray {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['['])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![']'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsIndexedArray {
	pub(crate) syntax: SyntaxNode,
}
impl TsIndexedArray {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['['])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![']'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTypeOperator {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeOperator {
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsIntersection {
	pub(crate) syntax: SyntaxNode,
}
impl TsIntersection {
	pub fn types(&self) -> AstNodeList<TsType> { support::node_list(&self.syntax, 0usize) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsUnion {
	pub(crate) syntax: SyntaxNode,
}
impl TsUnion {
	pub fn types(&self) -> AstNodeList<TsType> { support::node_list(&self.syntax, 0usize) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsFnType {
	pub(crate) syntax: SyntaxNode,
}
impl TsFnType {
	pub fn params(&self) -> SyntaxResult<ParameterList> { support::as_mandatory_node(&self.syntax) }
	pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [=>])
	}
	pub fn return_type(&self) -> Option<TsType> { support::as_optional_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsConstructorType {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstructorType {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![new])
	}
	pub fn params(&self) -> SyntaxResult<ParameterList> { support::as_mandatory_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> Option<TsType> { support::as_optional_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsConditionalType {
	pub(crate) syntax: SyntaxNode,
}
impl TsConditionalType {
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn extends(&self) -> SyntaxResult<TsExtends> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsObjectType {
	pub(crate) syntax: SyntaxNode,
}
impl TsObjectType {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> AstNodeList<TsTypeElement> { support::node_list(&self.syntax, 0usize) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsInfer {
	pub(crate) syntax: SyntaxNode,
}
impl TsInfer {
	pub fn infer_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![infer])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTupleElement {
	pub(crate) syntax: SyntaxNode,
}
impl TsTupleElement {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [?])
	}
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [...])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsEnumMember {
	pub(crate) syntax: SyntaxNode,
}
impl TsEnumMember {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [=])
	}
	pub fn value(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTemplateElement {
	pub(crate) syntax: SyntaxNode,
}
impl TsTemplateElement {
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsMappedTypeReadonly {
	pub(crate) syntax: SyntaxNode,
}
impl TsMappedTypeReadonly {
	pub fn minus_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [-])
	}
	pub fn plus_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [+])
	}
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![readonly])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsMappedTypeParam {
	pub(crate) syntax: SyntaxNode,
}
impl TsMappedTypeParam {
	pub fn l_brack_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T!['['])
	}
	pub fn name(&self) -> Option<TsTypeName> { support::as_optional_node(&self.syntax) }
	pub fn r_brack_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![']'])
	}
	pub fn ident(&self) -> Option<Ident> { support::as_optional_node(&self.syntax) }
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTypeName {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeName {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsExtends {
	pub(crate) syntax: SyntaxNode,
}
impl TsExtends {
	pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![extends])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsModuleBlock {
	pub(crate) syntax: SyntaxNode,
}
impl TsModuleBlock {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['{'])
	}
	pub fn items(&self) -> SyntaxResult<ModuleItem> { support::as_mandatory_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T!['}'])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTypeParam {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeParam {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::as_mandatory_node(&self.syntax) }
	pub fn constraint(&self) -> SyntaxResult<TsConstraint> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn default(&self) -> SyntaxResult<TsDefault> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsConstraint {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstraint {
	pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![extends])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsDefault {
	pub(crate) syntax: SyntaxNode,
}
impl TsDefault {
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [=])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsCallSignatureDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsCallSignatureDecl {
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn parameters(&self) -> SyntaxResult<ParameterList> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsConstructSignatureDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstructSignatureDecl {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T![new])
	}
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn parameters(&self) -> SyntaxResult<ParameterList> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn colon_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsPropertySignature {
	pub(crate) syntax: SyntaxNode,
}
impl TsPropertySignature {
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![readonly])
	}
	pub fn prop(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsMethodSignature {
	pub(crate) syntax: SyntaxNode,
}
impl TsMethodSignature {
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T![readonly])
	}
	pub fn key(&self) -> SyntaxResult<Expr> { support::as_mandatory_node(&self.syntax) }
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn parameters(&self) -> SyntaxResult<ParameterList> {
		support::as_mandatory_node(&self.syntax)
	}
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::as_optional_token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> SyntaxResult<TsType> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsQualifiedPath {
	pub(crate) syntax: SyntaxNode,
}
impl TsQualifiedPath {
	pub fn lhs(&self) -> SyntaxResult<TsEntityName> { support::as_mandatory_node(&self.syntax) }
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::as_mandatory_token(&self.syntax, T ! [.])
	}
	pub fn rhs(&self) -> SyntaxResult<TsTypeName> { support::as_mandatory_node(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
	BlockStmt(BlockStmt),
	EmptyStmt(EmptyStmt),
	ExprStmt(ExprStmt),
	IfStmt(IfStmt),
	DoWhileStmt(DoWhileStmt),
	WhileStmt(WhileStmt),
	ForStmt(ForStmt),
	ForInStmt(ForInStmt),
	ForOfStmt(ForOfStmt),
	ContinueStmt(ContinueStmt),
	BreakStmt(BreakStmt),
	ReturnStmt(ReturnStmt),
	WithStmt(WithStmt),
	LabelledStmt(LabelledStmt),
	SwitchStmt(SwitchStmt),
	ThrowStmt(ThrowStmt),
	TryStmt(TryStmt),
	DebuggerStmt(DebuggerStmt),
	Decl(Decl),
	JsUnknownStatement(JsUnknownStatement),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleItem {
	ImportDecl(ImportDecl),
	ExportNamed(ExportNamed),
	ExportDefaultDecl(ExportDefaultDecl),
	ExportDefaultExpr(ExportDefaultExpr),
	ExportWildcard(ExportWildcard),
	ExportDecl(ExportDecl),
	TsImportEqualsDecl(TsImportEqualsDecl),
	TsExportAssignment(TsExportAssignment),
	TsNamespaceExportDecl(TsNamespaceExportDecl),
	Stmt(Stmt),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Decl {
	FnDecl(FnDecl),
	ClassDecl(ClassDecl),
	VarDecl(VarDecl),
	TsEnum(TsEnum),
	TsTypeAliasDecl(TsTypeAliasDecl),
	TsNamespaceDecl(TsNamespaceDecl),
	TsModuleDecl(TsModuleDecl),
	TsInterfaceDecl(TsInterfaceDecl),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
	ArrowExpr(ArrowExpr),
	Literal(Literal),
	Template(Template),
	NameRef(NameRef),
	ThisExpr(ThisExpr),
	ArrayExpr(ArrayExpr),
	ObjectExpr(ObjectExpr),
	GroupingExpr(GroupingExpr),
	BracketExpr(BracketExpr),
	DotExpr(DotExpr),
	NewExpr(NewExpr),
	CallExpr(CallExpr),
	UnaryExpr(UnaryExpr),
	PreUpdateExpression(PreUpdateExpression),
	PostUpdateExpression(PostUpdateExpression),
	BinExpr(BinExpr),
	CondExpr(CondExpr),
	AssignExpr(AssignExpr),
	SequenceExpr(SequenceExpr),
	FnExpr(FnExpr),
	ClassExpr(ClassExpr),
	NewTarget(NewTarget),
	ImportMeta(ImportMeta),
	SuperCall(SuperCall),
	ImportCall(ImportCall),
	YieldExpr(YieldExpr),
	AwaitExpr(AwaitExpr),
	PrivatePropAccess(PrivatePropAccess),
	TsNonNull(TsNonNull),
	TsAssertion(TsAssertion),
	TsConstAssertion(TsConstAssertion),
	JsUnknownExpression(JsUnknownExpression),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForHead {
	VarDecl(VarDecl),
	Expr(Expr),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SwitchCase {
	CaseClause(CaseClause),
	DefaultClause(DefaultClause),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
	SinglePattern(SinglePattern),
	RestPattern(RestPattern),
	AssignPattern(AssignPattern),
	ObjectPattern(ObjectPattern),
	ArrayPattern(ArrayPattern),
	ExprPattern(ExprPattern),
	JsUnknownPattern(JsUnknownPattern),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsType {
	TsAny(TsAny),
	TsUnknown(TsUnknown),
	TsNumber(TsNumber),
	TsObject(TsObject),
	TsBoolean(TsBoolean),
	TsBigint(TsBigint),
	TsString(TsString),
	TsSymbol(TsSymbol),
	TsVoid(TsVoid),
	TsUndefined(TsUndefined),
	TsNull(TsNull),
	TsNever(TsNever),
	TsThis(TsThis),
	TsLiteral(TsLiteral),
	TsPredicate(TsPredicate),
	TsTuple(TsTuple),
	TsParen(TsParen),
	TsTypeRef(TsTypeRef),
	TsTemplate(TsTemplate),
	TsMappedType(TsMappedType),
	TsImport(TsImport),
	TsArray(TsArray),
	TsIndexedArray(TsIndexedArray),
	TsTypeOperator(TsTypeOperator),
	TsIntersection(TsIntersection),
	TsUnion(TsUnion),
	TsFnType(TsFnType),
	TsConstructorType(TsConstructorType),
	TsConditionalType(TsConditionalType),
	TsObjectType(TsObjectType),
	TsInfer(TsInfer),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArrowExprParams {
	Name(Name),
	ParameterList(ParameterList),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprOrBlock {
	Expr(Expr),
	BlockStmt(BlockStmt),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectProp {
	LiteralProp(LiteralProp),
	Getter(Getter),
	Setter(Setter),
	SpreadProp(SpreadProp),
	InitializedProp(InitializedProp),
	IdentProp(IdentProp),
	Method(Method),
	JsUnknownMember(JsUnknownMember),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassElement {
	EmptyStmt(EmptyStmt),
	Method(Method),
	PrivateProp(PrivateProp),
	ClassProp(ClassProp),
	Constructor(Constructor),
	TsIndexSignature(TsIndexSignature),
	Getter(Getter),
	Setter(Setter),
	JsUnknownMember(JsUnknownMember),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropName {
	ComputedPropertyName(ComputedPropertyName),
	Literal(Literal),
	Ident(Ident),
	Name(Name),
	JsUnknownBinding(JsUnknownBinding),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstructorParamOrPat {
	TsConstructorParam(TsConstructorParam),
	Pattern(Pattern),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprOrSpread {
	Expr(Expr),
	SpreadElement(SpreadElement),
	GroupingExpr(GroupingExpr),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternOrExpr {
	Pattern(Pattern),
	Expr(Expr),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectPatternProp {
	AssignPattern(AssignPattern),
	KeyValuePattern(KeyValuePattern),
	RestPattern(RestPattern),
	SinglePattern(SinglePattern),
	JsUnknownPattern(JsUnknownPattern),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImportClause {
	WildcardImport(WildcardImport),
	NamedImports(NamedImports),
	Name(Name),
	ImportStringSpecifier(ImportStringSpecifier),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefaultDecl {
	FnDecl(FnDecl),
	ClassDecl(ClassDecl),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsModuleRef {
	TsExternalModuleRef(TsExternalModuleRef),
	TsEntityName(TsEntityName),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsEntityName {
	TsTypeName(TsTypeName),
	TsQualifiedPath(TsQualifiedPath),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsThisOrMore {
	TsThis(TsThis),
	TsTypeName(TsTypeName),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsTypeElement {
	TsCallSignatureDecl(TsCallSignatureDecl),
	TsConstructSignatureDecl(TsConstructSignatureDecl),
	TsPropertySignature(TsPropertySignature),
	TsMethodSignature(TsMethodSignature),
	TsIndexSignature(TsIndexSignature),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsNamespaceBody {
	TsModuleBlock(TsModuleBlock),
	TsNamespaceDecl(TsNamespaceDecl),
}
impl AstNode for JsUnknownStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsUnknownExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsUnknownPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsUnknownMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsUnknownBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for JsUnknownAssignmentTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_ASSIGNMENT_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Ident {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IDENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Script {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SCRIPT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Module {
	fn can_cast(kind: SyntaxKind) -> bool { kind == MODULE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BlockStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == BLOCK_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for EmptyStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EMPTY_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExprStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPR_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for IfStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IF_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DoWhileStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == DO_WHILE_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for WhileStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == WHILE_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ForStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ForInStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_IN_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ForOfStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_OF_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ContinueStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CONTINUE_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BreakStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == BREAK_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ReturnStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == RETURN_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for WithStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == WITH_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for LabelledStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == LABELLED_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SwitchStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SWITCH_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ThrowStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == THROW_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TryStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TRY_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DebuggerStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == DEBUGGER_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Condition {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CONDITION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ForStmtInit {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT_INIT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ForStmtTest {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT_TEST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ForStmtUpdate {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT_UPDATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for VarDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == VAR_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for NameRef {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NAME_REF }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Name {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for CaseClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CASE_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DefaultClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == DEFAULT_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for CatchClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CATCH_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Finalizer {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FINALIZER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArrowExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ARROW_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Literal {
	fn can_cast(kind: SyntaxKind) -> bool { kind == LITERAL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Template {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TEMPLATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ThisExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == THIS_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArrayExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ARRAY_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ObjectExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == OBJECT_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for GroupingExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == GROUPING_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BracketExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == BRACKET_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DotExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == DOT_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for NewExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NEW_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for CallExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CALL_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for UnaryExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == UNARY_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PreUpdateExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PRE_UPDATE_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PostUpdateExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == POST_UPDATE_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BinExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == BIN_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for CondExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == COND_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for AssignExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ASSIGN_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SequenceExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SEQUENCE_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for FnExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FN_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ClassExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CLASS_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for NewTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NEW_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ImportMeta {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IMPORT_META }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SuperCall {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SUPER_CALL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ImportCall {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IMPORT_CALL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for YieldExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == YIELD_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for AwaitExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == AWAIT_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PrivatePropAccess {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PRIVATE_PROP_ACCESS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsNonNull {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NON_NULL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsAssertion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ASSERTION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsConstAssertion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONST_ASSERTION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTypeArgs {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_ARGS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArgList {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ARG_LIST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTypeParams {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_PARAMS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ParameterList {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PARAMETER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsExprWithTypeArgs {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXPR_WITH_TYPE_ARGS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ClassBody {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CLASS_BODY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Method {
	fn can_cast(kind: SyntaxKind) -> bool { kind == METHOD }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PrivateProp {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PRIVATE_PROP }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ClassProp {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CLASS_PROP }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Constructor {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CONSTRUCTOR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsIndexSignature {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INDEX_SIGNATURE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Getter {
	fn can_cast(kind: SyntaxKind) -> bool { kind == GETTER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Setter {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SETTER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsAccessibility {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ACCESSIBILITY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ConstructorParameters {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CONSTRUCTOR_PARAMETERS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsConstructorParam {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRUCTOR_PARAM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SpreadElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SPREAD_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Null {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NULL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Undefined {
	fn can_cast(kind: SyntaxKind) -> bool { kind == UNDEFINED }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SinglePattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SINGLE_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RestPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == REST_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for AssignPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ASSIGN_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ObjectPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == OBJECT_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArrayPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ARRAY_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExprPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPR_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for KeyValuePattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == KEY_VALUE_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for LiteralProp {
	fn can_cast(kind: SyntaxKind) -> bool { kind == LITERAL_PROP }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SpreadProp {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SPREAD_PROP }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for InitializedProp {
	fn can_cast(kind: SyntaxKind) -> bool { kind == INITIALIZED_PROP }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for IdentProp {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IDENT_PROP }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ComputedPropertyName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == COMPUTED_PROPERTY_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PrivateName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PRIVATE_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for FnDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FN_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ClassDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CLASS_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsEnum {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ENUM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTypeAliasDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_ALIAS_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsNamespaceDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NAMESPACE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsModuleDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MODULE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsInterfaceDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INTERFACE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Declarator {
	fn can_cast(kind: SyntaxKind) -> bool { kind == DECLARATOR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ImportDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IMPORT_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExportNamed {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_NAMED }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExportDefaultDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_DEFAULT_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExportDefaultExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_DEFAULT_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExportWildcard {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_WILDCARD }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExportDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsImportEqualsDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_IMPORT_EQUALS_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsExportAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXPORT_ASSIGNMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsNamespaceExportDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NAMESPACE_EXPORT_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for WildcardImport {
	fn can_cast(kind: SyntaxKind) -> bool { kind == WILDCARD_IMPORT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for NamedImports {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NAMED_IMPORTS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ImportStringSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IMPORT_STRING_SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Specifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsExternalModuleRef {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXTERNAL_MODULE_REF }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsAny {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ANY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsUnknown {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_UNKNOWN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsNumber {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NUMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsObject {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_OBJECT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsBoolean {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_BOOLEAN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsBigint {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_BIGINT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsString {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_STRING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsSymbol {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_SYMBOL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsVoid {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_VOID }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsUndefined {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_UNDEFINED }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsNull {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NULL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsNever {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NEVER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsThis {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_THIS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsLiteral {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_LITERAL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsPredicate {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_PREDICATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTuple {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TUPLE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsParen {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_PAREN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTypeRef {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_REF }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTemplate {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TEMPLATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsMappedType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MAPPED_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsImport {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_IMPORT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsArray {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ARRAY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsIndexedArray {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INDEXED_ARRAY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTypeOperator {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_OPERATOR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsIntersection {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INTERSECTION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsUnion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_UNION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsFnType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_FN_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsConstructorType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRUCTOR_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsConditionalType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONDITIONAL_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsObjectType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_OBJECT_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsInfer {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INFER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTupleElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TUPLE_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsEnumMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ENUM_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTemplateElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TEMPLATE_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsMappedTypeReadonly {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MAPPED_TYPE_READONLY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsMappedTypeParam {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MAPPED_TYPE_PARAM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTypeName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsExtends {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXTENDS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsModuleBlock {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MODULE_BLOCK }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTypeParam {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_PARAM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsConstraint {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRAINT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsDefault {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_DEFAULT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsCallSignatureDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CALL_SIGNATURE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsConstructSignatureDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRUCT_SIGNATURE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsPropertySignature {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_PROPERTY_SIGNATURE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsMethodSignature {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_METHOD_SIGNATURE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsQualifiedPath {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_QUALIFIED_PATH }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl From<BlockStmt> for Stmt {
	fn from(node: BlockStmt) -> Stmt { Stmt::BlockStmt(node) }
}
impl From<EmptyStmt> for Stmt {
	fn from(node: EmptyStmt) -> Stmt { Stmt::EmptyStmt(node) }
}
impl From<ExprStmt> for Stmt {
	fn from(node: ExprStmt) -> Stmt { Stmt::ExprStmt(node) }
}
impl From<IfStmt> for Stmt {
	fn from(node: IfStmt) -> Stmt { Stmt::IfStmt(node) }
}
impl From<DoWhileStmt> for Stmt {
	fn from(node: DoWhileStmt) -> Stmt { Stmt::DoWhileStmt(node) }
}
impl From<WhileStmt> for Stmt {
	fn from(node: WhileStmt) -> Stmt { Stmt::WhileStmt(node) }
}
impl From<ForStmt> for Stmt {
	fn from(node: ForStmt) -> Stmt { Stmt::ForStmt(node) }
}
impl From<ForInStmt> for Stmt {
	fn from(node: ForInStmt) -> Stmt { Stmt::ForInStmt(node) }
}
impl From<ForOfStmt> for Stmt {
	fn from(node: ForOfStmt) -> Stmt { Stmt::ForOfStmt(node) }
}
impl From<ContinueStmt> for Stmt {
	fn from(node: ContinueStmt) -> Stmt { Stmt::ContinueStmt(node) }
}
impl From<BreakStmt> for Stmt {
	fn from(node: BreakStmt) -> Stmt { Stmt::BreakStmt(node) }
}
impl From<ReturnStmt> for Stmt {
	fn from(node: ReturnStmt) -> Stmt { Stmt::ReturnStmt(node) }
}
impl From<WithStmt> for Stmt {
	fn from(node: WithStmt) -> Stmt { Stmt::WithStmt(node) }
}
impl From<LabelledStmt> for Stmt {
	fn from(node: LabelledStmt) -> Stmt { Stmt::LabelledStmt(node) }
}
impl From<SwitchStmt> for Stmt {
	fn from(node: SwitchStmt) -> Stmt { Stmt::SwitchStmt(node) }
}
impl From<ThrowStmt> for Stmt {
	fn from(node: ThrowStmt) -> Stmt { Stmt::ThrowStmt(node) }
}
impl From<TryStmt> for Stmt {
	fn from(node: TryStmt) -> Stmt { Stmt::TryStmt(node) }
}
impl From<DebuggerStmt> for Stmt {
	fn from(node: DebuggerStmt) -> Stmt { Stmt::DebuggerStmt(node) }
}
impl From<JsUnknownStatement> for Stmt {
	fn from(node: JsUnknownStatement) -> Stmt { Stmt::JsUnknownStatement(node) }
}
impl AstNode for Stmt {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			BLOCK_STMT | EMPTY_STMT | EXPR_STMT | IF_STMT | DO_WHILE_STMT | WHILE_STMT
			| FOR_STMT | FOR_IN_STMT | FOR_OF_STMT | CONTINUE_STMT | BREAK_STMT | RETURN_STMT
			| WITH_STMT | LABELLED_STMT | SWITCH_STMT | THROW_STMT | TRY_STMT | DEBUGGER_STMT
			| JS_UNKNOWN_STATEMENT => true,
			k if Decl::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			BLOCK_STMT => Stmt::BlockStmt(BlockStmt { syntax }),
			EMPTY_STMT => Stmt::EmptyStmt(EmptyStmt { syntax }),
			EXPR_STMT => Stmt::ExprStmt(ExprStmt { syntax }),
			IF_STMT => Stmt::IfStmt(IfStmt { syntax }),
			DO_WHILE_STMT => Stmt::DoWhileStmt(DoWhileStmt { syntax }),
			WHILE_STMT => Stmt::WhileStmt(WhileStmt { syntax }),
			FOR_STMT => Stmt::ForStmt(ForStmt { syntax }),
			FOR_IN_STMT => Stmt::ForInStmt(ForInStmt { syntax }),
			FOR_OF_STMT => Stmt::ForOfStmt(ForOfStmt { syntax }),
			CONTINUE_STMT => Stmt::ContinueStmt(ContinueStmt { syntax }),
			BREAK_STMT => Stmt::BreakStmt(BreakStmt { syntax }),
			RETURN_STMT => Stmt::ReturnStmt(ReturnStmt { syntax }),
			WITH_STMT => Stmt::WithStmt(WithStmt { syntax }),
			LABELLED_STMT => Stmt::LabelledStmt(LabelledStmt { syntax }),
			SWITCH_STMT => Stmt::SwitchStmt(SwitchStmt { syntax }),
			THROW_STMT => Stmt::ThrowStmt(ThrowStmt { syntax }),
			TRY_STMT => Stmt::TryStmt(TryStmt { syntax }),
			DEBUGGER_STMT => Stmt::DebuggerStmt(DebuggerStmt { syntax }),
			JS_UNKNOWN_STATEMENT => Stmt::JsUnknownStatement(JsUnknownStatement { syntax }),
			_ => {
				if let Some(decl) = Decl::cast(syntax.clone()) {
					return Some(Stmt::Decl(decl));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Stmt::BlockStmt(it) => &it.syntax,
			Stmt::EmptyStmt(it) => &it.syntax,
			Stmt::ExprStmt(it) => &it.syntax,
			Stmt::IfStmt(it) => &it.syntax,
			Stmt::DoWhileStmt(it) => &it.syntax,
			Stmt::WhileStmt(it) => &it.syntax,
			Stmt::ForStmt(it) => &it.syntax,
			Stmt::ForInStmt(it) => &it.syntax,
			Stmt::ForOfStmt(it) => &it.syntax,
			Stmt::ContinueStmt(it) => &it.syntax,
			Stmt::BreakStmt(it) => &it.syntax,
			Stmt::ReturnStmt(it) => &it.syntax,
			Stmt::WithStmt(it) => &it.syntax,
			Stmt::LabelledStmt(it) => &it.syntax,
			Stmt::SwitchStmt(it) => &it.syntax,
			Stmt::ThrowStmt(it) => &it.syntax,
			Stmt::TryStmt(it) => &it.syntax,
			Stmt::DebuggerStmt(it) => &it.syntax,
			Stmt::JsUnknownStatement(it) => &it.syntax,
			Stmt::Decl(it) => it.syntax(),
		}
	}
}
impl From<ImportDecl> for ModuleItem {
	fn from(node: ImportDecl) -> ModuleItem { ModuleItem::ImportDecl(node) }
}
impl From<ExportNamed> for ModuleItem {
	fn from(node: ExportNamed) -> ModuleItem { ModuleItem::ExportNamed(node) }
}
impl From<ExportDefaultDecl> for ModuleItem {
	fn from(node: ExportDefaultDecl) -> ModuleItem { ModuleItem::ExportDefaultDecl(node) }
}
impl From<ExportDefaultExpr> for ModuleItem {
	fn from(node: ExportDefaultExpr) -> ModuleItem { ModuleItem::ExportDefaultExpr(node) }
}
impl From<ExportWildcard> for ModuleItem {
	fn from(node: ExportWildcard) -> ModuleItem { ModuleItem::ExportWildcard(node) }
}
impl From<ExportDecl> for ModuleItem {
	fn from(node: ExportDecl) -> ModuleItem { ModuleItem::ExportDecl(node) }
}
impl From<TsImportEqualsDecl> for ModuleItem {
	fn from(node: TsImportEqualsDecl) -> ModuleItem { ModuleItem::TsImportEqualsDecl(node) }
}
impl From<TsExportAssignment> for ModuleItem {
	fn from(node: TsExportAssignment) -> ModuleItem { ModuleItem::TsExportAssignment(node) }
}
impl From<TsNamespaceExportDecl> for ModuleItem {
	fn from(node: TsNamespaceExportDecl) -> ModuleItem { ModuleItem::TsNamespaceExportDecl(node) }
}
impl AstNode for ModuleItem {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			IMPORT_DECL
			| EXPORT_NAMED
			| EXPORT_DEFAULT_DECL
			| EXPORT_DEFAULT_EXPR
			| EXPORT_WILDCARD
			| EXPORT_DECL
			| TS_IMPORT_EQUALS_DECL
			| TS_EXPORT_ASSIGNMENT
			| TS_NAMESPACE_EXPORT_DECL => true,
			k if Stmt::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			IMPORT_DECL => ModuleItem::ImportDecl(ImportDecl { syntax }),
			EXPORT_NAMED => ModuleItem::ExportNamed(ExportNamed { syntax }),
			EXPORT_DEFAULT_DECL => ModuleItem::ExportDefaultDecl(ExportDefaultDecl { syntax }),
			EXPORT_DEFAULT_EXPR => ModuleItem::ExportDefaultExpr(ExportDefaultExpr { syntax }),
			EXPORT_WILDCARD => ModuleItem::ExportWildcard(ExportWildcard { syntax }),
			EXPORT_DECL => ModuleItem::ExportDecl(ExportDecl { syntax }),
			TS_IMPORT_EQUALS_DECL => ModuleItem::TsImportEqualsDecl(TsImportEqualsDecl { syntax }),
			TS_EXPORT_ASSIGNMENT => ModuleItem::TsExportAssignment(TsExportAssignment { syntax }),
			TS_NAMESPACE_EXPORT_DECL => {
				ModuleItem::TsNamespaceExportDecl(TsNamespaceExportDecl { syntax })
			}
			_ => {
				if let Some(stmt) = Stmt::cast(syntax.clone()) {
					return Some(ModuleItem::Stmt(stmt));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ModuleItem::ImportDecl(it) => &it.syntax,
			ModuleItem::ExportNamed(it) => &it.syntax,
			ModuleItem::ExportDefaultDecl(it) => &it.syntax,
			ModuleItem::ExportDefaultExpr(it) => &it.syntax,
			ModuleItem::ExportWildcard(it) => &it.syntax,
			ModuleItem::ExportDecl(it) => &it.syntax,
			ModuleItem::TsImportEqualsDecl(it) => &it.syntax,
			ModuleItem::TsExportAssignment(it) => &it.syntax,
			ModuleItem::TsNamespaceExportDecl(it) => &it.syntax,
			ModuleItem::Stmt(it) => it.syntax(),
		}
	}
}
impl From<FnDecl> for Decl {
	fn from(node: FnDecl) -> Decl { Decl::FnDecl(node) }
}
impl From<ClassDecl> for Decl {
	fn from(node: ClassDecl) -> Decl { Decl::ClassDecl(node) }
}
impl From<VarDecl> for Decl {
	fn from(node: VarDecl) -> Decl { Decl::VarDecl(node) }
}
impl From<TsEnum> for Decl {
	fn from(node: TsEnum) -> Decl { Decl::TsEnum(node) }
}
impl From<TsTypeAliasDecl> for Decl {
	fn from(node: TsTypeAliasDecl) -> Decl { Decl::TsTypeAliasDecl(node) }
}
impl From<TsNamespaceDecl> for Decl {
	fn from(node: TsNamespaceDecl) -> Decl { Decl::TsNamespaceDecl(node) }
}
impl From<TsModuleDecl> for Decl {
	fn from(node: TsModuleDecl) -> Decl { Decl::TsModuleDecl(node) }
}
impl From<TsInterfaceDecl> for Decl {
	fn from(node: TsInterfaceDecl) -> Decl { Decl::TsInterfaceDecl(node) }
}
impl AstNode for Decl {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			FN_DECL | CLASS_DECL | VAR_DECL | TS_ENUM | TS_TYPE_ALIAS_DECL | TS_NAMESPACE_DECL
			| TS_MODULE_DECL | TS_INTERFACE_DECL => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			FN_DECL => Decl::FnDecl(FnDecl { syntax }),
			CLASS_DECL => Decl::ClassDecl(ClassDecl { syntax }),
			VAR_DECL => Decl::VarDecl(VarDecl { syntax }),
			TS_ENUM => Decl::TsEnum(TsEnum { syntax }),
			TS_TYPE_ALIAS_DECL => Decl::TsTypeAliasDecl(TsTypeAliasDecl { syntax }),
			TS_NAMESPACE_DECL => Decl::TsNamespaceDecl(TsNamespaceDecl { syntax }),
			TS_MODULE_DECL => Decl::TsModuleDecl(TsModuleDecl { syntax }),
			TS_INTERFACE_DECL => Decl::TsInterfaceDecl(TsInterfaceDecl { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Decl::FnDecl(it) => &it.syntax,
			Decl::ClassDecl(it) => &it.syntax,
			Decl::VarDecl(it) => &it.syntax,
			Decl::TsEnum(it) => &it.syntax,
			Decl::TsTypeAliasDecl(it) => &it.syntax,
			Decl::TsNamespaceDecl(it) => &it.syntax,
			Decl::TsModuleDecl(it) => &it.syntax,
			Decl::TsInterfaceDecl(it) => &it.syntax,
		}
	}
}
impl From<ArrowExpr> for Expr {
	fn from(node: ArrowExpr) -> Expr { Expr::ArrowExpr(node) }
}
impl From<Literal> for Expr {
	fn from(node: Literal) -> Expr { Expr::Literal(node) }
}
impl From<Template> for Expr {
	fn from(node: Template) -> Expr { Expr::Template(node) }
}
impl From<NameRef> for Expr {
	fn from(node: NameRef) -> Expr { Expr::NameRef(node) }
}
impl From<ThisExpr> for Expr {
	fn from(node: ThisExpr) -> Expr { Expr::ThisExpr(node) }
}
impl From<ArrayExpr> for Expr {
	fn from(node: ArrayExpr) -> Expr { Expr::ArrayExpr(node) }
}
impl From<ObjectExpr> for Expr {
	fn from(node: ObjectExpr) -> Expr { Expr::ObjectExpr(node) }
}
impl From<GroupingExpr> for Expr {
	fn from(node: GroupingExpr) -> Expr { Expr::GroupingExpr(node) }
}
impl From<BracketExpr> for Expr {
	fn from(node: BracketExpr) -> Expr { Expr::BracketExpr(node) }
}
impl From<DotExpr> for Expr {
	fn from(node: DotExpr) -> Expr { Expr::DotExpr(node) }
}
impl From<NewExpr> for Expr {
	fn from(node: NewExpr) -> Expr { Expr::NewExpr(node) }
}
impl From<CallExpr> for Expr {
	fn from(node: CallExpr) -> Expr { Expr::CallExpr(node) }
}
impl From<UnaryExpr> for Expr {
	fn from(node: UnaryExpr) -> Expr { Expr::UnaryExpr(node) }
}
impl From<PreUpdateExpression> for Expr {
	fn from(node: PreUpdateExpression) -> Expr { Expr::PreUpdateExpression(node) }
}
impl From<PostUpdateExpression> for Expr {
	fn from(node: PostUpdateExpression) -> Expr { Expr::PostUpdateExpression(node) }
}
impl From<BinExpr> for Expr {
	fn from(node: BinExpr) -> Expr { Expr::BinExpr(node) }
}
impl From<CondExpr> for Expr {
	fn from(node: CondExpr) -> Expr { Expr::CondExpr(node) }
}
impl From<AssignExpr> for Expr {
	fn from(node: AssignExpr) -> Expr { Expr::AssignExpr(node) }
}
impl From<SequenceExpr> for Expr {
	fn from(node: SequenceExpr) -> Expr { Expr::SequenceExpr(node) }
}
impl From<FnExpr> for Expr {
	fn from(node: FnExpr) -> Expr { Expr::FnExpr(node) }
}
impl From<ClassExpr> for Expr {
	fn from(node: ClassExpr) -> Expr { Expr::ClassExpr(node) }
}
impl From<NewTarget> for Expr {
	fn from(node: NewTarget) -> Expr { Expr::NewTarget(node) }
}
impl From<ImportMeta> for Expr {
	fn from(node: ImportMeta) -> Expr { Expr::ImportMeta(node) }
}
impl From<SuperCall> for Expr {
	fn from(node: SuperCall) -> Expr { Expr::SuperCall(node) }
}
impl From<ImportCall> for Expr {
	fn from(node: ImportCall) -> Expr { Expr::ImportCall(node) }
}
impl From<YieldExpr> for Expr {
	fn from(node: YieldExpr) -> Expr { Expr::YieldExpr(node) }
}
impl From<AwaitExpr> for Expr {
	fn from(node: AwaitExpr) -> Expr { Expr::AwaitExpr(node) }
}
impl From<PrivatePropAccess> for Expr {
	fn from(node: PrivatePropAccess) -> Expr { Expr::PrivatePropAccess(node) }
}
impl From<TsNonNull> for Expr {
	fn from(node: TsNonNull) -> Expr { Expr::TsNonNull(node) }
}
impl From<TsAssertion> for Expr {
	fn from(node: TsAssertion) -> Expr { Expr::TsAssertion(node) }
}
impl From<TsConstAssertion> for Expr {
	fn from(node: TsConstAssertion) -> Expr { Expr::TsConstAssertion(node) }
}
impl From<JsUnknownExpression> for Expr {
	fn from(node: JsUnknownExpression) -> Expr { Expr::JsUnknownExpression(node) }
}
impl AstNode for Expr {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			ARROW_EXPR
			| LITERAL
			| TEMPLATE
			| NAME_REF
			| THIS_EXPR
			| ARRAY_EXPR
			| OBJECT_EXPR
			| GROUPING_EXPR
			| BRACKET_EXPR
			| DOT_EXPR
			| NEW_EXPR
			| CALL_EXPR
			| UNARY_EXPR
			| PRE_UPDATE_EXPRESSION
			| POST_UPDATE_EXPRESSION
			| BIN_EXPR
			| COND_EXPR
			| ASSIGN_EXPR
			| SEQUENCE_EXPR
			| FN_EXPR
			| CLASS_EXPR
			| NEW_TARGET
			| IMPORT_META
			| SUPER_CALL
			| IMPORT_CALL
			| YIELD_EXPR
			| AWAIT_EXPR
			| PRIVATE_PROP_ACCESS
			| TS_NON_NULL
			| TS_ASSERTION
			| TS_CONST_ASSERTION
			| JS_UNKNOWN_EXPRESSION => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			ARROW_EXPR => Expr::ArrowExpr(ArrowExpr { syntax }),
			LITERAL => Expr::Literal(Literal { syntax }),
			TEMPLATE => Expr::Template(Template { syntax }),
			NAME_REF => Expr::NameRef(NameRef { syntax }),
			THIS_EXPR => Expr::ThisExpr(ThisExpr { syntax }),
			ARRAY_EXPR => Expr::ArrayExpr(ArrayExpr { syntax }),
			OBJECT_EXPR => Expr::ObjectExpr(ObjectExpr { syntax }),
			GROUPING_EXPR => Expr::GroupingExpr(GroupingExpr { syntax }),
			BRACKET_EXPR => Expr::BracketExpr(BracketExpr { syntax }),
			DOT_EXPR => Expr::DotExpr(DotExpr { syntax }),
			NEW_EXPR => Expr::NewExpr(NewExpr { syntax }),
			CALL_EXPR => Expr::CallExpr(CallExpr { syntax }),
			UNARY_EXPR => Expr::UnaryExpr(UnaryExpr { syntax }),
			PRE_UPDATE_EXPRESSION => Expr::PreUpdateExpression(PreUpdateExpression { syntax }),
			POST_UPDATE_EXPRESSION => Expr::PostUpdateExpression(PostUpdateExpression { syntax }),
			BIN_EXPR => Expr::BinExpr(BinExpr { syntax }),
			COND_EXPR => Expr::CondExpr(CondExpr { syntax }),
			ASSIGN_EXPR => Expr::AssignExpr(AssignExpr { syntax }),
			SEQUENCE_EXPR => Expr::SequenceExpr(SequenceExpr { syntax }),
			FN_EXPR => Expr::FnExpr(FnExpr { syntax }),
			CLASS_EXPR => Expr::ClassExpr(ClassExpr { syntax }),
			NEW_TARGET => Expr::NewTarget(NewTarget { syntax }),
			IMPORT_META => Expr::ImportMeta(ImportMeta { syntax }),
			SUPER_CALL => Expr::SuperCall(SuperCall { syntax }),
			IMPORT_CALL => Expr::ImportCall(ImportCall { syntax }),
			YIELD_EXPR => Expr::YieldExpr(YieldExpr { syntax }),
			AWAIT_EXPR => Expr::AwaitExpr(AwaitExpr { syntax }),
			PRIVATE_PROP_ACCESS => Expr::PrivatePropAccess(PrivatePropAccess { syntax }),
			TS_NON_NULL => Expr::TsNonNull(TsNonNull { syntax }),
			TS_ASSERTION => Expr::TsAssertion(TsAssertion { syntax }),
			TS_CONST_ASSERTION => Expr::TsConstAssertion(TsConstAssertion { syntax }),
			JS_UNKNOWN_EXPRESSION => Expr::JsUnknownExpression(JsUnknownExpression { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Expr::ArrowExpr(it) => &it.syntax,
			Expr::Literal(it) => &it.syntax,
			Expr::Template(it) => &it.syntax,
			Expr::NameRef(it) => &it.syntax,
			Expr::ThisExpr(it) => &it.syntax,
			Expr::ArrayExpr(it) => &it.syntax,
			Expr::ObjectExpr(it) => &it.syntax,
			Expr::GroupingExpr(it) => &it.syntax,
			Expr::BracketExpr(it) => &it.syntax,
			Expr::DotExpr(it) => &it.syntax,
			Expr::NewExpr(it) => &it.syntax,
			Expr::CallExpr(it) => &it.syntax,
			Expr::UnaryExpr(it) => &it.syntax,
			Expr::PreUpdateExpression(it) => &it.syntax,
			Expr::PostUpdateExpression(it) => &it.syntax,
			Expr::BinExpr(it) => &it.syntax,
			Expr::CondExpr(it) => &it.syntax,
			Expr::AssignExpr(it) => &it.syntax,
			Expr::SequenceExpr(it) => &it.syntax,
			Expr::FnExpr(it) => &it.syntax,
			Expr::ClassExpr(it) => &it.syntax,
			Expr::NewTarget(it) => &it.syntax,
			Expr::ImportMeta(it) => &it.syntax,
			Expr::SuperCall(it) => &it.syntax,
			Expr::ImportCall(it) => &it.syntax,
			Expr::YieldExpr(it) => &it.syntax,
			Expr::AwaitExpr(it) => &it.syntax,
			Expr::PrivatePropAccess(it) => &it.syntax,
			Expr::TsNonNull(it) => &it.syntax,
			Expr::TsAssertion(it) => &it.syntax,
			Expr::TsConstAssertion(it) => &it.syntax,
			Expr::JsUnknownExpression(it) => &it.syntax,
		}
	}
}
impl From<VarDecl> for ForHead {
	fn from(node: VarDecl) -> ForHead { ForHead::VarDecl(node) }
}
impl AstNode for ForHead {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			VAR_DECL => true,
			k if Expr::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			VAR_DECL => ForHead::VarDecl(VarDecl { syntax }),
			_ => {
				if let Some(expr) = Expr::cast(syntax.clone()) {
					return Some(ForHead::Expr(expr));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ForHead::VarDecl(it) => &it.syntax,
			ForHead::Expr(it) => it.syntax(),
		}
	}
}
impl From<CaseClause> for SwitchCase {
	fn from(node: CaseClause) -> SwitchCase { SwitchCase::CaseClause(node) }
}
impl From<DefaultClause> for SwitchCase {
	fn from(node: DefaultClause) -> SwitchCase { SwitchCase::DefaultClause(node) }
}
impl AstNode for SwitchCase {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			CASE_CLAUSE | DEFAULT_CLAUSE => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			CASE_CLAUSE => SwitchCase::CaseClause(CaseClause { syntax }),
			DEFAULT_CLAUSE => SwitchCase::DefaultClause(DefaultClause { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			SwitchCase::CaseClause(it) => &it.syntax,
			SwitchCase::DefaultClause(it) => &it.syntax,
		}
	}
}
impl From<SinglePattern> for Pattern {
	fn from(node: SinglePattern) -> Pattern { Pattern::SinglePattern(node) }
}
impl From<RestPattern> for Pattern {
	fn from(node: RestPattern) -> Pattern { Pattern::RestPattern(node) }
}
impl From<AssignPattern> for Pattern {
	fn from(node: AssignPattern) -> Pattern { Pattern::AssignPattern(node) }
}
impl From<ObjectPattern> for Pattern {
	fn from(node: ObjectPattern) -> Pattern { Pattern::ObjectPattern(node) }
}
impl From<ArrayPattern> for Pattern {
	fn from(node: ArrayPattern) -> Pattern { Pattern::ArrayPattern(node) }
}
impl From<ExprPattern> for Pattern {
	fn from(node: ExprPattern) -> Pattern { Pattern::ExprPattern(node) }
}
impl From<JsUnknownPattern> for Pattern {
	fn from(node: JsUnknownPattern) -> Pattern { Pattern::JsUnknownPattern(node) }
}
impl AstNode for Pattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SINGLE_PATTERN | REST_PATTERN | ASSIGN_PATTERN | OBJECT_PATTERN | ARRAY_PATTERN
			| EXPR_PATTERN | JS_UNKNOWN_PATTERN => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SINGLE_PATTERN => Pattern::SinglePattern(SinglePattern { syntax }),
			REST_PATTERN => Pattern::RestPattern(RestPattern { syntax }),
			ASSIGN_PATTERN => Pattern::AssignPattern(AssignPattern { syntax }),
			OBJECT_PATTERN => Pattern::ObjectPattern(ObjectPattern { syntax }),
			ARRAY_PATTERN => Pattern::ArrayPattern(ArrayPattern { syntax }),
			EXPR_PATTERN => Pattern::ExprPattern(ExprPattern { syntax }),
			JS_UNKNOWN_PATTERN => Pattern::JsUnknownPattern(JsUnknownPattern { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Pattern::SinglePattern(it) => &it.syntax,
			Pattern::RestPattern(it) => &it.syntax,
			Pattern::AssignPattern(it) => &it.syntax,
			Pattern::ObjectPattern(it) => &it.syntax,
			Pattern::ArrayPattern(it) => &it.syntax,
			Pattern::ExprPattern(it) => &it.syntax,
			Pattern::JsUnknownPattern(it) => &it.syntax,
		}
	}
}
impl From<TsAny> for TsType {
	fn from(node: TsAny) -> TsType { TsType::TsAny(node) }
}
impl From<TsUnknown> for TsType {
	fn from(node: TsUnknown) -> TsType { TsType::TsUnknown(node) }
}
impl From<TsNumber> for TsType {
	fn from(node: TsNumber) -> TsType { TsType::TsNumber(node) }
}
impl From<TsObject> for TsType {
	fn from(node: TsObject) -> TsType { TsType::TsObject(node) }
}
impl From<TsBoolean> for TsType {
	fn from(node: TsBoolean) -> TsType { TsType::TsBoolean(node) }
}
impl From<TsBigint> for TsType {
	fn from(node: TsBigint) -> TsType { TsType::TsBigint(node) }
}
impl From<TsString> for TsType {
	fn from(node: TsString) -> TsType { TsType::TsString(node) }
}
impl From<TsSymbol> for TsType {
	fn from(node: TsSymbol) -> TsType { TsType::TsSymbol(node) }
}
impl From<TsVoid> for TsType {
	fn from(node: TsVoid) -> TsType { TsType::TsVoid(node) }
}
impl From<TsUndefined> for TsType {
	fn from(node: TsUndefined) -> TsType { TsType::TsUndefined(node) }
}
impl From<TsNull> for TsType {
	fn from(node: TsNull) -> TsType { TsType::TsNull(node) }
}
impl From<TsNever> for TsType {
	fn from(node: TsNever) -> TsType { TsType::TsNever(node) }
}
impl From<TsThis> for TsType {
	fn from(node: TsThis) -> TsType { TsType::TsThis(node) }
}
impl From<TsLiteral> for TsType {
	fn from(node: TsLiteral) -> TsType { TsType::TsLiteral(node) }
}
impl From<TsPredicate> for TsType {
	fn from(node: TsPredicate) -> TsType { TsType::TsPredicate(node) }
}
impl From<TsTuple> for TsType {
	fn from(node: TsTuple) -> TsType { TsType::TsTuple(node) }
}
impl From<TsParen> for TsType {
	fn from(node: TsParen) -> TsType { TsType::TsParen(node) }
}
impl From<TsTypeRef> for TsType {
	fn from(node: TsTypeRef) -> TsType { TsType::TsTypeRef(node) }
}
impl From<TsTemplate> for TsType {
	fn from(node: TsTemplate) -> TsType { TsType::TsTemplate(node) }
}
impl From<TsMappedType> for TsType {
	fn from(node: TsMappedType) -> TsType { TsType::TsMappedType(node) }
}
impl From<TsImport> for TsType {
	fn from(node: TsImport) -> TsType { TsType::TsImport(node) }
}
impl From<TsArray> for TsType {
	fn from(node: TsArray) -> TsType { TsType::TsArray(node) }
}
impl From<TsIndexedArray> for TsType {
	fn from(node: TsIndexedArray) -> TsType { TsType::TsIndexedArray(node) }
}
impl From<TsTypeOperator> for TsType {
	fn from(node: TsTypeOperator) -> TsType { TsType::TsTypeOperator(node) }
}
impl From<TsIntersection> for TsType {
	fn from(node: TsIntersection) -> TsType { TsType::TsIntersection(node) }
}
impl From<TsUnion> for TsType {
	fn from(node: TsUnion) -> TsType { TsType::TsUnion(node) }
}
impl From<TsFnType> for TsType {
	fn from(node: TsFnType) -> TsType { TsType::TsFnType(node) }
}
impl From<TsConstructorType> for TsType {
	fn from(node: TsConstructorType) -> TsType { TsType::TsConstructorType(node) }
}
impl From<TsConditionalType> for TsType {
	fn from(node: TsConditionalType) -> TsType { TsType::TsConditionalType(node) }
}
impl From<TsObjectType> for TsType {
	fn from(node: TsObjectType) -> TsType { TsType::TsObjectType(node) }
}
impl From<TsInfer> for TsType {
	fn from(node: TsInfer) -> TsType { TsType::TsInfer(node) }
}
impl AstNode for TsType {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			TS_ANY | TS_UNKNOWN | TS_NUMBER | TS_OBJECT | TS_BOOLEAN | TS_BIGINT | TS_STRING
			| TS_SYMBOL | TS_VOID | TS_UNDEFINED | TS_NULL | TS_NEVER | TS_THIS | TS_LITERAL
			| TS_PREDICATE | TS_TUPLE | TS_PAREN | TS_TYPE_REF | TS_TEMPLATE | TS_MAPPED_TYPE
			| TS_IMPORT | TS_ARRAY | TS_INDEXED_ARRAY | TS_TYPE_OPERATOR | TS_INTERSECTION
			| TS_UNION | TS_FN_TYPE | TS_CONSTRUCTOR_TYPE | TS_CONDITIONAL_TYPE
			| TS_OBJECT_TYPE | TS_INFER => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_ANY => TsType::TsAny(TsAny { syntax }),
			TS_UNKNOWN => TsType::TsUnknown(TsUnknown { syntax }),
			TS_NUMBER => TsType::TsNumber(TsNumber { syntax }),
			TS_OBJECT => TsType::TsObject(TsObject { syntax }),
			TS_BOOLEAN => TsType::TsBoolean(TsBoolean { syntax }),
			TS_BIGINT => TsType::TsBigint(TsBigint { syntax }),
			TS_STRING => TsType::TsString(TsString { syntax }),
			TS_SYMBOL => TsType::TsSymbol(TsSymbol { syntax }),
			TS_VOID => TsType::TsVoid(TsVoid { syntax }),
			TS_UNDEFINED => TsType::TsUndefined(TsUndefined { syntax }),
			TS_NULL => TsType::TsNull(TsNull { syntax }),
			TS_NEVER => TsType::TsNever(TsNever { syntax }),
			TS_THIS => TsType::TsThis(TsThis { syntax }),
			TS_LITERAL => TsType::TsLiteral(TsLiteral { syntax }),
			TS_PREDICATE => TsType::TsPredicate(TsPredicate { syntax }),
			TS_TUPLE => TsType::TsTuple(TsTuple { syntax }),
			TS_PAREN => TsType::TsParen(TsParen { syntax }),
			TS_TYPE_REF => TsType::TsTypeRef(TsTypeRef { syntax }),
			TS_TEMPLATE => TsType::TsTemplate(TsTemplate { syntax }),
			TS_MAPPED_TYPE => TsType::TsMappedType(TsMappedType { syntax }),
			TS_IMPORT => TsType::TsImport(TsImport { syntax }),
			TS_ARRAY => TsType::TsArray(TsArray { syntax }),
			TS_INDEXED_ARRAY => TsType::TsIndexedArray(TsIndexedArray { syntax }),
			TS_TYPE_OPERATOR => TsType::TsTypeOperator(TsTypeOperator { syntax }),
			TS_INTERSECTION => TsType::TsIntersection(TsIntersection { syntax }),
			TS_UNION => TsType::TsUnion(TsUnion { syntax }),
			TS_FN_TYPE => TsType::TsFnType(TsFnType { syntax }),
			TS_CONSTRUCTOR_TYPE => TsType::TsConstructorType(TsConstructorType { syntax }),
			TS_CONDITIONAL_TYPE => TsType::TsConditionalType(TsConditionalType { syntax }),
			TS_OBJECT_TYPE => TsType::TsObjectType(TsObjectType { syntax }),
			TS_INFER => TsType::TsInfer(TsInfer { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsType::TsAny(it) => &it.syntax,
			TsType::TsUnknown(it) => &it.syntax,
			TsType::TsNumber(it) => &it.syntax,
			TsType::TsObject(it) => &it.syntax,
			TsType::TsBoolean(it) => &it.syntax,
			TsType::TsBigint(it) => &it.syntax,
			TsType::TsString(it) => &it.syntax,
			TsType::TsSymbol(it) => &it.syntax,
			TsType::TsVoid(it) => &it.syntax,
			TsType::TsUndefined(it) => &it.syntax,
			TsType::TsNull(it) => &it.syntax,
			TsType::TsNever(it) => &it.syntax,
			TsType::TsThis(it) => &it.syntax,
			TsType::TsLiteral(it) => &it.syntax,
			TsType::TsPredicate(it) => &it.syntax,
			TsType::TsTuple(it) => &it.syntax,
			TsType::TsParen(it) => &it.syntax,
			TsType::TsTypeRef(it) => &it.syntax,
			TsType::TsTemplate(it) => &it.syntax,
			TsType::TsMappedType(it) => &it.syntax,
			TsType::TsImport(it) => &it.syntax,
			TsType::TsArray(it) => &it.syntax,
			TsType::TsIndexedArray(it) => &it.syntax,
			TsType::TsTypeOperator(it) => &it.syntax,
			TsType::TsIntersection(it) => &it.syntax,
			TsType::TsUnion(it) => &it.syntax,
			TsType::TsFnType(it) => &it.syntax,
			TsType::TsConstructorType(it) => &it.syntax,
			TsType::TsConditionalType(it) => &it.syntax,
			TsType::TsObjectType(it) => &it.syntax,
			TsType::TsInfer(it) => &it.syntax,
		}
	}
}
impl From<Name> for ArrowExprParams {
	fn from(node: Name) -> ArrowExprParams { ArrowExprParams::Name(node) }
}
impl From<ParameterList> for ArrowExprParams {
	fn from(node: ParameterList) -> ArrowExprParams { ArrowExprParams::ParameterList(node) }
}
impl AstNode for ArrowExprParams {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			NAME | PARAMETER_LIST => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			NAME => ArrowExprParams::Name(Name { syntax }),
			PARAMETER_LIST => ArrowExprParams::ParameterList(ParameterList { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ArrowExprParams::Name(it) => &it.syntax,
			ArrowExprParams::ParameterList(it) => &it.syntax,
		}
	}
}
impl From<BlockStmt> for ExprOrBlock {
	fn from(node: BlockStmt) -> ExprOrBlock { ExprOrBlock::BlockStmt(node) }
}
impl AstNode for ExprOrBlock {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			BLOCK_STMT => true,
			k if Expr::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			BLOCK_STMT => ExprOrBlock::BlockStmt(BlockStmt { syntax }),
			_ => {
				if let Some(expr) = Expr::cast(syntax.clone()) {
					return Some(ExprOrBlock::Expr(expr));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ExprOrBlock::BlockStmt(it) => &it.syntax,
			ExprOrBlock::Expr(it) => it.syntax(),
		}
	}
}
impl From<LiteralProp> for ObjectProp {
	fn from(node: LiteralProp) -> ObjectProp { ObjectProp::LiteralProp(node) }
}
impl From<Getter> for ObjectProp {
	fn from(node: Getter) -> ObjectProp { ObjectProp::Getter(node) }
}
impl From<Setter> for ObjectProp {
	fn from(node: Setter) -> ObjectProp { ObjectProp::Setter(node) }
}
impl From<SpreadProp> for ObjectProp {
	fn from(node: SpreadProp) -> ObjectProp { ObjectProp::SpreadProp(node) }
}
impl From<InitializedProp> for ObjectProp {
	fn from(node: InitializedProp) -> ObjectProp { ObjectProp::InitializedProp(node) }
}
impl From<IdentProp> for ObjectProp {
	fn from(node: IdentProp) -> ObjectProp { ObjectProp::IdentProp(node) }
}
impl From<Method> for ObjectProp {
	fn from(node: Method) -> ObjectProp { ObjectProp::Method(node) }
}
impl From<JsUnknownMember> for ObjectProp {
	fn from(node: JsUnknownMember) -> ObjectProp { ObjectProp::JsUnknownMember(node) }
}
impl AstNode for ObjectProp {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			LITERAL_PROP | GETTER | SETTER | SPREAD_PROP | INITIALIZED_PROP | IDENT_PROP
			| METHOD | JS_UNKNOWN_MEMBER => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			LITERAL_PROP => ObjectProp::LiteralProp(LiteralProp { syntax }),
			GETTER => ObjectProp::Getter(Getter { syntax }),
			SETTER => ObjectProp::Setter(Setter { syntax }),
			SPREAD_PROP => ObjectProp::SpreadProp(SpreadProp { syntax }),
			INITIALIZED_PROP => ObjectProp::InitializedProp(InitializedProp { syntax }),
			IDENT_PROP => ObjectProp::IdentProp(IdentProp { syntax }),
			METHOD => ObjectProp::Method(Method { syntax }),
			JS_UNKNOWN_MEMBER => ObjectProp::JsUnknownMember(JsUnknownMember { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ObjectProp::LiteralProp(it) => &it.syntax,
			ObjectProp::Getter(it) => &it.syntax,
			ObjectProp::Setter(it) => &it.syntax,
			ObjectProp::SpreadProp(it) => &it.syntax,
			ObjectProp::InitializedProp(it) => &it.syntax,
			ObjectProp::IdentProp(it) => &it.syntax,
			ObjectProp::Method(it) => &it.syntax,
			ObjectProp::JsUnknownMember(it) => &it.syntax,
		}
	}
}
impl From<EmptyStmt> for ClassElement {
	fn from(node: EmptyStmt) -> ClassElement { ClassElement::EmptyStmt(node) }
}
impl From<Method> for ClassElement {
	fn from(node: Method) -> ClassElement { ClassElement::Method(node) }
}
impl From<PrivateProp> for ClassElement {
	fn from(node: PrivateProp) -> ClassElement { ClassElement::PrivateProp(node) }
}
impl From<ClassProp> for ClassElement {
	fn from(node: ClassProp) -> ClassElement { ClassElement::ClassProp(node) }
}
impl From<Constructor> for ClassElement {
	fn from(node: Constructor) -> ClassElement { ClassElement::Constructor(node) }
}
impl From<TsIndexSignature> for ClassElement {
	fn from(node: TsIndexSignature) -> ClassElement { ClassElement::TsIndexSignature(node) }
}
impl From<Getter> for ClassElement {
	fn from(node: Getter) -> ClassElement { ClassElement::Getter(node) }
}
impl From<Setter> for ClassElement {
	fn from(node: Setter) -> ClassElement { ClassElement::Setter(node) }
}
impl From<JsUnknownMember> for ClassElement {
	fn from(node: JsUnknownMember) -> ClassElement { ClassElement::JsUnknownMember(node) }
}
impl AstNode for ClassElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			EMPTY_STMT | METHOD | PRIVATE_PROP | CLASS_PROP | CONSTRUCTOR | TS_INDEX_SIGNATURE
			| GETTER | SETTER | JS_UNKNOWN_MEMBER => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			EMPTY_STMT => ClassElement::EmptyStmt(EmptyStmt { syntax }),
			METHOD => ClassElement::Method(Method { syntax }),
			PRIVATE_PROP => ClassElement::PrivateProp(PrivateProp { syntax }),
			CLASS_PROP => ClassElement::ClassProp(ClassProp { syntax }),
			CONSTRUCTOR => ClassElement::Constructor(Constructor { syntax }),
			TS_INDEX_SIGNATURE => ClassElement::TsIndexSignature(TsIndexSignature { syntax }),
			GETTER => ClassElement::Getter(Getter { syntax }),
			SETTER => ClassElement::Setter(Setter { syntax }),
			JS_UNKNOWN_MEMBER => ClassElement::JsUnknownMember(JsUnknownMember { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ClassElement::EmptyStmt(it) => &it.syntax,
			ClassElement::Method(it) => &it.syntax,
			ClassElement::PrivateProp(it) => &it.syntax,
			ClassElement::ClassProp(it) => &it.syntax,
			ClassElement::Constructor(it) => &it.syntax,
			ClassElement::TsIndexSignature(it) => &it.syntax,
			ClassElement::Getter(it) => &it.syntax,
			ClassElement::Setter(it) => &it.syntax,
			ClassElement::JsUnknownMember(it) => &it.syntax,
		}
	}
}
impl From<ComputedPropertyName> for PropName {
	fn from(node: ComputedPropertyName) -> PropName { PropName::ComputedPropertyName(node) }
}
impl From<Literal> for PropName {
	fn from(node: Literal) -> PropName { PropName::Literal(node) }
}
impl From<Ident> for PropName {
	fn from(node: Ident) -> PropName { PropName::Ident(node) }
}
impl From<Name> for PropName {
	fn from(node: Name) -> PropName { PropName::Name(node) }
}
impl From<JsUnknownBinding> for PropName {
	fn from(node: JsUnknownBinding) -> PropName { PropName::JsUnknownBinding(node) }
}
impl AstNode for PropName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			COMPUTED_PROPERTY_NAME | LITERAL | IDENT | NAME | JS_UNKNOWN_BINDING => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			COMPUTED_PROPERTY_NAME => {
				PropName::ComputedPropertyName(ComputedPropertyName { syntax })
			}
			LITERAL => PropName::Literal(Literal { syntax }),
			IDENT => PropName::Ident(Ident { syntax }),
			NAME => PropName::Name(Name { syntax }),
			JS_UNKNOWN_BINDING => PropName::JsUnknownBinding(JsUnknownBinding { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			PropName::ComputedPropertyName(it) => &it.syntax,
			PropName::Literal(it) => &it.syntax,
			PropName::Ident(it) => &it.syntax,
			PropName::Name(it) => &it.syntax,
			PropName::JsUnknownBinding(it) => &it.syntax,
		}
	}
}
impl From<TsConstructorParam> for ConstructorParamOrPat {
	fn from(node: TsConstructorParam) -> ConstructorParamOrPat {
		ConstructorParamOrPat::TsConstructorParam(node)
	}
}
impl AstNode for ConstructorParamOrPat {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			TS_CONSTRUCTOR_PARAM => true,
			k if Pattern::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_CONSTRUCTOR_PARAM => {
				ConstructorParamOrPat::TsConstructorParam(TsConstructorParam { syntax })
			}
			_ => {
				if let Some(pattern) = Pattern::cast(syntax.clone()) {
					return Some(ConstructorParamOrPat::Pattern(pattern));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ConstructorParamOrPat::TsConstructorParam(it) => &it.syntax,
			ConstructorParamOrPat::Pattern(it) => it.syntax(),
		}
	}
}
impl From<SpreadElement> for ExprOrSpread {
	fn from(node: SpreadElement) -> ExprOrSpread { ExprOrSpread::SpreadElement(node) }
}
impl From<GroupingExpr> for ExprOrSpread {
	fn from(node: GroupingExpr) -> ExprOrSpread { ExprOrSpread::GroupingExpr(node) }
}
impl AstNode for ExprOrSpread {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			SPREAD_ELEMENT | GROUPING_EXPR => true,
			k if Expr::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SPREAD_ELEMENT => ExprOrSpread::SpreadElement(SpreadElement { syntax }),
			GROUPING_EXPR => ExprOrSpread::GroupingExpr(GroupingExpr { syntax }),
			_ => {
				if let Some(expr) = Expr::cast(syntax.clone()) {
					return Some(ExprOrSpread::Expr(expr));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ExprOrSpread::SpreadElement(it) => &it.syntax,
			ExprOrSpread::GroupingExpr(it) => &it.syntax,
			ExprOrSpread::Expr(it) => it.syntax(),
		}
	}
}
impl AstNode for PatternOrExpr {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			k if Pattern::can_cast(k) => true,
			k if Expr::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if let Some(pattern) = Pattern::cast(syntax.clone()) {
			return Some(PatternOrExpr::Pattern(pattern));
		}
		if let Some(expr) = Expr::cast(syntax.clone()) {
			return Some(PatternOrExpr::Expr(expr));
		}
		None
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			PatternOrExpr::Pattern(it) => it.syntax(),
			PatternOrExpr::Expr(it) => it.syntax(),
		}
	}
}
impl From<AssignPattern> for ObjectPatternProp {
	fn from(node: AssignPattern) -> ObjectPatternProp { ObjectPatternProp::AssignPattern(node) }
}
impl From<KeyValuePattern> for ObjectPatternProp {
	fn from(node: KeyValuePattern) -> ObjectPatternProp { ObjectPatternProp::KeyValuePattern(node) }
}
impl From<RestPattern> for ObjectPatternProp {
	fn from(node: RestPattern) -> ObjectPatternProp { ObjectPatternProp::RestPattern(node) }
}
impl From<SinglePattern> for ObjectPatternProp {
	fn from(node: SinglePattern) -> ObjectPatternProp { ObjectPatternProp::SinglePattern(node) }
}
impl From<JsUnknownPattern> for ObjectPatternProp {
	fn from(node: JsUnknownPattern) -> ObjectPatternProp {
		ObjectPatternProp::JsUnknownPattern(node)
	}
}
impl AstNode for ObjectPatternProp {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			ASSIGN_PATTERN | KEY_VALUE_PATTERN | REST_PATTERN | SINGLE_PATTERN
			| JS_UNKNOWN_PATTERN => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			ASSIGN_PATTERN => ObjectPatternProp::AssignPattern(AssignPattern { syntax }),
			KEY_VALUE_PATTERN => ObjectPatternProp::KeyValuePattern(KeyValuePattern { syntax }),
			REST_PATTERN => ObjectPatternProp::RestPattern(RestPattern { syntax }),
			SINGLE_PATTERN => ObjectPatternProp::SinglePattern(SinglePattern { syntax }),
			JS_UNKNOWN_PATTERN => ObjectPatternProp::JsUnknownPattern(JsUnknownPattern { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ObjectPatternProp::AssignPattern(it) => &it.syntax,
			ObjectPatternProp::KeyValuePattern(it) => &it.syntax,
			ObjectPatternProp::RestPattern(it) => &it.syntax,
			ObjectPatternProp::SinglePattern(it) => &it.syntax,
			ObjectPatternProp::JsUnknownPattern(it) => &it.syntax,
		}
	}
}
impl From<WildcardImport> for ImportClause {
	fn from(node: WildcardImport) -> ImportClause { ImportClause::WildcardImport(node) }
}
impl From<NamedImports> for ImportClause {
	fn from(node: NamedImports) -> ImportClause { ImportClause::NamedImports(node) }
}
impl From<Name> for ImportClause {
	fn from(node: Name) -> ImportClause { ImportClause::Name(node) }
}
impl From<ImportStringSpecifier> for ImportClause {
	fn from(node: ImportStringSpecifier) -> ImportClause {
		ImportClause::ImportStringSpecifier(node)
	}
}
impl AstNode for ImportClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			WILDCARD_IMPORT | NAMED_IMPORTS | NAME | IMPORT_STRING_SPECIFIER => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			WILDCARD_IMPORT => ImportClause::WildcardImport(WildcardImport { syntax }),
			NAMED_IMPORTS => ImportClause::NamedImports(NamedImports { syntax }),
			NAME => ImportClause::Name(Name { syntax }),
			IMPORT_STRING_SPECIFIER => {
				ImportClause::ImportStringSpecifier(ImportStringSpecifier { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ImportClause::WildcardImport(it) => &it.syntax,
			ImportClause::NamedImports(it) => &it.syntax,
			ImportClause::Name(it) => &it.syntax,
			ImportClause::ImportStringSpecifier(it) => &it.syntax,
		}
	}
}
impl From<FnDecl> for DefaultDecl {
	fn from(node: FnDecl) -> DefaultDecl { DefaultDecl::FnDecl(node) }
}
impl From<ClassDecl> for DefaultDecl {
	fn from(node: ClassDecl) -> DefaultDecl { DefaultDecl::ClassDecl(node) }
}
impl AstNode for DefaultDecl {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			FN_DECL | CLASS_DECL => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			FN_DECL => DefaultDecl::FnDecl(FnDecl { syntax }),
			CLASS_DECL => DefaultDecl::ClassDecl(ClassDecl { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			DefaultDecl::FnDecl(it) => &it.syntax,
			DefaultDecl::ClassDecl(it) => &it.syntax,
		}
	}
}
impl From<TsExternalModuleRef> for TsModuleRef {
	fn from(node: TsExternalModuleRef) -> TsModuleRef { TsModuleRef::TsExternalModuleRef(node) }
}
impl AstNode for TsModuleRef {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			TS_EXTERNAL_MODULE_REF => true,
			k if TsEntityName::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_EXTERNAL_MODULE_REF => {
				TsModuleRef::TsExternalModuleRef(TsExternalModuleRef { syntax })
			}
			_ => {
				if let Some(ts_entity_name) = TsEntityName::cast(syntax.clone()) {
					return Some(TsModuleRef::TsEntityName(ts_entity_name));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsModuleRef::TsExternalModuleRef(it) => &it.syntax,
			TsModuleRef::TsEntityName(it) => it.syntax(),
		}
	}
}
impl From<TsTypeName> for TsEntityName {
	fn from(node: TsTypeName) -> TsEntityName { TsEntityName::TsTypeName(node) }
}
impl From<TsQualifiedPath> for TsEntityName {
	fn from(node: TsQualifiedPath) -> TsEntityName { TsEntityName::TsQualifiedPath(node) }
}
impl AstNode for TsEntityName {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			TS_TYPE_NAME | TS_QUALIFIED_PATH => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_TYPE_NAME => TsEntityName::TsTypeName(TsTypeName { syntax }),
			TS_QUALIFIED_PATH => TsEntityName::TsQualifiedPath(TsQualifiedPath { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsEntityName::TsTypeName(it) => &it.syntax,
			TsEntityName::TsQualifiedPath(it) => &it.syntax,
		}
	}
}
impl From<TsThis> for TsThisOrMore {
	fn from(node: TsThis) -> TsThisOrMore { TsThisOrMore::TsThis(node) }
}
impl From<TsTypeName> for TsThisOrMore {
	fn from(node: TsTypeName) -> TsThisOrMore { TsThisOrMore::TsTypeName(node) }
}
impl AstNode for TsThisOrMore {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			TS_THIS | TS_TYPE_NAME => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_THIS => TsThisOrMore::TsThis(TsThis { syntax }),
			TS_TYPE_NAME => TsThisOrMore::TsTypeName(TsTypeName { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsThisOrMore::TsThis(it) => &it.syntax,
			TsThisOrMore::TsTypeName(it) => &it.syntax,
		}
	}
}
impl From<TsCallSignatureDecl> for TsTypeElement {
	fn from(node: TsCallSignatureDecl) -> TsTypeElement { TsTypeElement::TsCallSignatureDecl(node) }
}
impl From<TsConstructSignatureDecl> for TsTypeElement {
	fn from(node: TsConstructSignatureDecl) -> TsTypeElement {
		TsTypeElement::TsConstructSignatureDecl(node)
	}
}
impl From<TsPropertySignature> for TsTypeElement {
	fn from(node: TsPropertySignature) -> TsTypeElement { TsTypeElement::TsPropertySignature(node) }
}
impl From<TsMethodSignature> for TsTypeElement {
	fn from(node: TsMethodSignature) -> TsTypeElement { TsTypeElement::TsMethodSignature(node) }
}
impl From<TsIndexSignature> for TsTypeElement {
	fn from(node: TsIndexSignature) -> TsTypeElement { TsTypeElement::TsIndexSignature(node) }
}
impl AstNode for TsTypeElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			TS_CALL_SIGNATURE_DECL
			| TS_CONSTRUCT_SIGNATURE_DECL
			| TS_PROPERTY_SIGNATURE
			| TS_METHOD_SIGNATURE
			| TS_INDEX_SIGNATURE => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_CALL_SIGNATURE_DECL => {
				TsTypeElement::TsCallSignatureDecl(TsCallSignatureDecl { syntax })
			}
			TS_CONSTRUCT_SIGNATURE_DECL => {
				TsTypeElement::TsConstructSignatureDecl(TsConstructSignatureDecl { syntax })
			}
			TS_PROPERTY_SIGNATURE => {
				TsTypeElement::TsPropertySignature(TsPropertySignature { syntax })
			}
			TS_METHOD_SIGNATURE => TsTypeElement::TsMethodSignature(TsMethodSignature { syntax }),
			TS_INDEX_SIGNATURE => TsTypeElement::TsIndexSignature(TsIndexSignature { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsTypeElement::TsCallSignatureDecl(it) => &it.syntax,
			TsTypeElement::TsConstructSignatureDecl(it) => &it.syntax,
			TsTypeElement::TsPropertySignature(it) => &it.syntax,
			TsTypeElement::TsMethodSignature(it) => &it.syntax,
			TsTypeElement::TsIndexSignature(it) => &it.syntax,
		}
	}
}
impl From<TsModuleBlock> for TsNamespaceBody {
	fn from(node: TsModuleBlock) -> TsNamespaceBody { TsNamespaceBody::TsModuleBlock(node) }
}
impl From<TsNamespaceDecl> for TsNamespaceBody {
	fn from(node: TsNamespaceDecl) -> TsNamespaceBody { TsNamespaceBody::TsNamespaceDecl(node) }
}
impl AstNode for TsNamespaceBody {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			TS_MODULE_BLOCK | TS_NAMESPACE_DECL => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_MODULE_BLOCK => TsNamespaceBody::TsModuleBlock(TsModuleBlock { syntax }),
			TS_NAMESPACE_DECL => TsNamespaceBody::TsNamespaceDecl(TsNamespaceDecl { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsNamespaceBody::TsModuleBlock(it) => &it.syntax,
			TsNamespaceBody::TsNamespaceDecl(it) => &it.syntax,
		}
	}
}
impl std::fmt::Display for Stmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ModuleItem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Decl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForHead {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SwitchCase {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Pattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrowExprParams {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExprOrBlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ObjectProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PropName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ConstructorParamOrPat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExprOrSpread {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PatternOrExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ObjectPatternProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DefaultDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsModuleRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsEntityName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsThisOrMore {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNamespaceBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownAssignmentTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Ident {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Script {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Module {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BlockStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EmptyStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExprStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for IfStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DoWhileStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for WhileStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForInStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForOfStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ContinueStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BreakStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ReturnStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for WithStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LabelledStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SwitchStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ThrowStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TryStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DebuggerStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Condition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmtInit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmtTest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmtUpdate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for VarDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NameRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CaseClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DefaultClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CatchClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Finalizer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrowExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Template {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ThisExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrayExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ObjectExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GroupingExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BracketExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DotExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NewExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CallExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for UnaryExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PreUpdateExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PostUpdateExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BinExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CondExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AssignExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SequenceExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FnExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NewTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportMeta {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SuperCall {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportCall {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for YieldExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AwaitExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PrivatePropAccess {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNonNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeArgs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArgList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeParams {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ParameterList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExprWithTypeArgs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Method {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PrivateProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Constructor {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsIndexSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Getter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Setter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsAccessibility {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ConstructorParameters {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstructorParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SpreadElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Null {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Undefined {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SinglePattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RestPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AssignPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ObjectPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrayPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExprPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for KeyValuePattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LiteralProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SpreadProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for InitializedProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for IdentProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ComputedPropertyName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PrivateName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FnDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsEnum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeAliasDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNamespaceDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsModuleDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsInterfaceDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Declarator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportNamed {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportDefaultDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportDefaultExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportWildcard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsImportEqualsDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExportAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNamespaceExportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for WildcardImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NamedImports {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportStringSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Specifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExternalModuleRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsAny {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsUnknown {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsBoolean {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsBigint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsSymbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsVoid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsUndefined {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNever {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsThis {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsPredicate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTuple {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsParen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTemplate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMappedType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsIndexedArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsIntersection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsFnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstructorType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConditionalType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsObjectType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsInfer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTupleElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsEnumMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTemplateElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMappedTypeReadonly {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMappedTypeParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExtends {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsModuleBlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstraint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsCallSignatureDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstructSignatureDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsPropertySignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMethodSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsQualifiedPath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
