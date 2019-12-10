// TODO:
//   [x] Port all tests
//   [x] NaN, inf, -inf are valid.  problem?  no because my parser thinks they're vars.
//   [x] e() pi() ... or should i prefer variables?  Provide a default layer of variables?  Vars don't work well with TV symbols.
//   [x] Profile, boost critical sections.
//   [x] optimize the peek/read process -- be able to read N bytes if we peek successfully.
//   [x] optimize after parse
//   [x] custom functions  (i.e. Variables With Arguments)
//   [x] REPL Example with Variables
//   [x] Copy smart tests from other libs.
//   [x] Reduce work: Parser obj --> functions.  EvalNS --> BTreeMap.
//   [x] #[inline] last, using profile as a guide.
//   [ ] More examples:  UnsafeVar.  Mini Language.
//   [ ] Readme
//   [ ] Documentation
//
//   [ ] sprintf

//#![warn(missing_docs)]


#[macro_export]
macro_rules! eval_instr {
    ($evaler:ident, $slab_ref:expr, $ns_mut:expr) => {
        if let al::IConst(c) = $evaler {
            c
        } else {
            #[cfg(feature="unsafe-vars")]
            {
                if let al::IUnsafeVar{name:_,ptr} = $evaler {
                    unsafe { *ptr }
                } else {
                    $evaler.eval($slab_ref, $ns_mut)?
                }
            }

            #[cfg(not(feature="unsafe-vars"))]
            $evaler.eval($slab_ref, $ns_mut)?
        }
    };
    ($evaler:expr, $slab_ref:expr, $ns_mut:expr) => {
        {
            let evaler = $evaler;
            eval_instr!(evaler, $slab_ref, $ns_mut)
        }
    };
}

#[macro_export]
macro_rules! eval_instr_ref {
    ($evaler:ident, $slab_ref:expr, $ns_mut:expr) => {
        if let al::IConst(c) = $evaler {
            *c
        } else {
            #[cfg(feature="unsafe-vars")]
            {
                if let al::IUnsafeVar{name:_,ptr} = $evaler {
                    unsafe { **ptr }
                } else {
                    $evaler.eval($slab_ref, $ns_mut)?
                }
            }

            #[cfg(not(feature="unsafe-vars"))]
            $evaler.eval($slab_ref, $ns_mut)?
        }
    };
    ($evaler:expr, $slab_ref:expr, $ns_mut:expr) => {
        {
            let evaler = $evaler;
            eval_instr_ref!(evaler, $slab_ref, $ns_mut)
        }
    };
}

#[macro_export]
macro_rules! eval_instr_ref_or_panic {
    ($evaler:ident, $slab_ref:expr, $ns_mut:expr) => {
        if let al::IConst(c) = $evaler {
            *c
        } else {
            #[cfg(feature="unsafe-vars")]
            {
                if let al::IUnsafeVar{name:_,ptr} = $evaler {
                    unsafe { **ptr }
                } else {
                    $evaler.eval($slab_ref, $ns_mut).unwrap()
                }
            }

            #[cfg(not(feature="unsafe-vars"))]
            $evaler.eval($slab_ref, $ns_mut).unwrap()
        }
    };
    ($evaler:expr, $slab_ref:expr, $ns_mut:expr) => {
        {
            let evaler = $evaler;
            eval_instr_ref_or_panic!(evaler, $slab_ref, $ns_mut)
        }
    };
}

pub mod parser;
pub mod compiler;
pub mod evaler;
pub mod slab;
pub mod evalns;
pub mod ez;

pub use self::parser::{parse, Parser, Expression, ExpressionI, Value, ValueI};
pub use self::compiler::{Compiler, Instruction::{self, IConst}, InstructionI};
#[cfg(feature="unsafe-vars")]
pub use self::compiler::Instruction::IUnsafeVar;
pub use self::evaler::Evaler;
pub use self::slab::Slab;
pub use self::evalns::{EvalNamespace, EmptyNamespace, FlatNamespace, ScopedNamespace};
pub use self::ez::ez_eval;

