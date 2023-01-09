pub struct Class {
    account_id: i32,
    apply_assigment_group_weights: bool,
    blueprint: bool,
    calendar: Vec<serde_json::Value>, 
    course_code: String,
    //TODO: Change thsi
    course_color: None,
    created_at: String,
    default_view: String,
    end_at: None,
    enrollment_term_id: i32,
    enrollments: Vec<serde_json::Value>,
    friendly_name: None,
    grade_passback_setting: None 
    
}
