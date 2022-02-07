#[derive(Debug)]
struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl <T> LinkedList<T>{
    fn new() -> Self {
        LinkedList(None)
    }

    fn push_front(&mut self, data:T){
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    fn push_back(&mut self, data:T){
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;
    #[test]
    fn test0(){
        let mut ll = LinkedList::new();
        ll.push_front(3);
        ll.push_back(12);
        ll.push_front(1);
        println!("ll = {:?}", ll);
    }
}