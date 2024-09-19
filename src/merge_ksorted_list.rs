
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn solution(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>>
{
    let mut vector: Vec<i32> = Vec::new();
    for v in lists
    {
        if let Some(v) = v
        {
            recur(v, &mut vector);
        }
    }
    if vector.len() == 0
    {
        return None;
    }
    vector.sort();
    let mut ln = ListNode::new(vector[0]);
    for v in vector[1..].iter()
    {
        rec_insert(&mut ln, v);

    }
    Some(Box::new(ln))
}

fn rec_insert(ln: &mut ListNode, val: &i32)
{
    if ln.next.is_some()
    {
        rec_insert(ln.next.as_mut().unwrap(), val);
    }
    else 
    {
        ln.next = Some(Box::new(ListNode::new(*val))); 
    }
}

fn recur(ln: Box<ListNode>, vector: &mut Vec<i32>)
{
    vector.push(ln.val);
    if let Some(nxt) = ln.next
    {
        recur(nxt, vector);
    }
}
#[cfg(test)]
mod tests
{
    use crate::merge_ksorted_list::ListNode;

    #[test]
    fn test_1()
    {
        let mut node_1 = ListNode::new(1);
        let mut node_2 = ListNode::new(4);
        let node_3 = ListNode::new(5);
        node_2.next = Some(Box::new(node_3));
        node_1.next = Some(Box::new(node_2));
        println!("{:?}", &node_1);

        let mut  node_1_1 = ListNode::new(1);
        let mut  node_2_1 = ListNode::new(3);
        let node_3_1 = ListNode::new(4);
        node_2_1.next = Some(Box::new(node_3_1));
        node_1_1.next = Some(Box::new(node_2_1));
        println!("{:?}", &node_1_1);

        let mut node_1_2 = ListNode::new(2);
        let node_2_2 = ListNode::new(6);
        node_1_2.next = Some(Box::new(node_2_2));
        println!("{:?}", &node_1_2);



        println!("{:?}", super::solution(vec![Some(Box::new(node_1)), Some(Box::new(node_1_1)), Some(Box::new(node_1_2))]));
    }
}