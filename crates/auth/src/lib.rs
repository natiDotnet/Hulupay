pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod api;

pub use domain::{User, Role, AuthError as DomainAuthError};
pub use application::{RegisterUser, LoginUser, Claims, LoginRequest, RegisterUserRequest, RegisterUserResponse, LoginResponse, TokenService};
pub use infrastructure::{PgUserRepository, JwtTokenService};
pub use api::{router, AuthState};
