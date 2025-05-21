// #[ic_cdk::pre_upgrade]
// fn pre_upgrade_function() {
//     let monitor_stable_data = canistergeek_ic_rust::monitor::pre_upgrade_stable_data();
//     let logger_stable_data = canistergeek_ic_rust::logger::pre_upgrade_stable_data();
//     ic_cdk::storage::stable_save((monitor_stable_data, logger_stable_data));
// }

// #[ic_cdk::post_upgrade]
// fn post_upgrade_function() {
//     let stable_data: Result<(canistergeek_ic_rust::monitor::PostUpgradeStableData, canistergeek_ic_rust::logger::PostUpgradeStableData), String> = ic_cdk::storage::stable_restore();
//     match stable_data {
//         Ok((monitor_stable_data, logger_stable_data)) => {
//             canistergeek_ic_rust::monitor::post_upgrade_stable_data(monitor_stable_data);
//             canistergeek_ic_rust::logger::post_upgrade_stable_data(logger_stable_data);
//         }
//         Err(_) => {}
//     }
// }

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
