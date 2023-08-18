# are-type-systems-easy

I read this [excellent introductory explanation](https://langdev.stackexchange.com/questions/2692/how-should-i-read-type-system-notation/2693#2693)
of type system logic, construction, and notation the other day and found it dazzling
how seemingly easily applicable all the concepts shown are.
I found it so easy that I ought to implement a typechecker for the simple language
(minus functions and contexts; will likely do that soon) in the style of the `infer` function
using Rust's pattern matching.
The entire checker comes in at less than 100 lines and I got the implementation, as far as I can tell,
100% correct first try!
Goes to show the power of a good spec.

However, I want to make implementing the type system feel exactly like writing a series of inference
rules.
For decades, people have been transforming the grammars of programming languages directly into 
robust, safe, and easy-to-write implementations instead of implementing them by hand, using the notation
only as a place to noodle and later a guide.
Now that I see that the natural induction syntax of type inference is so simple, I wonder 
why there isn't similar tooling for creating robust, safe, and easy-to-create type systems.

I hope this repo can act as a basis for making typechecking even more declarative than it is
at the start of this project.
I thought that type systems were these monolitic beasts which can take tens of thousands of lines of
code to implement, but I now see the potential to have the type system of, say, a calculator, fit on a 
buisness card and *actually be executable*.
We'll see if it works out.
