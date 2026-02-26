# dj_tool

Full README to come one day, for now enjoy this

## Benchmarks:

Some vibe-coded benchmarks:

```
------------------------------------------------------------
  Backend: librosa
------------------------------------------------------------

  [librosa] Run 1/5: 3.0175s
  [librosa] Run 2/5: 2.0635s
  [librosa] Run 3/5: 2.0633s
  [librosa] Run 4/5: 2.0635s
  [librosa] Run 5/5: 2.0623s

  [librosa] Summary (5 runs):
    Average : 2.2540s
    Best    : 2.0623s
    Worst   : 3.0175s

------------------------------------------------------------
  Backend: pydub + numpy
------------------------------------------------------------

  [pydub] Run 1/5: 1.7959s
  [pydub] Run 2/5: 1.7839s
  [pydub] Run 3/5: 1.7815s
  [pydub] Run 4/5: 1.7944s
  [pydub] Run 5/5: 1.7836s

  [pydub] Summary (5 runs):
    Average : 1.7878s
    Best    : 1.7815s
    Worst   : 1.7959s

------------------------------------------------------------
  Backend: Rust (PyO3)
------------------------------------------------------------

  [rust] Run 1/5: 1.3641s
  [rust] Run 2/5: 1.3536s
  [rust] Run 3/5: 1.3566s
  [rust] Run 4/5: 1.3551s
  [rust] Run 5/5: 1.3569s

  [rust] Summary (5 runs):
    Average : 1.3573s
    Best    : 1.3536s
    Worst   : 1.3641s

============================================================
  Comparison (average over 5 runs)
============================================================
  Backend              Avg Time (s)    Speedup vs librosa
  -------------------- --------------- --------------------
  Rust (PyO3)          1.3573          1.66x
  pydub + numpy        1.7878          1.26x
  librosa              2.2585          1.00x

```

## Usage

Building docs to come

```
cargo run 
```

Folder with tracks should be entered, this will be changed after we wrap everything in typer.
For now main dev is done in Rust until PyO3 translation layer is shipped

