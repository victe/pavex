use crate::blueprint::constructor::{CloningStrategy, Lifecycle, RegisteredConstructor};
use crate::blueprint::conversions::raw_callable2registered_callable;
use crate::blueprint::reflection::RawCallable;
use crate::blueprint::Blueprint;
use pavex_bp_schema::Callable;

/// A constructor that has been configured but has not yet been registered with a [`Blueprint`].
///
/// # Guide
///
/// Check out the ["Dependency injection"](https://pavex.dev/docs/guide/dependency_injection)
/// section of Pavex's guide for a thorough introduction to dependency injection
/// in Pavex applications.
///
/// # Use cases
///
/// [`Constructor`] is primarily used by kits (e.g. [`ApiKit`](crate::kit::ApiKit))
/// to allow users to customize (or disable!)
/// the bundled constructors **before** registering them with a [`Blueprint`].
#[derive(Clone, Debug)]
pub struct Constructor {
    pub(in crate::blueprint) callable: Callable,
    pub(in crate::blueprint) lifecycle: Lifecycle,
    pub(in crate::blueprint) cloning_strategy: Option<CloningStrategy>,
    pub(in crate::blueprint) error_handler: Option<Callable>,
}

impl Constructor {
    /// Create a new (unregistered) constructor.
    ///
    /// Check out the documentation of [`Blueprint::constructor`] for more details
    /// on constructors.
    #[track_caller]
    pub fn new(callable: RawCallable, lifecycle: Lifecycle) -> Self {
        Self {
            callable: raw_callable2registered_callable(callable),
            lifecycle,
            cloning_strategy: None,
            error_handler: None,
        }
    }

    /// Register an error handler for this constructor.
    ///
    /// Check out the documentation of [`RegisteredConstructor::error_handler`] for more details.
    #[track_caller]
    pub fn error_handler(mut self, error_handler: RawCallable) -> Self {
        self.error_handler = Some(raw_callable2registered_callable(error_handler));
        self
    }

    /// Set the cloning strategy for the output type returned by this constructor.
    ///
    /// Check out the documentation of [`RegisteredConstructor::cloning`] for more details.
    pub fn cloning(mut self, cloning_strategy: CloningStrategy) -> Self {
        self.cloning_strategy = Some(cloning_strategy);
        self
    }

    /// Register this constructor with a [`Blueprint`].
    ///
    /// Check out the documentation of [`Blueprint::constructor`] for more details.
    pub fn register(self, bp: &mut Blueprint) -> RegisteredConstructor {
        bp.register_constructor(self)
    }
}
