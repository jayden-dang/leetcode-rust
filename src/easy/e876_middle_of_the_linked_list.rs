#![allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.as_ref();
        let mut slow = head.as_ref();

        while let Some(node) = fast {
            match node.next.as_ref() {
                None => break,
                Some(fast_next) => {
                    fast = fast_next.next.as_ref();
                    slow = slow.unwrap().next.as_ref();
                }
            }
        }
        slow.cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_node() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));

        let middle = Solution::middle_node(head);

        assert_eq!(middle.unwrap().val, 3);
    }

    #[test]
    #[should_panic]
    fn test_middle_node_empty() {
        let head = None;

        let middle = Solution::middle_node(head);

        assert_eq!(middle.unwrap().val, 0);
    }

    #[test]
    #[should_panic]
    fn test_middle_node_two() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));

        let middle = Solution::middle_node(head);

        assert_eq!(middle.unwrap().val, 1);
    }
}
