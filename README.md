# rscript

Rust で書かれた小さなスクリプト言語

Playground: https://glider1967.github.io/rscript-playground/

## 機能

- Rust like な構文

```text
let a = 1;
let b = 2;
a + b // output: 3
```

- 型
  `int`, `bool`, `string`の 3 つの型

```text
let a: int = 1;
let b: bool = true;
let c: string = "ccc";
if (b) { c ++ ~a } else { "ddd" } // output: "ccc1"
```

- 関数
  関数専用の構文は無く、全て lambda 式で書く

```text
let twice = lambda(f, x) { // 高階関数
    f(f(x))
};
let multFour: int -> int = twice(lambda(x) {x*2}); // 部分適用
multFour(7) // output: 28
```

- 代数的データ型

```text
enum List { // 整数のConsリスト
    Cons(int, List),
    Nil
};
let sum = lambda(l: List) {
    match (l) {
        Cons(x, xs) => x + sum(xs),
        Nil => 0
    }
};
let map = lambda(l: List, f: int -> int) {
    match (l) {
        Cons(x, xs) => Cons(f(x), map(xs, f)),
        Nil => Nil
    }
};
let l = Cons(1, Cons(2, Cons(3, Nil)));
sum(map(l, lambda(x){x*x})) // output: 1*1 + 2*2 + 3*3
```
