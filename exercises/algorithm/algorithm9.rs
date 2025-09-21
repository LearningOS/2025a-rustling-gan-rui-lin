/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T:PartialOrd> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        // 创建一个带有占位符的向量（索引 0 不使用）
        Self {
            count: 0,
            items: vec![T::default()], // 索引 0 是占位符，不参与堆操作
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        
        // 执行上浮操作，维护堆的性质
        let mut curr_idx = self.count;
        
        while curr_idx > 1 {
            let parent_idx = self.parent_idx(curr_idx);
            
            // 如果当前节点满足堆的比较条件（对小根堆是较小，对大根堆是较大）
            if (self.comparator)(&self.items[curr_idx], &self.items[parent_idx]) {
                self.items.swap(curr_idx, parent_idx);
                curr_idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // 如果只有左子节点，返回左子节点
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        if right_idx > self.count {
            return left_idx;
        }
        
        // 使用 comparator 函数比较左右子节点，返回满足堆性质的最佳子节点
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T:PartialOrd> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        
        // 保存堆顶元素
        let top = std::mem::replace(&mut self.items[1], T::default());
        
        // 如果这是最后一个元素，直接返回
        if self.count == 1 {
            self.count = 0;
            self.items.pop(); // 移除最后一个元素（原堆顶）
            return Some(top);
        }
        
        // 将最后一个元素移到堆顶，并从数组中移除
        let last = self.items.pop().unwrap();
        self.items[1] = last;
        self.count -= 1;
        
        // 向下调整堆
        let mut current = 1;
        while self.children_present(current) {
            let smallest_child = self.smallest_child_idx(current);
            
            if (self.comparator)(&self.items[smallest_child], &self.items[current]) {
                self.items.swap(smallest_child, current);
                current = smallest_child;
            } else {
                break;
            }
        }
        
        Some(top)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}