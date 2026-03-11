pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

pub use api::{AuthState, router};
pub use application::{
    LoginRequest, LoginResponse, LoginUser, RegisterUser, RegisterUserRequest,
    RegisterUserResponse, TokenService, UserContext,
};
pub use domain::{AuthError as DomainAuthError, Role, User};
pub use infrastructure::{JwtTokenService, PgUserRepository};
