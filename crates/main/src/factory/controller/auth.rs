use crate::factory;
use presentation::{DynController, SignInController, SignUpController};
use std::sync::Arc;

pub fn signin() -> DynController {
    let auth = factory::usecase::customer::auth();
    let controller = SignInController { auth };
    return Arc::new(controller);
}

pub fn signup() -> DynController {
    let auth = factory::usecase::customer::auth();
    let create = factory::usecase::customer::create();
    let controller = SignUpController { auth, create };
    return Arc::new(controller);
}
