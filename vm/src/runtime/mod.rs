pub mod vm_state;

use std::collections::HashMap;

use internment::Intern;
use vm_state::VMState;

use crate::{
    instruction::Instruction,
    memory::{
        object_map::{Memory, ObjectIndex, Structure},
        stack::Stack,
        vm_data::VMData,
    },
};

pub type CallBack = fn(vm_state::VMState) -> Result<VMData, ()>;

pub struct VM {
    pub stack: Stack,
    pub object_map: Memory,
    pub extern_fn: Vec<CallBack>,
    constants: Vec<VMData>,
    call_stack: Vec<usize>,
    hooks: HashMap<Intern<String>, usize>,
    pc: usize,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            stack: Stack::default(),
            object_map: Memory::new(16),
            extern_fn: vec![],
            constants: vec![],
            call_stack: vec![],
            hooks: HashMap::default(),

            pc: usize::default(),
        }
    }
}
impl VM {
    pub fn new(mem_space: usize, constants: Vec<VMData>) -> Self {
        Self {
            stack: Stack::new(),
            object_map: Memory::new(mem_space),
            extern_fn: vec![],
            constants,
            call_stack: vec![],
            hooks: HashMap::default(),
            pc: usize::default(),
        }
    }
    pub fn add_extern_call(&mut self, call: CallBack) -> &mut Self {
        self.extern_fn.push(call);
        self
    }
    #[inline(always)]
    pub fn clean(&mut self) {
        self.stack.top = 1;
        self.call_stack = vec![];
        self.pc = usize::default();
    }

    pub fn execute(&mut self, ins: &[Instruction]) {
        while self.pc < ins.len() {
            let ins = &ins[self.pc];
            #[cfg(debug_assertions)]
            println!("{:?}", ins);
            //#[cfg(debug_assertions)]
            //println!("Memory: [{:?}, {:?}, {:?}]", self.object_map.get(ObjectIndex::new(0)), self.object_map.get(ObjectIndex::new(1)), self.object_map.get(ObjectIndex::new(2)));
            match ins {
                Instruction::HLT => break,
                _ => {
                    self.execute_instruction(ins);
                }
            }
            #[cfg(debug_assertions)]
            println!("{}", self.stack);

            #[cfg(debug_assertions)]
            {
                use std::thread;
                use std::time::Duration;
                thread::sleep(Duration::from_millis(250));
            }
        }
        self.clean();
    }
    pub fn execute_instruction(&mut self, ins: &Instruction) {
        use Instruction::*;
        match ins {
            PushI(i) => self.stack.push(VMData::new_i64(*i)),
            PushF(f) => self.stack.push(VMData::new_f64(*f)),
            PushU(u) => self.stack.push(VMData::new_u64(*u)),
            LoadConst(u) => {
                //constants aren't loaded as is, but are fetched from constants: Vec<VMData>
                #[cfg(debug_assertions)]
                if self.constants.len() < *u {
                    panic!("Can't load that constant, it doesn't exist");
                }
                self.stack.push(self.constants[*u]);
            }
            Pop => {
                self.stack.pop().expect("Stack Underflow");
            }
            Print => {
                let value = self.stack.last().expect("Stack Underflow");
                println!("val: {}", value)
            }
            AddI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a + b));
            }
            AddF => {
                let b = self.stack.pop().expect("Stack Underflow").as_f64();
                let a = self.stack.pop().expect("Stack Underflow").as_f64();
                self.stack.push(VMData::new_f64(a + b));
            }
            AddU => {
                let b = self.stack.pop().expect("Stack Underflow").as_u64();
                let a = self.stack.pop().expect("Stack Underflow").as_u64();
                self.stack.push(VMData::new_u64(a + b));
            }
            MulI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a * b));
            }
            MulF => {
                let b = self.stack.pop().expect("Stack Underflow").as_f64();
                let a = self.stack.pop().expect("Stack Underflow").as_f64();
                self.stack.push(VMData::new_f64(a * b));
            }
            MulU => {
                let b = self.stack.pop().expect("Stack Underflow").as_u64();
                let a = self.stack.pop().expect("Stack Underflow").as_u64();
                self.stack.push(VMData::new_u64(a + b));
            }
            DivI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                if b == 0 {
                    panic!("Can't divide by 0");
                }
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a / b));
            }
            DivF => {
                let b = self.stack.pop().expect("Stack Underflow").as_f64();
                if b == 0.0 {
                    panic!("Can't divide by 0");
                }
                let a = self.stack.pop().expect("Stack Underflow").as_f64();
                self.stack.push(VMData::new_f64(a / b));
            }
            DivU => {
                let b = self.stack.pop().expect("Stack Underflow").as_u64();
                if b == 0 {
                    panic!("Can't divide by 0");
                }
                let a = self.stack.pop().expect("Stack Underflow").as_u64();
                self.stack.push(VMData::new_u64(a / b));
            }
            SubI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a - b));
            }
            SubF => {
                let b = self.stack.pop().expect("Stack Underflow").as_f64();
                let a = self.stack.pop().expect("Stack Underflow").as_f64();
                self.stack.push(VMData::new_f64(a - b));
            }
            SubU => {
                let b = self.stack.pop().expect("Stack Underflow").as_u64();
                let a = self.stack.pop().expect("Stack Underflow").as_u64();
                self.stack.push(VMData::new_u64(a - b));
            }
            Dup => {
                let last = self.stack.last().expect("Stack Underflow");
                self.stack.push(*last);
            }
            Swap => {
                let a = self.stack.pop().expect("Stack Underflow");
                let b = self.stack.pop().expect("Stack Underflow");
                self.stack.push(a);
                self.stack.push(b);
            }
            Rot => {
                let a = self.stack.pop().expect("Stack Underflow");
                let b = self.stack.pop().expect("Stack Underflow");
                let c = self.stack.pop().expect("Stack Underflow");
                self.stack.push(c);
                self.stack.push(b);
                self.stack.push(a);
            }
            Jmp(address) => {
                self.pc = address.into();
                return;
            }
            JmpNZ(address) => {
                let val = self.stack.pop().expect("Stack Underflow").as_u64();
                if val != 0 {
                    self.pc = address.into();
                    return;
                }
            }
            JmpZ(address) => {
                let val = self.stack.pop().expect("Stack Underflow").as_u64();
                if val == 0 {
                    self.pc = address.into();
                    return;
                }
            }
            ExternCall(address) => {
                let vm_state = VMState::new(&mut self.stack, &mut self.object_map, &self.constants);
                match self.extern_fn[*address](vm_state) {
                    Ok(val) => {
                        self.stack.push(val);
                    }
                    Err(_) => {
                        println!("there was an error with extern call");
                    }
                }
            }
            Call(address) => {
                self.call_stack.push(self.pc + 1);
                self.pc = address.into();
                return;
            }
            Ret => {
                self.pc = self.call_stack.pop().expect("Call Stack Underflow");
                //println!("return: {}", self.stack);
                return;
            }
            CastToI => {
                let val = self.stack.pop().expect("Stack Underflow");
                let res = match val.tag {
                    VMData::TAG_CHAR => val.as_char() as i64,
                    VMData::TAG_I64 => val.as_i64(),
                    VMData::TAG_FLOAT => val.as_f64() as i64,
                    VMData::TAG_U64 => val.as_u64() as i64,
                    VMData::TAG_BOOL => val.as_bool() as i64,
                    f if f > 256 | VMData::TAG_STR => val.as_object().idx as i64,
                    _ => {
                        if val.tag > 256 || val.tag == VMData::TAG_STR {
                            val.as_object().idx as i64
                        } else {
                            unimplemented!("cast_to_int isn't implemented for tag: {}", val.tag)
                        }
                    }
                };
                self.stack.push(VMData::new_i64(res));
            }
            CastToPtr => {
                let val = self.stack.pop().expect("Stack Underflow");
                let res = match val.tag {
                    VMData::TAG_I64 => ObjectIndex::new(val.as_i64() as u64),
                    VMData::TAG_U64 => ObjectIndex::new(val.as_u64()),
                    f if f > 256 | VMData::TAG_STR => val.as_object(),
                    _ => unimplemented!("cast_to_ptr isn't implemented for tag; {}", val.tag),
                };
                self.stack.push(VMData::new_object(257, res));
            }
            CastToF => {
                let val = self.stack.pop().expect("Stack Underflow");
                let res = match val.tag {
                    VMData::TAG_CHAR => val.as_char() as i64 as f64,
                    VMData::TAG_I64 => val.as_i64() as f64,
                    VMData::TAG_FLOAT => val.as_f64(),
                    VMData::TAG_U64 => val.as_u64() as f64,
                    VMData::TAG_BOOL => val.as_bool() as i64 as f64,
                    _ => unimplemented!("cast_to_float isn't implement for tag: {}", val.tag),
                };
                self.stack.push(VMData::new_f64(res));
            }
            CastToU => {
                let val = self.stack.pop().expect("Stack Underflow");
                let res = match val.tag {
                    VMData::TAG_CHAR => val.as_char() as u64,
                    VMData::TAG_I64 => val.as_i64() as u64,
                    VMData::TAG_FLOAT => val.as_f64() as u64,
                    VMData::TAG_U64 => val.as_u64(),
                    VMData::TAG_BOOL => val.as_bool() as u64,
                    _ => unimplemented!("cast_to_uint isn't implement for tag: {}", val.tag),
                };
                self.stack.push(VMData::new_u64(res));
            }
            CastToChar => {
                let val = self.stack.pop().expect("Stack Underflow");
                let res = match val.tag {
                    VMData::TAG_CHAR => val.as_char(),
                    VMData::TAG_I64 => val.as_i64() as u8 as char,
                    VMData::TAG_FLOAT => char::from_u32(val.as_f64() as u32).unwrap(),
                    VMData::TAG_U64 => char::from_u32(val.as_u64() as u32).unwrap(),
                    VMData::TAG_BOOL => val.as_bool() as u8 as char,
                    _ => unimplemented!(
                        "cast_to_int isn't implement for tag: {} [{}]",
                        val.tag,
                        self.pc
                    ),
                };
                self.stack.push(VMData::new_char(res));
            }
            CastToBool => {
                let val = self.stack.pop().expect("Stack Underflow");
                let res = match val.tag {
                    VMData::TAG_CHAR => val.as_char() as i64,
                    VMData::TAG_I64 => val.as_i64(),
                    VMData::TAG_FLOAT => val.as_f64() as i64,
                    VMData::TAG_U64 => val.as_u64() as i64,
                    VMData::TAG_BOOL => val.as_bool() as i64,
                    _ => unimplemented!("cast_to_int isn't implement for tag: {}", val.tag),
                };
                self.stack.push(VMData::new_i64(res));
            }
            Read => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let val = String::from(input.trim());
                match self.object_map.put(val.into()) {
                    Ok(i) => {
                        self.stack.push(VMData::new_string(i));
                    }
                    Err(o) => {
                        panic!("Memory full, can't insert: [{:?}]", o);
                    }
                }
            }
            ReadI => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let val = match String::from(input.trim()).parse::<i64>() {
                    Ok(i) => i,
                    Err(e) => {
                        panic!("{}", e);
                    }
                };
                self.stack.push(VMData::new_i64(val));
            }
            SetStruct(u) => {
                let ptr = self.stack.pop().expect("Stack underflow").as_object();
                let val = self.stack.pop().expect("Stack underflow");
                self.object_map.get_mut(ptr).structure_mut().fields[*u] = val;
            }
            GetStruct(u) => {
                let ptr = self.stack.pop().expect("Stack underflow").as_object();
                let field = self.object_map.get(ptr).structure().fields[*u];
                self.stack.push(field);
            }
            CreateStruct(u) => {
                let s = Structure {
                    fields: vec![VMData::new_unit(); *u],
                };
                match self.object_map.put(s.into()) {
                    Ok(ptr) => {
                        self.stack.push(VMData::new_object(257, ptr));
                    }
                    Err(o) => {
                        panic!("Can't add :[{:?}] in the memory", o);
                    }
                }
            }
            CreateString => match self.object_map.put(String::new().into()) {
                Ok(ptr) => {
                    self.stack.push(VMData::new_string(ptr));
                }
                Err(o) => {
                    panic!("Can't add :[{:?}] in the memory", o);
                }
            },
            StrLen => {
                let ptr = self.stack.pop().expect("Stack underflow").as_object();
                let len = self.object_map.get(ptr).string().len();
                self.stack.push(VMData::new_i64(len as i64));
            }
            WriteCharToString => {
                let ptr = self.stack.pop().expect("Stack underflow").as_object();
                let ch = self.stack.pop().expect("Stack underflow").as_char();
                self.object_map.get_mut(ptr).string_mut().push(ch);
            }
            ReadCharFromString => {
                let ptr = self.stack.pop().expect("Stack underflow").as_object();
                let i = self.stack.pop().expect("Stack underflow").as_u64();
                let ch = match self.object_map.get(ptr).string().chars().nth(i as usize) {
                    Some(c) => c,
                    None => {
                        panic!("Index out of bound for string: {}[{}]", ptr, i);
                    }
                };
                self.stack.push(VMData::new_char(ch));
            }
            Instruction::Eq => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a == b));
            }
            Instruction::Neq => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a != b));
            }
            Instruction::Lt => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a < b));
            }
            Instruction::Gt => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a > b));
            }
            Instruction::Lte => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a <= b));
            }
            Instruction::Gte => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a >= b));
            }
            Instruction::And => {
                let b = self.stack.pop().expect("Stack underflow").as_bool();
                let a = self.stack.pop().expect("Stack underflow").as_bool();
                self.stack.push(VMData::new_bool(a && b));
            }
            Instruction::Or => {
                let b = self.stack.pop().expect("Stack underflow").as_bool();
                let a = self.stack.pop().expect("Stack underflow").as_bool();
                self.stack.push(VMData::new_bool(a || b));
            }
            Instruction::Not => {
                let value = self.stack.pop().expect("Stack underflow").as_bool();
                self.stack.push(VMData::new_bool(!value));
            }
            PrintChar => {
                let value = self.stack.pop().expect("Stack Underflow").as_char();
                print!("{}", value);
            }
            Nop => {}
            _ => unimplemented!(),
        }
        self.pc += 1;
    }
}
