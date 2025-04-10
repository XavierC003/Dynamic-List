use linked_list_project::static_list::StaticList;

#[test]
fn test_static_insert() {
    let mut list = StaticList::new();
    list.insert(5);
    list.insert(10);
    assert_eq!(list.get(0), Some(5));
    assert_eq!(list.get(1), Some(10));
}
