#[derive(Debug, PartialEq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }
        
        let mut temp_head = ListNode::new(-1); // Временный узел
        let mut temp_head_cursor = &mut temp_head; // Указатель на текущий узел

        let mut addition = 0; // Перенос
        let mut first_cursor = l1; // Курсор для первого списка
        let mut second_cursor = l2; // Курсор для второго списка

        while first_cursor.is_some() || second_cursor.is_some() {
            // Получаем значение узлов или 0, если узел не существует
            let first_val = first_cursor.as_ref().map_or(0, |node| node.val);
            let second_val = second_cursor.as_ref().map_or(0, |node| node.val);
            let res = first_val + second_val + addition;

            addition = res / 10; // Определяем перенос
            let new_node = ListNode::new(res % 10); // Создаем новый узел с остатком

            // Присоединяем новый узел к результату
            temp_head_cursor.next = Some(Box::new(new_node));
            temp_head_cursor = temp_head_cursor.next.as_mut().unwrap(); // Сдвигаем указатель

            // Переходим к следующим узлам в l1 и l2
            if let Some(node) = first_cursor {
                first_cursor = node.next;
            } else {
                first_cursor = None;
            }
            if let Some(node) = second_cursor {
                second_cursor = node.next;
            } else {
                second_cursor = None;
            }
        }

        // Если остался перенос, добавляем его узлом
        if addition != 0 {
            temp_head_cursor.next = Some(Box::new(ListNode::new(addition)));
        }

        // Возвращаем результат, начиная с узла после фиктивного
        temp_head.next
    }
}

#[cfg(test)]
mod tests
{
    use super::{Solution, ListNode};

    #[test]
    fn main() {
        // Создаем первый связанный список: 2 -> 4 -> 3 (342)
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));
    
        // Создаем второй связанный список: 5 -> 6 -> 4 (465)
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
    
        let result = Solution::add_two_numbers(l1, l2);
        
        let mut current = result;
        while current.is_some() {
            let node = current.as_ref().unwrap();
            print!("{} ", node.val);
            current = node.next.clone();
        }
        // Вывод: 7 0 8 (807)
    }
}
// Пример использования
