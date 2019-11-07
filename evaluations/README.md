# Evaluations

## LLVM opt

| Call type              | Resolved %       | 
| ---------------------- | ----------------:|
| static dispatch        |             100% |
| dynamic dispatch       |               0% |
| generic <sup>1</sup>   |             100% |
| function pointer       |               0% |
| macro                  |             100% |
| conditionally compiled | 50% <sup>2</sup> |

LLVM opt is able to resolve static dispatch calls. On the other hand, it is unable to resolve
function calls that have dynamic features, i.e., dynamic dispatch and function pointer calls.

Regarding calls that reside in conditionally compiled code blocks, only those that are inside code
blocks whose condition evaluates to true are analyzed.

A potential limitation of LLVM opt is that it is not able to analyze the dependencies of a package.
Of course, this can turn to an advantage if we want to analyze only the application code and ignore
the dependencies (library) code.

<p>
    <sup>1</sup> Generic calls (and generics in general) are monomorphized (concretized) during
    LLVM IR code generation. This way they are basically equivalent to static dispatch calls and
    are fully resolved by LLVM opt, which operates on the LLVM IR level. However, such an analysis
    decides to not take into account possible concretizations of generic functions that are not yet
    known due to the unavailability of the code that could potentially call these functions. In
    other words the analysis assumes all the codes that could possibly call these generic functions
    are available and can be analyzed.
</p>
<p>
    <sup>2</sup> Conditional compilation conditions are evaluated by cargo before rustc's main
    compilation task begins. The result of 50% is not indicative, as in our benchmark each branch
    of a conditional compilation condition contains the same number of calls. Depending on the
    analyzed code the resolution percentage could be anywhere between 0% and 100%. The way LLVM opt
    works, it is unable to consider all conditional compilation branches and be sound under any
    compilation scenario.
</p>

