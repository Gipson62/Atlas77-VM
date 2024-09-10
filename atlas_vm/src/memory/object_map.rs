use crate::memory::vm_data::VMData;

use super::{stack::Stack, vm_data::TAG};

#[derive(Debug)]
pub struct Memory {
    mem: Vec<Object>,
    pub(crate) free: ObjectIndex,
    memory_pressure: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ObjectIndex {
    pub(crate) idx: u64,
}

impl ObjectIndex {
    pub const fn new(i: u64) -> ObjectIndex {
        ObjectIndex { idx: i }
    }
}
impl std::fmt::Display for ObjectIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[@{}]", self.idx)
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ObjectMap: [{}]",
            self.mem
                .iter()
                .map(|obj| { obj.to_string() })
                .collect::<String>()
        )
    }
}

impl Memory {
    /// space should be at least 1
    pub(crate) fn new(space: usize) -> Self {
        if space == 0 {
            panic!("The object_map should have 1 or more memory block at the start.")
        }
        Self {
            free: ObjectIndex::new(0),
            mem: (0..space)
                .map(|x| Object::Free {
                    next: ObjectIndex::new(((x + 1) % space) as u64),
                })
                .collect(),
            memory_pressure: usize::default(),
        }
    }

    // Need to add a way to increase `mem` size if we out of memory
    // And a way to clean it when there's too much memory (basically shrink and grow)
    pub(crate) fn put(&mut self, object: Object) -> Result<ObjectIndex, Object> {
        if let Object::Free { next: _ } = self.get(self.free) {
            let idx = self.free;
            let v = self.get_mut(self.free);
            let repl = std::mem::replace(v, object);

            match repl {
                Object::Free { next } => {
                    self.free = next;
                    self.memory_pressure += 1;
                    Ok(idx)
                }
                _ => {
                    let obj = std::mem::replace(v, repl);
                    Err(obj)
                }
            }
        } else {
            self.grow();
            self.put(object)
        }
    }

    fn grow(&mut self) {
        let current_size = self.mem.len();
        let new_size = current_size + (current_size / 10) + 1;

        self.mem.reserve(new_size - current_size);
        for i in current_size..new_size {
            self.mem.push(Object::Free {
                next: ObjectIndex {
                    idx: (i + 1) as u64,
                },
            })
        }

        self.mem[new_size - 1] = Object::Free { next: self.free };

        self.free = ObjectIndex::new(current_size as u64);
    }

    #[inline(always)]
    pub(crate) fn get(&self, index: ObjectIndex) -> &Object {
        &self.mem[index.idx as usize]
    }

    #[inline(always)]
    pub(crate) fn get_mut(&mut self, index: ObjectIndex) -> &mut Object {
        &mut self.mem[index.idx as usize]
    }

    #[inline(always)]
    pub(crate) fn raw(&self) -> &[Object] {
        &self.mem
    }

    #[inline(always)]
    pub(crate) fn raw_mut(&mut self) -> &mut [Object] {
        &mut self.mem
    }

    #[inline(always)]
    /// Return None if the ptr points to a free block and return the pointed object if not
    pub(crate) fn free_obj(&mut self, index: ObjectIndex, stack: &mut Stack) -> Option<Object> {
        if let &Object::Free { next: _ } = self.get(index) {
            None
        } else {
            let new_free = Object::Free { next: self.free };
            let v = self.get_mut(index);
            let repl = std::mem::replace(v, new_free);
            self.free = index;
            self.memory_pressure -= 1;
            if self.memory_pressure < self.mem.len() / 2 {
                self.shrink(stack)
            }
            Some(repl)
        }
    }
    //Should be doing something, but for now it's too hard for my little brain.
    fn shrink(&mut self, _stack: &mut Stack) {}
}

#[derive(Clone, Debug)]
pub enum Object {
    String(String),
    Structure(Structure),
    Class(Class),
    Vector(Vector),
    Free { next: ObjectIndex },
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::String(s) => {
                    format!("String: {}", s)
                }
                Object::Structure(s) => {
                    format!("Structure: {{ {} }}", {
                        let mut str_ = String::new();
                        s.fields.iter().for_each(|f| str_.push_str(&f.to_string()));
                        str_
                    })
                }
                Object::Class(c) => {
                    format!("Class {{ {} }}", {
                        let mut str_ = String::new();
                        c.fields.iter().for_each(|f| str_.push_str(&f.to_string()));
                        str_
                    })
                }
                Object::Vector(v) => {
                    format!("Vector [{}]", {
                        let mut str_ = String::new();
                        v.vec.iter().for_each(|f| str_.push_str(&f.to_string()));
                        str_
                    })
                }
                Object::Free { next } => {
                    format!("Free: {}", next)
                }
            }
        )
    }
}

impl Object {
    pub fn new(data: impl Into<Object>) -> Self {
        data.into()
    }

    pub fn string(&self) -> &String {
        match &self {
            Object::String(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn string_mut(&mut self) -> &mut String {
        match self {
            Object::String(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn structure(&self) -> &Structure {
        match &self {
            Object::Structure(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn structure_mut(&mut self) -> &mut Structure {
        match self {
            Object::Structure(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn class(&self) -> &Class {
        match self {
            Object::Class(c) => c,
            _ => unreachable!(),
        }
    }

    pub fn class_mut(&mut self) -> &mut Class {
        match self {
            Object::Class(c) => c,
            _ => unreachable!(),
        }
    }

    pub fn vector(&self) -> &Vector {
        match self {
            Object::Vector(v) => v,
            _ => unreachable!(),
        }
    }

    pub fn vector_mut(&mut self) -> &mut Vector {
        match self {
            Object::Vector(v) => v,
            _ => unreachable!(),
        }
    }
}

impl From<Structure> for Object {
    fn from(value: Structure) -> Self {
        Object::Structure(value)
    }
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Object::String(value)
    }
}

impl From<Class> for Object {
    fn from(value: Class) -> Self {
        Object::Class(value)
    }
}

impl From<Vector> for Object {
    fn from(value: Vector) -> Self {
        Object::Vector(value)
    }
}

#[derive(Clone, Debug)]
pub struct Structure {
    pub fields: Vec<VMData>,
}

#[derive(Clone, Debug)]
///This should have a VTable
pub struct Class {
    pub prototype: Option<Box<Class>>,
    pub fields: Vec<VMData>,
}

#[derive(Clone, Debug)]
pub struct Vector {
    pub vec: Vec<VMData>,
    ///To know what's the type of the value in the Vec and ensure everything works correctly
    pub tag: TAG,
}
