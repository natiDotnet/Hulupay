use crate::application::{RegisterUser, LoginUser};

#[derive(Clone)]
pub struct AuthState {
    pub register_use_case: RegisterUser,
    pub login_use_case: LoginUser,
}
