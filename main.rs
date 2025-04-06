#[derive(Debug)]
struct LinkList {
    head: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

type Pointer = Option<Box<Node>>;

impl LinkList {
    fn new() -> LinkList {
        LinkList { head: None }
    }

    //Add an item at the beginning of the list
    fn add(&mut self, element: i32) {
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }
    //remove an item from bottom of the list
    fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }
}

fn main() {
    let mut link_list = LinkList::new();
    link_list.add(23);
    link_list.add(653);
    link_list.add(23);
    link_list.add(44);
    link_list.add(786);
    link_list.remove();
    println!("{:?}", link_list);
}
