# pi_benchmarks
This is a little project for benchmarking implementations of calculating pi in different languages. The main project (the glue so to speak) is implemented in rust. The purpose of this project is learning for my self (especially how to integrate other languages with rust), and a little bit of shits and giggles.

## Supported languages
At the moment, I only support three languages, namely rust, C and x86_64 assembly. The choice of languages is very easy to explain. At the moment I am learning rust, so obviously it had to be rust, the allmighty C is the unfought ultimate truth in regards to performance, and x86_64 because I wanted to show those damn high level language compilers how to do things right (at which I failed miserably, as you'll see in a minute). At the moment I'm not feeling like adding something new, but if someone feels like adding an implementation in a new language, feel free to make a pull request or something.

## Methodology
I am aware that rust provides a builtin benchmarking mechanism, but I wanted to see how someone may go at implementing it himself. So I used the dead simplest approach I could think of. Take the time, before a start, call the method, take the time elapsed. Then just a little formatting and voila, a nice benchmark.

## Usage
### Prerequisites
To use this benchmark you need to have a working [rust installation](https://doc.rust-lang.org/book/ch01-00-getting-started.html) (not necessarily with cargo, but it is greatly advised, because I don't know shit about doing this without cargo, so if you want to build/link/execute shit on your own, you are exactly that: on your own). You also need a functioning c toolchain (if I understand rusts inner workings correct, it doesn't matter if clang or gcc) and you'll have to have [nasm installed](https://www.nasm.us/). This obviously only makes sense, if your CPU implements the x86_64 architecture. The OS of your system shouldn't matter, it is only tested on Arch Linux (btw.). If you encounter difficulties on other OSs (only regarding this specific project, I can't install and fix any of the needed tools for you) open an issue or something. Just don't expect it to get solved. Oh yeah, git would be a good thing to have installed, but if you haven't, what are you even doing here?

### Run that bitch
It should be es simple as ```cargo run``` in the root folder of this project. If it isn't, I'm stumped.

## Example data
Rust and C are both compiled with optimization level 3 (rust is configured in the Cargo.toml, C in the build.rs), x86_64 of course without any optimization. This output was generated on my pc using Arch Linux (btw.) with my Intel(R) Core(TM) i5-6600K CPU @ 3.50GHz. If you think "Man, isn't that CPU a little old?" shut the fuck up, I love him, okay? It's complicated. If you weren't thinking that, I see, you're a man of povery as well. Where was I? Ah, yeah, example output:

```
$ cargo run   Compiling pi_benchmarks v0.1.0 (/home/***/programming/rust/pi_benchmarks)
    Finished dev [optimized + debuginfo] target(s) in 0.26s
     Running `target/debug/pi_benchmarks`
Approximating PI using 1000000000 iterations...
--- C ---
start: 2022-02-02T17:44:17.181035722Z
duration: 2s 891ms 860us 767ns
result: 3.141592653589787

--- x86_64 ---
start: 2022-02-02T17:44:20.072913651Z
duration: 7s 51ms 300us 760ns
result: 3.141592653589787

--- Rust ---
start: 2022-02-02T17:44:27.124226552Z
duration: 2s 525ms 159us 216ns
result: 3.141592653589787
```

As you can see, c and rust are pretty close, with rust taking a small lead. And then, unmatched by a gap wider than the grand canyon comes my own assembly. I really showed those compiler writers how to do it right. I reran the program a few times, it seems to be somewhat consistent.

## How can I contribute?
If you have some feedback regarding the code, it is greatly appreciated. You also can add a new implementation in another language I haven't covered. Also, if you want, you can run this program and add your specs to the list, to get a nice overview, if different hardware makes a difference.

## Why aren't you funnier?
Shut the fuck up, I'm hillarious.
