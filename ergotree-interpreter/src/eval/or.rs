use ergotree_ir::mir::constant::TryExtractInto;
use ergotree_ir::mir::or::Or;
use ergotree_ir::mir::value::Value;

use crate::eval::env::Env;
use crate::eval::EvalContext;
use crate::eval::EvalError;
use crate::eval::Evaluable;

impl Evaluable for Or {
    fn eval(&self, env: &Env, ctx: &mut EvalContext) -> Result<Value, EvalError> {
        let input_v = self.input.eval(env, ctx)?;
        let input_v_bools = input_v.try_extract_into::<Vec<bool>>()?;
        Ok(input_v_bools.iter().any(|b| *b).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::context::Context;
    use crate::eval::tests::eval_out;
    use ergotree_ir::mir::expr::Expr;
    use proptest::collection;
    use proptest::prelude::*;
    use std::rc::Rc;
    use test_util::force_any_val;

    proptest! {

        #[test]
        fn eval(bools in collection::vec(any::<bool>(), 0..10)) {
            let expr: Expr = Or {input: Expr::Const(bools.clone().into()).into()}.into();
            let ctx = Rc::new(force_any_val::<Context>());
            let res = eval_out::<bool>(&expr, ctx);
            prop_assert_eq!(res, bools.iter().any(|b| *b));
        }
    }
}
