use linked_list_project::dynamic_list::DynamicList;

#[test]
fn test_insert() {
    let mut list = DynamicList::new();
    list.insert(1);
    list.insert(2);
    assert_eq!(list.get(0), Some(1));
    assert_eq!(list.get(1), Some(2));
}

#[test]
fn test_delete_element() {
    let mut list = DynamicList::new();
    list.insert(1);
    list.insert(2);
    assert!(list.delete_element(1));
    assert_eq!(list.get(0), Some(2));
    assert!(!list.delete_element(3));
}