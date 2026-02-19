use self::LinkedList::*;
use im::list::*;

#[derive(Debug, PartialEq)]
pub enum LinkedList<T>{
    Tail,
    Head(List<T>),
}

impl<T> LinkedList<T>{
    // Add your code here.
    pub fn empty() -> Self {
        Tail
    }

    pub fn new(t: T) -> Self {
        Head(cons(t, List::new()))
    }

    pub fn push(self, t: T) -> Self {
        match self {
            Tail => Head(cons(t, List::new())),
            Head(list) => Head(cons(t, list)),
        }
    }

    pub fn push_back(&mut self, t: T) {
        match self {
            Tail => {
                *self = Head(cons(t, List::new()));
            },
            Head(ref mut list) => {
                *list = list.clone().push_back(t);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let mut l = LinkedList::new(3);
        l = l.push(4);
        assert_eq!(l,Head(cons(4, cons(3, List::new()))));
        l.push_back(2);   
        println!("{:?}", l);

        assert_eq!(l,Head(cons(4, cons(3, cons(2, List::new())))));
    }
}

fn main(){
}   