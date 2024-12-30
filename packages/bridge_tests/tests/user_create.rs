use bridge_tests::{
    common::{bridge_update, setup},
    siwe::full_login,
    types::{RpcResult, UserDto},
};
use candid::{encode_one, Principal};

#[test]
fn test_user_create_anon_should_fail() {
    let (ic, _, bridge) = setup();
    let response: RpcResult<UserDto> = bridge_update(
        &ic,
        bridge,
        Principal::anonymous(),
        "user_create",
        encode_one(()).unwrap(),
    );
    assert!(response.is_err());
    assert!(matches!(response.unwrap_err().code, 401));
}

//
// #[test]
// fn test_user_create_anon_should_fail() {
//     let (ic, siwe, bridge) = setup();
//     let (_, identity) = full_login(&ic, siwe, catts, None);
//     let response: RpcResult<UserDto> = bridge_update(
//         &ic,
//         bridge,
//         Principal::anonymous(),
//         "user_create",
//         encode_one(()).unwrap(),
//     );
//     assert!(response.is_err());
//     assert!(matches!(response.unwrap_err().code, 401));
//}
