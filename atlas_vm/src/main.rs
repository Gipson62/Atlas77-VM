use atlas_vm::instruction::compiler::parser::Parser;

use atlas_vm::memory::vm_data::VMData;
use atlas_vm::runtime::vm_state::VMState;
use atlas_vm::runtime::VM;

fn main() {
let tmp = std::time::Instant::now();
    if let Ok(content) = std::fs::read_to_string("./atlas_vm/examples/mem_test.txt") {
        let mut lexer = atlas_vm::instruction::compiler::lexer::AtlasLexer::default();
        lexer.set_path("examples/mem_test.txt");
        lexer.set_source(content);
        lexer.add_system(atlas_vm::instruction::compiler::lexer::identifier_system);
        lexer.add_system(atlas_vm::instruction::compiler::lexer::comment_system);
        let res = lexer.tokenize();
        match res {
            Ok(t) => {
                println!("Ok Lexer: {:?}", tmp.elapsed());
                let tmp = std::time::Instant::now();
                //t.clone().into_iter().for_each(|ins| println!("{:?}, ", ins.kind()));
                let parser = Parser::parse(t);
                match parser {
                    Ok(code) => {
                        println!("Ok Parser: {:?}", tmp.elapsed());
                        let tmp = std::time::Instant::now();
                        //code.clone().into_iter().for_each(|ins| println!("{:?}", ins));
                        let mut vm = VM::new(1, code.constants);
                        vm.add_extern_call(fib_extern).execute(code.ins.as_slice());
                        println!("{}", vm.object_map);
                        println!("Ok Excution: {:?}", tmp.elapsed())
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
}

pub fn fib_extern(vm_state: VMState) -> Result<VMData, ()> {
    fn fib(n: i64) -> i64 {
        if n < 2 {
            n
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }
    let res = fib(vm_state.stack.pop().expect("Stack Underflow").as_i64());
    Ok(VMData::new_i64(res))
}
