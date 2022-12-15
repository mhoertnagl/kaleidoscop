View disassembled file:

```bash
llvm-dis main.bc -o main.ll
```

Run IR file and print return code:

```bash
lli main.ll; echo $?
lli main.bc; echo $?
```