#[ic_cdk::query(name = "getCanistergeekInformation")]
pub async fn get_canistergeek_information(
    request: canistergeek_ic_rust::api_type::GetInformationRequest,
) -> canistergeek_ic_rust::api_type::GetInformationResponse<'static> {
    validate_caller();
    canistergeek_ic_rust::get_information(request)
}

#[ic_cdk::update(name = "updateCanistergeekInformation")]
pub async fn update_canistergeek_information(
    request: canistergeek_ic_rust::api_type::UpdateInformationRequest,
) {
    validate_caller();
    canistergeek_ic_rust::update_information(request);
}

fn validate_caller() {
    match candid::Principal::from_text(
        "vghom-n6t3c-2bgm5-qvkep-lqjvf-s6znf-xq2yt-ebpew-tgytx-hhbzl-5qe",
    ) {
        Ok(caller) if caller == ic_cdk::caller() => (),
        _ => ic_cdk::trap("Invalid caller"),
    }
}
