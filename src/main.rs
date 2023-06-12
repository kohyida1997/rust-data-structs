mod my_linked_lists;
use my_linked_lists::int32_linked_list::Int32LinkedList;

fn i32_linked_list_test1() {
    let mut ll = Int32LinkedList::new();

    // Assert that it is initially empty
    assert!(ll.empty());

    // Add one element to back
    ll.add_to_back(0);
    assert_eq!(ll.get_head().unwrap(), 0);
    assert_eq!(ll.get_head().unwrap(), 0);
    assert_eq!(ll.size(), 1 as usize);
    assert_eq!(ll.size(), 1 as usize);

    ll.add_to_back(1);
    assert_eq!(ll.get_head().unwrap(), 0);
    assert_eq!(ll.get_tail().unwrap(), 1);
    assert_eq!(ll.size(), 2 as usize);

    ll.add_to_back(3);
    assert_eq!(ll.get_head().unwrap(), 0);
    assert_eq!(ll.get_tail().unwrap(), 3);
    assert_eq!(ll.size(), 3 as usize);

    ll.add_to_back(2);
    assert_eq!(ll.get_head().unwrap(), 0);
    assert_eq!(ll.get_tail().unwrap(), 2);
    assert_eq!(ll.size(), 4 as usize);
    println!("Final state of [ll]: {}", ll);

    let ll2 = make_i32_ll![9, 8, 7, 6, 5, 4, 3, 2, 1, 9999];
    assert_eq!(ll2.size(), 10);
    assert_eq!(ll2.get_head().unwrap(), 9);
    assert_eq!(ll2.get_tail().unwrap(), 9999);
    println!("Final state of [ll2]: {}", ll2);

    let ll3 = Int32LinkedList::new();
    assert!(ll3.empty());
    println!("Final state of [ll3]: {}", ll3);
    println!("Finished test ===== {}", "i32_linked_list_test1");
}

fn main() {
    i32_linked_list_test1();
}
