use sb_sbity::project::Project;

#[test]
pub fn test_boolean_on_list() {
    let file = include_str!("./test_case/boolean_on_list.json");
    let project = serde_json::from_str::<Project>(file).unwrap();
    let json = serde_json::to_string(&project).unwrap();
    let _ = serde_json::from_str::<Project>(&json).unwrap();
}
