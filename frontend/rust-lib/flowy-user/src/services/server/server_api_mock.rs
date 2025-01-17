use crate::{
    entities::{SignInParams, SignInResponse, SignUpParams, SignUpResponse, UpdateUserParams, UserProfile},
    errors::UserError,
};

use crate::services::server::UserServerAPI;
use lib_infra::{future::ResultFuture, uuid};

pub struct UserServerMock {}

impl UserServerMock {}

impl UserServerAPI for UserServerMock {
    fn sign_up(&self, params: SignUpParams) -> ResultFuture<SignUpResponse, UserError> {
        let uid = uuid();
        ResultFuture::new(async move {
            Ok(SignUpResponse {
                user_id: uid.clone(),
                name: params.name,
                email: params.email,
                token: uid,
            })
        })
    }

    fn sign_in(&self, params: SignInParams) -> ResultFuture<SignInResponse, UserError> {
        let user_id = uuid();
        ResultFuture::new(async {
            Ok(SignInResponse {
                user_id: user_id.clone(),
                name: params.name,
                email: params.email,
                token: user_id,
            })
        })
    }

    fn sign_out(&self, _token: &str) -> ResultFuture<(), UserError> { ResultFuture::new(async { Ok(()) }) }

    fn update_user(&self, _token: &str, _params: UpdateUserParams) -> ResultFuture<(), UserError> {
        ResultFuture::new(async { Ok(()) })
    }

    fn get_user(&self, _token: &str) -> ResultFuture<UserProfile, UserError> {
        ResultFuture::new(async { Ok(UserProfile::default()) })
    }

    fn ws_addr(&self) -> String { "ws://localhost:8000/ws/".to_owned() }
}
