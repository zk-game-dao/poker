use candid::Principal;

pub fn validate_caller(principals: Vec<Principal>) {
    if principals.contains(&ic_cdk::api::msg_caller()) {
    } else {
        ic_cdk::trap("Invalid caller");
    }
}
