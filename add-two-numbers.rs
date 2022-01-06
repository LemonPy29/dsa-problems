#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> 
Option<Box<ListNode>> {
    let mut current_sum: i32;
    let mut digit = 0;
    let mut r = 0;

    let mut sum = ListNode::new(0);
    let mut current_node = &mut sum;

    let (mut current_l1, mut current_l2) = (l1, l2);

    loop {
        match (&mut current_l1, &mut current_l2) {
            (Some(node1), Some(node2)) => {
                current_sum = node1.val + node2.val + r;
                current_l1 = node1.next.take();
                current_l2 = node2.next.take();
            },
            (Some(node1), None) => {
                current_sum = node1.val + r;
                current_l1 = node1.next.take();
            },
            (None, Some(node2)) => {
                current_sum = node2.val + r;
                current_l2 = node2.next.take();
            }
            (None, None) => {
                break;
            }
        };

        digit = current_sum % 10;
        r = current_sum / 10;

        current_node.next = Some(Box::new(ListNode::new(digit)));
        current_node = current_node.next.as_mut().unwrap().as_mut();
    }

    if r > 0 {
        current_node.next = Some(Box::new(ListNode::new(r)));
    }

    sum.next
}


fn main() {
    let mut m1 = ListNode::new(4);
    m1.next = Some(Box::new(ListNode::new(5)));
    let mut l1 = ListNode::new(2);
    l1.next = Some(Box::new(m1));

    let mut m2 = ListNode::new(6);
    m2.next = Some(Box::new(ListNode::new(5)));
    let mut l2 = ListNode::new(5);
    l2.next = Some(Box::new(m2));

    let res = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    println!("{:?}", res);
}
