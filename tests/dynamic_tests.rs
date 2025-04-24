use linked_list_project::dynamic_list::DynamicList;

/// Tests inserting a single element into the dynamic linked list.
#[test]
fn test_insert() {
    let mut list = DynamicList::new();
    list.insert(10);
    assert_eq!(list.get(0), Some(10)); // Check if the element was inserted
}

/// Tests inserting an element at a specific index in the dynamic linked list.
#[test]
fn test_insert_at_index() {
    let mut list = DynamicList::new();
    list.insert(10);
    list.insert(20);
    list.insert_at_index(1, 15); // Insert 15 at index 1
    assert_eq!(list.get(1), Some(15)); // Verify insertion at index 1
}


/// Tests deletion of the first occurrence of a specified element.
#[test]
fn test_delete_element() {
    let mut list = DynamicList::new();
    list.insert(10);
    list.insert(20);
    assert!(list.delete_element(10)); // Should delete 10
    assert_eq!(list.get(0), Some(20)); // Verify deletion
}


/// Tests deletion of the element at a specific index.
#[test]
fn test_delete_at_index() {
    let mut list = DynamicList::new();
    list.insert(10);
    list.insert(20);
    assert!(list.delete_at_index(0)); // Should delete element at index 0
    assert_eq!(list.get(0), Some(20)); // Verify the remaining element
}

/// Tests updating the first occurrence of a specified element.

#[test]
fn test_update_element() {
    let mut list = DynamicList::new();
    list.insert(10);
    list.insert(20);
    assert!(list.update_element(10, 30)); // Should update 10 to 30
    assert_eq!(list.get(0), Some(30)); // Verify the update
}

/// Tests finding an element in the dynamic linked list.

#[test]
fn test_find() {
    let mut list = DynamicList::new();
    list.insert(10);
    assert!(list.find(10)); // Should find 10 in the list
    assert!(!list.find(20)); // Should not find 20 in the list
}
