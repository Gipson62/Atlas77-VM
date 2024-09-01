use criterion::{criterion_group, criterion_main, Criterion};
use vm::{instruction::compiler::parser::Parser, runtime::VM};

fn vm_test_benchmark(c: &mut Criterion) {
    c.bench_function("vm_instruction", |b| {
        b.iter(|| {
            if let Ok(content) = std::fs::read_to_string("./examples/fib.txt") {
                let mut lexer = vm::instruction::compiler::lexer::AtlasLexer::default();
                lexer.set_path("examples/fib.txt");
                lexer.set_source(content);
                lexer.add_system(vm::instruction::compiler::lexer::identifier_system);
                lexer.add_system(vm::instruction::compiler::lexer::comment_system);
                let res = lexer.tokenize();
                match res {
                    Ok(t) => {
                        let parser = Parser::parse(t);
                        match parser {
                            Ok(code) => {
                                let mut vm = VM::new(0, code.constants);
                                vm.execute(code.ins.as_slice());
                            }
                            Err(e) => {
                                panic!("{:?}", e);
                            }
                        };
                    }
                    Err(_e) => {
                        println!("Error1");
                    }
                }
            } else {
                println!("Error2")
            }
        })
    });
}

criterion_group!(benches, vm_test_benchmark);
criterion_main!(benches);
