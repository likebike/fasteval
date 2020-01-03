//! A `Slab` is a pre-allocated block of memory, used during the
//! parse/compile/eval phases to reduce memory allocation/deallocation.
//!
//! A `Slab` enables you to perform one single, large allocation at the
//! beginning of the parse-compile-eval process, rather than many small
//! allocations.  You can also re-use a `Slab` for multiple expression
//! parse-compile-eval cycles, greatly reducing the amount of memory
//! operations.  The `Slab` is the main key to `fasteval`'s excellent
//! performance.
//!
//! You use `ExpressionI`, `ValueI`, and `InstructionI` index types to refer to
//! elements within the `Slab`.  These special index types are necessary to
//! side-step the Rust borrow checker, which is not able to understand
//! borrow-splitting of contiguous allocations (like arrays).
//! (In other words, these special index types allows `fasteval` to mutate a
//! `Slab` while simultaneously holding references to its contents.)
//!
//! You usually won't use any of the `Slab` method directly.  Instead, you'll
//! just pass a reference to other functions like [`parse()`](../parser/index.html),
//! [`compile()`](../compiler/trait.Compiler.html) and [`eval()`](../evaler/trait.Evaler.html).
//! We treat a `Slab` sort of like a Context in other programming systems.
//!
//! The `Slab` contains two fields: `ps` ("Parse Slab") and `cs`
//! ("Compile Slab").  It is structured like this because of Rust's borrowing
//! rules, so that the two fields can be borrowed and mutated independently.
//!
//! If you use the [`ez_eval()`](../ez/fn.ez_eval.html) function, it allocates
//! a Slab for you.  If you are performing the parse/compile/eval process
//! yourself, then you'll need to allocate a Slab at the beginning.
//!
//! # Examples
//!
//! Here is an example of re-using one `Slab` for multiple parse/eval cycles:
//! ```
//! use fasteval::Evaler;  // import this trait so we can call eval().
//! fn main() -> Result<(), fasteval::Error> {
//!     let mut slab = fasteval::Slab::new();
//!
//!     let val = fasteval::parse("1+2*3-4", &mut slab.ps)?.from(&slab.ps).eval(&slab, &mut fasteval::EmptyNamespace)?;
//!     assert_eq!(val, 3.0);
//!
//!     // Let's re-use the same slab again to save memory operations.
//!
//!     // `parse()` will clear the Slab's data.  It is important that you
//!     // do not use an old expression after the Slab has been cleared.
//!     let val = fasteval::parse("5+6*7-8", &mut slab.ps)?.from(&slab.ps).eval(&slab, &mut fasteval::EmptyNamespace)?;
//!     assert_eq!(val, 39.0);
//!
//!     Ok(())
//! }
//! ```

use crate::error::Error;
use crate::parser::{ExpressionI, ValueI,
                    Expression,  Value};
use crate::compiler::{Instruction::{self, IConst}, InstructionI};

use std::fmt;
use std::mem;

#[cfg(feature="unsafe-vars")]
use std::collections::BTreeMap;


// Eliminate function call overhead:
macro_rules! get_expr {
    ($pslab:expr, $i_ref:ident) => {
        match $pslab.exprs.get($i_ref.0) {
            Some(expr_ref) => expr_ref,
            None => &$pslab.def_expr,
        }
    };
}
macro_rules! get_val {
    ($pslab:expr, $i_ref:ident) => {
        match $pslab.vals.get($i_ref.0) {
            Some(val_ref) => val_ref,
            None => &$pslab.def_val,
        }
    };
}
// The CompileSlab::get_instr method is in the hot path of compiled evaluation:
macro_rules! get_instr {
    ($cslab:expr, $i_ref:ident) => {
        match $cslab.instrs.get($i_ref.0) {
            Some(instr_ref) => instr_ref,
            None => &$cslab.def_instr,
        }
    };
}


impl ExpressionI {
    /// Gets an Expression reference from the ParseSlab.
    ///
    /// This is actually just a convenience function built on top of
    /// `ParseSlab.get_expr`, but it enables you to perform the entire
    /// parse/compile/eval process in one line without upsetting the Rust
    /// borrow checker.  (If you didn't have this function, the borrow checker
    /// would force you to split the process into at least two lines.)
    ///
    #[inline]
    pub fn from(self, ps:&ParseSlab) -> &Expression {
        get_expr!(ps,self)
    }
}
impl ValueI {
    /// Gets a Value reference from the ParseSlab.
    ///
    /// See the comments on [ExpressionI::from](struct.ExpressionI.html#method.from).
    ///
    #[inline]
    pub fn from(self, ps:&ParseSlab) -> &Value {
        get_val!(ps,self)
    }
}

/// [See the `slab module` documentation.](index.html)
pub struct Slab {
    pub ps:ParseSlab,
    pub cs:CompileSlab,
}

/// `ParseSlab` is where `parse()` results are stored, located at `Slab.ps`.
///
/// # Unsafe Variable Registration with `add_unsafe_var()`
///
/// (This is documented here because the
/// [`add_unsafe_var()`](#method.add_unsafe_var) method and its documentation
/// only appears if `fasteval` is built with the `unsafe-vars` feature (`cargo
/// build --features unsafe-vars`).  I want this documentation to appear
/// regardless of the build mode, so I'm putting it here.)
///
/// Here is the function signature of the `add_unsafe_var()` method:
///
/// ```text
/// pub unsafe fn add_unsafe_var(&mut self, name: String, ptr: &f64)
/// ```
///
/// If you are using [Unsafe Variables](../index.html#unsafe-variables), you
/// need to pre-register the unsafe variable names and pointers *before*
/// calling `parse()`.  This is because Unsafe Variables are represented
/// specially in the parse AST; therefore, `parse()` needs to know what
/// variables are unsafe and which ones are normal so that it can produce the
/// correct AST.
///
/// If you forget to pre-register an unsafe variable before `parse()`, the
/// variable will be treated like a Normal Variable, and you'll probably get an
/// [`Undefined`](../error/enum.Error.html#variant.Undefined) error during evaluation.
///
/// ## Safety
///
/// You must guarantee that Unsafe Variable pointers remain valid for the
/// lifetime of the resulting expression.  If you continue to use an expression
/// after the memory of an unsafe variable has been reclaimed, you will have
/// undefined behavior.
///
///
/// ## Examples
///
/// Here is an example of correct and incorrect use of unsafe variable pointers:
///
/// ```
/// use fasteval::Evaler;    // use this trait so we can call eval().
/// use fasteval::Compiler;  // use this trait so we can call compile().
///
/// // Here is an example of INCORRECT registration.  DO NOT DO THIS!
/// fn bad_unsafe_var(slab_mut:&mut fasteval::Slab) {
///     let bad : f64 = 0.0;
///
///     // Saves a pointer to 'bad':
///     unsafe { slab_mut.ps.add_unsafe_var("bad".to_string(), &bad); }  // `add_unsafe_var()` only exists if the `unsafe-vars` feature is enabled: `cargo test --features unsafe-vars`
///
///     // 'bad' goes out-of-scope here, and the pointer we registered is no longer valid!
///     // This will result in undefined behavior.
/// }
///
/// fn main() -> Result<(), fasteval::Error> {
///     let mut slab = fasteval::Slab::new();
///
///     // The Unsafe Variable will use a pointer to read this memory location:
///     // You must make sure that this variable stays in-scope as long as the
///     // expression is in-use.
///     let mut deg : f64 = 0.0;
///
///     // Unsafe Variables must be registered before 'parse()'.
///     // (Normal Variables only need definitions during the 'eval' phase.)
///     unsafe { slab.ps.add_unsafe_var("deg".to_string(), &deg); }  // `add_unsafe_var()` only exists if the `unsafe-vars` feature is enabled: `cargo test --features unsafe-vars`
///
///     // bad_unsafe_var(&mut slab);  // Don't do it this way.
///
///     let expr_str = "sin(deg/360 * 2*pi())";
///     let expr_ref = fasteval::parse(expr_str, &mut slab.ps)?.from(&slab.ps);
///
///     // The main reason people use Unsafe Variables is to maximize performance.
///     // Compilation also helps performance, so it is usually used together with Unsafe Variables:
///     let compiled = expr_ref.compile(&slab.ps, &mut slab.cs);
///
///     let mut ns = fasteval::EmptyNamespace;  // We only define unsafe variables, not normal variables,
///                                             // so EmptyNamespace is fine.
///
///     for d in 0..360 {
///         deg = d as f64;
///         let val = fasteval::eval_compiled!(compiled, &slab, &mut ns);
///         eprintln!("sin({}°) = {}", deg, val);
///     }
///
///     Ok(())
/// }
/// 
/// ```
pub struct ParseSlab {
    pub(crate) exprs      :Vec<Expression>,
    pub(crate) vals       :Vec<Value>,
    pub(crate) def_expr   :Expression,
    pub(crate) def_val    :Value,
    pub(crate) char_buf   :String,
    #[cfg(feature="unsafe-vars")]
    pub(crate) unsafe_vars:BTreeMap<String, *const f64>,
}

/// `CompileSlab` is where `compile()` results are stored, located at `Slab.cs`.
pub struct CompileSlab {
    pub(crate) instrs   :Vec<Instruction>,
    pub(crate) def_instr:Instruction,
}

impl ParseSlab {
    /// Returns a reference to the [`Expression`](../parser/struct.Expression.html)
    /// located at `expr_i` within the `ParseSlab.exprs'.
    ///
    /// If `expr_i` is out-of-bounds, a reference to a default `Expression` is returned.
    ///
    #[inline]
    pub fn get_expr(&self, expr_i:ExpressionI) -> &Expression {
        // I'm using this non-panic match structure to boost performance:
        match self.exprs.get(expr_i.0) {
            Some(expr_ref) => expr_ref,
            None => &self.def_expr,
        }
    }

    /// Returns a reference to the [`Value`](../parser/enum.Value.html)
    /// located at `val_i` within the `ParseSlab.vals'.
    ///
    /// If `val_i` is out-of-bounds, a reference to a default `Value` is returned.
    ///
    #[inline]
    pub fn get_val(&self, val_i:ValueI) -> &Value {
        match self.vals.get(val_i.0) {
            Some(val_ref) => val_ref,
            None => &self.def_val,
        }
    }

    /// Appends an `Expression` to `ParseSlab.exprs`.
    ///
    /// # Errors
    ///
    /// If `ParseSlab.exprs` is already full, a `SlabOverflow` error is returned.
    ///
    #[inline]
    pub(crate) fn push_expr(&mut self, expr:Expression) -> Result<ExpressionI,Error> {
        let i = self.exprs.len();
        if i>=self.exprs.capacity() { return Err(Error::SlabOverflow); }
        self.exprs.push(expr);
        Ok(ExpressionI(i))
    }

    /// Appends a `Value` to `ParseSlab.vals`.
    ///
    /// # Errors
    ///
    /// If `ParseSlab.vals` is already full, a `SlabOverflow` error is returned.
    ///
    #[inline]
    pub(crate) fn push_val(&mut self, val:Value) -> Result<ValueI,Error> {
        let i = self.vals.len();
        if i>=self.vals.capacity() { return Err(Error::SlabOverflow); }
        self.vals.push(val);
        Ok(ValueI(i))
    }

    /// Clears all data from `ParseSlab.exprs` and `ParseSlab.vals`.
    #[inline]
    pub fn clear(&mut self) {
        self.exprs.clear();
        self.vals.clear();
    }

    /// [See the `add_unsafe_var()` documentation above.](#unsafe-variable-registration-with-add_unsafe_var)
    #[cfg(feature="unsafe-vars")]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub unsafe fn add_unsafe_var(&mut self, name:String, ptr:&f64) {
        self.unsafe_vars.insert(name, ptr as *const f64);
    }
}

impl CompileSlab {
    /// Returns a reference to the [`Instruction`](../compiler/enum.Instruction.html)
    /// located at `instr_i` within the `CompileSlab.instrs'.
    ///
    /// If `instr_i` is out-of-bounds, a reference to a default `Instruction` is returned.
    ///
    #[inline]
    pub fn get_instr(&self, instr_i:InstructionI) -> &Instruction {
        match self.instrs.get(instr_i.0) {
            Some(instr_ref) => instr_ref,
            None => &self.def_instr,
        }
    }

    /// Appends an `Instruction` to `CompileSlab.instrs`.
    pub(crate) fn push_instr(&mut self, instr:Instruction) -> InstructionI {
        if self.instrs.capacity()==0 { self.instrs.reserve(32); }
        let i = self.instrs.len();
        self.instrs.push(instr);
        InstructionI(i)
    }

    /// Removes an `Instruction` from `CompileSlab.instrs` as efficiently as possible.
    pub(crate) fn take_instr(&mut self, i:InstructionI) -> Instruction {
        if i.0==self.instrs.len().saturating_sub(1) {
            match self.instrs.pop() {
                Some(instr) => instr,
                None => IConst(std::f64::NAN),
            }
        } else {
            match self.instrs.get_mut(i.0) {
                Some(instr_ref) => mem::replace(instr_ref, IConst(std::f64::NAN)),  // Conspicuous Value
                None => IConst(std::f64::NAN),
            }
        }
    }

    /// Clears all data from `CompileSlab.instrs`.
    #[inline]
    pub fn clear(&mut self) {
        self.instrs.clear();
    }
}

impl Slab {
    /// Creates a new default-sized `Slab`.
    #[inline]
    pub fn new() -> Self { Self::with_capacity(64) }

    /// Creates a new `Slab` with the given capacity.
    #[inline]
    pub fn with_capacity(cap:usize) -> Self {
        Self{
            ps:ParseSlab{
                exprs      :Vec::with_capacity(cap),
                vals       :Vec::with_capacity(cap),
                def_expr   :Default::default(),
                def_val    :Default::default(),
                char_buf   :String::with_capacity(64),
                #[cfg(feature="unsafe-vars")]
                unsafe_vars:BTreeMap::new(),
            },
            cs:CompileSlab{
                instrs   :Vec::new(),  // Don't pre-allocate for compilation.
                def_instr:Default::default(),
            },
        }
    }

    /// Clears all data from [`Slab.ps`](struct.ParseSlab.html) and [`Slab.cs`](struct.CompileSlab.html).
    #[inline]
    pub fn clear(&mut self) {
        self.ps.exprs.clear();
        self.ps.vals.clear();
        self.cs.instrs.clear();
    }
}


fn write_indexed_list<T>(f:&mut fmt::Formatter, lst:&[T]) -> Result<(), fmt::Error> where T:fmt::Debug {
    write!(f, "{{")?;
    let mut nonempty = false;
    for (i,x) in lst.iter().enumerate() {
        if nonempty { write!(f, ",")?; }
        nonempty = true;
        write!(f, " {}:{:?}",i,x)?;
    }
    if nonempty { write!(f, " ")?; }
    write!(f, "}}")?;
    Ok(())
}
impl fmt::Debug for Slab {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Slab{{ exprs:")?;
        write_indexed_list(f, &self.ps.exprs)?;
        write!(f, ", vals:")?;
        write_indexed_list(f, &self.ps.vals)?;
        write!(f, ", instrs:")?;
        write_indexed_list(f, &self.cs.instrs)?;
        write!(f, " }}")?;
        Ok(())
    }
}
impl fmt::Debug for ParseSlab {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "ParseSlab{{ exprs:")?;
        write_indexed_list(f, &self.exprs)?;
        write!(f, ", vals:")?;
        write_indexed_list(f, &self.vals)?;
        write!(f, " }}")?;
        Ok(())
    }
}
impl fmt::Debug for CompileSlab {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "CompileSlab{{ instrs:")?;
        write_indexed_list(f, &self.instrs)?;
        write!(f, " }}")?;
        Ok(())
    }
}

impl Default for Slab {
    fn default() -> Self { Self::with_capacity(64) }
}

