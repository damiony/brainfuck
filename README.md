# brainfuck 解释器

用 rust 实现的 brainfuck 解释器。

## 可执行文件

该解释器包含两个版本，一个是性能较差的 brainfuck-simple，一个是性能较好的 brainfuck-ir。

运行构建命令，生成可执行文件：

```bash
cargo build
```

然后就能选择执行 brainfuck-simple：

```bash
./target/debug/brainfuck-simple asserts/helloworld.bf
```

或者执行 brainfuck-ir：

```bash
./target/debug/brainfuck-ir asserts/helloworld.bf
```

## brainfuck 程序

所有的 brainfuck 程序都被存放在 asserts 目录下，也可以使用本地的任何 brainfuck 程序文件，只要路径指定正确即可。
