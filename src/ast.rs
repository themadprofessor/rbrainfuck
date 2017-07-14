use std::io::Read;

use nom;

use error::*;

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum Node {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Print,
    Read,
    Loop(Vec<Node>)
}

pub fn from_read<T>(mut read: T) -> Result<Vec<Node>> where T: Read {
    let mut buff = Vec::new();
    read.read_to_end(&mut buff).map_err(|err| Error::from(ErrorKind::IoError(err)))?;
    buff = buff.into_iter().filter(is_valid).collect::<Vec<_>>();
    let res = parse(&buff);
    if res.is_incomplete() {
        return Err(Error::from(ErrorKind::UnexpectedEndError(res.unwrap_inc())))
    }

    res.to_result().map_err(|err| Error::from(ErrorKind::NomError(err)))
}

named!(mov_r<Node>, map!(char!('>'), |_| Node::MoveRight));
named!(mov_l<Node>, map!(char!('<'), |_| Node::MoveLeft));
named!(inc<Node>, map!(char!('+'), |_| Node::Increment));
named!(dec<Node>, map!(char!('-'), |_| Node::Decrement));
named!(read<Node>, map!(char!(','), |_| Node::Read));
named!(print<Node>, map!(char!('.'), |_| Node::Print));
named!(lop<Node>, do_parse!(
    char!('[') >>
    data: terminated!(call!(parse), dbg_dmp!(char!(']'))) >>
    (Node::Loop(data))
));

named!(parse_node<Node>, alt!(call!(mov_r) | call!(mov_l) | call!(dec) | call!(inc) | call!(read) | call!(print) | call!(lop)));
//named!(parse<Vec<Node>>, flat_map!(take_while!(is_valid), many0!(call!(parse_node))));
//named!(parse<Vec<Node>>, map!(many0!(flat_map!(take_while!(is_valid), many0!(call!(parse_node)))), flatten));
/*named!(parse<Vec<Node>>, map!(many0!(do_parse!(
    res: flat_map!(is_a!(",.<>[]-+"), many0!(call!(parse_node))) >>
    is_not!(",.<>[]-+") >>
    (res)
)), flatten));*/
named!(parse<Vec<Node>>, many0!(call!(parse_node)));

fn is_valid(c: &u8) -> bool {
    is_one!(c, &b',', &b'.', &b'<', &b'>', &b'[', &b']', &b'-', &b'+')
}

fn flatten<T>(data: Vec<Vec<T>>) -> Vec<T> {
    let mut vec = Vec::new();
    for sub in data {
        vec.extend(sub.into_iter());
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

}