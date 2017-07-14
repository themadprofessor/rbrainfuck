use std::io;
use std::io::Read;
use std::iter;
use std::iter::FromIterator;

use ast::Node;
use error::*;

pub struct Machine {
    ptr: usize,
    data: Vec<u8>
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            ptr: 0,
            data: Vec::from_iter(iter::once(0))
        }
    }

    pub fn with_capacity(size: usize) -> Machine {
        Machine {
            ptr: 0,
            data: Vec::from_iter(iter::repeat(0).take(size))
        }
    }

    pub fn exec_ast(&mut self, ast: &[Node]) -> Result<()> {
        for node in ast.iter() {
            self.exec(node)?;
        }
        Ok(())
    }

    pub fn exec(&mut self, node: &Node) -> Result<()> {
        match *node {
            Node::MoveRight => {
                self.ptr = self.ptr.saturating_add(1);
                self.ensure_size();
            },
            Node::MoveLeft => {
                self.ptr = self.ptr.saturating_sub(1);
                self.ensure_size();
            },
            Node::Increment => {
                self.data[self.ptr] = self.data[self.ptr].wrapping_add(1)
            },
            Node::Decrement => {
                self.data[self.ptr] = self.data[self.ptr].wrapping_sub(1)
            },
            Node::Print => print!("{}", char::from(self.data[self.ptr])),
//            Node::Print => println!("{}", self.data[self.ptr]),
            Node::Read => {
                let mut buff = [0];
                io::stdin().read(&mut buff).map_err(|err| Error::from(ErrorKind::IoError(err)))?;
                self.data[self.ptr] = buff[0];
            },
            Node::Loop(ref internal) => {
                while self.data[self.ptr] != 0 {
                    self.exec_ast(internal)?;
                }
            }
        }
        Ok(())
    }

    fn ensure_size(&mut self) {
        if self.data.len() <= self.ptr {
            let offset = self.ptr - self.data.len() + 1;
            self.data.extend(::std::iter::repeat(0).take(offset))
        }
    }
}

impl Default for Machine {
    fn default() -> Machine {
        Machine::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(ast: &[Node]) -> Result<Machine> {
        let mut machine = Machine::new();
        machine.exec_ast(ast).map(|_| machine)
    }

    #[test]
    fn empty() {
        let res = setup(&vec![]);
        assert!(res.is_ok());
        let machine = res.unwrap();
        assert_eq!(machine.data, vec![0]);
        assert_eq!(machine.ptr, 0);
    }

    #[test]
    fn move_right() {
        let res = setup(&vec![Node::MoveRight]);
        assert!(res.is_ok());
        let machine = res.unwrap();
        assert_eq!(machine.ptr, 1);
        assert_eq!(machine.data, vec![0, 0]);
    }

    #[test]
    fn move_left() {
        let res = setup(&vec![Node::MoveLeft]);
        assert!(res.is_ok());
        let machine = res.unwrap();
        assert_eq!(machine.ptr, 0);
        assert_eq!(machine.data, vec![0]);
    }
}