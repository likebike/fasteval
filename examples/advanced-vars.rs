// usage:  cargo run --release --example advanced-vars

fn main() -> Result<(), fasteval::Error> {
    let mut cb = |name:&str, args:Vec<f64>| -> Option<f64> {
        let mydata : [f64; 3] = [11.1, 22.2, 33.3];
        match name {
            // Custom constants/variables:
            "x" => Some(3.0),
            "y" => Some(4.0),

            // Custom function:
            "sum" => Some(args.into_iter().fold(0.0, |s,f| s+f)),

            // Custom array-like objects:
            // The `args.get...` code is the same as:
            //     mydata[args[0] as usize]
            // ...but it won't panic if either index is out-of-bounds.
            "data" => args.get(0).and_then(|f| mydata.get(*f as usize).copied()),

            // A wildcard to handle all undefined names:
            _ => None,
        }
    };

    let val = fasteval::ez_eval("sum(x^2, y^2)^0.5 + data[0]",    &mut cb)?;
    //                           |   |                   |
    //                           |   |                   square-brackets act like parenthesis
    //                           |   variables are like custom functions with zero args
    //                           custom function

    assert_eq!(val, 16.1);

    // Let's explore some of the hidden complexities of variables:
    //
    //     * There's really no difference between a variable and a custom function.
    //       Therefore, variables can receive arguments too,
    //       which will probably be ignored.
    //       Therefore, these two expressions evaluate to the same thing:
    //           eval("x + y")  ==  eval("x(1,2,3) + y(x, y, sum(x,y))")
    //                                      ^^^^^      ^^^^^^^^^^^^^^
    //                                      All this stuff is ignored.
    //
    //     * Built-in functions take precedence WHEN CALLED AS FUNCTIONS.
    //       This design was chosen so that builtin functions do not pollute
    //       the variable namespace, which is important for some applications.
    //       Here are some examples:
    //           pi        -- Uses the custom 'pi' variable, NOT the builtin 'pi' function.  
    //           pi()      -- Uses the builtin 'pi' function even if a custom variable is defined.
    //           pi(1,2,3) -- Uses the builtin 'pi' function, and produces a WrongArgs error
    //                        during parse because the builtin does not expect any arguments.
    //           x         -- Uses the custom 'x' variable.
    //           x()       -- Uses the custom 'x' variable because there is no 'x' builtin.
    //           x(1,2,3)  -- Uses the custom 'x' variable.  The args are ignored.
    //           sum       -- Uses the custom 'sum' function with no arguments.
    //           sum()     -- Uses the custom 'sum' function with no arguments.
    //           sum(1,2)  -- Uses the custom 'sum' function with two arguments.

    Ok(())
}
