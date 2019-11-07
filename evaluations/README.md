# Evaluations

## LLVM opt

| Call type                           | Resolved %  | 
| ----------------------------------- | -----------:|
| static dispatch                     |        100% |
| dynamic dispatch                    |          0% |
| generic <sup>1</sup>                |        100% |
| function pointer                    |          0% |
| macro                               |        100% |
| conditionally compiled <sup>2</sup> |         50% |

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

