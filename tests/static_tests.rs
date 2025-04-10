use linked_list_project::static_list::StaticList;

#[test]
fn test_insert() {
    let mut list = StaticList::<i32, 5>::new();
    assert!(list.insert(10)); // Should insert 10
    assert_eq!(list.get(0), Some(10)); // Check if inserted correctly
}

#[test]
fn test_insert_at_index() {
    let mut list = StaticList::<i32, 5>::new();
    list.insert(10);
    list.insert(20);
    list.insert_at_index(1, 15); // Insert 15 at index 1
    assert_eq!(list.get(1), Some(15)); // Verify insertion at index 1
}

#[test]
fn test_delete_element() {
    let mut list = StaticList::<i32, 5>::new();
    list.insert(10);
    list.insert(20);
    assert!(list.delete_element(10)); // Should delete 10
    assert_eq!(list.get(0), Some(20)); // Verify the list after deletion
}

#[test]
fn test_delete_at_index() {
    let mut list = StaticList::<i32, 5>::new();
    list.insert(10);
    list.insert(20);
    assert!(list.delete_at_index(0)); // Delete first element
    assert_eq!(list.get(0), Some(20)); // Verify the remaining element
}

#[test]
fn test_update_element() {
    let mut list = StaticList::<i32, 5>::new();
    list.insert(10);
    list.insert(20);
    assert!(list.update_element(10, 30)); // Update 10 to 30
    assert_eq!(list.get(0), Some(30)); // Verify the update
}

#[test]
fn test_find() {
    let mut list = StaticList::<i32, 5>::new();
    list.insert(10);
    assert!(list.find(10)); // Should find 10
    assert!(!list.find(20)); // Should not find 20
}
